"""Build annotation base images (-raw) for 1010 from evidence screenshots.
Pixels are only cropped, never altered. Run: uv run --with pillow python -I m-raw-crops.py <images dir>"""
from PIL import Image
import sys, os
os.chdir(sys.argv[1])

def crop(src, box):
    return Image.open(src).convert('RGB').crop(box)

def whole(src):
    return Image.open(src).convert('RGB')

# 41 OpenAI report: header + Summary, 01 (whole)
whole('01-report-summary.png').save('41-openai-summary-raw.png')
# 42 OpenAI report: container program, tool server, system directories, 03 (lower part)
crop('03-delete-python-container-sysdirs.png', (0, 1285, 1400, 2314)).save('42-openai-deletion-raw.png')
# 43 OpenAI report: later retry + Investigation and response, 04 (whole)
whole('04-retry-and-response.png').save('43-openai-response-raw.png')
# 44 Shane Mac thread, posts 2-3 (post card only), 21 (whole)
whole('21-mac-why-connect-bank-card.png').save('44-mac-thread-raw.png')
# 45 Grok Bot docs: shared-computer boundary, 25 (whole; re-captured at 480 CSS px for legibility)
whole('25-docs-boundary-narrow.png').save('45-docs-boundary-raw.png')
# 46 Grok Bot docs: Connect an app ... account-wide, 26 (whole; re-captured at 480 CSS px)
whole('26-docs-connectors-narrow.png').save('46-docs-connectors-raw.png')
# 47 Business Insider as-told-to: same connections + fix claim, 15 (whole)
whole('15-bi-exec-team-same-connections.png').save('47-bi-fix-claim-raw.png')
