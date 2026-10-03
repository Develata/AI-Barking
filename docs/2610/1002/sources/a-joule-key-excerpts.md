# Joule关键摘句（HTML节定位，PDF页码未取得）

URL: https://www.cell.com/joule/fulltext/S2542-4351(26)00362-4
DOI: 10.1016/j.joule.2026.102678

作者：Blaise Agüera y Arcas, Travis Beals, Maria Biggs, Jessica V. Bloom, Thomas Fischbacher, Konstantin Gromov, Urs Köster, Rishiraj Pravahan, James Manyika

## HTML纯文本第70行（关键词 illustrative, planar）
Our proposed system design, at scale, will be significantly larger and entail much closer formation flight (due to inter-satellite communications requirements) than any previous or current satellite constellations. We considered a set of constraints including maintaining a stable set of nearest neighbors, minimizing latency, and maximizing solar exposure and ISL performance. Based on these constraints, Figure 2 shows one possible configuration for an illustrative, planar 81-satellite constellation—all placed in the orbital plane, at a mean cluster altitude of 650 km that exemplifies the underlying general design principles.

## HTML纯文本第167行（关键词 current launch price $3,600）
For the following analysis, we use current launch price $3,600/kg, based on Falcon 9 (reusable configuration), since that is used for the Starlink constellation,82,83 and “potential” price $200/kg. Note that we based this analysis purely on SpaceX data since they are by far the leading launch provider. Other incumbent and new entrants into the market (e.g., RocketLab and Blue Origin) are likely only to hasten price declines via competition and erosion of high SpaceX margins.55

## HTML纯文本第160行（关键词 not intended as a comprehensive）
We projected launch prices via two methods: learning curve projection and analysis of planned Starship 4 specifications and reuse targets. While there is inherent uncertainty (e.g., in future regulatory obstruction, competitive dynamics from new entrants into the launch market, and unforeseen technical challenges), both methods support our conclusion that reaching customer prices of ≲$200/kg by mid 2030s is plausible under reasonable assumptions for reuse and cumulative mass launched during the time. We stress that the following is not intended as a comprehensive economic feasibility study but rather a high-level evaluation of the potential for launch costs, specifically, to affect the viability of our proposal.

## HTML纯文本第98行（关键词 Effective thermal management）
Effective thermal management is a critical optimization challenge for power-dense TPUs operating in a vacuum.

## HTML纯文本第99行（关键词 A published ballpark）
A published ballpark figure for contemporary GPU/TPU power density is 90 W/cm2 for Nvidia’s H100,57 and ideally, silicon junction temperatures should be kept below 90∘C, while near a satellite radiator temperature of 50∘C, blackbody radiation decreases by about 1.2% per K of lower temperature, so the temperature differential between the very hot and compact computational cores and radiators needs to be engineered to be small. Heat pipes (for space applications often Al/ammonia but sometimes also Cu/water) typically play a key role in such designs, but thermal management is a major consideration for spacecraft design. A basic introduction to the topic is available, e.g., in Silk.58

## HTML纯文本第147行（关键词 Testing was conducted at the UC Davis）
Testing was conducted at the UC Davis Crocker Nuclear Laboratory, utilizing their 76-inch cyclotron to produce a 67 MeV proton beam. A large 8 cm diameter aperture was used to ensure uniform irradiation of the entire TPU package, including the logic die and HBM stacks. Beam intensities ranged from 2 pA (∼2 rad/min) to 1 nA (1 krad/min).

## HTML纯文本第103行（关键词 full life cycle assessment）
Looking further ahead, while our proposed constellation design is naturally suited for growing clusters in space, unlocking the full potential of compute in orbit will likely require new design approaches for individual satellites. While we anticipate launch costs to continue to decrease as the industry scales, the floor on fuel price means that there will always be an incentive to minimize mass. Our system design work to this point assumes a relatively conventional, discrete compute payload, satellite bus, thermal radiator, and solar panel design. However, as has been seen in other industries (such as smartphones), massively scaled production motivates highly integrated designs (such as the system on chip, or SoC). Eventually, scaled space-based computing would similarly involve an integrated compute, radiator, and power design based on next-generation architectures, such as computational substrates based on neural cellular automata.63 There are clear environmental benefits to minimizing AI’s terrestrial footprint,18 but a full life cycle assessment (LCA) is outside the scope of this work, and the environmental impacts of moving compute to orbit would be a worthy subject for future research. Some aspects of this analysis will be similar to the terrestrial case, but others, such as rocket launch and re-entry and satellite demise, will be unique to space.64,65

## HTML纯文本第55行
We propose working toward a future where we would host the Google tensor processing unit (TPU) accelerator chips on a constellation of solar-powered satellites. In terrestrial data centers, chips are closely connected within racks. While initial satellites may be smaller, performance considerations may eventually result in satellites evolving toward an amount of compute up to that equivalent to one or more data center racks. The number of chips per rack varies between TPU generations, but typical AI rack power footprint today is ∼50–100 kW.19,20 Satellites substantially larger than what is common today would likely require adjustments to constellation design and formation flight, e.g., increased spacing between satellites.

## HTML纯文本第62行
The networking requirements of large-scale terrestrial ML clusters far exceed the capabilities of current ISL technology. Google’s TPU supercomputers, for instance, utilize a two-tiered networking architecture. A high-speed data center network provides pod-level connectivity,28 while a custom, low-latency optical inter-chip interconnect (ICI) with throughputs on the order of hundreds of gigabits per second per chip facilitates the tightly coupled communication required for large-scale training workloads.29 In contrast, commercially available optical ISLs offer data rates in the range of 1–100 Gbps.
