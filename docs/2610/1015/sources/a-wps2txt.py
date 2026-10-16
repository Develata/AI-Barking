"""a-wps2txt.py: extract plain text from a Word97-format .wps/.doc (OLE2 WordDocument + piece table).
usage: python -I a-wps2txt.py in.wps out.txt   (needs olefile)"""
import sys, struct, olefile
o = olefile.OleFileIO(sys.argv[1])
wd = o.openstream('WordDocument').read()
flags = struct.unpack_from('<H', wd, 0xA)[0]
tbl = o.openstream('1Table' if flags & 0x200 else '0Table').read()
# FIB: fcClx at 0x1A2, lcbClx at 0x1A6 (Word97 FibRgFcLcb97 index 33)
fcClx, lcbClx = struct.unpack_from('<II', wd, 0x1A2)
clx = tbl[fcClx:fcClx + lcbClx]
i = 0
while clx[i] == 1:
    i += 3 + struct.unpack_from('<H', clx, i + 1)[0]
assert clx[i] == 2
n = struct.unpack_from('<I', clx, i + 1)[0]
plc = clx[i + 5:i + 5 + n]
cnt = (n - 4) // 12
cps = struct.unpack_from('<%dI' % (cnt + 1), plc, 0)
out = []
for k in range(cnt):
    pcd = struct.unpack_from('<HIH', plc, 4 * (cnt + 1) + 8 * k)
    fc = pcd[1]
    ln = cps[k + 1] - cps[k]
    if fc & 0x40000000:
        fc = (fc & 0x3FFFFFFF) // 2
        out.append(wd[fc:fc + ln].decode('cp1252', 'replace'))
    else:
        out.append(wd[fc:fc + ln * 2].decode('utf-16-le', 'replace'))
t = ''.join(out).replace('\r', '\n').replace('\x07', '\t').replace('\x0b', '\n')
open(sys.argv[2], 'w', encoding='utf-8').write(t)
print(len(t), 'chars')
