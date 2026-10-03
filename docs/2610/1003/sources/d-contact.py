from PIL import Image,ImageOps,ImageDraw
from pathlib import Path
p=Path(__file__).parent; files=sorted((p.parent/'images').glob('*.png'));sheet=Image.new('RGB',(1200,((len(files)+3)//4)*500),'#ddd');draw=ImageDraw.Draw(sheet)
for i,f in enumerate(files):
 im=Image.open(f);im.thumbnail((290,465));x=(i%4)*300;y=(i//4)*500;sheet.paste(im,(x,y+25));draw.text((x+4,y+4),f.name,fill='black')
sheet.save(p/'d-screenshot-contact.jpg')
