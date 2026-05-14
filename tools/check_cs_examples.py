#!/usr/bin/env python3
"""Compile and test standalone Rust examples in the CS track."""

from __future__ import annotations

import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
EXAMPLES_ROOT = ROOT / "computer_science" / "levels"


def has_tests(path: Path) -> bool:
    text = path.read_text(encoding="utf-8")
    return "#[cfg(test)]" in text or "#[test]" in text


def run(command: list[str]) -> None:
    subprocess.run(command, cwd=ROOT, check=True)


def main() -> int:
    examples = sorted(EXAMPLES_ROOT.glob("*/examples/*.rs"))
    if not examples:
        print("no CS examples found", file=sys.stderr)
        return 1

    compiled = 0
    tested = 0

    with tempfile.TemporaryDirectory(prefix="zero_to_rust_cs_examples_") as tmp:
        out_dir = Path(tmp)

        for example in examples:
            rel = example.relative_to(ROOT)
            binary = out_dir / example.with_suffix("").name
            print(f"compile {rel}", flush=True)
            run(["rustc", "--edition=2021", str(example), "-o", str(binary)])
            compiled += 1

            if has_tests(example):
                test_binary = out_dir / f"{example.with_suffix('').name}_test"
                print(f"test    {rel}", flush=True)
                run(
                    [
                        "rustc",
                        "--edition=2021",
                        "--test",
                        str(example),
                        "-o",
                        str(test_binary),
                    ]
                )
                run([str(test_binary)])
                tested += 1

    print(f"cs examples ok: compiled={compiled} tested={tested}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
