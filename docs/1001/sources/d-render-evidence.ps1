$ErrorActionPreference='Stop'
$root='E:/gitclone/AI-Barking/docs/1001'
Add-Type -AssemblyName System.Drawing
function Pages($pdf,$pages,$target){
 $paths=@()
 foreach($n in $pages){
  $prefix="$root/sources/$pdf-page-$n"
  pdftoppm -f $n -l $n -singlefile -scale-to 1700 -png "$root/sources/$pdf.pdf" $prefix
  $paths+= "$prefix.png"
 }
 $ims=@($paths | ForEach-Object {[Drawing.Image]::FromFile($_)})
 $width=($ims | Measure-Object Width -Maximum).Maximum
 $height=($ims | Measure-Object Height -Sum).Sum
 $bm=[Drawing.Bitmap]::new([int]$width,[int]$height);$g=[Drawing.Graphics]::FromImage($bm);$g.Clear([Drawing.Color]::White)
 $y=0;foreach($im in $ims){$g.DrawImage($im,0,$y,$im.Width,$im.Height);$y+=$im.Height;$im.Dispose()}
 $bm.Save("$root/images/$target",[Drawing.Imaging.ImageFormat]::Png);$g.Dispose();$bm.Dispose()
}
Pages 'b-update' @(1) '28-research-update-title.png'
Pages 'b-update' @(3,4) '29-research-update-timeline.png'
Pages 'c-anthropic' @(1,2) '30-robots-title-findings.png'
Pages 'c-anthropic' @(4,5) '31-robots-rubric.png'
Pages 'c-anthropic' @(6,16) '32-robots-method.png'
Pages 'c-anthropic' @(7) '33-robots-figure2.png'
Pages 'c-anthropic' @(8,9) '34-robots-figure3.png'
Pages 'c-anthropic' @(15) '35-robots-capabilities.png'
Pages 'c-anthropic' @(17) '36-robots-cost-example.png'
Pages 'c-anthropic' @(18) '37-robots-figure7.png'
Pages 'c-anthropic' @(19,20) '38-robots-cost-decline.png'
Pages 'c-anthropic' @(22) '39-robots-footnote.png'
