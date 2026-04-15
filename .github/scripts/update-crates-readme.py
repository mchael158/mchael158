#!/usr/bin/env python3
import json
import os
import sys
import urllib.request
from datetime import datetime, timezone


README_PATH = "README.md"
START_MARKER = "<!-- CRATES-START -->"
END_MARKER = "<!-- CRATES-END -->"
CRATES_API_BASE = "https://crates.io/api/v1"
GITHUB_API_BASE = "https://api.github.com"


def fetch_json(url: str) -> dict:
    req = urllib.request.Request(
        url,
        headers={
            "User-Agent": "mchael158-profile-readme-updater",
            "Accept": "application/json",
        },
    )
    with urllib.request.urlopen(req, timeout=20) as response:
        return json.loads(response.read().decode("utf-8"))


def fetch_all_crates(user_id: int) -> list[dict]:
    crates = []
    page = 1
    per_page = 100
    while True:
        url = f"{CRATES_API_BASE}/crates?user_id={user_id}&page={page}&per_page={per_page}"
        data = fetch_json(url)
        page_items = data.get("crates", [])
        crates.extend(page_items)

        meta = data.get("meta", {})
        total = int(meta.get("total", len(crates)))
        if len(crates) >= total or not page_items:
            break
        page += 1

    crates.sort(key=lambda c: int(c.get("downloads", 0)), reverse=True)
    return crates


def fetch_rust_repositories(username: str) -> list[dict]:
    data = fetch_json(f"{GITHUB_API_BASE}/users/{username}/repos?per_page=100&type=owner&sort=updated")
    repos = [r for r in data if (r.get("language") == "Rust" or "rust" in [t.lower() for t in r.get("topics", [])])]
    repos.sort(key=lambda r: int(r.get("stargazers_count", 0)), reverse=True)
    return repos[:10]


def render_crates_block(username: str, crates: list[dict], has_user_id: bool) -> list[str]:
    profile_url = f"https://crates.io/users/{username}"
    lines: list[str] = []
    if not has_user_id:
        lines.extend(
            [
                "- Para listar crates publicadas automaticamente, configure `CRATES_IO_USER_ID` no workflow.",
                f"- Perfil crates.io (quando existir): [{profile_url}]({profile_url})",
            ]
        )
        return lines

    lines.extend(
        [
            f"- Perfil crates.io: [{profile_url}]({profile_url})",
            f"- Total de crates públicas: **{len(crates)}**",
        ]
    )

    if not crates:
        lines.append("- Nenhuma crate pública encontrada para o `CRATES_IO_USER_ID` informado.")
        return lines

    lines.extend(
        [
            "",
            "| Crate | Versão | Downloads | Documentação |",
            "|---|---:|---:|---|",
        ]
    )
    for crate in crates[:10]:
        name = crate.get("id", "-")
        version = crate.get("max_version", "-")
        downloads = f"{int(crate.get('downloads', 0)):,}".replace(",", ".")
        crate_url = f"https://crates.io/crates/{name}"
        docs_url = crate.get("documentation") or f"https://docs.rs/{name}"
        lines.append(
            f"| [{name}]({crate_url}) | `{version}` | **{downloads}** | [docs.rs]({docs_url}) |"
        )
    return lines


def render_repos_block(username: str, repos: list[dict]) -> list[str]:
    lines = [f"- Repositórios Rust detectados no GitHub: **{len(repos)}**"]
    if not repos:
        lines.append(f"- Nenhum repositório Rust público encontrado para [{username}](https://github.com/{username}).")
        return lines

    lines.extend(
        [
            "",
            "| Projeto | Stars | Atualizado | Descrição |",
            "|---|---:|---|---|",
        ]
    )
    for repo in repos:
        name = repo.get("name", "-")
        url = repo.get("html_url", f"https://github.com/{username}/{name}")
        stars = int(repo.get("stargazers_count", 0))
        updated = str(repo.get("pushed_at", ""))[:10]
        desc = (repo.get("description") or "-").replace("|", "\\|")
        lines.append(f"| [{name}]({url}) | **{stars}** | `{updated}` | {desc} |")
    return lines


def render_crates_section(username: str, crates: list[dict], repos: list[dict], has_user_id: bool) -> str:
    now = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
    lines = [
        "### Crates no crates.io",
        *render_crates_block(username, crates, has_user_id),
        "",
        "### Projetos Rust no GitHub",
        *render_repos_block(username, repos),
        "",
        f"- Última atualização automática: `{now}`",
    ]
    return "\n".join(lines)


def update_readme(content: str, section: str) -> str:
    if START_MARKER not in content or END_MARKER not in content:
        raise RuntimeError("Marcadores de seção de crates não encontrados no README.")
    start = content.index(START_MARKER) + len(START_MARKER)
    end = content.index(END_MARKER)
    return content[:start] + "\n" + section + "\n" + content[end:]


def main() -> int:
    username = os.getenv("CRATES_IO_USERNAME") or os.getenv("GITHUB_REPOSITORY_OWNER") or "mchael158"
    crates_user_id = os.getenv("CRATES_IO_USER_ID")

    try:
        crates = fetch_all_crates(int(crates_user_id)) if crates_user_id else []
        repos = fetch_rust_repositories(username)
        section = render_crates_section(username, crates, repos, bool(crates_user_id))
    except Exception as exc:
        section = "\n".join(
            [
                f"- Falha ao atualizar seção automática: `{exc}`",
                "- Tente novamente via GitHub Actions (workflow_dispatch).",
            ]
        )

    with open(README_PATH, "r", encoding="utf-8") as f:
        readme = f.read()

    updated = update_readme(readme, section)

    with open(README_PATH, "w", encoding="utf-8") as f:
        f.write(updated)

    return 0


if __name__ == "__main__":
    sys.exit(main())
