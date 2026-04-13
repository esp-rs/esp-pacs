#!/usr/bin/env python3
"""
Cross-check ESP-IDF esp32p4 hw_ver3 register headers against patched esp32p4.svd.

Run from esp-pacs repo root. Invokes `cargo xtask patch esp32p4` first so esp32p4.svd
reflects YAML patches only (never edits *.base.svd from this script).
"""

from __future__ import annotations

import re
import subprocess
import sys
import xml.etree.ElementTree as ET
from pathlib import Path


def eval_base(expr: str, env: dict[str, int]) -> int:
    expr = expr.strip()
    if expr.startswith("(") and expr.endswith(")"):
        expr = expr[1:-1].strip()
    if re.fullmatch(r"0x[0-9A-Fa-f]+", expr):
        return int(expr, 0)
    if re.fullmatch(r"\d+", expr):
        return int(expr, 10)
    m = re.match(r"([A-Z0-9_]+)\s*\+\s*(.+)$", expr)
    if m:
        return env[m.group(1)] + eval_base(m.group(2), env)
    return env[expr]


def load_reg_bases(path: Path) -> dict[str, int]:
    text = path.read_text(encoding="utf-8", errors="replace")
    env: dict[str, int] = {}
    pat = re.compile(r"^#define\s+(DR_REG_\w+)\s+(.+)$")
    for line in text.splitlines():
        m = pat.match(line.strip())
        if not m:
            continue
        name, rhs = m.group(1), m.group(2).strip()
        if rhs.endswith("\\"):
            continue
        try:
            env[name] = eval_base(rhs, env)
        except (KeyError, ValueError):
            continue
    if "DR_REG_GDMA_BASE" in env:
        env.setdefault("DR_REG_DMAC_BASE", env["DR_REG_GDMA_BASE"])
    if "DR_REG_AXI_ICM_BASE" in env:
        # QoS indirect-access window (icm_sys_qos_reg.h uses DR_REG_ICM_AXI_BASE).
        env.setdefault("DR_REG_ICM_AXI_BASE", env["DR_REG_AXI_ICM_BASE"] + 0x400)
    return env


def strip_ns(tag: str) -> str:
    return tag.split("}", 1)[1] if tag.startswith("{") else tag


def collect_svd_regs(node: ET.Element, base_off: int, out: list[tuple[int, str]]) -> None:
    for c in node:
        t = strip_ns(c.tag)
        if t == "register":
            ao = c.find("addressOffset")
            nm = c.find("name")
            if ao is None or nm is None or not ao.text or not nm.text:
                continue
            off = int(ao.text.strip(), 0) + base_off
            name = nm.text.strip()
            dim = c.find("dim")
            inc = c.find("dimIncrement")
            if dim is not None and dim.text and inc is not None and inc.text:
                d = int(dim.text.strip(), 0)
                step = int(inc.text.strip(), 0)
                for i in range(d):
                    out.append((off + i * step, name.replace("%s", str(i))))
            else:
                out.append((off, name))
        elif t == "cluster":
            ao = c.find("addressOffset")
            co = int(ao.text.strip(), 0) if ao is not None and ao.text else 0
            dim_el = c.find("dim")
            inc_el = c.find("dimIncrement")
            if dim_el is not None and dim_el.text and inc_el is not None and inc_el.text:
                d = int(dim_el.text.strip(), 0)
                step = int(inc_el.text.strip(), 0)
                for i in range(d):
                    collect_svd_regs(c, base_off + co + i * step, out)
            else:
                collect_svd_regs(c, base_off + co, out)
        else:
            collect_svd_regs(c, base_off, out)


def load_svd(path: Path) -> tuple[dict[str, int], dict[str, dict[int, list[str]]]]:
    root = ET.parse(path).getroot()
    bases: dict[str, int] = {}
    regs: dict[str, dict[int, list[str]]] = {}
    for p in root.find("peripherals").findall("peripheral"):
        if p.find("derivedFrom") is not None and p.find("derivedFrom").text:
            continue
        n = p.find("name")
        b = p.find("baseAddress")
        if n is None or b is None or not n.text or not b.text:
            continue
        pname = n.text.strip()
        pbase = int(b.text.strip(), 0)
        bases[pname] = pbase
        flat: list[tuple[int, str]] = []
        collect_svd_regs(p, 0, flat)
        m: dict[int, list[str]] = {}
        for off, nm in flat:
            m.setdefault(off, []).append(nm)
        regs[pname] = m
    return bases, regs


def base_macro_to_svd_periph(base_name: str) -> str | None:
    """Map e.g. DR_REG_UART0_BASE -> UART0."""
    if not base_name.startswith("DR_REG_") or not base_name.endswith("_BASE"):
        return None
    inner = base_name[len("DR_REG_") : -len("_BASE")]
    special = {
        "GDMA": "DMA",
        "DMAC": "DMA",
        "INTR": "INTERRUPT_CORE0",
        "LCDCAM": "LCD_CAM",
        "PARIO": "PARL_IO",
        "SDMMC": "SDHOST",
        "USB2JTAG": "USB_DEVICE",
        "ETM": "SOC_ETM",
        "LP_ANALOG_PERI": "LP_ANA",
        "LP_CLKRST": "LP_AON_CLKRST",
        "H264_CORE": "H264",
        "REGDMA": "PAU",
        "FLASH_SPI0": "SPI0",
        "FLASH_SPI1": "SPI1",
        "CSI_HOST": "MIPI_CSI_HOST",
        "CSI_BRG": "MIPI_CSI_BRIDGE",
        "DSI_HOST": "MIPI_DSI_HOST",
        "DSI_BRG": "MIPI_DSI_BRIDGE",
        "ICM": "AXI_ICM",
        "ICM_AXI": "ICM_AXI_QOS",
        "LP_TRNG": "LP_TRNG",
        "LP_SPI": "LP_SPI",
        "LP_MB": "LP_MAILBOX",
        "RTC": "LP_SYS",
        "CRYPTO": None,
        "AES": "AES",
        "SHA": "SHA",
        "RSA": "RSA",
        "ECC_MULT": "ECC",
        "DS": "DS",
        "HMAC": "HMAC",
        "ECDSA": "ECDSA",
        "AUDIO_ADDC": None,
        "H264_DMA_2D": None,
        "USB2": None,
        "USB11": None,
        "USBPHY": None,
        "DDRPHY": None,
        "MSPI": None,
        "PSRAM_MSPI0": None,
        "PSRAM_MSPI1": None,
    }
    if inner in special:
        return special[inner]
    return inner


def main() -> int:
    repo = Path(__file__).resolve().parents[1]
    idf_soc = Path(
        "/Users/playfulfence/esp/esp-idf/components/soc/esp32p4/register/hw_ver3/soc"
    )
    svd_path = repo / "esp32p4/svd/esp32p4.svd"
    reg_base_h = idf_soc / "reg_base.h"

    if not idf_soc.is_dir():
        print("IDF hw_ver3 not found", file=sys.stderr)
        return 1

    subprocess.run(
        ["cargo", "xtask", "patch", "esp32p4"],
        cwd=repo,
        check=True,
        stdout=subprocess.DEVNULL,
    )

    bases = load_reg_bases(reg_base_h)
    svd_bases, svd_regs = load_svd(svd_path)

    reg_pat = re.compile(
        r"^#define\s+(\w+)\s+\((DR_REG_\w+_BASE)\s*\+\s*(0x[0-9A-Fa-f]+|\d+)\)\s*$"
    )

    missing: list[tuple[str, int, str]] = []
    skipped_bases: set[str] = set()

    for hdr in sorted(idf_soc.glob("*_reg.h")):
        if hdr.name == "reg_base.h":
            continue
        raw = hdr.read_text(encoding="utf-8", errors="replace")
        raw = raw.replace("DR_REG_DMAC_BASE", "DR_REG_GDMA_BASE")
        raw = raw.replace("DR_REG_ICM_BASE", "DR_REG_AXI_ICM_BASE")
        for line in raw.splitlines():
            m = reg_pat.match(line.strip())
            if not m:
                continue
            dname, base_name, off_s = m.group(1), m.group(2), m.group(3)
            if base_name not in bases:
                skipped_bases.add(base_name)
                continue
            periph = base_macro_to_svd_periph(base_name)
            if periph is None or periph not in svd_regs:
                continue
            rel = int(off_s, 0)
            if rel not in svd_regs[periph]:
                missing.append((periph, rel, dname))

    print(f"Missing register slots (IDF define vs patched SVD): {len(missing)}")
    byp: dict[str, list[tuple[int, str]]] = {}
    for p, off, d in missing:
        byp.setdefault(p, []).append((off, d))
    for p in sorted(byp):
        print(f"\n== {p} ({len(byp[p])}) ==")
        for off, d in sorted(byp[p])[:40]:
            print(f"  0x{off:04x}  {d}")
        if len(byp[p]) > 40:
            print(f"  ... {len(byp[p]) - 40} more")

    if skipped_bases:
        print("\nBases referenced in reg headers but not in reg_base.h:")
        for x in sorted(skipped_bases):
            print(" ", x)

    # SVD peripherals with no IDF base macro of the same resolved address
    idf_abs_bases = set(bases.values())
    print("\n--- SVD peripherals vs IDF base address set ---")
    for name in sorted(svd_bases):
        b = svd_bases[name]
        tag = "OK" if b in idf_abs_bases else "PAC-only or alias"
        print(f"{hex(b):>12}  {name:22}  {tag}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
