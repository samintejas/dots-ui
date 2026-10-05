#!/usr/bin/env python3
"""Regenerate src/icon_data.rs from the design system's components/bundle.js.

usage: scripts/gen_icons.py path/to/components/bundle.js
"""
import json
import re
import sys
from pathlib import Path

src = Path(sys.argv[1]).read_text()
icons = json.loads(re.search(r"var ICONS = (\{.*?\});\n", src, re.S).group(1))
out = [
    "// generated from the dots design system (components/bundle.js `ICONS`). lucide, ISC license: see assets/LICENSE-lucide.txt.",
    "// do not hand-edit; regenerate with scripts/gen_icons.py.",
    "",
    "/// Every icon name that ships with the library.",
    "pub const NAMES: &[&str] = &[",
    *[f'    "{k}",' for k in icons],
    "];",
    "",
    "/// Inner svg markup of a Lucide icon (24x24 viewbox, stroke only).",
    "pub fn inner(name: &str) -> Option<&'static str> {",
    "    Some(match name {",
    *[f'        "{k}" => r##"{v}"##,' for k, v in icons.items()],
    "        _ => return None,",
    "    })",
    "}",
]
Path(__file__).resolve().parent.parent.joinpath("src/icon_data.rs").write_text("\n".join(out) + "\n")
print(f"{len(icons)} icons")
