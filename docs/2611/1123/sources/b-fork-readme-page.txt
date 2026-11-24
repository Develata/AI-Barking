nari-llama.cpp/README.md at nariq/b12431 · nari-labs/nari-llama.cpp · GitHub

 Skip to content [#start-of-content]

Navigation Menu

 [/]

Sign in [/login?return_to=https%3A%2F%2Fgithub.com%2Fnari-labs%2Fnari-llama.cpp%2Fblob%2Fnariq%2Fb12431%2FREADME.md]

Products

Platform

GitHubRepository hosting, code review, and collaboration [https://github.com/platform]

CopilotAI coding agent for tasks, projects, and automated work [https://github.com/features/copilot]

Build & Ship

ActionsCI/CD and automation [https://github.com/features/actions]

Code QualityQuality analysis, fixes, and enforcement [https://github.com/features/code-quality]

CodespacesCloud development environments [https://github.com/features/codespaces]

Secure

Code SecurityVulnerability detection and remediation [https://github.com/security/advanced-security/code-security]

Secret ProtectionSecret scanning and leak prevention [https://github.com/security/advanced-security/secret-protection]

All Features [https://github.com/features]

Developers

Learn

Documentation [https://docs.github.com/en]

Training [https://learn.github.com/skills]

Certifications [https://learn.github.com/credentials]

Build

Build with Copilot [https://github.com/features/ai]

Customize Copilot [https://docs.github.com/en/copilot/how-tos/copilot-on-github/customize-copilot/customize-copilot-overview]

Integrations [https://github.com/marketplace]

Explore

Trending repos [https://github.com/trending]

GitHub Sponsors [https://github.com/open-source/sponsors]

Developer articles [https://github.com/resources/articles]

Changelog

ChangelogYour source for the latest features, improvements, and fixes across GitHub products, all in one place. [https://github.blog/changelog/]

Businesses

By Stage

Enterprises [https://github.com/enterprise]

Team [https://github.com/team]

Startups [https://github.com/enterprise/startups]

Nonprofits [https://github.com/solutions/industry/nonprofits]

By Industry

Healthcare [https://github.com/solutions/industry/healthcare]

Financial services [https://github.com/solutions/industry/financial-services]

Manufacturing [https://github.com/solutions/industry/manufacturing]

Government [https://github.com/solutions/industry/government]

See all industries [https://github.com/solutions/industry]

Evaluate GitHub

Why GitHub [https://github.com/why-github]

Customer Stories [https://github.com/customer-stories]

Business insights [https://github.com/solutions/executive-insights]

Trust Center [https://github.com/trust-center]

Add-ons

GitHub Advanced SecurityEnterprise-grade code security [https://github.com/security/advanced-security]

Copilot for businessEnterprise-grade AI features [https://github.com/features/copilot/copilot-business]

Premium SupportEnterprise-grade 24/7 support [https://github.com/enterprise/premium-support]

Resources

Learn & Connect

Documentation [https://docs.github.com/en]

Resource center [https://github.com/resources]

Blog [https://github.blog/]

Events & webinars [https://github.com/resources/events]

Community Forum [https://github.com/orgs/community/discussions]

Customer Stories [https://github.com/customer-stories]

Integrations [https://github.com/marketplace]

Support

Customer support [https://help.github.com/]

Training [https://learn.github.com/skills]

GitHub Status [https://www.githubstatus.com/]

Company

About [https://github.com/about]

Partners [https://github.com/partners]

Changelog [https://github.blog/changelog/]

GitHub Universe

GitHub UniverseThe event for developers, by developers — talks, workshops, and what’s next for GitHub. [https://githubuniverse.com/]

Pricing [https://github.com/pricing]

Search/

Sign in [/login?return_to=https%3A%2F%2Fgithub.com%2Fnari-labs%2Fnari-llama.cpp%2Fblob%2Fnariq%2Fb12431%2FREADME.md]
Sign up [/signup?ref_cta=Sign+up&ref_loc=header+logged+out&ref_page=%2F%3Cuser-name%3E%2F%3Crepo-name%3E%2Fblob%2Fshow&source=header-repo&source_repo=nari-labs%2Fnari-llama.cpp]
Appearance settings

 You signed in with another tab or window. Reload to refresh your session.
 You signed out in another tab or window. Reload to refresh your session.
 You switched accounts on another tab or window. Reload to refresh your session.

Dismiss alert

{{ message }}

nari-labs [/nari-labs]

nari-llama.cpp [/nari-labs/nari-llama.cpp]

Repository navigation

Code [/nari-labs/nari-llama.cpp]

Issues [/nari-labs/nari-llama.cpp/issues]

Pull requests1 (1) [/nari-labs/nari-llama.cpp/pulls]

Actions [/nari-labs/nari-llama.cpp/actions]

Security and quality [/nari-labs/nari-llama.cpp/security]

Insights [/nari-labs/nari-llama.cpp/insights]

More items

 [/nari-labs/nari-llama.cpp] 

FilesExpand file tree

nariq/b12431

Breadcrumbs

nari-llama.cpp [/nari-labs/nari-llama.cpp/tree/nariq/b12431]

/
README.md

Copy path

Blame

More file actions

Blame

More file actions

Latest commit

History
History [/nari-labs/nari-llama.cpp/commits/nariq/b12431/README.md]

 [/nari-labs/nari-llama.cpp/commits/nariq/b12431/README.md]History

163 lines (119 loc) · 8.61 KB

nariq/b12431

Breadcrumbs

nari-llama.cpp [/nari-labs/nari-llama.cpp/tree/nariq/b12431]

/
README.md

Copy path

Top

File metadata and controls

Preview

Code

Blame

163 lines (119 loc) · 8.61 KB

Raw [https://github.com/nari-labs/nari-llama.cpp/raw/refs/heads/nariq/b12431/README.md]

Copy raw file

Download raw file

Outline
Edit and raw actions

nari-llama.cpp
 [#nari-llamacpp]

The Nari Labs fork of llama.cpp [https://github.com/ggml-org/llama.cpp] that runs the NariQ trellis-coded
Qwen3.8-27B GGUFs [https://huggingface.co/nari-labs/Qwen3.8-27B-NariQ-GGUF] at 3.0, 2.5 and 2.0 bits per weight.
Stock llama.cpp, Ollama and LM Studio cannot load these files yet. Based on upstream b12431 (a946eb5).

(Nov 20, 2026) First release, b12431-nariv0 [https://github.com/nari-labs/nari-llama.cpp/releases/tag/b12431-nariv0]:
NariQ v1 GGUFs of Qwen3.8-27B at 3.0, 2.5 and 2.0 bits per weight.

Performance
 [#performance]

Qwen3.8-27B
Size
DeepSWE solved
DeepSWE score
Terminal-Bench 2.1
AIME 25+26
RealWorldQA
MGSM

BF16
53.8 GB
44
83.0
62.6
97.7
82.4
94.0

NariQ 3.0bpw
10.1 GB
47
84.9
63.8
98.3
82.4
93.1

NariQ 2.5bpw
8.4 GB
46
84.5
60.2
96.5
81.3
91.8

NariQ 2.0bpw
6.8 GB
33
75.7
52.4
86.0
80.9
90.6

DeepSWE v1.1: tasks fully solved of 113, and the mean share of each task's new tests that pass (%), one attempt per
task. Terminal-Bench 2.1: mean test reward over 82 tasks × 3 attempts (%). Both use Claude Code driving the model.
AIME, RealWorldQA and MGSM: % correct. Details are in the technical report [https://arxiv.org/abs/2611.26157] and on the
model card [https://huggingface.co/nari-labs/Qwen3.8-27B-NariQ-GGUF].

Decode on one RTX PRO 6000, one session, at 4K / 64K tokens of context: 88.9 / 70.7 (3.0bpw), 99.2 / 77.5 (2.5bpw) and
111.7 / 85.0 (2.0bpw) tokens/s.

Quick start (Linux x86-64, NVIDIA GPU)
 [#quick-start-linux-x86-64-nvidia-gpu]

curl -fsSLO https://github.com/nari-labs/nari-llama.cpp/releases/download/b12431-nariv0/run.sh
bash run.sh 3.0 # or 2.5, 2.0

run.sh downloads the binaries and model once, checks every sha256, fits the context to free GPU memory and serves a
chat page and an OpenAI-compatible API at http://127.0.0.1:8080 [http://127.0.0.1:8080]. Options: --port N, --no-vision, --docker,
--download-only; arguments after -- go to llama-server (bash run.sh --help). Files go to
~/.cache/nari-llama.cpp, about 13 GB with the 3.0bpw file. With the hf CLI installed (pip install -U huggingface_hub), the model files download through it, in parallel and often much faster than one curl stream, and
are still checked against their sha256; TRELLIS_NO_HF=1 turns this off.

It needs an NVIDIA driver of 570 or newer, glibc 2.34 or newer, libstdc++ with GLIBCXX_3.4.30 and libgomp (Ubuntu
22.04 or newer, Debian 12 or newer). On RHEL 9, Rocky 9 or Amazon Linux 2023, use bash run.sh 3.0 --docker.

A 24 GB GPU fits about 135K / 160K / 184K tokens of context with the 3.0 / 2.5 / 2.0bpw file and the vision projector;
the full 262K context needs about 30 / 28 / 27 GB.

Windows (experimental)
 [#windows-experimental]

Download run.ps1 from the release [https://github.com/nari-labs/nari-llama.cpp/releases/tag/b12431-nariv0] and
run it in PowerShell (NVIDIA driver 572 or newer):

powershell -ExecutionPolicy Bypass -File run.ps1 -Budget 3.0 # or 2.5, 2.0

It downloads the native Windows build and the model, checks every sha256 and serves the chat page and API at
http://127.0.0.1:8080 [http://127.0.0.1:8080]. Manual setup: README-windows.md [/nari-labs/nari-llama.cpp/blob/nariq/b12431/nariq/README-windows.md].

On Windows, close to 1.5 GB of GPU memory can go to the desktop, which shortens the fitted context. For example, an
RTX 5060 Ti 16 GB fits about 47K tokens of context with the 2.5bpw file and the vision projector.

The Windows binaries are not code-signed yet, so Windows 11 blocks them when Smart App Control is on. To run them, turn
it off in Windows Security > App & browser control > Smart App Control settings (on many Windows 11 versions it cannot
be turned back on without resetting Windows), or use the WSL2 workaround below.

Workaround: keep Smart App Control on and run the Linux build in WSL2

Set up a distribution once with wsl --install (Ubuntu 22.04 or newer), open it, and run the Linux quick start:

curl -fsSLO https://github.com/nari-labs/nari-llama.cpp/releases/download/b12431-nariv0/run.sh
bash run.sh 3.0 # or 2.5, 2.0

Files go to ~/.cache/nari-llama.cpp inside WSL, and the server is reachable from Windows at http://127.0.0.1:8080 [http://127.0.0.1:8080].
In one test it decoded about 10% slower than the native build. From the next release, run.ps1 -Wsl does this from
PowerShell.

Manual setup
 [#manual-setup]

mkdir nari-llama.cpp
tar -xzf nari-llama.cpp-b12431-nariv0-bin-linux-cuda-12.8-x64.tar.gz --strip-components=1 -C nari-llama.cpp
tar -xzf cudart-nari-llama.cpp-b12431-nariv0-linux-cuda-12.8-x64.tar.gz --strip-components=1 -C nari-llama.cpp
hf download nari-labs/Qwen3.8-27B-NariQ-GGUF \
 Qwen3.8-27B-NariQ-3.0bpw.gguf Qwen3.8-27B-NariQ-mmproj-BF16.gguf --local-dir . # pip install -U huggingface_hub
./nari-llama.cpp/llama-server -m Qwen3.8-27B-NariQ-3.0bpw.gguf --mmproj Qwen3.8-27B-NariQ-mmproj-BF16.gguf \
 --fit-target 2560 --port 8080 --temp 1.0 --top-p 0.95 --top-k 20 --min-p 0

The archives are on the release page [https://github.com/nari-labs/nari-llama.cpp/releases/tag/b12431-nariv0].
Without -c, the context is fitted to free GPU memory (--fit-target 2560 keeps room for the vision projector); add
-c N to cap it. Keep --port 8080: this version otherwise listens on 9931. The sampling flags are the thinking-mode
settings the scores used.

Docker
 [#docker]

With the NVIDIA Container Toolkit, from the folder that holds the GGUFs:

docker run --rm --gpus all -p 127.0.0.1:8080:8080 -v "$PWD:/models:ro" ghcr.io/nari-labs/nari-llama.cpp:b12431-nariv0 \
 -m /models/Qwen3.8-27B-NariQ-3.0bpw.gguf --mmproj /models/Qwen3.8-27B-NariQ-mmproj-BF16.gguf \
 --fit-target 2560 --min-p 0 --host 0.0.0.0 --port 8080

Hardware
 [#hardware]

Hardware
Status

NVIDIA, compute capability 8.0+, Linux
Tested on RTX PRO 6000, H100, L4 and A10G. Native code for 8.6, 8.9, 9.0 and 12.0; other cards compile PTX on first start (about 50 s).

NVIDIA, Windows
Experimental. Run on a GeForce RTX 5060 Ti (Windows 11), not qualified yet. Blocked by Smart App Control when it is on (not code-signed yet).

NVIDIA GeForce
One card run so far, not qualified: RTX 5060 Ti 16 GB, 2.5bpw with the vision projector, about 38 tokens/s decode on Windows (512 tokens, short prompt).

NVIDIA, Linux ARM64 (GH200, DGX Spark)
No release binaries; build from source, untested.

Apple Silicon
Experimental, build from source, about 3.8 (3.0bpw) to 4.9 (2.0bpw) tokens/s on an M3 Pro (Metal notes [/nari-labs/nari-llama.cpp/blob/nariq/b12431/nariq/README-metal.md]).

CPU
Every coded operation runs on the CPU, slowly for a 27B model.

AMD, Intel GPUs
No NariQ kernels.

Kernel tuning has focused on the RTX PRO 6000; on other GPUs NariQ files are slower than comparable stock GGUFs for
now. Coded embedding lookups run on the CPU. LoRA adapters on NariQ weights are rejected.

Build from source
 [#build-from-source]

Needs CMake 3.18 or newer, a C++17 compiler and, for CUDA, the CUDA toolkit (the releases use 12.8).

On Apple Silicon, run bash nariq/run-mac.sh 2.0 from the checkout to verify the weights, build the Metal server and start local chat. See the Mac quick start [/nari-labs/nari-llama.cpp/blob/nariq/b12431/nariq/README-metal.md] for prerequisites and options.

git clone --branch b12431-nariv0 https://github.com/nari-labs/nari-llama.cpp && cd nari-llama.cpp
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DGGML_CUDA=ON # or -DGGML_CUDA=OFF for CPU only
cmake --build build -j 4
./build/bin/llama-server -m model.gguf --mmproj projector.gguf --port 8080

Contributing
 [#contributing]

We welcome open-source contributions that speed up NariQ inference on more hardware: faster kernels for GeForce,
Ampere, Ada and Hopper cards, Apple Silicon, AMD and Intel GPUs, and the CPU path. Report bugs or results on your
hardware in Issues [https://github.com/nari-labs/nari-llama.cpp/issues], and open pull requests against
nariq/b12431; a change should keep the CUDA qualification [/nari-labs/nari-llama.cpp/blob/nariq/b12431/nariq/cuda-qualification.md] parity checks passing and
report decode and prefill tokens/s before and after.

More
 [#more]

Tensor format [/nari-labs/nari-llama.cpp/blob/nariq/b12431/nariq/FORMAT.md]

Packaging, conversion and checks [/nari-labs/nari-llama.cpp/blob/nariq/b12431/nariq/README.md]

CUDA qualification record [/nari-labs/nari-llama.cpp/blob/nariq/b12431/nariq/cuda-qualification.md]

Metal [/nari-labs/nari-llama.cpp/blob/nariq/b12431/nariq/README-metal.md]

General llama.cpp documentation is in docs [/nari-labs/nari-llama.cpp/blob/nariq/b12431/docs] and upstream [https://github.com/ggml-org/llama.cpp], which does
not endorse this fork.

Contact us
 [#contact-us]

For technical consulting, compute sponsorships, or partnership inquiries, please contact founders@narilabs.com [mailto:founders@narilabs.com].

License
 [#license]

llama.cpp and the NariQ additions are under the MIT license [/nari-labs/nari-llama.cpp/blob/nariq/b12431/LICENSE]. The vendored libtrellis keeps its
Apache-2.0 license [/nari-labs/nari-llama.cpp/blob/nariq/b12431/ggml/src/ggml-cuda/libtrellis/LICENSE]. The coded layout follows
exllamav3 [https://github.com/turboderp-org/exllamav3] (MIT); see NOTICE [/nari-labs/nari-llama.cpp/blob/nariq/b12431/nariq/NOTICE].

Footer

 [https://github.com]

 © 2026 GitHub, Inc.

Footer navigation

 Terms [https://docs.github.com/site-policy/github-terms/github-terms-of-service]

 Privacy [https://docs.github.com/site-policy/privacy-policies/github-privacy-statement]

 Security [https://github.com/security]

 Status [https://www.githubstatus.com/]

 Community [https://github.community/]

 Docs [https://docs.github.com/]

 Contact [https://support.github.com?tags=dotcom-footer]

 Manage cookies

 Do not share my personal information

 You can’t perform that action at this time.
