Add-Type -AssemblyName System.Drawing
$base = 'E:/gitclone/AI-Barking/docs/0928'
function Join-Crops($out, $parts) {
  $width=0; $height=0
  foreach($p in $parts){$width=[Math]::Max($width,$p[3]);$height+=$p[4]}
  $height+=2*($parts.Count-1)
  $bmp=[Drawing.Bitmap]::new($width,$height)
  $g=[Drawing.Graphics]::FromImage($bmp);$g.Clear([Drawing.Color]::White);$y=0
  foreach($p in $parts){
    $im=[Drawing.Image]::FromFile("$base/$($p[0])")
    $src=[Drawing.Rectangle]::new($p[1],$p[2],$p[3],$p[4])
    $dst=[Drawing.Rectangle]::new(0,$y,$p[3],$p[4])
    $g.DrawImage($im,$dst,$src,[Drawing.GraphicsUnit]::Pixel);$im.Dispose();$y+=$p[4]
    if($y -lt $height){$g.FillRectangle([Drawing.Brushes]::Gray,0,$y,$width,2);$y+=2}
  }
  $g.Dispose();$bmp.Save("$base/images/$out",[Drawing.Imaging.ImageFormat]::Png);$bmp.Dispose()
}
Join-Crops '01-sonnet-benchmarks.png' @(@('sources/a-title.png',180,280,900,160),@('sources/a-benchmark.png',180,40,900,470))
Join-Crops '02-sonnet-footnotes.png' @(@('sources/a-safeguards-direct.png',300,425,650,230),@('sources/a-notes-good.png',300,115,650,580))
Join-Crops '03-casp-title-authors.png' @(,@('sources/b-casp-viewport.png',25,20,825,600))
Join-Crops '06-mimo-repetition-rates.png' @(@('sources/c-mimo-full.png',200,115,880,965),@('sources/c-mimo-full.png',200,1555,880,380))
Join-Crops '07-mimo-rl-worsening.png' @(,@('sources/c-mimo-full.png',200,1935,880,655))
Join-Crops '08-mimo-cost.png' @(@('sources/c-mimo-full.png',200,3720,880,260),@('sources/c-mimo-full.png',200,8230,880,130))
Join-Crops '09-muse-inc-datasource.png' @(@('sources/d-inc-full.png',215,615,820,370),@('sources/d-inc-full.png',215,5300,820,190))
Join-Crops '10-muse-meta-promise.png' @(@('sources/d-meta-full.png',30,270,890,160),@('sources/d-meta-full.png',30,3750,890,88))
Join-Crops '04-casp-caveats.png' @(@('sources/b-paper-render-06.png',75,70,417,590),@('sources/b-paper-render-06.png',500,750,417,250))
Join-Crops '05-casp-millions.png' @(@('sources/b-paper-render-04.png',500,1020,417,275),@('sources/b-paper-render-05.png',75,697,417,205))
Join-Crops '11-overclaim-a-bilibili.png' @(,@('sources/e-a-bilibili-full.png',35,75,1170,82))
Join-Crops '11-overclaim-b-axios.png' @(,@('sources/e-b-axios-full.png',260,360,740,235))
Join-Crops '11-overclaim-c-chan.png' @(,@('sources/e-c-chan-full.png',240,2015,710,205))
Join-Crops '11-overclaim-d-icwork.png' @(@('sources/e-d-icwork-full.png',65,92,760,160),@('sources/e-d-icwork-full.png',75,715,745,177))

