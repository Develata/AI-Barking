# Derive 1400-px-wide PNGs from ORIGINAL official chart images (no redraw, only Lanczos downscale). Usage: python b-chartimg.py <src> <dst>
import sys,hashlib
from PIL import Image
src,dst=sys.argv[1:3]
im=Image.open(src).convert('RGB')
w=1400;h=round(im.height*w/im.width)
im.resize((w,h),Image.LANCZOS).save(dst)
print(dst,(w,h),'src',src,im.size,hashlib.sha256(open(src,'rb').read()).hexdigest())
