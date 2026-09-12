#!/usr/bin/env python3
"""Extract and run `$CLI rpc` commands from cli.examples.md."""
from __future__ import annotations

import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path("/home/users/jakubmh/Derive/src/cockpit_v3")
MD = ROOT / "client/cli.examples.md"
CLI_BIN = ROOT / "target/debug/derive-rs"
LOG_DIR = Path(tempfile.mkdtemp(prefix="cli-examples-"))

ENV = os.environ.copy()
ENV.update(
    {
        "CLI": str(CLI_BIN),
        "SUBACCOUNT_ID": "78631",
        "INSTRUMENT": "ETH-PERP",
        "OPTION": "ETH-20260925-2500-C",
        "WALLET": "0x33f33399A43299C624aAf5051A161D04920c712D",
        "VAULT_ID": "78604",
        "RUST_LOG": "error",
    }
)

SKIP_PREFIXES = (
    "$CLI auctions",
    "$CLI sub ",
    "$CLI orderbook",
)

SKIP_METHODS = {
    "public/start_auction",
    "private/deposit",
    "private/withdraw",
    "private/force_burn",
    "private/create_vault",
    "private/reject_deposit_request",
    "private/mint_vault_shares",
    "private/burn_vault_shares",
    "private/request_vault_deposit",
    "private/request_vault_withdraw",
    "private/cancel_all_vault_requests",
    "private/update_vault_info",
    "private/liquidate",
}


def extract_rpc_commands(text: str) -> list[str]:
    blocks = re.findall(r"```bash\n(.*?)```", text, re.S)
    cmds: list[str] = []
    for block in blocks:
        current: list[str] = []
        in_cmd = False
        for raw in block.splitlines():
            line = raw.rstrip()
            stripped = line.strip()
            if stripped.startswith("#"):
                continue
            if stripped.startswith("$CLI"):
                if in_cmd and current:
                    cmds.append("\n".join(current))
                current = [line]
                in_cmd = True
                continue
            if in_cmd:
                current.append(line)
        if in_cmd and current:
            cmds.append("\n".join(current))
    rpc = []
    for c in cmds:
        first = c.strip().splitlines()[0]
        if first.startswith(SKIP_PREFIXES):
            continue
        rpc.append(c)
    out = []
    for c in rpc:
        m = method_of(c)
        if m in SKIP_METHODS:
            continue
        out.append(c)
    return out


def method_of(cmd: str) -> str:
    m = re.search(r"-m\s+(\S+)", cmd)
    return m.group(1) if m else "?"


def run_one(idx: int, cmd: str) -> dict:
    method = method_of(cmd)
    rendered = cmd.replace("$CLI", str(CLI_BIN))
    script = f"""set -euo pipefail
SUBACCOUNT_ID={ENV["SUBACCOUNT_ID"]}
INSTRUMENT={ENV["INSTRUMENT"]!r}
OPTION={ENV["OPTION"]!r}
WALLET={ENV["WALLET"]}
VAULT_ID={ENV["VAULT_ID"]}
{rendered}
"""
    log = LOG_DIR / f"{idx:03d}-{method.replace('/', '_')}.log"
    proc = subprocess.run(
        ["bash", "-lc", script],
        cwd=str(ROOT),
        env=ENV,
        capture_output=True,
        text=True,
        timeout=90,
    )
    log.write_text(proc.stdout + "\n--- STDERR ---\n" + proc.stderr)
    return {
        "idx": idx,
        "method": method,
        "exit": proc.returncode,
        "destructive": False,
        "placeholder_sig": "0xSIGNATURE" in cmd or "0xHASH" in cmd,
        "stdout_tail": (proc.stdout or "")[-800:],
        "stderr_tail": (proc.stderr or "")[-800:],
        "log": str(log),
    }


def classify(row: dict) -> str:
    blob = (row["stdout_tail"] + row["stderr_tail"]).lower()
    if row["exit"] == 0:
        if "rpc error" in blob or "error:" in blob:
            return "rpc_error_nonzero_ok"
        return "ok"
    if row["placeholder_sig"] and (
        "signature" in blob or "invalid" in blob or "nonce" in blob
    ):
        return "expected_placeholder"
    if "data did not match" in blob or "failed to parse" in blob:
        return "parse_bug"
    if "cli failed" in blob:
        return "cli_fail"
    return "fail"


def main() -> int:
    cmds = extract_rpc_commands(MD.read_text())
    print(f"extracted {len(cmds)} rpc commands; logs {LOG_DIR}", flush=True)
    results = []
    for i, cmd in enumerate(cmds, 1):
        print(f"\n=== {i:03d} {method_of(cmd)} ===", flush=True)
        try:
            row = run_one(i, cmd)
        except subprocess.TimeoutExpired:
            row = {
                "idx": i,
                "method": method_of(cmd),
                "exit": -1,
                "destructive": False,
                "placeholder_sig": False,
                "stdout_tail": "",
                "stderr_tail": "TIMEOUT",
                "log": "",
            }
        row["class"] = classify(row)
        print(
            f"exit={row['exit']} class={row['class']} placeholder={row['placeholder_sig']}",
            flush=True,
        )
        if row["class"] not in ("ok", "expected_placeholder"):
            print(row["stderr_tail"][-400:] or row["stdout_tail"][-400:], flush=True)
        results.append(row)

    summary = {}
    for r in results:
        summary[r["class"]] = summary.get(r["class"], 0) + 1
    print("\nSUMMARY", json.dumps(summary, indent=2))
    print("LOG_DIR", LOG_DIR)
    (LOG_DIR / "results.json").write_text(json.dumps(results, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
