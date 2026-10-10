"""Build annotation base images (-raw) for 1009 from evidence screenshots.
Pixels are only cropped/stacked, never altered. Stacks use a 24 px gap in the page background colour."""
from PIL import Image
import sys, os
os.chdir(sys.argv[1])

def crop(src, box):
    return Image.open(src).convert('RGB').crop(box)

def stack(parts, bg):
    w = max(p.width for p in parts); gap = 24
    h = sum(p.height for p in parts) + gap * (len(parts) - 1)
    out = Image.new('RGB', (w, h), bg); y = 0
    for p in parts:
        out.paste(p, (0, y)); y += p.height + gap
    return out

# 41 Anthropic report: third form example (police tip), 02
crop('02-report-forms-philly.png', (0, 915, 1400, 1600)).save('41-report-tip-raw.png')
# 42 PPD statement via 6abc, 05: spam / timeline / unacceptable
bg = Image.open('05-ppd-statement-6abc.png').convert('RGB').getpixel((5, 5))
stack([crop('05-ppd-statement-6abc.png', (0, 1092, 1400, 1218)),
       crop('05-ppd-statement-6abc.png', (0, 1750, 1400, 2070)),
       crop('05-ppd-statement-6abc.png', (0, 3430, 1400, 3600))], bg).save('42-ppd-statement-raw.png')
# 43 Anthropic remediation, 03 (whole)
Image.open('03-report-remediation.png').convert('RGB').save('43-report-remediation-raw.png')
# 44 lawmakers' letter, 15 (whole)
Image.open('15-letter-numbers.png').convert('RGB').save('44-letter-numbers-raw.png')
# 45 ombudsman supplemental report, 18: conclusion + footnote 3
crop('18-ombudsman-conclusion.png', (0, 1015, 1224, 1344)).save('45-ombudsman-raw.png')
# 46 Anthropic UV post, 25: map + caption
crop('25-hero-caption.png', (0, 680, 1400, 1760)).save('46-uv-caption-raw.png')
# 47 technical page, 29: error sentence + peer-review sentence
bg = Image.open('29-uvmap-trust.png').convert('RGB').getpixel((5, 5))
stack([crop('29-uvmap-trust.png', (0, 0, 1370, 380)),
       crop('29-uvmap-trust.png', (0, 1265, 1370, 1594))], bg).save('47-uv-trust-raw.png')
