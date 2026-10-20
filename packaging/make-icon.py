#!/usr/bin/env python3
"""Draw DeckCraft's placeholder app icon in the house colours and write every size.

    python3 packaging/make-icon.py

Writes assets/app-icon/hicolor/scalable/apps/<ID>.svg, the hicolor PNGs (16-512 px),
deckcraft-1024.png, deckcraft.ico and deckcraft.icns. The SVG and the PNGs are drawn from the
same shape list below, so they match. Original work, MIT OR Apache-2.0. Needs Pillow.
"""
import os
import sys

from PIL import Image, ImageDraw

ID = "app.freedev.ainz.ks.DeckCraft"
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(ROOT, "assets", "app-icon")

# House tokens (craft-tokens): sheet background, line, steel-blue accent, ink, on-accent, gold.
BG = "#0b0e14"
LINE = "#2a3442"
ACCENT = "#8fb8d8"
ACCENT_LOW = "#3d5a73"
INK = "#e6eef5"
ON_ACCENT = "#0b0f14"
GOLD = "#d9c08a"

# (kind, x, y, w, h, radius, fill) on a 512 x 512 canvas.
SHAPES = [
    ("rect", 16, 16, 480, 480, 112, BG),
    ("rect", 246, 340, 20, 92, 0, ACCENT_LOW),          # stand
    ("rect", 176, 424, 160, 18, 9, ACCENT_LOW),         # foot
    ("rect", 80, 100, 352, 248, 22, ACCENT),            # the slide
    ("rect", 116, 136, 168, 22, 11, ON_ACCENT),         # title bar
    ("rect", 116, 278, 40, 38, 6, ON_ACCENT),           # bars
    ("rect", 176, 236, 40, 80, 6, ON_ACCENT),
    ("rect", 236, 196, 40, 120, 6, ON_ACCENT),
    ("circle", 370, 182, 30, 30, 0, GOLD),              # "now" marker
]
BORDER = (16, 16, 480, 480, 112, LINE, 6)


def svg():
    parts = ['<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">']
    for kind, x, y, w, h, r, fill in SHAPES:
        if kind == "circle":
            parts.append(f'<circle cx="{x}" cy="{y}" r="{w}" fill="{fill}"/>')
        else:
            parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{r}" fill="{fill}"/>')
    x, y, w, h, r, stroke, sw = BORDER
    i = sw / 2
    parts.append(
        f'<rect x="{x + i}" y="{y + i}" width="{w - sw}" height="{h - sw}" rx="{r - i}" fill="none" stroke="{stroke}" stroke-width="{sw}"/>'
    )
    parts.append("</svg>\n")
    return "".join(parts)


def raster(size):
    scale = 4
    big = size * scale
    k = big / 512
    img = Image.new("RGBA", (big, big), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    for kind, x, y, w, h, r, fill in SHAPES:
        if kind == "circle":
            d.ellipse([(x - w) * k, (y - w) * k, (x + w) * k, (y + w) * k], fill=fill)
        else:
            d.rounded_rectangle([x * k, y * k, (x + w) * k - 1, (y + h) * k - 1], radius=r * k, fill=fill)
    x, y, w, h, r, stroke, sw = BORDER
    d.rounded_rectangle([x * k, y * k, (x + w) * k - 1, (y + h) * k - 1], radius=r * k, outline=stroke, width=max(1, round(sw * k)))
    return img.resize((size, size), Image.LANCZOS)


def main():
    scalable = os.path.join(OUT, "hicolor", "scalable", "apps")
    os.makedirs(scalable, exist_ok=True)
    with open(os.path.join(scalable, f"{ID}.svg"), "w") as f:
        f.write(svg())
    sizes = [16, 24, 32, 48, 64, 128, 256, 512]
    for s in sizes:
        d = os.path.join(OUT, "hicolor", f"{s}x{s}", "apps")
        os.makedirs(d, exist_ok=True)
        raster(s).save(os.path.join(d, f"{ID}.png"), optimize=True)
    big = raster(1024)
    big.save(os.path.join(OUT, "deckcraft-1024.png"), optimize=True)
    big.save(os.path.join(OUT, "deckcraft.ico"), sizes=[(s, s) for s in [16, 24, 32, 48, 64, 128, 256]])
    big.save(os.path.join(OUT, "deckcraft.icns"))
    return 0


if __name__ == "__main__":
    sys.exit(main())
