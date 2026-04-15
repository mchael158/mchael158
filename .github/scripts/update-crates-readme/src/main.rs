use chrono::Utc;
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, USER_AGENT};
use serde::Deserialize;
use std::cmp::Reverse;
use std::env;
use std::fs;

const README_PATH: &str = "README.md";
const START_MARKER: &str = "<!-- CRATES-START -->";
const END_MARKER: &str = "<!-- CRATES-END -->";
const CRATES_API_BASE: &str = "https://crates.io/api/v1";
const GITHUB_API_BASE: &str = "https://api.github.com";

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
}

fn http_client() -> Result<Client, String> {
    Client::builder()
        .build()
        .map_err(|e| format!("falha ao criar cliente HTTP: {e}"))
}

fn fetch_json<T: for<'de> Deserialize<'de>>(client: &Client, url: &str) -> Result<T, String> {
    client
        .get(url)
        .header(USER_AGENT, "mchael158-profile-readme-updater-rust")
        .header(ACCEPT, "application/json")
        .send()
        .and_then(|resp| resp.error_for_status())
        .map_err(|e| format!("erro HTTP em {url}: {e}"))?
        .json::<T>()
        .map_err(|e| format!("falha ao parsear JSON de {url}: {e}"))
}

fn fetch_all_crates(client: &Client, user_id: u64) -> Result<Vec<CrateItem>, String> {
    let mut crates: Vec<CrateItem> = Vec::new();
    let mut page = 1usize;
    let per_page = 100usize;

    loop {
        let url = format!(
            "{CRATES_API_BASE}/crates?user_id={user_id}&page={page}&per_page={per_page}"
        );
        let data: CratesResponse = fetch_json(client, &url)?;
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

fn fetch_rust_repositories(client: &Client, username: &str) -> Result<Vec<RepoItem>, String> {
    let url = format!("{GITHUB_API_BASE}/users/{username}/repos?per_page=100&type=owner&sort=updated");
    let repos: Vec<RepoItem> = fetch_json(client, &url)?;
    let mut rust_repos: Vec<RepoItem> = repos
        .into_iter()
        .filter(|r| {
            r.language.as_deref() == Some("Rust")
                || r.topics.iter().any(|t| t.eq_ignore_ascii_case("rust"))
        })
        .collect();
    rust_repos.sort_by_key(|r| Reverse(r.stargazers_count));
    rust_repos.truncate(10);
    Ok(rust_repos)
}

fn render_crates_block(username: &str, crates: &[CrateItem], has_user_id: bool) -> Vec<String> {
    let profile_url = format!("https://crates.io/users/{username}");
    if !has_user_id {
        return vec![
            "- Para listar crates publicadas automaticamente, configure `CRATES_IO_USER_ID` no ambiente.".to_string(),
            format!("- Perfil crates.io (quando existir): [{profile_url}]({profile_url})"),
        ];
    }

    let mut lines = vec![
        format!("- Perfil crates.io: [{profile_url}]({profile_url})"),
        format!("- Total de crates publicas: **{}**", crates.len()),
    ];

    if crates.is_empty() {
        lines.push("- Nenhuma crate publica encontrada para o `CRATES_IO_USER_ID` informado.".to_string());
        return lines;
    }

    lines.extend([
        String::new(),
        "| Crate | Versao | Downloads | Documentacao |".to_string(),
        "|---|---:|---:|---|".to_string(),
    ]);

    for crate_item in crates.iter().take(10) {
        let crate_url = format!("https://crates.io/crates/{}", crate_item.id);
        let docs_url = crate_item
            .documentation
            .clone()
            .unwrap_or_else(|| format!("https://docs.rs/{}", crate_item.id));
        lines.push(format!(
            "| [{}]({}) | `{}` | **{}** | [docs.rs]({}) |",
            crate_item.id,
            crate_url,
            crate_item.max_version,
            format_downloads(crate_item.downloads),
            docs_url
        ));
    }

    lines
}

fn render_repos_block(username: &str, repos: &[RepoItem]) -> Vec<String> {
    let mut lines = vec![format!(
        "- Repositorios Rust detectados no GitHub: **{}**",
        repos.len()
    )];

    if repos.is_empty() {
        lines.push(format!(
            "- Nenhum repositorio Rust publico encontrado para [{0}](https://github.com/{0}).",
            username
        ));
        return lines;
    }

    lines.extend([
        String::new(),
        "| Projeto | Stars | Atualizado | Descricao |".to_string(),
        "|---|---:|---|---|".to_string(),
    ]);

    for repo in repos {
        let updated = repo.pushed_at.chars().take(10).collect::<String>();
        let desc = repo
            .description
            .clone()
            .unwrap_or_else(|| "-".to_string())
            .replace('|', "\\|");
        lines.push(format!(
            "| [{}]({}) | **{}** | `{}` | {} |",
            repo.name, repo.html_url, repo.stargazers_count, updated, desc
        ));
    }

    lines
}

fn render_section(username: &str, crates: &[CrateItem], repos: &[RepoItem], has_user_id: bool) -> String {
    let now = Utc::now().format("%Y-%m-%d %H:%M UTC").to_string();
    let mut lines = vec!["### Crates no crates.io".to_string()];
    lines.extend(render_crates_block(username, crates, has_user_id));
    lines.push(String::new());
    lines.push("### Projetos Rust no GitHub".to_string());
    lines.extend(render_repos_block(username, repos));
    lines.push(String::new());
    lines.push(format!("- Ultima atualizacao automatica: `{now}`"));
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
    let username = env::var("CRATES_IO_USERNAME")
        .or_else(|_| env::var("GITHUB_REPOSITORY_OWNER"))
        .unwrap_or_else(|_| "mchael158".to_string());
    let crates_user_id = env::var("CRATES_IO_USER_ID").ok();
    let has_user_id = crates_user_id.is_some();
    let client = http_client()?;

    let crates = if let Some(raw_id) = crates_user_id {
        let user_id = raw_id
            .parse::<u64>()
            .map_err(|e| format!("CRATES_IO_USER_ID invalido: {e}"))?;
        fetch_all_crates(&client, user_id)?
    } else {
        Vec::new()
    };

    let repos = fetch_rust_repositories(&client, &username)?;
    let section = render_section(&username, &crates, &repos, has_user_id);

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
