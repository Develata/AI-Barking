from pathlib import Path
import subprocess
pdf=Path('docs/2610/1004/sources/c-tech-report.pdf')
out=Path('docs/2610/1004/images')
pages={
 1:'17-tech-report-abstract.png',
 5:'18-tech-report-main-claims.png',
 41:'19-tech-report-context-extrapolation.png',
 45:'20-tech-report-baseline-protocol.png',
 100:'21-tech-report-posttraining-table-1.png',
 101:'22-tech-report-posttraining-table-2.png',
 102:'23-tech-report-customer-proxies.png',
 103:'24-tech-report-grounding-figure.png',
}
for page,name in pages.items():
 base=out/Path(name).stem
 r=subprocess.run(['pdftoppm','-f',str(page),'-l',str(page),'-singlefile','-scale-to-x','1400','-scale-to-y','-1','-png',str(pdf),str(base)],capture_output=True,text=True)
 p=base.with_suffix('.png')
 print(f'{page} / 189 -> {p} rc={r.returncode} bytes={p.stat().st_size if p.exists() else 0}')
 if r.returncode:
  print(r.stderr)
