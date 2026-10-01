from PIL import Image
from pathlib import Path
p=Path('docs/0929')
items=[('a-robb-2104090601411293303-1.jpg','09-robb-agent-chat.png'),('a-robb-2104396139587879234-1.jpg','10-robb-buyer-chat.png'),('a-robb-2104396139587879234-2.jpg','11-robb-buyer-rating.png')]
for src,dst in items:
 im=Image.open(p/'sources'/src); im.save(p/'images'/dst);print(dst,im.size)
