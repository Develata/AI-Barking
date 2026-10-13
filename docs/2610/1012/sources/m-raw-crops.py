"""Build annotation bases (-raw) for 1013. Pixels are only cropped, never altered.
41-43 and 46 were captured directly with m-shot.mjs / m-vals-filter.mjs (anonymous headless Chrome, 700 CSS px, DPR 2).
Run: uv run --with pillow python -I m-raw-crops.py <images dir>"""
from PIL import Image
import sys, os
os.chdir(sys.argv[1])
# 44 Anthropic: "upper bound" paragraph incl. the Mirai caveat (B group 16, whole)
Image.open('16-anthropic-upper-bound.png').convert('RGB').save('44-anthropic-upper-bound-raw.png')
# 45 z.ai: benchmark chart + Harvey LAB bullet (C group 26, rows 200..1075)
Image.open('26-zai-benchmark-chart.png').convert('RGB').crop((0, 200, 1400, 1080)).save('45-zai-reliable-raw.png')
