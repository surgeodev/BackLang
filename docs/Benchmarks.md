# Benchmarks

BackLang is built for speed. A single **2.4 MB** binary replaces an entire Node.js + Express + SQLite stack (~200 MB), boots in **2ms**, and handles **~120,000 HTTP requests per second**.

## HTTP Server Throughput

**Test:** `wrk -t4 -c100 -d10s http://localhost:8080/` — single endpoint returning `{ok: true}`

| Runtime | Framework | req/s | Latency (avg) |
|---------|-----------|-------|---------------|
| **BackLang** | **Built-in (Axum)** | **~120,000** | **~0.8 ms** |
| Rust | Axum (raw) | ~250,000 | ~0.4 ms |
| Go | net/http | ~150,000 | ~0.6 ms |
| Node.js 22 | Fastify | ~45,000 | ~2.1 ms |
| Deno 2 | Oak | ~40,000 | ~2.5 ms |
| Python 3.12 | FastAPI + uvicorn | ~15,000 | ~6.5 ms |
| Python 3.12 | Flask + gunicorn | ~8,000 | ~12 ms |
| Lua 5.4 | Lapis (OpenResty) | ~25,000 | ~3.8 ms |

## Startup Time

**Test:** Time to first byte of output (`time bl file.bl` with a hello-world program)

| Runtime | Cold start | Warm start |
|---------|-----------|------------|
| **BackLang** | **2.1 ms** | **1.8 ms** |
| Lua 5.4 | 1.5 ms | 1.2 ms |
| Python 3.12 | 48 ms | 35 ms |
| Node.js 22 | 290 ms | 45 ms |
| Deno 2 | 180 ms | 25 ms |

## SQLite Query Performance

**Test:** Execute 10,000 SELECT queries on a table with 1000 rows

| Runtime | Time |
|---------|------|
| **BackLang (bundled rusqlite)** | **42 ms** |
| Rust (rusqlite) | 38 ms |
| Node.js (better-sqlite3) | 65 ms |
| Python 3.12 (sqlite3) | 180 ms |
| Lua 5.4 (lsqlite3) | 95 ms |

## Binary Size

| Runtime | Size | Included |
|---------|------|----------|
| **BackLang** | **2.4 MB** | **Interpreter + HTTP server + SQLite + debugger** |
| Lua 5.4 | ~1.2 MB | Interpreter only |
| Python 3.12 | ~12 MB | Interpreter only |
| Node.js 22 | ~45 MB | Runtime only |
| Python + FastAPI + uvicorn | ~55 MB | + packages |
| Node.js + Express + better-sqlite3 | ~200 MB | + node_modules |

## Memory Usage (idle)

| Runtime | RSS |
|---------|-----|
| BackLang | 3.2 MB |
| Lua 5.4 | 2.1 MB |
| Python 3.12 | 12 MB |
| Node.js 22 | 35 MB |
| Deno 2 | 40 MB |

## Methodology

- **CPU:** AMD Ryzen 9 7950X (16C/32T)
- **RAM:** 64 GB DDR5-6000
- **Storage:** Samsung 990 Pro NVMe (ext4)
- **OS:** Ubuntu 24.04 LTS, Linux 6.8
- **wrk:** `wrk -t4 -c100 -d10s http://localhost:8080/`
- All runtimes tested in **release** mode (where applicable)
- Network benchmarks use **localhost** loopback (no network overhead)
- Each test repeated 5 times, median reported
- Source code for benchmarks available in the [BackLang repo](https://github.com/surgeodev/BackLang/tree/main/bench)

> ⚠️ These are synthetic benchmarks. Real-world performance depends on workload, hardware, and application architecture.
