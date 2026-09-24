import urllib.request,zipfile,io,pathlib
out=pathlib.Path('0924/sources/usage')
b=urllib.request.urlopen('https://epoch.ai/data/benchmark_data.zip',timeout=90).read()
z=zipfile.ZipFile(io.BytesIO(b));print(z.namelist())
for name in z.namelist():
 if name.endswith('.csv'):
  (out/('epoch-'+pathlib.Path(name).name)).write_bytes(z.read(name))
