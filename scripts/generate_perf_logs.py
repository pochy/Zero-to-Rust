#!/usr/bin/env python3
import argparse
import json
import random
from datetime import datetime, timedelta, timezone
from pathlib import Path


METHODS = ["GET", "GET", "GET", "POST", "PUT", "DELETE"]
PATHS = [
    "/api/items",
    "/api/items/{id}",
    "/api/login",
    "/api/search",
    "/static/app.js",
    "/health",
    "/metrics",
]
STATUSES = [200, 200, 200, 200, 200, 201, 204, 400, 401, 404, 429, 500, 503]


def ip_pool(size: int) -> list[str]:
    return [f"203.0.113.{i % 250 + 1}" for i in range(size)]


def choose_latency(path: str, status: int, rng: random.Random) -> int:
    base = {
        "/health": 2,
        "/metrics": 5,
        "/static/app.js": 12,
        "/api/items": 18,
        "/api/items/{id}": 22,
        "/api/login": 35,
        "/api/search": 48,
    }[path]
    if status >= 500:
        base *= 4
    elif status >= 400:
        base *= 2
    return max(1, int(rng.gauss(base, max(1, base / 3))))


def choose_bytes(path: str, status: int, rng: random.Random) -> int:
    if path == "/health":
        return rng.randint(2, 32)
    if path == "/metrics":
        return rng.randint(100, 2000)
    if path == "/static/app.js":
        return rng.randint(20_000, 250_000)
    if status >= 400:
        return rng.randint(80, 800)
    return rng.randint(200, 20_000)


def broken_line(index: int) -> str:
    variants = [
        "{not json",
        "",
        '{"ts":"broken","ip":null',
        f"this is not json line {index}",
    ]
    return variants[index % len(variants)]


def generate(rows: int, seed: int, out: Path, broken_rate: float) -> None:
    rng = random.Random(seed)
    out.parent.mkdir(parents=True, exist_ok=True)
    start = datetime(2026, 5, 14, 12, 0, 0, tzinfo=timezone.utc)
    ips = ip_pool(500)

    with out.open("w", encoding="utf-8") as handle:
        for i in range(rows):
            if rng.random() < broken_rate:
                handle.write(broken_line(i) + "\n")
                continue

            path = rng.choice(PATHS)
            status = rng.choice(STATUSES)
            event = {
                "ts": (start + timedelta(seconds=i)).strftime("%Y-%m-%dT%H:%M:%SZ"),
                "ip": rng.choice(ips),
                "method": rng.choice(METHODS),
                "path": path,
                "status": status,
                "bytes": choose_bytes(path, status, rng),
                "latency_ms": choose_latency(path, status, rng),
            }
            handle.write(json.dumps(event, separators=(",", ":")) + "\n")


def main() -> None:
    parser = argparse.ArgumentParser(description="Generate deterministic JSONL access logs.")
    parser.add_argument("--rows", type=int, default=10_000)
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--out", required=True)
    parser.add_argument("--broken-rate", type=float, default=0.001)
    args = parser.parse_args()

    if args.rows < 0:
        raise SystemExit("--rows must be >= 0")
    if not 0.0 <= args.broken_rate <= 1.0:
        raise SystemExit("--broken-rate must be between 0.0 and 1.0")

    generate(args.rows, args.seed, Path(args.out), args.broken_rate)


if __name__ == "__main__":
    main()
