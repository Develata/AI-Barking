[Technology](/technology)

# Meta’s New Muse AI Agent Read My Private Messages. I Never Asked It To

Permission isn’t the same and what a user actually expects your AI product will do with their personal information.

EXPERT OPINION BY [JASON ATEN](/author/jason-aten), TECH COLUMNIST [@JASONATEN](https://x.com/jasonaten)

Sep 19, 2026

![](https://img-cdn.inc.com/image/upload/f_webp,c_fit,w_1920,q_auto/vip/2026/09/meta_muse_privacy.jpg)

(Photo by Samuel Boivin/NurPhoto)

Listen to this Article[More info](/about/index.html#voice)

0:00 / 0:00

Earlier this week, [Meta](https://www.inc.com/jason-aten/after-15-years-facebook-is-taking-a-page-directly-out-of-apples-playbook-its-not-going-to-end-well.html) rolled out a new [AI agent](https://www.inc.com/ben-sherry/what-are-ai-agents-heres-how-they-can-help-you-get-stuff-done/91160443 "ai agent") called [Muse](https://ai.meta.com/muse/). On paper, it’s impressive. It runs on a dedicated Linux VM with 8GB of memory and 8GB of storage. Basically, [Meta](https://www.inc.com/jason-aten/no-facebook-isnt-reading-your-private-whatsapp-messages-problem-is-much-worse.html) is making a little computer available for free for you to run an agent that will interact with your digital life, and even shop for you.

I downloaded it on my iPhone and Mac for the same reason I’ve downloaded most of the [other AI agents you’d be familiar with](https://www.inc.com/jason-aten/ai-companies-love-to-tell-us-how-dangerous-their-products-are-this-time-they-mean-it/91404736)—because they are interesting and I want to know how they work so I can tell you about them. Muse seemed especially interesting because of what Meta was promising.

That obviously requires a certain amount of trust. An AI agent isn’t especially useful if it can’t see your files, interact with your apps, or understand what you’re working on. Meta says Muse is designed around that reality, while still putting users in control of what it can access.

At least, that’s what I thought.

![Inc Logo](/_public/newsletters/inc_this_morning.svg)

Top Tech

Weekly roundup of the latest in tech news

[Privacy Policy](https://www.mansueto.com/privacy-policy)

[Privacy Policy](https://www.mansueto.com/privacy-policy)

Featured Video

An Inc.com Featured Presentation

I installed Muse on my iPhone and then on a Mac mini I have basically for this very purpose. I asked it to write a bio of me from what it knew about me. After some back and forth where it told me it only knew my name, I suggested it use its browser to find some info. It did, and it came back with a reasonable bio. It isn’t one I would ever use, but that wasn’t the point. I just wanted it to do its first research on me.

![](https://img-cdn.inc.com/image/upload/f_webp,q_auto,c_fit,w_1024/vip/2026/09/CleanShot-2026-09-19-at-16.12.13@2x.png)

I then asked it to suggest things it might do to help me, based on what it knows about me. It suggested researching topics for articles, helping book podcast guests, and creating a morning briefing each day on stories and events it thinks I might want to write about.

Then, yesterday, I was having a conversation with my Primary Technology podcast co-host, Stephen Robles, about the new iPhones. Moments later, I got a push notification from Muse suggesting that the conversation we were having would make for a good column and offered to put together research for me to write about. It even flagged a message from my editor about having a column ready for Monday.

![](https://img-cdn.inc.com/image/upload/f_webp,q_auto,c_fit,w_1024/vip/2026/09/CleanShot-2026-09-19-at-16.14.55@2x.png)

Not only had I not asked it to do that sort of thing, I never gave it permission to read my messages. In fact, I remember explicitly choosing not to let it have access to my messages, calendar, and other personal information.

Stranger still was what happened when I asked Muse how it knew. It told me it didn’t have access to my message history at all. Instead, it said the Muse app on my Mac was simply passing along the text of incoming notification banners.

“When a notification pops up on your paired Mac, the text of that notification gets relayed to me—basically what you’d see in the banner itself,” Muse told me.

It went even further. “I can’t open your Messages app, scroll threads, or read history. It’s the incoming notification stream only, not access to your texts.”

Except that wasn’t true.

I did a little digging. Muse does sync your messages from the local Messages database and uploads that information as a data source. On my device, it had been activated and synced to row 187,462 of my Messages database. I’m not sure how many messages that equates to, but it’s obviously more than whatever Muse told me about what it was doing.

Which, honestly, is the point. I never gave it permission to do that. And, more importantly, I never would do so on purpose. That’s a problem because I’d like to think I’m a relatively tech-savvy person who understands how things like Full Disk Access work on a Mac, and there’s nowhere on any of the prompts I recall that said: “Hey, I’ll scoop up all your messages and then I’ll interact with you based on them.”

In fact, I explicitly told Muse what I wanted it to do for me, and it had nothing to do with reading my messages, but it also had nothing to do with suggesting article ideas. According to Meta, it “built Muse from the ground up to be a safe, secure, private, and widely available personal AI agent.” Snooping through the private database of messages on my Mac doesn’t feel safe or private to me. Lying about it isn’t great either.

Actually, that’s the only area I’m going to give Muse a pass on. The robot chatting with me isn’t lying and has no idea how it works. It also has no idea how it found out about what was in my messages. It’s just an LLM spitting out words.

The point, however, is that this isn’t the kind of thing you should surprise a user with if you want to build trust. No one should be surprised that an AI Agent is reading their messages, regardless of what they clicked, and especially if they didn’t ask.

I reached out to Meta twice, but did not immediately receive a response to my questions. David Singleton [did reply to my post](https://www.threads.com/share/JEVmarjjh/) with what reads like a very technical answer to a very human problem. I appreciate the amount of detail he provided, but—again—you should not design a system that catches people off guard. Also, he seems to imply that I allowed all of the settings required for this to happen.

![](https://img-cdn.inc.com/image/upload/f_webp,q_auto,c_fit,w_1024/vip/2026/09/CleanShot-2026-09-19-at-16.20.06@2x.png)

I would bet that I’m about as privacy-focused as any tech journalist out there. The settings pane in the Muse Mac app shows that Full Disk Access is not enabled, and it does not appear in the Security and Privacy settings at all. It’s not something I would just trip over and give permission to read all of my messages.

A quick search shows more than 125 articles I’ve published in the past few years about technology, privacy, and the companies sitting in the middle of both. I don’t say that because I want you to think I’m an expert. I tell you all that only because I want you to know I think about this a lot, and I’m well aware of how companies use our personal information.

Still, if we grant Singleton’s premise that I managed to unknowingly click something that enabled this capability, I still think that’s really bad. You should not design your system in a way that people end up surprised by this kind of thing.

I also want to be clear about something else: If your response to this is that, of course, Meta would do this–it’s Meta–you are just letting them off the hook. You’re normalizing behavior that should not be normal, and you should not let them off the hook.

![](https://img-cdn.inc.com/image/upload/f_webp,q_auto,c_fit,w_1024/vip/2026/09/CleanShot-2026-09-19-at-16.18.03@2x.png)

This is a problem precisely because Meta has said that it made user control the central pitch for Muse. The company has said its users get to decide which apps to connect and how much access the agent gets. That’s how this should work. An AI agent is useful precisely because you give it access to information. But it shouldn’t try to do more by grabbing more information. The only way this works is if users understand exactly what they’re sharing.

[AI agents](https://www.inc.com/ben-sherry/what-are-ai-agents-heres-how-they-can-help-you-get-stuff-done/91160443 "ai agents") are inevitably going to know more about us than most of the software we use. If you want them to, they can read and interact with email, calendars, and even the files on your computer. But you should never be surprised that they did. If an AI agent is going to read my private messages, there should be a moment when it clearly asks me whether that’s okay. It definitely shouldn’t surprise me with something I didn’t ask for.

Like this column? Sign up to [subscribe to email alerts](https://www.jasonaten.net/subscribe "Subscribe to this Inc. author's newsletter for more great content") and you'll never miss a post.

The opinions expressed here by Inc.com columnists are their own, not those of Inc.com.

Get _[1 Smart Business Story](https://inc-1-smart-business-story.beehiiv.com/?modal=signup)_ delivered straight to your inbox when you subscribe to Inc.’s free daily newsletter.
