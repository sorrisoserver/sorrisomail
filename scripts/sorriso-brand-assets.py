#!/usr/bin/env python3
"""Generate Sorriso Server Mail Windows and app icons from the approved official logo.

Usage:
  python -m pip install Pillow
  python scripts/sorriso-brand-assets.py path/to/logo-oficial.png

No generative redrawing: this only crops the cloud symbol from the official artwork.
Review the crop before packaging; logos with a different layout need adjusted geometry.
"""
from pathlib import Path
import sys
from PIL import Image

def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("Uso: python scripts/sorriso-brand-assets.py caminho/para/logo.png")
    image = Image.open(sys.argv[1]).convert("RGB")
    w, h = image.size
    # Approved artwork: cloud in upper ~2/3; company name and tagline below.
    crop = image.crop((int(w * .20), int(h * .04), int(w * .80), int(h * .66)))
    # Preserve dark background while retaining the supplied logo's original pixels.
    size = max(crop.size)
    canvas = Image.new("RGBA", (size, size), (6, 22, 37, 255))
    canvas.alpha_composite(crop.convert("RGBA"), ((size-crop.width)//2, (size-crop.height)//2))
    png = canvas.resize((256, 256), Image.Resampling.LANCZOS)
    base = Path("assets/sorriso-server")
    base.mkdir(parents=True, exist_ok=True)
    png.save(base / "sorriso-server-icon.png")
    png.save(base / "sorriso-server.ico", format="ICO", sizes=[
        (16,16),(24,24),(32,32),(48,48),(64,64),(128,128),(256,256)
    ])
    image.save(base / "sorriso-server-logo-oficial.png")
    print(f"Gerados em: {base}")

if __name__ == "__main__":
    main()
