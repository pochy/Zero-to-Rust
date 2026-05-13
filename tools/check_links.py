#!/usr/bin/env python3
"""Check local Markdown links in this repository."""

from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import unquote, urlparse


ROOT = Path(__file__).resolve().parents[1]
LINK_RE = re.compile(r"(?<!!)\[[^\]]+\]\(([^)]+)\)")


def is_external(target: str) -> bool:
    parsed = urlparse(target)
    return parsed.scheme in {"http", "https", "mailto"}


def normalize_target(source: Path, raw_target: str) -> Path | None:
    target = raw_target.split("#", 1)[0].strip()
    if not target or is_external(target):
        return None
    if target.startswith("<") and target.endswith(">"):
        target = target[1:-1]
    target = unquote(target)
    if target.startswith("/"):
        return ROOT / target.lstrip("/")
    return (source.parent / target).resolve()


def main() -> int:
    failures: list[str] = []

    for path in sorted(ROOT.rglob("*.md")):
        if ".git" in path.parts or "target" in path.parts:
            continue
        text = path.read_text(encoding="utf-8")
        for match in LINK_RE.finditer(text):
            raw_target = match.group(1)
            target = normalize_target(path, raw_target)
            if target is None:
                continue
            if not target.exists():
                rel_source = path.relative_to(ROOT)
                failures.append(f"{rel_source}: missing link target {raw_target}")

    if failures:
        print("\n".join(failures), file=sys.stderr)
        return 1

    print("local markdown links ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
