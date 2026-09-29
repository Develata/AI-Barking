<!-- 来源: https://openai.com/index/how-we-build-safety-security-and-privacy-into-dots/ ; 工具: opencli browser (document.body.innerText, en-US); 抓取时间: 北京时间 2026-09-30 01:32 (UTC 2026-09-29 17:32); 以下为页面可见文本原样存档 -->

Skip to main content
Research
Products
Business
Developers
Company
Foundation
(opens in a new window)
Log in
Try ChatGPT
(opens in a new window)
How we build safety, security, and privacy into dots | OpenAI

September 29, 2026

Safety
Security
How we build safety, security, and privacy into dots

As your dots take on more work, we help protect your information and keep important decisions in your hands.

00:0002:49

Share
A model built to understand what you mean
Security for agents that can take action
A protected workspace for each dot
Secure sign-ins
Control your information and follow the work
Dots keep looking for ways to help
Clear rules for taking action
Set your preferences with Custom Rules
A separate check before dots act
More help, with you in control
Learn more

Dots are always-on agents that take on ongoing responsibility, giving you more time and attention for the things you care about. Each dot has its own computer and works across the apps you choose to connect. You give dots goals and define what they can do on their own. They can work through the steps, adapt as things change, and keep making progress between conversations. They might follow a developing story and prepare a newsletter draft, or help update a project as new information comes in—bringing back results for your review and turning to you for decisions that need your judgment.

Giving dots that responsibility means trusting them with more of your information and work. You should understand what each dot can access, how it uses that access, and when it needs to involve you. Those questions matter because an agent’s mistakes can have consequences beyond a conversation: misunderstanding a request could lead it to change the wrong file or share information you intended to keep private.

We build protections around each part of that work. We start with GPT‑6 Astra, which we train to understand your intent and follow safety rules, then add safeguards around the information dots use, the environments they work in, and the actions they plan to take. A protected cloud workspace helps contain mistakes, secure sign-in flows help protect sensitive credentials, and separate action checks help catch steps that fall outside your instructions or safety requirements. You choose what to connect, set preferences, and follow or redirect the work as it develops.

Dots can still make mistakes, and we’ll keep improving these protections as we learn from real use. Here, we explain how the safeguards work together, what you control, and where their limits remain—so you can make informed decisions about the work you entrust to dots.

A model built to understand what you mean

Dots are powered by GPT‑6 Astra, our most capable and aligned model. Astra excels at understanding your goal, staying within the scope of your request, and asking focused questions when the answers could change what it should do. These capabilities help dots follow tasks over time without losing sight of what you intended.

We’ve trained Astra to refuse harmful requests, including requests involving biological or cybersecurity misuse. We test how dots handle changing instructions, ambiguous requests, and attempts to push them beyond your permissions. Human and automated red teaming help us find weaknesses, improve the model and the systems around it, and test those changes again.

Security for agents that can take action

We build on those model safeguards with ChatGPT’s security protections and additional measures for agents that can keep working on your behalf.

ChatGPT’s built-in protections include encryption for your content while it is stored and while it travels between you, OpenAI, and our service providers, along with access controls that help protect stored information. Our account-security systems also look for suspicious use of your OpenAI account and can require a fresh sign-in if a session appears compromised.

Agents also need protection from the information they encounter while doing their jobs. A webpage, email, or document can contain malicious instructions that try to redirect them or make them share private information—a problem called prompt injection. We combine model safeguards with tool restrictions, checks before actions, and monitoring to help prevent that content from turning into an unwanted action.

We also monitor dots as they plan and carry out actions, looking for potentially harmful behavior, such as acting outside your instructions or trying to bypass safeguards. If safety monitoring flags a concern in a dot’s active work, it can pause that work and show you a warning to review. This monitoring is built in and helps detect concerns as a task develops, alongside the protections that limit what dots can access and do.

A protected workspace for each dot

Each dot has its own cloud computer, where it can browse, analyze information, create files, and run tools. Dots can keep making progress in these workspaces, even when you are not actively engaged.

You choose which accounts dots can use, either by connecting apps within the permissions you have granted to ChatGPT or by signing into a website in a dot’s browser, using secure sign-in where supported. You can also choose to connect your personal computer to give dots more ways to help—for example, analyzing a spreadsheet saved on your desktop or using coding tools installed on your device.

Within each dot’s protected workspace, sandboxing restricts what code and tools that dot can access, helping contain the impact of harmful code or a mistaken command. We also isolate users’ cloud environments from one another and maintain the underlying Linux operating system and Chrome browser.

Dots run code in environments that are separate from the systems that coordinate their work and enforce key safeguards. They can create files and run tools there, but they cannot use that access to change those safety systems or turn off required checks. This separation helps keep those safeguards in place even if a dot makes a mistake.

The action rules described below govern what dots can do with their access.

Each dot’s cloud workspace brings together its computer and the tools it can use. You choose which apps to connect and whether to connect your personal computer. Auto-review checks actions that need review before they run. If it blocks a step, it tells the dot why so that dot can decide what to do next.

Secure sign-ins

Secure sign-in lets dots work in accounts while keeping passwords out of the conversation. For supported sign-ins, the model is paused while you complete a secure login form, which sends your credentials directly to the browser environment and submits them without exposing them to the model’s context. The dot then resumes work in the signed-in account. Keeping the password out of the conversation reduces the risk of it appearing in an answer or being shared by mistake.

Supported saved-password flows maintain that separation through a dedicated encrypted credential service. The service supplies the password for sign-in without passing it to the model, so dots can work in accounts you have authorized while passwords stay outside the model’s context. These protections are specific to secure sign-in and saved-password flows; a secret placed separately in a readable message or document may still be visible to the model.

Control your information and follow the work

You manage which apps dots can use through your existing ChatGPT connections and permissions, which are shared across ChatGPT, ChatGPT Work, and Codex. In the Plugins section of Settings, you can review connected accounts and change what dots can access going forward. Disconnecting an app stops new information sharing through that connection. But information a dot has already learned remains in its own context. These permissions govern access to your apps; they do not override mandatory safety requirements.

If you want dots to work with local files or tools, you can also choose to connect your computer. That connection remains subject to the local sandbox and applicable action checks. Using your computer’s microphone or camera also requires the device permissions, which you can choose to grant to ChatGPT.

Activity View in the desktop app lets you follow and guide ongoing work. It shows ongoing and delegated tasks and their status. You can add context, correct a misunderstanding, change direction, or ask a dot to stop.

Dots build context over time to make their help more useful. When they delegate work, the other agents are instructed to keep the information needed for the task and avoid retaining unnecessary sensitive details. Each dot has its own context that you can reset at any time.

For ChatGPT Business, Enterprise and Edu workspaces, we don't use your content to train our models by default. For personal ChatGPT plans, you control whether your dots’ conversations and the work they carry out are used to improve our models, through your “Improve the model for everyone” setting. When model training is enabled, this can include actions dots take, and automations you set up, after we work to remove personal identifiers. You can read more about how your data is used in this article⁠
(opens in a new window)
 in our help center.

Dots keep looking for ways to help

Alongside the work you ask them to do, dots can start background research tasks to look for other ways to help—for example, noticing a change to your travel plans. We call this proactive research.

These tasks run in each dot’s cloud environment. They use read-only tools to gather information from permitted connected sources, then save private notes for that dot. We enforce these limits in code: the research tasks cannot directly send messages to other people, change content in connected apps, or control a browser or desktop. Dots can use the findings to decide how to help, but any follow-up action must follow the usual rules and checks.

Proactive research runs as a background task within each dot’s cloud environment. It reads permitted connected sources and returns private notes to that dot, which decides how to help. Any follow-up action follows the usual rules and checks.

We don't train our models directly on those background research threads or the notes they produce. But a dot may receive notes from its background research task or read parts of its research threads. Information brought into an eligible conversation or task this way may be used for model training, depending on your settings. For example, a background research task might gather information on several destinations for an upcoming trip to Europe. That research thread and its notes aren’t directly used for training. Later, when you ask your dot to help plan a trip to Lisbon, it might read a note from its background research task about Lisbon. That note becomes part of the context for your travel-planning conversation and may then be used for training, depending on your settings. The remaining background research is not used for training unless it is also brought into an eligible conversation or task.

Clear rules for taking action

Dots follow built-in guardrails designed to prevent harm and keep consequential decisions with you. Their safety rules require them to refuse harmful requests, including biological or cybersecurity misuse. For tasks they can help with, action rules define when they can proceed, when they must ask for confirmation, and when they must hand a step back to you.

Within those boundaries, dots can read information they have permission to access, analyze it, and prepare drafts in your conversations. To send a message or share a file, they are taught to seek authorization that covers the information and the type of recipient—more sensitive data requires more specificity in recipients. For instance, health data will always require the user to specify a named recipient (“share my medical history with Dr. Thompson”). Less sensitive personal data, like an e-mail address or phone number, will by default require the user to specify classes of recipients (“any airline company”). The user can further broaden the rules on less sensitive personal data (“share with any online form”) by creating a Custom Rule—see the next section for more details on this feature. That authorization stays tied to your instructions for the task; continuing later or delegating work does not expand it.

Dots can make purchases using cards you’ve already saved on a merchant’s website. Those purchases require your approval.

Some actions require your confirmation each time, including permanently deleting data, installing or running software from an unrecognized source, or granting new security-sensitive access. These requirements give you a chance to review changes that could be hard to undo or give someone new access.

For other sensitive steps, including changing a password or transferring money between financial accounts, dots can help with the surrounding task but must hand those sensitive steps back to you.

Set your preferences with Custom Rules

Custom Rules let you shape how dots help within these built-in protections—for example, by telling them never to send emails. You can also give task-specific direction, such as asking a dot to tell a colleague you are away without sharing the personal reason. Those instructions continue to apply when that dot delegates work or works in the background.

Dots can help you write Custom Rules, but they need your approval to change them. These preferences work within the built-in guardrails and cannot remove mandatory confirmations, handoffs, or core safety requirements.

A separate check before dots act

Before dots take actions such as sending emails or changing files, a separate safety system called Auto-review checks the planned steps against your instructions, Custom Rules, and safety requirements. For an email, it checks the recipient and message to help catch a wrong address or information you did not intend to share.

If Auto-review allows a step, the dot that proposed it carries it out using the appropriate computer or app tool, then uses the result to continue your task. It can rely on approval you’ve already given when that approval covers the action and the rules do not require a new confirmation.

If Auto-review blocks a step, it prevents the action from running and tells the dot why. That dot can ask for more information or your approval if that could resolve the block, then submit the step for review again. Depending on the reason, it may instead try a permitted alternative, hand a sensitive step back to you, or stop. Your approval cannot override core safety requirements.

We keep the controls that enforce Auto-review outside the environments dots can change, so they cannot change or turn off a required check. Ordinary read-only steps still follow app permissions, tool restrictions, and other safeguards, but do not need this extra review.

When Auto-review allows an action, the dot runs it and receives the result. When it blocks an action, it returns the reason to that dot, which decides whether to ask you, try a permitted alternative, hand the step back, or stop.

More help, with you in control

Trusting dots with ongoing work means knowing how they will follow your instructions and respond when something unexpected happens. Astra helps them understand your intent, while protections around their data, workspaces, credentials, and actions help limit the harm from mistakes or malicious content. Together, these safeguards give dots room to do useful work while reducing the chance that an error becomes an unwanted action or disclosure.

You choose what dots can connect to and how they help, and you can follow their progress or change direction as your needs change. Approvals and handoffs keep key decisions with you. Dots can still make mistakes, so we will keep testing and improving their protections as we learn from real use. Our goal is to make it easier to leave work with dots and return to useful progress, with your preferences guiding the task and important decisions still in your hands.

Learn more

Read the system card⁠
(opens in a new window)
 for more detail on our safeguards, safety evaluations, and remaining limitations.

For the security foundations behind ChatGPT, see Security and privacy at OpenAI. For a closer look at one web-security protection, see Keeping your data safe when an AI agent clicks a link. Create your dot⁠
(opens in a new window)
.

ChatGPT
2026
Author
OpenAI
Research
Research Index
Research Overview
Economic Research
Latest Advancements
GPT-6
GPT-5.6
GPT-5.5
GPT-5.4
Safety
Safety Approach
Deployment Safety
(opens in a new window)
Security & Privacy
Trust & Transparency
Products
ChatGPT
(opens in a new window)
ChatGPT Business
(opens in a new window)
ChatGPT Enterprise
(opens in a new window)
ChatGPT for Education
(opens in a new window)
Codex
Release Notes
API Platform
Overview
API Log In
(opens in a new window)
Docs
(opens in a new window)
Business
Overview
Solutions
Resources
Plugins
Customer Stories
Partner Network
Contact Sales
Developers
Apps SDK
(opens in a new window)
Open Models
Docs
(opens in a new window)
Resources
(opens in a new window)
Developer Forum
(opens in a new window)
Company
About Us
Our Charter
Careers
News
Support
Help Center
(opens in a new window)
More
Stories
Academy
Supply Co.
Livestreams
Podcast
RSS
Terms & Policies
Terms of Use
Privacy Policy
Other Policies
(opens in a new window)
(opens in a new window)
(opens in a new window)
(opens in a new window)
(opens in a new window)
(opens in a new window)
(opens in a new window)
OpenAI © 2015–2026
Your privacy choices
English
United States


