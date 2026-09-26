$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$root = 'E:/gitclone/AI-Barking/docs/0927'
function Crop-Parts($source, $dest, $rects) {
  $src = [System.Drawing.Image]::FromFile("$root/sources/$source")
  if ($rects[0] -is [int]) { $rects = ,$rects }
  $width = ($rects | ForEach-Object { $_[2] } | Measure-Object -Maximum).Maximum
  $height = ($rects | ForEach-Object { $_[3] } | Measure-Object -Sum).Sum + 2 * ($rects.Count - 1)
  $bmp = [System.Drawing.Bitmap]::new([int]$width,[int]$height)
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.Clear([System.Drawing.Color]::White)
  $y = 0
  foreach ($r in $rects) {
    $g.DrawImage($src,[System.Drawing.Rectangle]::new(0,$y,$r[2],$r[3]),[System.Drawing.Rectangle]::new($r[0],$r[1],$r[2],$r[3]),[System.Drawing.GraphicsUnit]::Pixel)
    $y += $r[3]
    if ($y -lt $height) { $g.FillRectangle([System.Drawing.Brushes]::Gray,0,$y,$width,2); $y += 2 }
  }
  $bmp.Save("$root/images/$dest",[System.Drawing.Imaging.ImageFormat]::Png)
  $g.Dispose(); $bmp.Dispose(); $src.Dispose()
}
Crop-Parts 'a-openai-dns-full.png' '01-openai-dns-pause.png' @(@(285,120,660,525))
Crop-Parts 'a-openai-dns-full.png' '02-openai-dns-timeline.png' @(@(285,3710,660,640))
Crop-Parts 'a-openai-third-parties-en-full.png' '03-openai-third-parties.png' @(@(215,140,820,205),@(215,1978,820,155))
Crop-Parts 'a-openai-third-parties-en-full.png' '03b-openai-53-official.png' @(@(240,3300,780,735))
Crop-Parts 'a-techcrunch-full.png' '04-techcrunch-53-images.png' @(@(630,620,620,330),@(105,1190,705,375))
Crop-Parts 'b-anthropic-full.png' '05-anthropic-toy-model.png' @(@(235,150,780,155),@(235,2985,780,190))
Crop-Parts 'b-anthropic-full.png' '06-anthropic-known-methods.png' @(@(290,5180,670,220))
Crop-Parts 'b-anthropic-full.png' '07-anthropic-cost.png' @(@(290,4545,670,250))
Crop-Parts 'b-anthropic-full.png' '08-anthropic-song-he.png' @(@(290,4790,670,270),@(290,8335,670,190))
Crop-Parts 'b-overclaim-36kr-full.png' '10-overclaim-b-36kr.png' @(@(180,55,730,220))


Crop-Parts 'a-overclaim-taisounds-full.png' '10-overclaim-a-taisounds.png' @(@(0,235,940,260))
Crop-Parts 'a-overclaim-tnw-full.png' '10-overclaim-a-tnw.png' @(@(15,170,1220,300))
Crop-Parts 'b-overclaim-blockchain-full.png' '10-overclaim-b-blockchain.png' @(@(100,305,785,175))

Crop-Parts 'b-song-he-full.png' '09-song-he-dataset.png' @(@(250,100,750,320))
