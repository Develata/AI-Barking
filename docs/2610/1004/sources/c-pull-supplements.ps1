$ErrorActionPreference = 'Stop'
$dest = 'docs/2610/1004/sources'
$captures = @(
    @{ Name = 'c-hn-firebase-49942706.json'; Url = 'https://hacker-news.firebaseio.com/v0/item/49942706.json' },
    @{ Name = 'c-hn-firebase-49943034.json'; Url = 'https://hacker-news.firebaseio.com/v0/item/49943034.json' },
    @{ Name = 'c-hn-firebase-49946069.json'; Url = 'https://hacker-news.firebaseio.com/v0/item/49946069.json' },
    @{ Name = 'c-olmo3.html'; Url = 'https://allenai.org/blog/olmo3' },
    @{ Name = 'c-apertus.html'; Url = 'https://www.cscs.ch/science/computer-science-hpc/2025/apertus-a-fully-open-transparent-multilingual-language-model' },
    @{ Name = 'c-nemotron3.html'; Url = 'https://research.nvidia.com/labs/nemotron/Nemotron-3/' },
    @{ Name = 'c-nemotron3-super.html'; Url = 'https://research.nvidia.com/labs/nemotron/Nemotron-3-Super/' },
    @{ Name = 'c-qwen35.html'; Url = 'https://qwen.ai/blog?id=qwen3.5' },
    @{ Name = 'c-qwen36.html'; Url = 'https://github.com/AlibabaCloud-Official/Qwen3.6' },
    @{ Name = 'c-qwen38-api.json'; Url = 'https://huggingface.co/api/models/Qwen/Qwen3.8-27B' },
    @{ Name = 'c-glm45.html'; Url = 'https://z.ai/blog/glm-4.5' },
    @{ Name = 'c-gemma4.html'; Url = 'https://opensource.googleblog.com/2026/03/gemma-4-expanding-the-gemmaverse-with-apache-20.html' },
    @{ Name = 'c-cohere-agreement.html'; Url = 'https://aleph-alpha.com/en/news/cohere-agreement-transatlantic-sovereign-ai/' },
    @{ Name = 'c-swr-kolibri.html'; Url = 'https://www.tagesschau.de/inland/regional/badenwuerttemberg/swr-heidelberger-ki-unternehmen-entwickelt-sprachmodell-und-wirbt-mit-ki-souveraenitaet-made-in-germany-100.html' },
    @{ Name = 'c-marktechpost-kolibri.html'; Url = 'https://www.marktechpost.com/2026/10/04/aleph-alpha-releases-kolibri-a-78-1b-open-weight-english-german-moe-model-with-only-3-46b-active-parameters/' },
    @{ Name = 'c-cleverhack-kolibri.html'; Url = 'https://cleverhack.com/the-urgency-of-open-source-ai' }
)
foreach ($entry in $captures) {
    try {
        $response = Invoke-WebRequest -Uri $entry.Url -UseBasicParsing -TimeoutSec 35
        [System.IO.File]::WriteAllText((Join-Path $dest $entry.Name), $response.Content, [System.Text.UTF8Encoding]::new($false))
        '{0} {1} bytes={2} url={3}' -f $entry.Name, $response.StatusCode, $response.RawContentLength, $entry.Url
    }
    catch {
        '{0} FAILED {1} url={2}' -f $entry.Name, $_.Exception.Message, $entry.Url
    }
}
'capture_finished_utc={0:o}' -f [DateTime]::UtcNow
