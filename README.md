# mchael158

**Backend · Hardware · Embedded — Rust**

I build systems that sit between the wire and the service layer:  
async backends, embeddable runtimes, and `no_std` protocols for constrained environments.

<p>
  <a href="https://crates.io/users/mchael158"><img src="https://img.shields.io/badge/crates.io-mchael158-f74c00?style=flat-square&logo=rust&logoColor=white" alt="crates.io" /></a>
  <img src="https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/no__std-111111?style=flat-square" alt="no_std" />
  <img src="https://img.shields.io/badge/Tokio-111111?style=flat-square" alt="Tokio" />
  <img src="https://img.shields.io/badge/Axum-111111?style=flat-square" alt="Axum" />
  <img src="https://img.shields.io/badge/PostgreSQL-316192?style=flat-square&logo=postgresql&logoColor=white" alt="PostgreSQL" />
  <img src="https://img.shields.io/badge/Redis-DC382D?style=flat-square&logo=redis&logoColor=white" alt="Redis" />
  <img src="https://img.shields.io/badge/Docker-2496ED?style=flat-square&logo=docker&logoColor=white" alt="Docker" />
  <img src="https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black" alt="Linux" />
</p>

<p>
  <img src="https://skillicons.dev/icons?i=rust,postgres,redis,docker,linux,git,githubactions" />
</p>

---

## What I work on

<table>
<tr>
<td width="50%" valign="top">

### Backend

- High-throughput HTTP APIs with Axum / Tokio
- PostgreSQL + SQLx (typed queries, migrations)
- Redis for sessions, caching, and rate limits
- Background jobs and event-driven pipelines
- Linux containers and GitHub Actions CI/CD

</td>
<td width="50%" valign="top">

### Hardware / Embedded

- `no_std` / heapless protocol stacks
- Fixed frames, ACK/CRC, sliding windows
- Embeddable actor runtimes (M:N scheduling)
- Host ↔ device I/O integration layers
- Deterministic links for constrained targets

</td>
</tr>
</table>

---

## Open source

| Project | Role | Stack |
|---|---|---|
| [byteflow-actors](https://github.com/mchael158/byteflow-actors) | Embeddable flow runtime — M:N scheduler, Atomic Hop, supervisor | Rust |
| [byteflow-io](https://github.com/mchael158/byteflow-io) | Host-side I/O layer for the Byteflow runtime | Rust |
| [psicose](https://github.com/mchael158/PSICOSE) | Heapless reliable link — 4-byte frame, ACK/CRC, P2P Node/PeerLink | Rust · `no_std` |
| [CUDA-G834L8MD0SD0K3](https://github.com/mchael158/CUDA-G834L8MD0SD0K3) | Low-level / compute experiments | Rust |
| [Registrator](https://github.com/mchael158/Registrator) | Registration / systems tooling | Rust |
| [terminal-prompt](https://github.com/mchael158/terminal-prompt) | Terminal UX utilities | Rust |

---

## Published crates

<!-- CRATES-START -->
<a href="https://crates.io/users/mchael158"><img src="https://img.shields.io/badge/crates.io-mchael158-f74c00?style=flat-square&logo=rust&logoColor=white" alt="crates.io" /></a>

| Crate | Version | Downloads | Docs |
|---|---|---:|---|
| [byteflow-actors](https://crates.io/crates/byteflow-actors) | `0.9.24` | 460 | [docs](https://docs.rs/byteflow-actors) |
| [psicose](https://crates.io/crates/psicose) | `0.4.3` | 121 | [docs](https://docs.rs/psicose) |
| [byteflow-io](https://crates.io/crates/byteflow-io) | `0.2.0` | 43 | [docs](https://docs.rs/byteflow-io) |

<sub>Updated `2026-11-12 17:22 UTC`</sub>
<!-- CRATES-END -->

---

## Stack map

```text
mchael158/
├─ backend/
│  ├── axum + tokio        # async HTTP edge
│  ├── sqlx + postgres     # typed data layer
│  └── redis               # hot path / sessions
├─ embedded/
│  ├── psicose             # heapless reliable link
│  ├── no_std frames       # ACK / CRC / windowing
│  └── host I/O bridges    # byteflow-io
├─ runtime/
│  ├── byteflow-actors     # M:N flows + supervisor
│  └── message hop         # Atomic Hop (Message+Cap)
└─ ops/
   ├── docker + linux
   └── github actions
```

---

## Activity

<p>
  <img height="160" src="https://github-readme-stats.vercel.app/api?username=mchael158&show_icons=true&theme=dark&hide_border=true&bg_color=0d1117&title_color=f74c00&icon_color=f74c00&text_color=c9d1d9" alt="stats" />
  <img height="160" src="https://github-readme-stats.vercel.app/api/top-langs/?username=mchael158&layout=compact&theme=dark&hide_border=true&bg_color=0d1117&title_color=f74c00&text_color=c9d1d9" alt="languages" />
</p>
