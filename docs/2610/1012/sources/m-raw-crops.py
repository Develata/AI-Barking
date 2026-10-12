"""Build annotation base images (-raw) for 1012 from evidence screenshots.
Pixels are only cropped, never altered. Run: uv run --with pillow python -I m-raw-crops.py <images dir>
After the visual review, 41 (bottom line clipped), 44 (no author bar) and 45 (text too small) were re-captured
with m-shot.mjs and replaced; the lines for 41 below and the 43-46 note record the first version only."""
from PIL import Image
import sys, os
os.chdir(sys.argv[1])

def crop(src, box):
    return Image.open(src).convert('RGB').crop(box)

def whole(src):
    return Image.open(src).convert('RGB')

# 41 Karpathy post, header + first two tiers (napkin math line), 01 (whole)
whole('01-karpathy-funnel-top.png').save('41-karpathy-napkin-raw.png')
# 42 Karpathy post, tiers 3-4 (~5,000 with internal access), 02 (whole)
whole('02-karpathy-funnel-layers34.png').save('42-karpathy-5000-raw.png')
# 43-46 are not built here: Claude re-captured them with m-shot.mjs (anonymous headless Chrome, 700 CSS px, DPR 2;
# runs logged in m-browser-records.jsonl) as 51-54 and renamed them 43 (Tibo default 500k), 44 (Tibo no-fee reply),
# 45 (API model page pricing; the capture was cropped to rows 180..end to start at "Text tokens"), 46 (Codex pricing FAQ).
# 47 PhotoCraft README header: clean-room line + status badge, 25 (lower part)
crop('25-photocraft-readme-top.png', (0, 2540, 1400, 3240)).save('47-photocraft-cleanroom-raw.png')
# 48 PhotoCraft README Important status box, 29 (whole)
whole('29-photocraft-readme-status.png').save('48-photocraft-status-raw.png')
