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

fn with_default_headers(request: RequestBuilder, token: Option<&str>) -> RequestBuilder {
    let req = request
        .header(USER_AGENT, "mchael158-readme-crates-rust")
        .header(ACCEPT, "application/json");
    if let Some(value) = token {
        req.header(AUTHORIZATION, format!("Token {value}"))
    } else {
        req
    }
}

fn fetch_json<T: for<'de> Deserialize<'de>>(
    client: &Client,
    url: &str,
    token: Option<&str>,
) -> Result<T, String> {
    with_default_headers(client.get(url), token)
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

fn fetch_all_crates(client: &Client, user_id: u64, token: Option<&str>) -> Result<Vec<CrateItem>, String> {
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

fn render_crates_icons(crates: &[CrateItem]) -> Vec<String> {
    if crates.is_empty() {
        return vec!["<sub>sem crates publicadas encontradas</sub>".to_string()];
    }

    let mut lines = vec!["<p align=\"left\">".to_string()];
    for crate_item in crates.iter().take(8) {
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
            "<a href=\"{crate_url}\"><img src=\"https://img.shields.io/badge/{name}-{version}-f74c00?style=flat-square&logo=rust&logoColor=white\" /></a>\
 <a href=\"{docs_url}\"><img src=\"https://img.shields.io/badge/-docs-2f80ed?style=flat-square&logo=readthedocs&logoColor=white\" /></a>\
 <img src=\"https://img.shields.io/badge/-{downloads}-222?style=flat-square&logo=download&logoColor=white\" />",
            downloads = format_downloads(crate_item.downloads)
        ));
    }
    lines.push("</p>".to_string());
    lines
}

fn render_section(crates: &[CrateItem], profile_login: &str) -> String {
    let now = Utc::now().format("%Y-%m-%d %H:%M UTC").to_string();
    let profile = format!("https://crates.io/users/{profile_login}");
    let mut lines = vec![
        format!("<a href=\"{profile}\"><img src=\"https://img.shields.io/badge/-crates.io-151515?style=flat-square&logo=rust&logoColor=white\" /></a>"),
        String::new(),
    ];
    lines.extend(render_crates_icons(crates));
    lines.push(String::new());
    lines.push(format!("<sub>{now}</sub>"));
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

fn run() -> Result<(), String> {
    let fallback_login = env::var("CRATES_IO_USERNAME")
        .or_else(|_| env::var("GITHUB_REPOSITORY_OWNER"))
        .unwrap_or_else(|_| "mchael158".to_string());
    let token = env::var("CRATES_IO_TOKEN").ok();
    let user_id_env = env::var("CRATES_IO_USER_ID").ok();
    let client = http_client()?;

    let (user_id, login) = match (user_id_env, token.as_deref()) {
        (Some(raw_id), _) => {
            let parsed = raw_id
                .parse::<u64>()
                .map_err(|e| format!("CRATES_IO_USER_ID invalido: {e}"))?;
            (parsed, fallback_login.clone())
        }
        (None, Some(tok)) => {
            let me = fetch_me(&client, tok)?;
            (me.id, me.login)
        }
        (None, None) => {
            return Err(
                "defina CRATES_IO_TOKEN (recomendado) ou CRATES_IO_USER_ID para buscar crates"
                    .to_string(),
            )
        }
    };

    let crates = fetch_all_crates(&client, user_id, token.as_deref())?;
    let section = render_section(&crates, &login);

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
