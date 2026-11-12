use chrono::Utc;
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use std::cmp::Reverse;
use std::env;
use std::fs;

const README_PATH: &str = "README.md";
const START_MARKER: &str = "<!-- CRATES-START -->";
const END_MARKER: &str = "<!-- CRATES-END -->";
const CRATES_API_BASE: &str = "https://crates.io/api/v1";
const DEFAULT_USERNAME: &str = "mchael158";
const DEFAULT_CRATES_USER_ID: u64 = 407_044;

#[derive(Debug, Deserialize)]
struct CratesResponse {
    crates: Vec<CrateItem>,
    #[serde(default)]
    meta: CratesMeta,
}

#[derive(Debug, Default, Deserialize)]
struct CratesMeta {
    #[serde(default)]
    total: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct CrateItem {
    id: String,
    #[serde(default)]
    max_version: String,
    #[serde(default)]
    downloads: u64,
    #[serde(default)]
    documentation: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MeResponse {
    user: MeUser,
}

#[derive(Debug, Deserialize)]
struct MeUser {
    id: u64,
    login: String,
}

fn http_client() -> Result<Client, String> {
    Client::builder()
        .build()
        .map_err(|e| format!("falha ao criar cliente HTTP: {e}"))
}

fn with_headers(request: RequestBuilder, token: Option<&str>) -> RequestBuilder {
    let mut req = request
        .header(USER_AGENT, "mchael158-readme-crates-rust")
        .header(ACCEPT, "application/json");

    if let Some(token) = token {
        // crates.io espera o token cru no Authorization (sem Bearer/Token).
        req = req.header(AUTHORIZATION, token);
    }

    req
}

fn fetch_json<T: for<'de> Deserialize<'de>>(
    client: &Client,
    url: &str,
    token: Option<&str>,
) -> Result<T, String> {
    with_headers(client.get(url), token)
        .send()
        .and_then(|resp| resp.error_for_status())
        .map_err(|e| format!("erro HTTP em {url}: {e}"))?
        .json::<T>()
        .map_err(|e| format!("falha ao parsear JSON de {url}: {e}"))
}

fn fetch_me(client: &Client, token: &str) -> Result<MeUser, String> {
    let url = format!("{CRATES_API_BASE}/me");
    let data: MeResponse = fetch_json(client, &url, Some(token))?;
    Ok(data.user)
}

fn fetch_all_crates(
    client: &Client,
    user_id: u64,
    token: Option<&str>,
) -> Result<Vec<CrateItem>, String> {
    let mut crates: Vec<CrateItem> = Vec::new();
    let mut page = 1usize;
    let per_page = 100usize;

    loop {
        let url = format!(
            "{CRATES_API_BASE}/crates?user_id={user_id}&page={page}&per_page={per_page}"
        );
        let data: CratesResponse = fetch_json(client, &url, token)?;
        let page_items = data.crates;
        let total = data.meta.total.max(crates.len());
        crates.extend(page_items.clone());
        if crates.len() >= total || page_items.is_empty() {
            break;
        }
        page += 1;
    }

    crates.sort_by_key(|c| Reverse(c.downloads));
    Ok(crates)
}

fn render_section(login: &str, crates: &[CrateItem]) -> String {
    let now = Utc::now().format("%Y-%m-%d %H:%M UTC").to_string();
    let profile = format!("https://crates.io/users/{login}");

    let mut lines = vec![
        format!(
            "<a href=\"{profile}\"><img src=\"https://img.shields.io/badge/crates.io-{login}-f74c00?style=flat-square&logo=rust&logoColor=white\" alt=\"crates.io\" /></a>"
        ),
        String::new(),
    ];

    if crates.is_empty() {
        lines.push("No published crates found.".to_string());
    } else {
        lines.extend([
            "| Crate | Version | Downloads | Docs |".to_string(),
            "|---|---|---:|---|".to_string(),
        ]);

        for crate_item in crates.iter().take(10) {
            let name = &crate_item.id;
            let crate_url = format!("https://crates.io/crates/{name}");
            let docs_url = crate_item
                .documentation
                .clone()
                .unwrap_or_else(|| format!("https://docs.rs/{name}"));
            let version = if crate_item.max_version.is_empty() {
                "-".to_string()
            } else {
                crate_item.max_version.clone()
            };

            lines.push(format!(
                "| [{name}]({crate_url}) | `{version}` | {} | [docs]({docs_url}) |",
                format_downloads(crate_item.downloads)
            ));
        }
    }

    lines.push(String::new());
    lines.push(format!("<sub>Updated `{now}`</sub>"));
    lines.join("\n")
}

fn update_readme(content: &str, section: &str) -> Result<String, String> {
    let start_pos = content
        .find(START_MARKER)
        .ok_or_else(|| "marcador de inicio nao encontrado no README".to_string())?;
    let end_pos = content
        .find(END_MARKER)
        .ok_or_else(|| "marcador de fim nao encontrado no README".to_string())?;
    if end_pos <= start_pos {
        return Err("ordem invalida dos marcadores no README".to_string());
    }
    let start_idx = start_pos + START_MARKER.len();
    Ok(format!(
        "{}\n{}\n{}",
        &content[..start_idx],
        section,
        &content[end_pos..]
    ))
}

fn format_downloads(value: u64) -> String {
    let mut chars: Vec<char> = value.to_string().chars().rev().collect();
    let mut out = String::new();
    for (i, ch) in chars.drain(..).enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push('.');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}

fn resolve_identity(
    client: &Client,
    token: Option<&str>,
) -> Result<(u64, String), String> {
    let fallback_login = env::var("CRATES_IO_USERNAME")
        .or_else(|_| env::var("GITHUB_REPOSITORY_OWNER"))
        .unwrap_or_else(|_| DEFAULT_USERNAME.to_string());

    if let Ok(raw_id) = env::var("CRATES_IO_USER_ID") {
        let parsed = raw_id
            .parse::<u64>()
            .map_err(|e| format!("CRATES_IO_USER_ID invalido: {e}"))?;
        return Ok((parsed, fallback_login));
    }

    if let Some(tok) = token {
        let me = fetch_me(client, tok)?;
        return Ok((me.id, me.login));
    }

    Ok((DEFAULT_CRATES_USER_ID, fallback_login))
}

fn run() -> Result<(), String> {
    let token = env::var("CRATES_IO_TOKEN").ok().filter(|v| !v.is_empty());
    let client = http_client()?;

    let (user_id, login) = resolve_identity(&client, token.as_deref())?;
    let crates = fetch_all_crates(&client, user_id, token.as_deref())?;
    let section = render_section(&login, &crates);

    let readme = fs::read_to_string(README_PATH)
        .map_err(|e| format!("falha ao ler {README_PATH}: {e}"))?;
    let updated = update_readme(&readme, &section)?;
    fs::write(README_PATH, updated).map_err(|e| format!("falha ao escrever {README_PATH}: {e}"))?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("erro: {err}");
        std::process::exit(1);
    }
}
