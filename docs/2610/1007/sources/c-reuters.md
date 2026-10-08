# C 组存档：Reuters 报道（firecrawl_scrape 取得，maxAge=0）

- URL：https://www.reuters.com/legal/litigation/anthropic-opens-its-most-powerful-ai-models-more-security-teams-2026-10-06/
- 抓取：2026-10-07T17:27Z 前后（北京 2026-10-08 01:27 前后），工具 firecrawl_scrape（markdown，maxAge 0），statusCode 200。curl 直连同 URL 返回 HTTP 401（774 字节，见 c-http-records.tsv），headless 浏览器未另试；此文本是 firecrawl 返回的页面正文，页面元数据 `article:content_tier` 为 `metered`，抓取结果含完整正文、无付费墙提示，没有做任何绕过。
- 页面元数据：article:published_time = 2026-10-06T19:02:15.308Z（北京 2026-10-07 03:02:15）；article:modified_time = 2026-10-06T20:37:46.214Z（北京 2026-10-07 04:37:46）；记者署名 Anzar Mehraj（Bengaluru）、Jeffrey Dastin（San Francisco）；编辑 Leroy Leo。
- 标题：Anthropic opens its most powerful AI models to more security teams

## 正文（逐字，来自抓取结果；零宽字符已去除，其余未改）

Oct 6 (Reuters) - Anthropic is expanding a program that allows vetted cybersecurity professionals to test its most powerful AI models with fewer safeguards, after its Project Glasswing initiative helped uncover more than 100,000 software vulnerabilities this year.

Its partners under Glasswing, an initiative aimed at securing the world's most critical software, found at least 129,000 verified vulnerabilities between April and July. Anthropic's own open-source scanning found 5,500 more between April and October.

More than 33,000 have so far been rated critical or high severity.

Anthropic said the figures are likely an undercount and expects the true impact to be at least five times higher, as the data comes from a survey of a limited number of partners.

The revamped Cyber Verification Program, or CVP, announced on Tuesday combines two programs Anthropic has run for the past six months.

The first, Glasswing, gave organizations securing critical software access to Claude Mythos, Anthropic's most cyber-capable family of models. The second, the original CVP, gave vetted security teams reduced safeguards on Claude Opus and Sonnet models.

The company's April unveiling of Claude Mythos Preview raised fears that AI could hack software before it had been secured.

The new program has three tiers, each with its own verification requirements and security controls. All three include access to Claude Opus 5.5, Sonnet 5.5, Mythos 5.1 and future models.

The Defense tier covers work such as incident response and malware analysis. Security teams, critical infrastructure operators, open-source maintainers and researchers with a record of reported vulnerabilities can apply.

The Red Team tier adds authorized penetration testing and red-teaming, but only organizations can apply.

The Specialized tier has the fewest restrictions and is reserved for a small group of organizations authorized to test safety-critical systems such as power grids, flight systems and interbank transfer infrastructure.

Anthropic vets each member together with the US government, and existing Glasswing members will move into this tier.

Reporting by Anzar Mehraj in Bengaluru and Jeffrey Dastin in San Francisco; Editing by Leroy Leo
