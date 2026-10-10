On the Navier–Stokes Millennium Prize Problem \| OpenAI

September 8, 2026

[Research](https://openai.com/news/research/) [Publication](https://openai.com/research/index/publication/)

# On the Navier–Stokes Millennium Prize Problem

[Read the paper](https://cdn.openai.com/pdf/32d9f210-8b73-45e0-91bc-82a30aef8a9a/navier-stokes.pdf) [Link to Lean formalized proof](https://github.com/openai/NavierStokesAndEuler)

Loading…

Share

We’re sharing a solution to the Navier–Stokes existence and smoothness problem, one of the Millennium Prize Problems. This proof, produced by an internal OpenAI system, shows that the dynamics of the Navier-Stokes equations for fluid motion can develop a singularity in finite time. We’re sharing both a writeup of the proof and a formalization in Lean.

The [Millennium Prize Problems⁠(opens in a new window)](https://www.claymath.org/millennium-problems/) represent some of the deepest questions at the frontier of mathematics. The question of whether smooth three-dimensional fluid motion can break down has remained unresolved for roughly 90 years.

A major goal of our work is to empower scientists to advance research and technology that benefits all of humanity. To solve the Navier–Stokes problem, we used an internal model that is significantly more capable than GPT‑6 Astra. We believe it is important to inform the world about the pace of AI progress and what to expect from upcoming models.

## The problem

The Navier–Stokes equations use Newton’s second law of motion (“F=ma”) to describe how fluids move. Importantly, they treat a fluid as a continuous medium rather than tracking individual molecules. These equations are used for aircraft design, weather forecasting, and the study of blood flow.

A fundamental open question for these dynamical equations has been whether the continuum approximation of the fluid can break down. Specifically, can the Navier–Stokes equations for a three-dimensional incompressible fluid with constant density develop a “singularity,” even when the motion starts smoothly? Here, a singularity means the dynamics lead to speeds in the fluid growing without bound within a finite amount of time. The development of a singularity would have to happen despite the presence of viscosity, which tends to smooth out motion. Because a real fluid cannot move infinitely fast, this would mark a breakdown in how the equations model the fluid. To continue modeling the system, one would then need to track the behaviour of each particle individually.

The equations date to the nineteenth-century work of Claude-Louis Navier and George Gabriel Stokes. In 1934, Jean Leray proved that solutions exist in a generalized sense, but whether they always remain smooth became a central unanswered question. In 2000, the Clay Mathematics Institute named the Navier–Stokes existence and smoothness problem one of seven Millennium Prize Problems.

## The result

Our system produced an analytical proof and a Lean formalization that an initially smooth fluid at rest can develop a singularity in a finite time. The fluid has a smooth force applied to it, and its energy remains finite through the entire dynamics, from rest to the formation of the singularity. This resolves the Navier–Stokes Millennium Prize problem by establishing statement “C” (and also “D”) in the [official Millennium Prize formulation⁠(opens in a new window)](https://www.claymath.org/wp-content/uploads/2022/06/navierstokes.pdf).

The solution is a vortex, a spinning swirl of fluid, that spirals inward and gets increasingly elongated, like spaghetti. This central region shrinks while it speeds up in such a way that its energy still stays finite, as required by the laws of physics. The technical challenge is for the equations to develop the breakdown through the motion of the fluid itself, rather than, for example, us putting in an infinite force by hand. More mathematically, the terms in the Navier–Stokes equations that describe the motion—acceleration, pressure gradients, momentum transfer, viscosity—must both become _big_ yet _cancel_ in a precise way. This detailed balance leaves a smooth external force even as the velocity of the fluid grows without bound.

![Diagram of a swirling vortex illustrating inward spiral and axial stretching.](https://images.ctfassets.net/kftzwdyauwt9/75EbpsuBOy5LbUgCppWXD1/88da8c19dcf76d6f4f8fd7485dcb6346/navier-stokes-light-master.png?w=3840&q=90&fm=webp)

_A snapshot of local incompressible motion. Orange marks faster angular rotation; teal marks slower rotation. Circulating speed also depends on radius. The trajectories show inward spiraling and axial stretching._

## How we found the proof

Since August 28 we have been training a new internal model that has exhibited unprecedented performance in our benchmarks, including mathematics. This model’s training is ongoing and its performance continues to improve.

On Tuesday, September 1, we heard rumors that two Millennium Prize problems had been resolved. Inspired by these rumors and by the step change in performance of our internal model, we launched an effort to evaluate it on all open Millennium Prize problems and a few other high-impact problems.

We used a system of coordinating agents powered by our internal model. The agents had access to tools such as the ability to read from a cached version of the internet and the ability to run code. Agents were subdivided into groups with the ability to communicate within the group. The groups varied in size, and the group that produced the Navier–Stokes resolution involved on the order of 10,000 concurrent agents. At all times we maintained the same strict safeguards that we apply to all our frontier model evaluations, including monitoring and isolation.

For each problem, we prompted different groups of agents with different variants of the problem statement, covering all variants of the problem. For the Navier–Stokes problem, we suggested versions “A” and “B” (particular forms of the Navier–Stokes problem which would result in a proof) and versions “C” and “D” (which would result in a disproof) to separate groups of agents.

In addition to the full Millennium Prize problems, we asked our multiagent system to try a set of “easier” problems. One of these problems was a similar blowup question for the limit of the Navier–Stokes problem with the viscosity term removed. This is known as the regularity problem for the Euler equations, and our agents surprised us by resolving this question. The specific variant of the question that they resolved was the _unforced_ version, where no external force is applied to the fluid. Nearly 100 agents worked together for approximately 50 hours to produce our Euler regularity disproof[1](https://openai.com/index/navier-stokes-solution/#citation-bottom-1).

Once we saw the Euler solution, we thought that Navier–Stokes was the most promising problem to work on. Thus, we decided to devote our resources to Navier–Stokes. To do so, we shifted agents away from the other Millennium Problems and prompted these agents with the Euler resolution. When a further trained version of our internal model became available over the course of the effort, we updated our agents to that model.

We encouraged different groups of agents to explore a diversity of approaches. After some time, we cross-pollinated the agent groups by using Codex to consolidate the most useful insights from each agent group. These follow-up prompts drew on the agents’ own intermediate results. The group that found the solution to Navier–Stokes was guided in such a way.

The agents arrived at their resolution on Saturday, September 5, about 88 hours after the first agents were launched. Lean formalization and verification took an additional 17 hours via GPT‑6 Astra.

Across all attempted problems, the agents sent 4.9 million messages and used about 300 billion output tokens. In the process of resolving the Navier–Stokes problem, the agents sent 2.7 million messages and used approximately 130 billion output tokens.

## Concurrent work

Our effort began on September 1st after hearing a rumor which we later realized was related to Levent Alpöge, an Anthropic employee, and Tristan Buckmaster, a math professor at NYU.[2](https://openai.com/index/navier-stokes-solution/#citation-bottom-2) After the completion of our full project and Lean verification (on September 6th), believing from the rumor they also had a solution of Navier–Stokes, we reached out to them to offer a concurrent release of our result and to recognize their priority in a joint announcement. At that point we found out that, using an internal Anthropic model, they had produced a resolution of the forced Euler problem. In these discussions we offered them visibility into all of the prompts we used and later to see the proof. We recognize the priority of their work on forced Euler and congratulate them on their remarkable mathematical achievement.

We (the researchers and the agents) did not see any of their work through any means until they released it publicly — in particular, no specific user data was accessed in order to solve this problem.

Following an investigation, we have confirmed that Buckmaster’s Codex prompts over the two months preceding this announcement and paper on September 8, 2026, could not have influenced the system in any way, including through training. The OpenAI internal model used for this result was developed through large-scale reinforcement learning on top of a previously pretrained model. Our proofs also differ significantly. In the Euler case, Alpöge and Buckmaster proved a result with external forcing, while OpenAI’s system proved a result without external forcing.

## Progress and responsibility

Our goal in releasing this result is to report on the substantial progress of our AI models. We do not intend to claim the Millennium Prize for this result.

This milestone represents substantial work by mathematicians and AI researchers. However, this is not a culmination, but rather a snapshot in time, of progress on AI development.

[We believe we are now in the next period of AI progress⁠](https://openai.com/index/research-acceleration-view-inside-openai/), and today’s results provide further evidence of this. We are focusing on understanding this model, and using what we learn to help us guide and pace how we pursue further advances in capability. One of our [key goals⁠](https://openai.com/index/built-to-benefit-everyone-our-plan/) is to build AI systems which are steerable, accountable, and connected to people, which may require more deliberate choices about the pace of progress, as we continue our mission to ensure AGI benefits all of humanity.

- [2026](https://openai.com/news/?tags=2026)
- [Generative Models](https://openai.com/news/?tags=generative-models)

## Author

OpenAI

## Footnotes

1. 1


[Read the Euler proof paper⁠(opens in a new window)](https://cdn.openai.com/pdf/315b36cd-ec98-4023-8342-93345194ece1/euler.pdf) · [Link to Lean formalized proof⁠(opens in a new window)](https://github.com/openai/NavierStokesAndEuler)[Scroll to citation 1 reference](https://openai.com/index/navier-stokes-solution/#citation-top-1)

2. 2


Update — September 10, 2026: We have updated “Concurrent work” with findings from our investigation into whether user inputs could have influenced this result.[Scroll to citation 2 reference](https://openai.com/index/navier-stokes-solution/#citation-top-2)


## Keep reading

[View all](https://openai.com/news/)

![Sharing AI progress in mathematics — Cover](https://images.ctfassets.net/kftzwdyauwt9/49FAj0M0Ly8nk84Uu8btWC/9919efdae7fff54b0f8b0d80398248ac/ai-progress-in-mathematics-cover.png?w=3840&q=90&fm=webp)

[Sharing AI progress in mathematics\\
\\
ResearchOct 6, 2026](https://openai.com/index/sharing-ai-progress-in-mathematics/)

![oai ironclad cua 1x1](https://images.ctfassets.net/kftzwdyauwt9/23y88t0gUd7FSqVFa9n1uq/6d227695a77e7a2cf3e6607bc69e1cf2/oai_ironclad_cua_1x1.png?w=3840&q=90&fm=webp)

[Advancing computer use with Ironclad\\
\\
PublicationOct 6, 2026](https://openai.com/index/advancing-computer-use-with-ironclad/)

![Mental Health Bench art card](https://images.ctfassets.net/kftzwdyauwt9/1GVQAEpVME68kCI7Ol2ApW/abde191546741612492ae27f9baf7f3e/Mental_Health_Bench__art_card.png?w=3840&q=90&fm=webp)

[Introducing MentalHealthBench\\
\\
PublicationSep 23, 2026](https://openai.com/index/introducing-mentalhealthbench/)