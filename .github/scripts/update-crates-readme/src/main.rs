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
const GITHUB_API_BASE: &str = "https://api.github.com";
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
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    repository: Option<String>,
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

#[derive(Debug, Clone, Deserialize)]
struct RepoItem {
    name: String,
    html_url: String,
    #[serde(default)]
    stargazers_count: u64,
    #[serde(default)]
    pushed_at: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    language: Option<String>,
    #[serde(default)]
    topics: Vec<String>,
    #[serde(default)]
    fork: bool,
}

fn http_client() -> Result<Client, String> {
    Client::builder()
        .build()
        .map_err(|e| format!("falha ao criar cliente HTTP: {e}"))
}

fn with_headers(
    request: RequestBuilder,
    crates_token: Option<&str>,
    github_token: Option<&str>,
    for_github: bool,
) -> RequestBuilder {
    let mut req = request
        .header(USER_AGENT, "mchael158-readme-crates-rust")
        .header(ACCEPT, "application/json");

    if for_github {
        if let Some(token) = github_token {
            req = req.header(AUTHORIZATION, format!("Bearer {token}"));
        }
    } else if let Some(token) = crates_token {
        // crates.io espera o token cru no Authorization (sem Bearer/Token).
        req = req.header(AUTHORIZATION, token);
    }

    req
}

fn fetch_json<T: for<'de> Deserialize<'de>>(
    client: &Client,
    url: &str,
    crates_token: Option<&str>,
    github_token: Option<&str>,
    for_github: bool,
) -> Result<T, String> {
    with_headers(client.get(url), crates_token, github_token, for_github)
        .send()
        .and_then(|resp| resp.error_for_status())
        .map_err(|e| format!("erro HTTP em {url}: {e}"))?
        .json::<T>()
        .map_err(|e| format!("falha ao parsear JSON de {url}: {e}"))
}

fn fetch_me(client: &Client, token: &str) -> Result<MeUser, String> {
    let url = format!("{CRATES_API_BASE}/me");
    let data: MeResponse = fetch_json(client, &url, Some(token), None, false)?;
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
        let data: CratesResponse = fetch_json(client, &url, token, None, false)?;
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

fn fetch_rust_repositories(
    client: &Client,
    username: &str,
    github_token: Option<&str>,
) -> Result<Vec<RepoItem>, String> {
    let url = format!(
        "{GITHUB_API_BASE}/users/{username}/repos?per_page=100&type=owner&sort=updated"
    );
    let repos: Vec<RepoItem> = fetch_json(client, &url, None, github_token, true)?;
    let mut rust_repos: Vec<RepoItem> = repos
        .into_iter()
        .filter(|r| !r.fork)
        .filter(|r| {
            r.language.as_deref() == Some("Rust")
                || r.topics.iter().any(|t| t.eq_ignore_ascii_case("rust"))
        })
        .collect();
    rust_repos.sort_by_key(|r| Reverse(r.stargazers_count));
    rust_repos.truncate(10);
    Ok(rust_repos)
}

fn shield_label(value: &str) -> String {
    value
        .replace('-', "--")
        .replace('_', "__")
        .replace(' ', "%20")
}

fn truncate_desc(value: &str, max: usize) -> String {
    let clean = value.replace('|', "\\|").replace('\n', " ");
    if clean.chars().count() <= max {
        return clean;
    }
    let mut out: String = clean.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

fn render_crates_block(crates: &[CrateItem]) -> Vec<String> {
    if crates.is_empty() {
        return vec!["<sub>no published crates found</sub>".to_string()];
    }

    let mut lines = vec![
        format!(
            "<img src=\"https://img.shields.io/badge/🦀_published-{count}-f74c00?style=for-the-badge&labelColor=0b0d10&color=f74c00\" alt=\"published crates\" />",
            count = crates.len()
        ),
        String::new(),
    ];

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
        let label = shield_label(name);
        let version_label = shield_label(&version);
        let downloads = format_downloads(crate_item.downloads);

        lines.push(format!("**🦀 [{name}]({crate_url})**"));
        lines.push(format!(
            "<a href=\"{crate_url}\"><img src=\"https://img.shields.io/badge/{label}-{version_label}-f74c00?style=flat-square&logo=rust&logoColor=white\" alt=\"{name}\" /></a> \
<a href=\"{docs_url}\"><img src=\"https://img.shields.io/badge/docs-DEA584?style=flat-square&logo=readthedocs&logoColor=0b0d10\" alt=\"docs\" /></a> \
<img src=\"https://img.shields.io/badge/⬇_{downloads}-0DB7ED?style=flat-square&labelColor=0b0d10\" alt=\"downloads\" />"
        ));

        if let Some(repo) = crate_item.repository.as_deref() {
            lines.push(format!(
                "<a href=\"{repo}\"><img src=\"https://img.shields.io/badge/source-2088FF?style=flat-square&logo=github&logoColor=white\" alt=\"source\" /></a>"
            ));
        }

        if let Some(desc) = crate_item
            .description
            .as_deref()
            .map(str::trim)
            .filter(|d| !d.is_empty())
        {
            lines.push(format!("<sub>{}</sub>", truncate_desc(desc, 110)));
        }

        lines.push(String::new());
    }

    lines
}

fn render_repos_block(username: &str, repos: &[RepoItem]) -> Vec<String> {
    let mut lines = vec![format!(
        "<img src=\"https://img.shields.io/badge/🦀_repos-{count}-DEA584?style=for-the-badge&labelColor=0b0d10&color=DEA584\" alt=\"rust repos\" />",
        count = repos.len()
    )];

    if repos.is_empty() {
        lines.push(format!(
            "- no public rust repositories found for [{0}](https://github.com/{0})",
            username
        ));
        return lines;
    }

    lines.push(String::new());
    lines.push("<p>".to_string());

    for repo in repos {
        let updated = repo.pushed_at.chars().take(10).collect::<String>();
        let updated_label = shield_label(&updated);
        let name_label = shield_label(&repo.name);
        let lang = repo.language.as_deref().unwrap_or("Rust");
        let lang_label = shield_label(lang);
        let stars = repo.stargazers_count;

        lines.push(format!(
            "  <a href=\"{url}\"><img src=\"https://img.shields.io/badge/🦀_{name_label}-{lang_label}-f74c00?style=flat-square&labelColor=0b0d10&logo=github&logoColor=white\" alt=\"{name}\" /></a> \
<img src=\"https://img.shields.io/badge/★_{stars}-1a1a1a?style=flat-square\" alt=\"stars\" /> \
<img src=\"https://img.shields.io/badge/{updated_label}-2088FF?style=flat-square&labelColor=0b0d10\" alt=\"updated\" />",
            url = repo.html_url,
            name = repo.name
        ));

        if let Some(desc) = repo
            .description
            .as_deref()
            .map(str::trim)
            .filter(|d| !d.is_empty())
        {
            lines.push(format!("  <br/><sub>{}</sub><br/>", truncate_desc(desc, 100)));
        } else {
            lines.push("  <br/>".to_string());
        }
    }

    lines.push("</p>".to_string());
    lines
}

fn render_section(login: &str, crates: &[CrateItem], repos: &[RepoItem]) -> String {
    let now = Utc::now().format("%Y-%m-%d %H:%M UTC").to_string();
    let profile = format!("https://crates.io/users/{login}");

    let mut lines = vec![
        format!(
            "<a href=\"{profile}\"><img src=\"https://img.shields.io/badge/🦀_crates.io-{login}-f74c00?style=for-the-badge&logo=rust&logoColor=white&labelColor=0b0d10\" alt=\"crates.io\" /></a>"
        ),
        String::new(),
    ];
    lines.extend(render_crates_block(crates));
    lines.extend(render_repos_block(login, repos));
    lines.push(String::new());
    lines.push(format!("<sub>🦀 auto-synced · `{now}`</sub>"));
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
    let github_token = env::var("GITHUB_TOKEN").ok().filter(|v| !v.is_empty());
    let client = http_client()?;

    let (user_id, login) = resolve_identity(&client, token.as_deref())?;
    let crates = fetch_all_crates(&client, user_id, token.as_deref())?;
    let repos = fetch_rust_repositories(&client, &login, github_token.as_deref())?;
    let section = render_section(&login, &crates, &repos);

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
