#!/usr/bin/env python3
"""Parse TOML examples in the bilingual installation documentation."""
import re
import sys
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:
    raise SystemExit("Python 3.11+ is required")

ROOT = Path(__file__).resolve().parents[1]
FILES = (
    "README.md",
    "README.zh_CN.md",
    "src/argument/README.md",
    "src/argument/README.zh_CN.md",
)


def main():
    failed = False
    for name in FILES:
        try:
            source = (ROOT / name).read_text(encoding="utf-8")
            blocks = re.findall(r"^```toml\s*\n(.*?)^```\s*$", source, re.M | re.S)
            if not blocks:
                raise ValueError("no TOML examples found")
            for index, block in enumerate(blocks, 1):
                try:
                    tomllib.loads(block)
                except tomllib.TOMLDecodeError as error:
                    print(f"{name}: block {index}: {error}", file=sys.stderr)
                    failed = True
        except (OSError, ValueError) as error:
            print(f"{name}: {error}", file=sys.stderr)
            failed = True
    return int(failed)


if __name__ == "__main__":
    raise SystemExit(main())
