$ErrorActionPreference='Stop'
$EvidenceRoot='E:/gitclone/AI-Barking/docs/1001'
function Record-Capture($url,$result,$file){
 $stamp=[DateTimeOffset]::UtcNow.ToOffset([TimeSpan]::FromHours(8)).ToString('o')
 @{time=$stamp;url=$url;tool='opencli browser';result=$result;file=$file}|ConvertTo-Json -Compress|Add-Content "$EvidenceRoot/sources/d-browser-log.jsonl"
}
function Archive-Page($name,$url){
 opencli browser evidence1001 open $url | Out-Null
 opencli browser evidence1001 state | Out-Null
 opencli browser evidence1001 extract --chunk-size 100000 | Set-Content "$EvidenceRoot/sources/$name-extract.json"
 opencli browser evidence1001 eval 'JSON.stringify({meta:[...document.querySelectorAll("meta[property],time")].map(e=>e.outerHTML),headings:[...document.querySelectorAll("h1,h2,h3")].map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY})),images:[...document.images].map(e=>({src:e.currentSrc,alt:e.alt,y:e.getBoundingClientRect().top+scrollY,h:e.height})),paragraphs:[...document.querySelectorAll("main p,article p")].map(e=>({text:e.innerText,y:e.getBoundingClientRect().top+scrollY,h:e.getBoundingClientRect().height}))})' | Set-Content "$EvidenceRoot/sources/$name-layout.json"
 Record-Capture $url '页面提取完成；内容有效性见 evidence.md' "$name-extract.json; $name-layout.json"
 Write-Output $name
}
function Capture-Region($name,$y,$height){
 opencli browser evidence1001 scroll up --amount 100000 | Out-Null
 opencli browser evidence1001 scroll down --amount $y | Out-Null
 opencli browser evidence1001 screenshot "$EvidenceRoot/images/$name" --width 1266 --height $height | Out-Null
 $url=opencli browser evidence1001 get url
 Record-Capture $url '截图写盘；内容需视觉复核' $name
 Write-Output $name
}
function Save-PublicAsset($url,$file){
 opencli browser pdf1001 open $url | Out-Null
 $quoted=ConvertTo-Json $url -Compress
 $js='fetch('+ $quoted + ',{credentials:"omit"}).then(r=>{if(!r.ok)throw Error(r.status);return r.blob()}).then(b=>new Promise(resolve=>{const f=new FileReader();f.onload=()=>resolve(f.result);f.readAsDataURL(b)}))'
 $data=opencli browser pdf1001 eval $js
 if($data -match '^data:.*;base64,'){
  [IO.File]::WriteAllBytes("$EvidenceRoot/sources/$file",[Convert]::FromBase64String(($data -split ',',2)[1]))
  Record-Capture $url '成功；浏览器无凭据原站资源下载' $file
 }else{Record-Capture $url '失败；未取得资源字节' $file}
}
function Scale-Image($source,$target){
 Add-Type -AssemblyName System.Drawing
 $im=[Drawing.Image]::FromFile("$EvidenceRoot/sources/$source")
 $w=[Math]::Min(1400,$im.Width);$bm=[Drawing.Bitmap]::new($w,[int]($im.Height*$w/$im.Width))
 $gr=[Drawing.Graphics]::FromImage($bm);$gr.InterpolationMode=[Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
 $gr.DrawImage($im,0,0,$bm.Width,$bm.Height);$bm.Save("$EvidenceRoot/images/$target",[Drawing.Imaging.ImageFormat]::Png)
 $gr.Dispose();$bm.Dispose();$im.Dispose()
}
