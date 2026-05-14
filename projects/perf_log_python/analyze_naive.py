#!/usr/bin/env python3
import json
import sys
import time
from collections import Counter
from pathlib import Path


def percentile_95(values: list[int]) -> int:
    if not values:
        return 0
    values.sort()
    index = int((len(values) - 1) * 0.95)
    return values[index]


def analyze(path: Path) -> dict:
    started = time.perf_counter()
    lines = path.read_text(encoding="utf-8").splitlines()
    events = []
    broken_lines = 0

    for line in lines:
        try:
            event = json.loads(line)
            event["status"] = str(int(event["status"]))
            event["bytes"] = int(event["bytes"])
            event["latency_ms"] = int(event["latency_ms"])
            event["path"] = str(event["path"])
            event["ip"] = str(event["ip"])
            events.append(event)
        except (json.JSONDecodeError, KeyError, TypeError, ValueError):
            broken_lines += 1

    total_lines = len(lines)
    total_bytes = sum(event["bytes"] for event in events)
    total_latency = sum(event["latency_ms"] for event in events)
    latencies = [event["latency_ms"] for event in events]
    statuses = Counter(event["status"] for event in events)
    paths = Counter(event["path"] for event in events)
    ips = Counter(event["ip"] for event in events)

    ok_lines = total_lines - broken_lines
    elapsed = time.perf_counter() - started
    return {
        "total_lines": total_lines,
        "ok_lines": ok_lines,
        "broken_lines": broken_lines,
        "total_bytes": total_bytes,
        "avg_latency_ms": round(total_latency / ok_lines, 3) if ok_lines else 0.0,
        "p95_latency_ms": percentile_95(latencies),
        "status_counts": dict(sorted(statuses.items())),
        "path_counts": dict(sorted(paths.items())),
        "top_10_ips": [
            {"ip": ip, "count": count}
            for ip, count in sorted(ips.items(), key=lambda item: (-item[1], item[0]))[:10]
        ],
        "elapsed_ms": round(elapsed * 1000, 3),
        "rows_per_second": round(total_lines / elapsed, 3) if elapsed else 0.0,
    }


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("usage: analyze_naive.py <logs.jsonl>")
    print(json.dumps(analyze(Path(sys.argv[1])), indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
