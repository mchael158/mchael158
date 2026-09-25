<p align="center">
  <img src="https://img.shields.io/badge/mchael158-runtime-0b0d10?style=for-the-badge&labelColor=f74c00&color=0b0d10" alt="mchael158" />
  <img src="https://img.shields.io/badge/🦀-Ferris%20mode-DEA584?style=for-the-badge&labelColor=0b0d10&color=DEA584" alt="ferris" />
</p>

<table>
<tr>
<td width="48%" valign="top">

### `@mchael158`

**Runtime & Protocol Engineer**  
**Async systems in Rust**

Building actor flows, heapless protocols,  
and backends that stay predictable under load.

```text
latency over luck
ownership over abstraction
ship what you can observe
```

<p>
  <a href="https://crates.io/users/mchael158"><img src="https://img.shields.io/badge/crates.io-mchael158-f74c00?style=flat-square&logo=rust&logoColor=white" alt="crates.io" /></a>
  <img src="https://img.shields.io/badge/tokio-async-0DB7ED?style=flat-square&logo=tokio&logoColor=white" alt="tokio" />
  <img src="https://img.shields.io/badge/axum-http-7C3AED?style=flat-square" alt="axum" />
  <img src="https://img.shields.io/badge/no__std-aware-16A34A?style=flat-square" alt="no_std" />
</p>

</td>
<td width="52%" valign="top">

```text
🦀 mchael158/
├─ 🦀 runtime/
│  ├── 🦀 actors.rs       # M:N flows
│  ├── 🦀 hop.rs          # message + cap
│  ├── 🦀 supervisor.rs   # restart policy
│  └── 🦀 tokio.rs        # async core
├─ 🦀 wire/
│  ├── 🦀 frame.rs        # fixed headers
│  ├── 🦀 crc_ack.rs      # integrity
│  └── 🦀 psicose.rs      # heapless link
├─ 🦀 service/
│  ├── 🦀 axum.rs         # http edge
│  ├── 🦀 sqlx.rs         # postgres
│  └── 🦀 redis.rs        # hot path
└─ 🦀 ship/
   ├── 🦀 docker
   ├── 🦀 linux
   └── 🦀 actions.yml
```

</td>
</tr>
</table>

---

<p>
  <img src="https://img.shields.io/badge/signal-0b0d10?style=for-the-badge&labelColor=f74c00&color=0b0d10" alt="signal" />
</p>

Systems should remain boring in production.  
Fast paths are designed, not hoped for.  
If it moves bytes, it owns invariants.

<p>
  <img src="https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/Tokio-0DB7ED?style=flat-square" alt="Tokio" />
  <img src="https://img.shields.io/badge/Axum-7C3AED?style=flat-square" alt="Axum" />
  <img src="https://img.shields.io/badge/PostgreSQL-4169E1?style=flat-square&logo=postgresql&logoColor=white" alt="PostgreSQL" />
  <img src="https://img.shields.io/badge/SQLx-336791?style=flat-square" alt="SQLx" />
  <img src="https://img.shields.io/badge/Redis-DC382D?style=flat-square&logo=redis&logoColor=white" alt="Redis" />
  <img src="https://img.shields.io/badge/Docker-2496ED?style=flat-square&logo=docker&logoColor=white" alt="Docker" />
  <img src="https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black" alt="Linux" />
  <img src="https://img.shields.io/badge/GitHub%20Actions-2088FF?style=flat-square&logo=githubactions&logoColor=white" alt="Actions" />
</p>

<p>
  <img src="https://skillicons.dev/icons?i=rust,postgres,redis,docker,linux,git,githubactions" />
</p>

---

<p>
  <img src="https://img.shields.io/badge/crates%20%26%20rust%20surface-0b0d10?style=for-the-badge&labelColor=DEA584&color=0b0d10" alt="crates" />
</p>

<!-- CRATES-START -->
<a href="https://crates.io/users/mchael158"><img src="https://img.shields.io/badge/🦀_crates.io-mchael158-f74c00?style=for-the-badge&logo=rust&logoColor=white&labelColor=0b0d10" alt="crates.io" /></a>

<img src="https://img.shields.io/badge/🦀_published-2-f74c00?style=for-the-badge&labelColor=0b0d10&color=f74c00" alt="published crates" />

**🦀 [byteflow-actors](https://crates.io/crates/byteflow-actors)**
<a href="https://crates.io/crates/byteflow-actors"><img src="https://img.shields.io/badge/byteflow--actors-0.9.7-f74c00?style=flat-square&logo=rust&logoColor=white" alt="byteflow-actors" /></a> <a href="https://docs.rs/byteflow-actors"><img src="https://img.shields.io/badge/docs-DEA584?style=flat-square&logo=readthedocs&logoColor=0b0d10" alt="docs" /></a> <img src="https://img.shields.io/badge/⬇_243-0DB7ED?style=flat-square&labelColor=0b0d10" alt="downloads" />
<a href="https://github.com/mchael158/bytecode-vm"><img src="https://img.shields.io/badge/source-2088FF?style=flat-square&logo=github&logoColor=white" alt="source" /></a>
<sub>Embeddable flow runtime: M:N scheduler, Atomic Hop (Message+Cap), supervisor — use byteflow::</sub>

**🦀 [psicose](https://crates.io/crates/psicose)**
<a href="https://crates.io/crates/psicose"><img src="https://img.shields.io/badge/psicose-0.4.0-f74c00?style=flat-square&logo=rust&logoColor=white" alt="psicose" /></a> <a href="https://docs.rs/psicose"><img src="https://img.shields.io/badge/docs-DEA584?style=flat-square&logo=readthedocs&logoColor=0b0d10" alt="docs" /></a> <img src="https://img.shields.io/badge/⬇_90-0DB7ED?style=flat-square&labelColor=0b0d10" alt="downloads" />
<a href="https://github.com/mchael158/PSICOSE"><img src="https://img.shields.io/badge/source-2088FF?style=flat-square&logo=github&logoColor=white" alt="source" /></a>
<sub>no_std heapless reliable link motor: 4-byte frame, ACK/CRC, window N≤8, P2P Node/PeerLink, LinkFace — zero cr…</sub>

<img src="https://img.shields.io/badge/🦀_repos-3-DEA584?style=for-the-badge&labelColor=0b0d10&color=DEA584" alt="rust repos" />

<p>
  <a href="https://github.com/mchael158/byteflow-actors"><img src="https://img.shields.io/badge/🦀_byteflow--actors-Rust-f74c00?style=flat-square&labelColor=0b0d10&logo=github&logoColor=white" alt="byteflow-actors" /></a> <img src="https://img.shields.io/badge/★_0-1a1a1a?style=flat-square" alt="stars" /> <img src="https://img.shields.io/badge/2026--09--23-2088FF?style=flat-square&labelColor=0b0d10" alt="updated" />
  <br/>
  <a href="https://github.com/mchael158/PSICOSE"><img src="https://img.shields.io/badge/🦀_PSICOSE-Rust-f74c00?style=flat-square&labelColor=0b0d10&logo=github&logoColor=white" alt="PSICOSE" /></a> <img src="https://img.shields.io/badge/★_0-1a1a1a?style=flat-square" alt="stars" /> <img src="https://img.shields.io/badge/2026--09--20-2088FF?style=flat-square&labelColor=0b0d10" alt="updated" />
  <br/>
  <a href="https://github.com/mchael158/Registrator"><img src="https://img.shields.io/badge/🦀_Registrator-Rust-f74c00?style=flat-square&labelColor=0b0d10&logo=github&logoColor=white" alt="Registrator" /></a> <img src="https://img.shields.io/badge/★_0-1a1a1a?style=flat-square" alt="stars" /> <img src="https://img.shields.io/badge/2026--08--22-2088FF?style=flat-square&labelColor=0b0d10" alt="updated" />
  <br/>
</p>

<sub>🦀 auto-synced · `2026-09-25 14:26 UTC`</sub>
<!-- CRATES-END -->

---

<p align="center">
  <img height="168" src="https://github-readme-stats.vercel.app/api?username=mchael158&show_icons=true&theme=radical&hide_border=true&bg_color=0b0d10&title_color=f74c00&icon_color=f74c00&text_color=c9c9c9" alt="stats" />
  <img height="168" src="https://github-readme-stats.vercel.app/api/top-langs/?username=mchael158&layout=compact&theme=radical&hide_border=true&bg_color=0b0d10&title_color=f74c00&text_color=c9c9c9" alt="langs" />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/predictable%20under%20load-f74c00?style=flat-square&labelColor=0b0d10" alt="load" />
  <img src="https://img.shields.io/badge/owned%20invariants-DEA584?style=flat-square&labelColor=0b0d10" alt="invariants" />
  <img src="https://img.shields.io/badge/rust%20first-0DB7ED?style=flat-square&labelColor=0b0d10" alt="rust" />
</p>
