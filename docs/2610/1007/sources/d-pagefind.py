# usage: python3 d-pagefind.py <file.txt (pdftotext output with form feeds)> "<needle>" ...
# Prints the PDF page number(s) (1-based) on which each needle occurs (whitespace-normalised).
import re,sys
t=open(sys.argv[1],encoding='utf8').read().split('\f')
norm=lambda s:re.sub(r'\s+',' ',s)
for n in sys.argv[2:]:
    nn=norm(n)
    pages=[i+1 for i,p in enumerate(t) if nn in norm(p)]
    print(repr(n[:70]),'->',pages)
