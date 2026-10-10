# Full-sky ultraviolet maps from harmonised <em>GALEX, Swift</em>-UVOT, FIMS/SPEAR, TD-1 and <em>Gaia</em> data

## Full-sky ultraviolet maps from harmonised GALEX ,
## Swift -UVOT, FIMS/SPEAR, TD-1 and Gaia data
## Brice Ménard 1,2,3

Brice Ménard<sup>1,2,3</sup>

<sup>1</sup>Anthropic, San Francisco, CA, USA; <sup>2</sup>Department of Physics & Astronomy, Johns Hopkins University, Baltimore,
MD 21218, USA; <sup>3</sup>Santa Fe Institute, Santa Fe, NM 87501, USA

Draft, October 2026

#### Abstract

We present arcminute-sampled all-sky HEALPix maps of the total surface brightness in the *GALEX* far- and
near-ultraviolet bands, with per-pixel provenance weights and uncertainties. The *GALEX* GR6/7 mosaics are
cleaned in two steps—a cell-wise airglow and zodiacal foreground removal against the UV-BKGD diffuse maps
of Murthy (2014a), from which the zero point is inherited (systematic ≲ 100 CU), and a per-tile background
equalisation in which 38000 tile offsets and field-edge rims are fitted to neighbour differences and the scattered light
of avoided bright stars is modelled, removing 60/77 per cent (FUV/NUV) of the excess tile-edge step while leaving
the isotropic level, the degree-scale sky and point-source photometry unchanged. The cleaned *GALEX* layer is
combined with an NUV-informed FUV layer (imaged NUV times a learned diffuse colour, plus catalogue stars),
cross-calibrated *Swift*-UVOT imaging and the FIMS/SPEAR starless map. Sky without ultraviolet imaging is
filled by a 90-feature gradient-boosted prediction of the diffuse component from dust, Hα, starlight and positional
templates, trained on the star-subtracted GALEX sky, plus a full-depth Gaia DR3 source layer of 1.19 × 10<sup>8</sup>
stars with measured or predicted ultraviolet fluxes, galaxies and globular clusters; all layers are gain-harmonised
to *GALEX* and blended by priority-nested feathering. *GALEX* imaging sets three fifths of the FUV and three
quarters of the NUV sky and is reproduced to 0.003 dex; over a quarter of each band the sub-degree morphology is
predicted and flagged by the weight maps. The gap-matched, spatially held-out scatter of the diffuse prediction
is 0.047/0.057 dex (FUV/NUV; 0.052/0.058 dex for prediction plus stars against the total sky), rising to 0.08 dex
several degrees from data; where *GALEX* never observed, the maps agree with FIMS/SPEAR and UVOT to 0.2 dex;
stars in filled sky are complete to *G ≃* 17 and isolated UVOT point sources are half recovered at UVM2≃ 19.3.
The uncertainty is a calibrated function of pixel class, distance from data and dust column (held-out coverage 0.68;
heavy tails, 95.4 per cent at 2.5σ). Both bands are also published partitioned into resolved sources and diffuse
light. We discuss the inherited isotropic level, the FUV–*Hα* slope, a FUV-only high-latitude excess, and what each
pixel class supports.

$$
1.19\times10^{8}
$$

## 1 Introduction

The appearance of the sky between the Lyman limit and
the atmospheric cut-off is set by a small number of physical components: hot-star photospheres, dust-scattered
starlight, line and continuum emission from ionised and
molecular gas, and an extragalactic background. The farultraviolet interstellar radiation field controls heating,
ionisation balance and chemistry of the diffuse interstellar medium (Habing 1968; Draine 1978; Mathis et al.
1983); dust-scattered starlight constrains grain albedo
and phase function (Witt et al. 1997; Draine 2003b;
Murthy 2016; Hamden et al. 2013); fluorescent molecular hydrogen traces illuminated molecular-cloud surfaces
(Jo et al. 2017); two-photon and line emission trace the
warm and hot ionised media (Kulkarni 2022); and the
residual isotropic component bounds the ultraviolet extragalactic background (Henry et al. 2015; Akshaya et al.
2018, 2019; Chiang et al. 2019). The ultraviolet sky is
also a foreground: intensity mapping and clusteringredshift tomography (Ménard et al. 2013; Chiang &
Ménard 2019; Chiang et al. 2019) cross-correlate diffuse

ultraviolet maps with galaxy catalogues and require
a well-characterised Galactic component. Finally, current and planned wide-field missions—SPHEREx (Doré
et al. 2014; Crill et al. 2020), ULTRASAT (Shvartzvald
et al. 2024), UVEX (Kulkarni et al. 2021), CSST (Zhan
2021), UVIT (Tandon et al. 2017) and CASTOR (Côté
et al. 2019)—need realistic all-sky ultraviolet surfacebrightness estimates for planning.

Despite this breadth of applications there is no ultraviolet image of the *entire* sky. The *Galaxy Evolution Explorer* (*GALEX;* Martin et al. 2005; Morrissey
et al. 2007) imaged roughly two thirds of the sky in
its FUV (1350–1750Å) band and three quarters in the
NUV (1750–2800Å), but detector safety limits excluded
the Galactic plane and ultraviolet-bright stars (Bianchi
et al. 2014, 2017). The FIMS/SPEAR spectro-imager
on *STSAT-1* (Edelstein et al. 2006) observed 80 per cent
of the sky in the FUV (Seon et al. 2011; Jo et al. 2017),
and Jo et al. (2021) filled its gaps with a convolutional
network. Diffuse-background analyses of the *GALEX*
archive (Murthy et al. 2010; Murthy 2014a; Hamden
et al. 2013; Akshaya et al. 2018, 2019) have released

---

10^9 photons cm$^{-2}$ s$^{-1}$ sr$^{-1}$ Å$^{-1}$ 10^4

Figure 1: The published all-sky FUV (1350–1750Å) map: destriped and artefact-repaired *GALEX* FUV with censored bright stars
restored, the NUV-informed FUV layer where *GALEX* imaged only the NUV, mapped *Swift*-UVOT, and the FIMS/SPEAR-constrained,
gain-harmonised model layer of equation (6), blended with the priority-nested feathered weights of equations (8)–(9). Galactic Mollweide
projection centred on the Galactic Centre, *l* increasing to the left; logarithmic colour scale in photonscm<sup>−</sup><sup>2</sup>s<sup>−</sup><sup>1</sup>sr<sup>−</sup><sup>1</sup>Å<sup>−</sup><sup>1</sup>. The Sco–Oph
complex above the Galactic Centre, Orion–Eridanus at lower right, the Magellanic Clouds and the Gum/Vela region are prominent; the
dark high-latitude sky reaches *∼* 300–400 CU. Fig. 8(a) shows which of these pixels are imaging and which are predicted.

$$
\mathrm{cm^{-2}s^{-1}sr^{-1}\AA^{-1}}
$$

Full-sky near-UV (1750-2800 Å)

$10^{3}$ $10^{4}$

Figure 2: As Fig. 1 for the NUV (1750–2800Å). No wide-field imager has covered the Galactic plane in the NUV, so the low-latitude
band (26.5 per cent of the sky by dominant weight) is the harmonised diffuse prediction plus the full-depth *Gaia* DR3 stellar layer
(Section 3.5) and the galaxy layer; the grainier texture of the star-rich NUV sky continues without change of character into the filled
plane, where it is synthetic.

---

The ultraviolet sky
far-UV 1350-1750 Å + near-UV 1750-2800 Å Galactic Mollweide projection. Galactic centre at the middle

NuV intensity (ph cm⁻² s⁻¹ cr⁻¹ A⁻¹) [brightness]
91,625
7,499
601
log₁0 | FUV / NUV | [colour]

Figure 3: Two-band colour composite of the published maps. Brightness follows an asinh stretch of the FUV and NUV intensities; hue
encodes the band ratio log<sup>10</sup>(FUV/NUV), from pale gold where the NUV dominates, through blue, to violet where the FUV dominates
(colour key below the map, rendered through the same transfer function; the hue axis is offset by 0.24 dex so that the typical diffuse sky,
log<sub>10</sub>(FUV/NUV) ≃−0.3, is blue). Violet–blue regions are dominated by dust-scattered light from hot stars and by OB associations
(Sco–Cen above the Galactic Centre, Orion–Eridanus, the Gum/Vela region, the Magellanic Clouds); gold–white regions by cooler stellar
populations (the bulge, M31, and field stars at intermediate latitude).

$$
\log_{10}(\mathrm{FUV}/\mathrm{NUV})
$$

$$
\mathrm{log_{10}(FUV/NUV)\simeq-0.3},
$$

foreground-subtracted *diffuse*-background maps over
the *GALEX* footprint. No map of the *total* ultraviolet
surface brightness has been available over the full sky in
both *GALEX* bands, at arcminute sampling, with the
instruments photometrically harmonised, the *GALEX*
gaps filled by data-constrained layers, and per-pixel
provenance and uncertainty, with the qualification that
where no ultraviolet imaging exists, 28 (FUV) and 27
per cent (NUV) of the sky, the sub-degree morphology of
such a map is template-predicted rather than observed.

This paper presents an all-sky FUV/NUV surfacebrightness map built by destriping and per-tile equalising the *GALEX* mosaic against the Murthy (2014a)
foreground reference, filling its gaps with a templatetrained gap-fill predictor, a full-depth *Gaia* DR3 stellar
and galaxy layer, and a NUV-informed FUV layer, and
combining every layer with *Swift*-UVOT and FIMS/
SPEAR by priority-nested feathering. Where calibrated
imaging exists we use it; elsewhere we predict the ultraviolet surface brightness from all-sky templates—thermal
dust, Hα, and *Gaia* integrated starlight—via a gradientboosted regression trained on the *GALEX* sky, tie it to
surrounding data via a smooth gain field and, in the
FUV, to FIMS/SPEAR, add catalogue stars and galaxies, and feather the layers together. Blending weights
are published per pixel. The main contributions of this
work are (i) multi-instrument harmonisation and perpixel provenance; (ii) filling *GALEX* gaps with template
predictions and a synthetic stellar layer; (iii) an empir-

ical, spatially resolved error model for the filled sky;
and (iv) a full-depth *Gaia* DR3 stellar layer with an
explicit resolved-source/diffuse partition of both bands
(Sections 3.5–3.7). Within the *GALEX* footprint the
map inherits the established zero point.

Section 2 describes the input data; Section 3 the
method, in pipeline order, and published pixel classes;
Section 4 the validation of the published maps; Section 5 the physical content and absolute level; Section 6 the limitations; Section 7 the data products,
code and reproducibility; and Section 8 a summary.
Appendices give the gap-fill benchmark (A), statistical details and sources of every headline number (B),
additional validation (C) and a list of enabled measurements (D). Surface brightnesses are in continuum
units, 1 CU = 1 photonscm<sup>−2</sup> s<sup>−1</sup> sr<sup>−1</sup> Å<sup>−1</sup>, at effective wavelengths 1539 and 2316Å; 1 CU corresponds
to I<sub>λ</sub> = 1.29 × 10<sup>−11</sup> (FUV) and 8.58 × 10<sup>−12</sup> (NUV)
ergcm<sup>−2</sup> s<sup>−1</sup> sr<sup>−1</sup> Å<sup>−1</sup>, and νI<sub>ν</sub> = 1 nWm<sup>−2</sup> sr<sup>−1</sup> corresponds to 50.3 CU at any wavelength. Intensities are
observed (attenuated) values, not corrected for Galactic
extinction (A<sub>FUV</sub>/E(B −V) ≃ 8); logarithms are decimal; maps are in Galactic coordinates on the HEALPix
NESTED grid (Górski et al. 2005); weight names follow FITS column names (WGALEX, W<sub>NUVX</sub>, W<sub>UVOT</sub>,
W<sub>FIMS</sub>, WMODEL).

to I λ = 1 . 29 × 10 −11 (FUV) and 8 . 58 × 10 −12 (NUV)

$$
\mathrm{1}\mathrm{CU}=\mathrm{1}\mathrm{photons}\mathrm{cm}^{-2}\mathrm{s}^{-1}\mathrm{sr}^{-1}\mathrm{\AA}^{-1}
$$

$$
I_{\lambda}=1.29\times10^{-11}
$$

$$
8.58\times10^{-12}
$$

$$
\mathrm{cm^{-2}\;s^{-1}\;s^{-1}\;\mathring{A}^{-1}}
$$

$$
\nu I_{\nu}=1\mathrm{nW m^{-2}s r^{-1}}
$$

$$
\left(A_{\mathrm{FUV}}/E(B-V)\simeq8\right)
$$

$$
W_{FIMS},W_{MODEL})
$$

---

### 1.1 Relation to previous wide-area ul- traviolet background products

Table 1 places this work among earlier wide-area measurements, including the pre-*GALEX* experiments (TD-
1, *Voyager* UVS, FAUST and NUVIEWS) that established the correlation of diffuse Galactic light with column density, the forward-scattering phase function and a
few-hundred-CU high-latitude floor (reviewed by Bowyer
1991; Murthy et al. 2019). Two choices define this work
relative to them. First, every product with an absolute
level either fits a constant term jointly with a scattering
model (FAUST, NUVIEWS, Hamden et al. 2013) or subtracts an empirical per-observation foreground (Murthy
et al. 2010; Murthy 2014a; FIMS/SPEAR removes airglow spectrally); we do the second by inheritance. The
closest antecedent is the UV-BKGD product of Murthy
(2014a,b), tabulating the foreground-subtracted diffuse
background of every GR6/7 visit; because our destriping
ties the 14<sup>′</sup>-cell diffuse level of the mosaics to exactly
these values (Section 3.2), the two share a zero point by
construction—agreeing to a median of *−4* (FUV) and
*−6* CU (NUV) with 0.008/0.006 dex scatter (Fig. 4)—so
our contribution to the isotropic-background problem is
spatial rather than absolute (Section 5). The published
total map is brighter than the UV-BKGD diffuse map
by a median of 45 CU (FUV) and 225 CU (NUV), the
resolved-source light. The GR4/5 compilation of Murthy
et al. (2010) agrees with our diffuse estimate to +26 CU
in the FUV but is 256 CU lower in the NUV, illustrating
the sensitivity of any NUV level to the adopted zodiacal
model. Second, every previous product is either blank
or degree-resolution outside the *GALEX* footprint: the
FIMS/SPEAR continuum survey (Seon et al. 2011), the
only previous FUV map with plane coverage, has *∼* 1<sup>◦</sup>
resolution and is an input here (median *GALEX* /FIMS/
SPEAR ratio 0.97, scatter 0.13 dex at 55<sup>′</sup>), and Jo et al.
(2021) inpainted its gaps with a network trained on Hα,
E(B −V), N<sub>HI</sub>, soft X-rays and position—conceptually
close to our gap filling, but at 1<sup>◦</sup>, one band, without
a stellar layer or blocked validation. The tomographic
maps of Chiang et al. (2019), over the SDSS footprint,
give an extragalactic monopole a factor of three below our inherited diffuse zero points, confirming that
most of the isotropic term in any *GALEX*-based map
is Galactic or foreground. The present maps are thus
the first product to provide both bands over *4π* at a
common sampling with per-pixel provenance, at the
price that a documented quarter of their pixels carry
predicted rather than observed sub-degree structure
(Section 3.10).

$$
\sim1^{\circ}
$$

$$
E(B-V),N_{\mathrm{HI}}
$$

$$
1^{\circ},
$$

## 2 Input data

Table 2 lists the input data sets with their wavelength
coverage, resolution as used, sky fraction and role. All
are public (Section 7). Ultraviolet imaging enters as
*data layers;* the remaining sets are calibration references,
*discrete-source supplements* or *predictor templates.*

*GALEX. GALEX* imaged the sky simultaneously in

the FUV (λ<sub>eff</sub> = 1539 Å) and NUV (λ<sub>eff</sub> = 2316 Å)
◦
with a 1.2 field and 4.5–5.5<sup>′′</sup> resolution (Morrissey et al.
2007). We use the background-inclusive intensity images
of GR6/GR7 as served by the CDS in HiPS format
(Fernique et al. 2015)—the GR6/7 HiPS supplemented
by the GR6 AIS HiPS where the former is empty—
reading every order-3 tile, whose pixels map one-to-one
onto HEALPix N<sub>side</sub> = 4096 (0.86<sup>′</sup>). The valid footprint
after masking covers 64.1 per cent of the sky in the FUV
and 75.9 per cent in the NUV; 11.9 per cent has NUV but
no FUV. Count rates are converted to photon continuum
units with the Morrissey et al. (2007) unit responses
and the 1.5<sup>′′</sup> pixel solid angle Ω<sub>pix</sub> = 5.29 × 10<sup>−11</sup> sr:

$$
\left(\lambda_{eff}=1539\mathring{A}\right)
$$

$$
\left(\lambda_{eff}\right.=\left.2316\AA\right)
$$

$$
1.2^{\circ}
$$

$$
4.5–5.5^{\prime\prime}
$$

$$
N_{side}=4096\left(0.86'\right)
$$

$$
\Omega_{\mathrm{pix}}=5.29\times10^{-11}
$$

$$
1.5^{\prime\prime}
$$

$$
\begin{aligned}I\left[\mathrm{CU}\right]&=K r_{\mathrm{pix}},\qquad K=\frac{f_{\lambda}^{(1\operatorname{cps})}\lambda_{\mathrm{eff}}}{h c\Omega_{\mathrm{pix}}},\\K_{\mathrm{FUV}}&=2.05\times10^{6},\qquad K_{\mathrm{NUV}}=4.54\times10^{5},\\&in units of\ \mathrm{CU}\left(\mathrm{cps}\operatorname{pixel}^{-1}\right)^{-1}.\end{aligned}
$$

(1)

An N<sub>side</sub> = 2048 pixel contains 4720 GALEX pixels
and collects a = 2.30 × 10<sup>−3</sup> (FUV) and 1.04 × 10<sup>−2</sup>
(NUV) countss<sup>−1</sup> CU<sup>−1</sup>, so that at AIS depth (≃ 100 s)
and high-latitude raw levels the photon noise per 1.7<sup>′</sup>
pixel is *≃* 44 CU (FUV) and 33 CU (NUV). No colour
correction is applied.

$$
N_{side}=2048
$$

$$
a=\bar{2.30}\times10^{-3}
$$

$$
1.04\times10^{-2}
$$

$$
\mathrm{s^{-1}\mathrm{C}U^{-1}}
$$

$$
(\simeq100s)
$$

*Swift-UVOT.* Summed count and exposure HiPS of
the full UVOT archive (Roming et al. 2005; Poole et al.
2008) in UVW2 (λ<sub>c</sub> ≃ 1930 Å), UVM2 (≃ 2250 Å)
and UVW1 (*≃* 2600 Å) are distributed by HEASARC
through *SkyView.* Rate maps formed where exposure
exceeds 100s cover about 3.5 per cent of the sky. Conversion to *GALEX*-equivalent intensities is empirical
(Section 3.6); the FUV estimate receives reduced weight.

$$
(\lambda_{c}\simeq1930\mathring{A})
$$

$$
(\simeq2600 Å )
$$

$$
(\simeq2250 Å )
$$

*FIMS/SPEAR.* The L-band (1350–1750Å) continuum
HEALPix maps of the FIMS/SPEAR survey (Edelstein
et al. 2006; Seon et al. 2011; Jo et al. 2017), at exposureadaptive N<sub>side</sub> = 64–256, cover 80.7 per cent of the sky.
Their information content is degree-scale: the template
autocorrelation half-width is 35<sup>′</sup><sup>′</sup> and the coherence with
*GALEX* falls below 0.5 at scales under 137 . FIMS/
SPEAR is used as a large-scale (> 4 ) constraint on the<sup>◦</sup>
FUV model layer where *GALEX* support is weak, and
for validation elsewhere; it is never a predictor feature.

$$
N_{side}=64–256,
$$

$$
35'
$$

$$
137'
$$

$$
(>4^{\circ})
$$

*Diffuse-background reference.* Murthy (2014a) reprocessed the *GALEX* visit-level archive into point-sourcemasked diffuse background maps, subtracting an airglow
term and a zodiacal term (the Leinert et al. 1998 distribution; Murthy 2014b). Since this uses geometry and
epoch information lost in the co-added HiPS, we adopt
the UV-BKGD maps as the foreground-free reference
level (Section 3.2).

*Discrete sources.* The TD-1 catalogue (Thompson et al. 1978) supplies absolutely calibrated fluxes
of 31215 stars at 1565, 1965, 2365 and 2740Å; the
FUV flux is *F<sub>1565</sub>* and the NUV flux the combination
0.25F<sub>1965</sub>+0.45F<sub>2365</sub>+0.30F<sub>2740</sub>, rendered as 3<sup>′</sup>-FWHM
Gaussians (per-star accuracy *∼* 0.1 dex, set by the TD-
*1/IUE* scale offsets and the unvalidated NUV synthesis). The full-depth source layer draws on *Gaia* DR3
(Gaia Collaboration 2023): 1.19 × 10<sup>8</sup> stars (G < 16.5

$$
F_{1565}
$$

$$
0.25F_{1965}+0.45F_{2365}+0.30F_{2740}
$$

$$
1.19\times10^{8}
$$

$$
\left(G<16.5\right.
$$

---

Table 1: Wide-area maps and compilations of the diffuse ultraviolet sky. ‘Diffuse’ products mask point sources; the maps presented here
are total intensity with a derivable diffuse estimate. Zero point: how the absolute level at zero dust column is set. All *GALEX*-based
products share the GR6/7 photometric calibration (Morrissey et al. 2007).

| Product | Reference | Bands (\(\mathring{A}\)) | Coverage | Resolution | Foreground treatment | Zero point (FUV/NUV, CU) | Public format |
| --- | --- | --- | --- | --- | --- | --- | --- |
| TD-1 S2/68 sky background | Morgan et al. (1976)a | 1565, 1965, 2365, 2740 | near all-sky | ~ 2° scans | none (bright limits only) | upper limits | printed tables |
| Voyager UVS | Murthy et al. (1999)a | 500-1700 (spectra) | several hundred sight lines | 0.1° × 0.87° | interplanetary Lyα/Lyβ lines fitted; no airglow (heliocentric >5 au) | darkest \(\lesssim\) 100-300 at 1100 \(\mathring{A}\) | tables |
| FAUST | Bowyer et al. (1993)a; Witt et al. (1997) | 1400-1800 | 21 fields, ~ 5 per cent of sky | 8° FOV, 4' | constant airglow+EBL term fitted per field | model-separated | images (NSSDC) |
| NUVIEWS rocket | Schiminovich et al. (2001) | 1450, 1550, 1610, 1740 (narrow) | ~ 25 per cent | 5-10′; 6.25° bins | night-time rocket; stellar halos removed | fitted with DGL model | not released |
| GALEX GR4/5 tile medians | Murthy et al. (2010) | 1530, 2310 | \(\simeq 75\) per cent | ~ 1° (per tile) | per-tile airglow + zodiacal model subtracted | cosecant fits; NUV 250 below UV-BKGD | VizieR table |
| FIMS/SPEAR L-band continuum | Seon et al. (2011); Jo et al. (2017) | 1370-1710 (spectral, \(~ 1 \mathring{A}\)) | ~ 76-80 per cent | HEALPix Nside = 64 (~ 1°) | O \(\downarrow\) 1356 excluded spectrally; no zodiacal term | \(\simeq 270\) at NHI → 0 | HEALPix FITS (KASI/MAST) |
| GALEX AIS diffuse FUV | Hamden et al. (2013) | 1344-1786 | 65 per cent | HEALPix Nside = 1024 | none per visit; airglow scatter ± 50 | 300 isotropic (fits vs 100μ m, NHI, Hα) | not released |
| UV-BKGD HLSP | Murthy (2014a,b) | 1539, 2316 | 61 / 72 per cent (FUV/NUV) | 2' per visit; 6' Aitoff maps | per-visit airglow (Sun angle, local time) + zodiacal (Leinert × 0.65 colour) | \(\simeq 300\) / \(\simeq 600\) polar | ASCII per visit + FITS maps (MAST) |
| GALEX polar caps | Akshaya et al. (2018, 2019) | 1539, 2316 | \(\|b\| \gtrsim 70^{°}\) | 2-6' | UV-BKGD model | 230-290 / 480-580; 240 ± 18 / 394 ± 37 at E(B-V) = 0 | none |
| GALEX tomographic maps | Chiang et al. (2019) | 1539, 2316 | 5500 deg2 (SDSS) | Nside = 4096 sampling of 2' data | UV-BKGD model; monopole not needed | EBL 89-16+28 / 172-21+40 | analysis product |
| FIMS/SPEAR + CNN inpainting | Jo et al. (2021) | 1370-1710 | 100 per cent (24 per cent predicted) | Nside = 64 | as Seon et al. (2011) | as FIMS/SPEAR | HEALPix FITS |
| This work | — | 1350-1750, 1750-2800 | 100 per cent (61.9 / 73.3 imaging; rest NUV-informed, UVOT, FIMS/SPEAR-constrained or predicted) | HEALPix Nside = 2048 (1.7'); effective 2.5'-1° by class | UV-BKGD per-visit model transferred by 14' additive destriping; identity to ± 4 CU | inherited: 264 / 579 diffuse, 291 / 727 total (± 50-100 syst.) | HEALPix FITS: intensity, 5 weights, uncertainty, provenance; Galactic + equatorial |

$$
\mathrm{(FUV/NUV,\ CU)}
$$

$$
0.1^{\circ}\times0.87^{\circ}
$$

$$
8^{\circ}\ FOV,\ 4'
$$

$$
5–10';\;6.25^{\circ}
$$

$$
N_{side}=64
$$

$$
N_{HI}\rightarrow0
$$

$$
\overset{\frown}{\underset{\cdot}{\mathrm{HEALPix}}}
$$

$$
vs100\mu m,\stackrel{\cdot}{N_{HI}},
$$

$$
N_{side}=1024
$$

$$
\simeq300/\simeq600
$$

$$
|b|\gtrsim70^{\circ}
$$

$$
2–6'
$$

$$
480–580;240\pm18
$$

$$
5500deg^2
$$

$$
N_{side}=4096
$$

$$
E(B-V)=0
$$

$$
EBL89_{-16}^{+28}/
$$

$$
172_{-21}^{+40}
$$

$$
N_{side}=64
$$

$$
N_{side}=2048
$$

$$
(1.7');
$$

$$
2.5^{\prime}{-1}^{\circ}\mathrm{~b y}
$$

<sup>a</sup>Pre-*GALEX* entries are included for completeness; see Bowyer (1991) and Murthy et al. (2019) for reviews of the earlier measurements.

over the whole sky, blue and hot sources to *G* = 19,
plus white-dwarf, hot-subdwarf and ESP-HS catalogues),
with the flux of each star taken from a hierarchy TD-1
(where available) > a Yale Bright Star Catalogue (BSC;
Hoffleit & Jaschek 1991) model > GUVcat AIS photometry (Bianchi et al. 2017) (unsaturated, clean) > a
hot/degenerate-star model > a main-sequence model,
validated blind against *Swift* UVOTSSC, XMM-OM
SUSS, UVIT and TD-1 photometry (Section 3.5). Integrated *GALEX* photometry of 14283 nearby galaxies
(Leroy et al. 2019; Bai et al. 2015; Gil de Paz et al.
2007; de Vaucouleurs et al. 1991) and 146 globular clusters (Dalessandro et al. 2012; Harris 2010) supplies the
galaxy layer (Section 3.5). A small completion outside
the imaged footprint—bright BSC stars (306 FUV / 368
NUV), a cool-star patch (56 / 11 stars) and Canopus—is
added after fusion.

*Predictor templates.* The learned prediction uses templates that trace, at other wavelengths, the physical
components of the ultraviolet sky: the *Planck* 2013 thermal dust model (*E(B −* V), *τ<sub>353</sub>,* radiance *R;* Planck
Collaboration XI 2014), which sets the scattering mate-

$$
\mathcal{R};
$$

rial and, through the radiance, the illuminating field; the
Finkbeiner (2003) *Hα* composite of WHAM, VTSS and
SHASSA, tracing the warm ionised medium and hence
two-photon and free–bound continua and ionising stars;
the CDS Gaia DR3 HiPS of integrated G<sub>BP</sub> and G<sub>RP</sub>
flux and source density (Gaia Collaboration 2016, 2023),
predictors of starlight; and Galactic and ecliptic coordinates, absorbing residual zodiacal structure. Templates
at lower N<sub>side</sub> are upsampled by nearest neighbour, adequate because the regression works at N<sub>side</sub> = 512.

## are upsampled by nearest neighbour, ad- side

$$
G_{\mathrm{RP}}
$$

$$
G_{\mathrm{BP}}
$$

$$
N_{side}=512
$$

$$
N_{side}
$$

## 3 Method

### 3.1 Overview

All layers and products are full-sky HEALPix arrays
at N<sub>side</sub> = 2048 (1.7<sup>′</sup> pixels), NESTED ordering,
Galactic coordinates, in continuum units (CU, photonscm<sup>−2</sup> s<sup>−1</sup> sr<sup>−1</sup> Å<sup>−1</sup>) with airglow and zodiacal light
removed and no correction for extinction. Figure 6 shows
the data flow and Fig. 5 the corresponding stages on the
sky for the NUV. The *GALEX* mosaics *G* are destriped

$$
N_{side}=2048
$$

$$
\mathrm{cm^{-2}\:s^{-1}\:s^{-1}\:\mathring{A}^{-1})}
$$

---

Table 2: Input data sets. ‘Resolution’ is the effective angular resolution as used here (not necessarily the native instrumental value);
fskyis the fraction of the Nside= 2048 sky with valid data after masking. Roles: D = ultraviolet data layer entering the fused map;
R = photometric/foreground reference; S = discrete-source supplement; P = predictor template (base templates from which the 90
features of Section 3.7 are derived; templates marked *†* also enter the uncertainty model of Section 4.6); V = validation only. Acronyms:
AIS, All-sky Imaging Survey; HiPS, Hierarchical Progressive Survey; MCCM, MAST Mission Community-Contributed Models; HLSP,
high-level science product; UV-BKGD, the *GALEX* diffuse-background HLSP of Murthy (2014a).

$$
N_{side}=2048
$$

| Data set | Band / wavelength | Resolution | fsky | Role | Ref. |
| --- | --- | --- | --- | --- | --- |
| GALEX GR6/7 + AIS HiPS | FUV 1350-1750 \(\mathring{A}\) | ~ 5'' (used at 1.7') | 0.641 | D | 1, 2, 3 |
| GALEX GR6/7 + AIS HiPS | NUV 1750-2800 \(\mathring{A}\) | ~ 5'' (used at 1.7') | 0.759 | D (also FUV via colour) |  |
| Swift-UVOT summed-image HiPS | UVW2, UVM2, UVW1 (1900-2600 \(\mathring{A}\)) | ~ 2.5'' (used at 1.7') | 0.035 | D | 4, 5 |
| FIMS/SPEAR L-band (MAST MCCM) | 1360-1710 \(\mathring{A}\); ‘starry’ and ‘starless’ | 14'–55' (Nside 256/128/64) | 0.807 | D (> 4°), V | 6, 7, 8 |
| GALEX diffuse background HLSP (UV-BKGD) | FUV, NUV, foreground-subtracted | 2' bins, per visit | (GALEX) | R | 9 |
| TD-1 stellar catalogue | F1565, F1965, F2365, F2740 | 31 215 stars, rendered at 3' FWHM | 1.00 | S | 10 |
| Yale Bright Star Catalogue (BSC) | FUV, NUV (TD-1 or modelled); cool-star FUV (empirical GALEX relation) | bright-star completion 306 (FUV) / 368 (NUV) stars; cool-star patch 56 (FUV) / 11 (NUV) stars; 1618 stars in the hierarchy | 1.00 | S | 23 |
| Gaia DR3 full-depth stellar layer: G < 16.5 all-sky, blue/hot to G < 19, white dwarfs, hot subdwarfs, ESP-HS | measured (TD-1, GUVcat) or predicted FUV, NUV | 1.189 × 108 stars + 160 099 extragalactic points; 2' FWHM outside imaging, map-matched PSF inside | 1.00 | S | 10, 16, 17, 26, 27 |
| UVOTSSC, XMM-SUSS, UVIT point-source photometry | UVW2/UVM2/UVW1; F148W-N263M | 4.2 × 106 / 5.1 × 106 / 2.9 × 105 Gaia matches | — | V (stellar predictors) | 24, 25, 28 |
| Galaxies, globular clusters | integrated FUV, NUV | 14 283 + 146 objects, 2' FWHM | 1.00 | S | 18-22 |
| Planck 2013 thermal dust model† | E(B-V), τ353, radiance | 5' | 1.00 | P | 11 |
| Hα composite (WHAM, VTSS, SHASSA)† | 6563 \(\mathring{A}\) | 6' (Nside = 512) | 1.00 | P | 14 |
| Gaia DR3 flux and density HiPS† | GBP, GRP integrated flux; source density | ~ 1' | 1.00 | P | 15, 16 |
| Position† | \(\sin b, \|b\|\), \(\cos l\), \(\sin l\); \|β\|, \(\sin λ\), \(\cos λ\) | – | 1.00 | P | – |

$$
f_{\mathrm{sky}}
$$

$$
\sim5^{\prime\prime}
$$

$$
\sim2.5^{\prime\prime}
$$

$$
1.189\times10^{8}
$$

$$
4.2\times10^{6}/5.1\times10^{6}/
$$

$$
2.9\times10^{5}
$$

$$
6'(N_{side}=512)
$$

References: (1) Martin et al. (2005); (2) Morrissey et al. (2007); (3) Bianchi et al. (2017); (4) Roming et al. (2005); (5) Poole et al.
(2008); (6) Edelstein et al. (2006); (7) Seon et al. (2011); (8) Jo et al. (2017); (9) Murthy (2014a); (10) Thompson et al. (1978); (11)
Planck Collaboration XI (2014); (12, 13) unused; (14) Finkbeiner (2003); (15) Gaia Collaboration (2016); (16) Gaia Collaboration
(2023); (17) Bianchi et al. (2017) (GUVcat training photometry); (18) Leroy et al. (2019); (19) Bai et al. (2015); (20) Gil de Paz et al.
(2007); (21) de Vaucouleurs et al. (1991); (22) Harris (2010), Dalessandro et al. (2012); (23) Hoffleit & Jaschek (1991); (24) Yershov
(2014), Page et al. (2014); (25) Page et al. (2012); (26) Gentile Fusillo et al. (2021); (27) Culpan et al. (2022); (28) Tandon et al. (2017).
HiPS: Fernique et al. (2015).

---

300 CU 30000

300 CU 30000

-0.2 dex (grey: no UV-BKGD data) 0.2

| total FUV, this work (CU) | median (UV-BKGD diffuse FUV, CU) |
| --- | --- |
| 1000 | 0.031 |
| 2000 | 0.031 |
| 3000 | 0.031 |
| 4000 | 0.031 |
| 5000 | 0.031 |
| 6000 | 0.031 |
| 7000 | 0.031 |
| 8000 | 0.031 |
| 9000 | 0.031 |
| 10000 | 0.031 |
| 20000 | 0.031 |
| 30000 | 0.031 |
| 40000 | 0.031 |
| 50000 | 0.031 |
| 60000 | 0.031 |
| 70000 | 0.031 |
| 80000 | 0.031 |
| 90000 | 0.031 |
| 100000 | 0.031 |

Figure 4: Map-level comparison with the *GALEX* UV-BKGD diffuse FUV map of Murthy (2014a), regridded to Nside= 256. (a)
UV-BKGD diffuse FUV (62 per cent of the sky at *N*<sub>side</sub>= 256). (b) Total FUV, this work. (c) log<sub>10</sub>(UV-BKGD/this work); grey, no
UV-BKGD data. (d) Pixel-by-pixel comparison; the one-sided offset (median −0.031 dex) is the resolved-source light present in the total
map and masked in UV-BKGD. Against our *diffuse* estimate the agreement is −0.002 dex with 0.008 dex scatter, by construction of the
destriping.

$$
N_{side}=256.
$$

$$
N_{side}=256)
$$

cell by cell against UV-BKGD, giving G<sup>′</sup> (Section 3.2),
and equalised tile by tile, giving the *GALEX* layer *g*
(Section 3.3); residual instrumental features in *g* define
weight multipliers, and bright stars censored by *GALEX*
are topped up from TD-1 (Section 3.4). A source layer
S = S<sub>⋆</sub> + E + L<sub>gal</sub>—Gaia DR3 stars, extragalactic
point sources, and galaxies and globular clusters—is
rendered on the same grid from catalogues, using the
ultraviolet imaging only for training photometry and
the point-spread function (Section 3.5). UVOT is crosscalibrated onto *g* and the FIMS/SPEAR scale is verified
(Section 3.6). The star-subtracted *GALEX* sky *g − S*
is the training truth of a 90-feature gradient-boosted
predictor of the diffuse intensity, P<sub>D</sub> (Section 3.7), and,
in the FUV, of a diffuse colour model that converts
NUV-only *GALEX* imaging into an NUV-informed FUV
layer I<sub>X</sub> (Section 3.8). The fusion (Section 3.9) gainharmonises P<sub>D</sub> + S to g on degree scales, constrains
its large-scale FUV level with the FIMS/SPEAR starless map far from *GALEX,* and feathers the layers in
the priority order *GALEX* > NUV-informed FUV >
UVOT > FIMS/SPEAR-constrained model > model,
with weights WGALEX, W<sub>NUVX</sub>, W<sub>UVOT</sub>, W<sub>FIMS</sub> and
*W*<sub>MODEL</sub>that sum to unity in every pixel. A small set
of post-fusion edits and the derived products—weights,
provenance, data-only maps, the source/diffuse partition and the uncertainty maps—complete the chain
(Section 3.10). The two bands are processed identically
except that FIMS/SPEAR and I<sub>X</sub> exist only in the FUV.

$$
G'
$$

$$
\bar{S}=\bar{S}_{\star}+E+L_{\mathrm{gal}}-\mathrm{Ga}i\alpha
$$

$$
g-S
$$

$$
P_{\mathrm{D}}
$$

$$
I_{\mathrm{X}}
$$

$$
P_{\mathrm{D}}+S
$$

$$
W_{GALEX}
$$

$$
W_{MODEI}
$$

$$
I_{\mathrm{X}}
$$

Table 9 in Appendix B collects the notation.

### 3.2 <em>GALEX</em> assembly and foreground destriping

3.2 GALEX assembly and foreground

*Assembly.* HiPS tiles sample the sky on the NESTED
grid of N<sub>side</sub> = 512 × 2<sup>k</sup> at order k. GALEX and
Gaia order-3 tiles are placed into an equatorial N<sub>side</sub> =
4096 array, rotated to Galactic coordinates by nearestneighbour lookup and averaged over the four children
of each N<sub>side</sub> = 2048 pixel, conserving surface brightness and introducing scatter of at most 0.43<sup>′</sup>; GALEX
count rates are converted to CU. Equatorial copies are
produced by bilinear resampling of logI; quantitative
work should use the Galactic files.

$$
N_{side}=
N_{side}=512\times2^k
$$

$$
N_{side}=2048
$$

$$
I;
$$

*Destriping.* The co-added *GALEX* mosaics contain
airglow and zodiacal light whose level varies from visit
to visit (Murthy 2014b), appearing as tile-to-tile offsets
of tens (FUV) to hundreds (NUV) of CU superposed
on a smooth zodiacal gradient in the NUV. We remove
them as an additive offset field on the N<sub>side</sub> = 256 grid
(14<sup>′</sup> cells). For each cell c,

$$
N_{side}=256
$$

$$
c,
$$

$$
\begin{aligned}{F_{c}=\mathcal{P}_{30}\{G_{p}:p\in c\}-\operatorname{med}}&{{}\{M_{q}:q\in c\},}\\ {G_{p}^{\prime}=G_{p}-F}&{{}(\hat{\mathbf{n}}_{p}),}\\ \end{aligned}
$$

(2)

where G<sub>p</sub> are the GALEX intensities of the N<sub>side</sub> = 2048
children of the cell, *P<sub>30</sub>* denotes the 30th percentile, and
M<sub>q</sub> are the UV-BKGD diffuse intensities in the cell.
Where UV-BKGD has no data (9.4 per cent of *GALEX*

$$
G_{p}
$$

$$
N_{side}=2048
$$

$$
\mathcal{P}_{30}
$$

$$
M_{q}
$$

---

(a) GALEX NUV mosaic as observed (destriped + removed foreground)

(b) destriped GALEX NUV (foreground removed)

(c) ultraviolet inputs: GALEX (blue), Swift-UVOT (red), FIMS/SPEAR (yellow, FUV)

(d) harmonised gap-fill layer (prediction + source layers)

(e) blending weights: GALEX (blue), UVOT (orange), model (grey); paler = mixed

(f) released NUV map

Figure 5: Pipeline stages on the sky for the NUV band, rendered from the published files at *N*<sub>side</sub>= 256 (Galactic Mollweide centred
on *l* = 0, *l* increasing to the left; grey denotes no data). (a) *GALEX* NUV mosaic as observed, i.e. the destriped layer plus the removed
foreground field *F* (76 per cent of the sky). (b) The destriped mosaic *G*<sup>′</sup>after subtraction of the per-cell airglow-plus-zodiacal foreground
tied to Murthy (2014a) (Section 3.2). (c) Footprints of the ultraviolet inputs: *GALEX* (blue), *Swift*-UVOT (red) and FIMS/SPEAR
(yellow, FUV only). (d) The harmonised gap-fill layer: 90-feature diffuse prediction plus stellar and galaxy source layers (Sections 3.5
and 3.7). (e) Dominant blending weight: *GALEX* (blue), UVOT (orange), model (grey), paler where mixed. (f) The published NUV
map. Panels (a), (b), (d) and (f) share a logarithmic colour scale (300–3 *×* 10<sup>4</sup>CU).

$$
N_{side}=256
$$

$$
G'
$$

$$
(300-\overset{\frown}{3}\times10^{4}\mathrm{CU})
$$

---

Swift-UVOT 5  UVOT → GALEX
W2/M2/W1 HiPS isotonic cross-cal   →  U
GALEX 3  Artefact-repair
tile table weight masks   →  MF, MX
GALEX GR6/7
FUV + NUV HiPS
9  Fusion 10  Post-fusion edits
1  Foreground destriping 2  Per-tile background 4b  Bright-star saturation multiscale gains · FIMS ratio field · star completion · cold pixels ·
P30(GALEX) − Murthy, 14′ cells equalisation   →  g top-up (TD-1 fluxes)   →  g_c priority-nested feathering glint · Magellanic holes
Murthy 2014 GALEX > NUVX > UVOT > FIMS > MODEL
UV-BKGD
(zero point) galaxy / GC 7  Diffuse predictor 11  Released products
catalogues y = ⟨g − S⟩ at nside 512, spatial CV, FUV / NUV maps · weights · provenance
jittered HGB   →  P_D uncertainty (σ_model from step-7
primary data held-out residuals) · HiPS
TD-1 · BSC · 4  Full-depth source layer 8  NUV-informed FUV layer
secondary data GUVcat 118.9 M stars + galaxies   →  S C_diff · NUV_diff + S_F   →  I_X
deterministic step
learned component Gaia DR3
sources + 6  90-feature matrix
fusion / product BP/RP/density (multi-scale templates)
FIMS/SPEAR
starless FUV
Planck dust ·
Hα templates

Figure 6: Data flow of the pipeline, left to right. Cylinders are public inputs, drawn where they first enter (dark: the primary *GALEX*
imaging; light: secondary data); blue boxes are deterministic steps, purple boxes learned components (the source-layer flux models, the
diffuse predictor and the diffuse colour model of the NUV-informed FUV layer), green boxes the fusion and the published products.
The thick line is the *GALEX* spine: HiPS mosaic *→* foreground destriping (Section 3.2) *→* per-tile equalisation, *g* (Section 3.3) *→*
bright-star top-up, *g*<sub>c</sub> (Section 3.4) *→* fusion (Section 3.9) *→* post-fusion edits and products (Section 3.10). Every other layer—UVOT
(Section 3.6), the source layer (Section 3.5), the diffuse prediction (Section 3.7) and the NUV-informed FUV layer (Section 3.8)—is
calibrated or trained against *g* and enters only at the fusion. Numbers in the boxes are the step numbers used in the text.

FUV cells and 8.2 per cent of NUV cells at N<sub>side</sub> = 256)
*F* is inpainted by mask-normalised Gaussian smoothing
◦ ◦
(1.5, then 8). F is evaluated at every N<sub>side</sub> = 2048
pixel by bilinear interpolation and subtracted; the offset
fields are published. The pixel-weighted median offset
removed is 46 CU (FUV) and 480 CU (NUV), with 5–95
per cent ranges of −13 to 188 and 322 to 704 CU. A
handful of under-exposed rim pixels (faint pixels lying
more than 0.5 dex below their eight-neighbour median)
are replaced by that median, so that no non-positive
pixel remains.

$$
N_{side}=256)
$$

$$
(1.5^{\circ}
$$

$$
N_{side}=2048
$$

Equation (2) transfers the zero point of the UV-BKGD
maps to the mosaics cell by cell, leaving structure below 14<sup>′</sup> and all sources untouched; it inherits a foreground rather than determining one. Any error in the
Murthy (2014a) airglow-plus-zodiacal model propagates
unchanged into our maps: agreement with UV-BKGD
to −4/−6 CU demonstrates identity, not accuracy. We
adopt an additive systematic of ±50–100 CU on the absolute level of both bands, plus two smaller structured
terms: a *GALEX*-depth-dependent +13 CU term from
the order-statistic bias of *P<sub>30</sub>,* and a possible foreground
share, bounded to ≲ 30 CU. None is included in the
multiplicative per-pixel uncertainty maps; they dominate the error budget of the darkest high-latitude sky
(*≃* 300 CU FUV).

$$
14'
$$

$$
\mathcal{P}_{30}
$$

$$
0\lesssim30\mathrm{CU}
$$

The P<sub>30</sub> estimator. For photon noise σ<sub>pix</sub> per N<sub>side</sub> =
2048 child, the 30th percentile of a cell lies 0.52σ<sub>pix</sub>
(Poisson) to 0.33σ<sub>pix</sub> (measured) below the diffuse mean.
Because *F* is solved with *P<sub>30</sub>* on the noisy mosaic but
the median on UV-BKGD, destriped total intensities
are high relative to UV-BKGD by +0.34 ± 0.01σ<sub>pix</sub>
(FUV) and +0.35 ± 0.05σ<sub>pix</sub> (NUV), i.e. +13 CU at
high latitude, with steps of *≃* 10 CU (0.010 dex FUV,
0.005 dex NUV) at deep-field boundaries. In crowded
low-latitude cells *P<sub>30</sub>* is biased high by faint stars, absorbing unresolved stellar light into *F;* the effect is small

$$
\mathcal{P}_{30}
$$

$$
\sigma_{\mathrm{pi}}
$$

$$
N_{side}=
0.52\sigma_{\mathrm{pi}}
$$

$$
0.33\sigma_{\mathrm{pi}}
$$

$$
\mathcal{P}_{30}
$$

$$
F
$$

$$
+0.34\pm0.01\sigma_{\mathrm{pi}}
$$

$$
+0.35\pm0.05\sigma_{\mathrm{pix}}\quad(\mathrm{NUV}),\quad\mathrm{i.e.}\quad+13\mathrm{CU}
$$

$$
\mathcal{P}_{30}
$$

$$
F;
$$

and covered by plane systematics (Section 6).

### 3.3 Per-tile background equalisation

At high latitude the destriped mosaics still show the
survey tiling: 1.2<sup>◦</sup> discs whose level differs by a few per
cent from their neighbours, with bright or dark rims
◦
at *ρ ≃* 0.55–0.60 from the tile centre, most conspicuously in faint fields such as Coma and the Galactic poles
(Fig. 7). They are per-visit residual backgrounds (timevariable airglow and detector background of the short
AIS visits against deeper MIS/DIS coadds) that the
N<sub>side</sub> = 256 destriping cannot remove at the single-tile
level: the per-tile scatter shows no trend with ecliptic latitude and falls with exposure time. We remove
them with an explicit per-tile model. The tile list is
the *GALEX* GR6/7 image inventory at MAST (79164
image products; co-pointed visits within 3<sup>′</sup> merged),
giving 31449 FUV and 38473 NUV tiles with centre,
exposure and survey; the effective field radius measured
◦
from the coverage edge of isolated tiles is R<sub>eff</sub> = 0.595
(attribution radius 0.665<sup>◦</sup> with an exposure-weighted
soft edge).

$$
1.2^{\circ}
$$

$$
\rho\simeq0.55–0.60^{\circ}
$$

$$
N_{side}=256
$$

$$
3^{\prime}
$$

$$
R_{eff}=0.595^{\circ}
$$

Working at N<sub>side</sub> = 1024 on the relative residual
r = (G<sup>′</sup> − S<sup>′</sup>)/S<sup>′</sup> of the destriped map against a ro-
◦
bust local sky S<sup>′</sup> (the 2.5-FWHM smoothing of the
source-masked N<sub>side</sub> = 256 block median plus a locally
regressed sub-2.5<sup>◦</sup> dust term, so that cirrus does not
enter r), each tile t carries a constant offset o<sub>t</sub>, a rim
amplitude a<sub>t</sub> multiplying a stacked empirical radial template R(ρ), and—for the 2–4 per cent of tiles whose
residual shows a straight internal step (partial or multiepoch coadds) or whose coverage is chord-cut—a split
into two pseudo-tiles along the fitted chord, plus pertile gradients for flagged outliers; boundary tiles use
their measured footprints. The basis is fitted not to *r*
itself but to all N<sub>side</sub> = 1024 neighbour-pair differences

$$
N_{side}\;=\;1024
$$

$$
r=\left(G^{\prime}-S^{\prime}\right)/S^{\prime}
$$

$$
S'
$$

$$
2.5^{\circ}-FWHM
$$

$$
N_{side}=256
$$

$$
r)
$$

$$
o_{t},
$$

$$
a_{t}
$$

$$
R(\rho)
$$

$$
N_{side}=1024
$$ r<sub>i</sub> − r<sub>j</sub> inside the footprint (3.7 × 10<sup>7</sup> rows per band),
which are insensitive to any smooth real sky, with weak
level rows and ridge priors (τ<sub>o</sub> = 0.30; τ<sub>a</sub> = 0.30 about
the survey-level stacked rim amplitude, 0.05–0.08 FUV
and 0.10–0.12 NUV), solved by sparse LSQR inside a
Huber (*c* = 2.5) iteratively reweighted loop and iterated with S<sup>′</sup>, the source mask and R(ρ) re-estimated.
Tiles with |o<sub>t</sub>| > 0.25, or with a significant residual edge
step together with an interior offset of the same sign
(2.3 per cent), are refit without the ridge with their
discs masked from S<sup>′</sup>, and flattened non-parametrically
◦
at 0.35 inside the disc unless they lie on bright *Hα*
(> 3 R) or *E(B − V*) > 0.15, where sub-degree residuals may be nebular. Two targeted passes then treat
the tiles that still show a disc. The first addresses
short visits adjacent to coverage holes or inside clusters
of offset tiles, which bias their own local reference S<sup>′</sup>
and so escape the interior criterion, and pointings on
catalogued galaxies smaller than the field: every tile
whose residual step at the tile radius exceeds 0.04 at
> *4.5σ* with a disc-minus-surroundings contrast of the
same sign, or whose disc contrast exceeds 0.04 at > *6σ*
with an interior offset above 0.08 (S<sup>′</sup> re-estimated with
the candidate discs masked), receives a jointly fitted
constant over its full footprint, clipped to 1.25 times
the measured contrast and accepted only if it reduces
that tile’s edge step (or its disc contrast by 30 per cent
without raising the step) and does not move the tile’s
interior more than 0.02 further from the model reference
introduced below—without this guard the step between
a levelled scattered-light tile and its clean neighbour
is ‘repaired’ by darkening the neighbour—followed by
the same in-disc flattening outside protected regions
and nebular tiles; 3.1/2.1 per cent of FUV/NUV tiles
receive such a constant, and their median edge step falls
from 0.060/0.070 to 0.027/0.028. The second addresses
light scattered into the field by ultraviolet-bright stars
that *GALEX* itself avoided, for which a neighbour reference either does not exist or is itself contaminated,
and for which a per-tile flag necessarily stops at the
first ring of tiles while the scattered light does not. A
census of every tile within 2.5<sup>◦</sup> of an avoided TD-1 star
compares its 0.5<sup>◦</sup> interior with its 0.7<sup>◦</sup> –1.7<sup>◦</sup> surroundings and with the gap-fill prediction and identifies 30
FUV and 74 NUV stars (Magellanic fields excluded)
with at least one tile whose excess is above 0.15 dex, or
above 0.08 dex with a gradient > 0.15 dex directed at
the star. Around each such star the correction is rebuilt
on all tiles within 3.6<sup>◦</sup> (2.9/6.8 per cent of FUV/NUV
tiles) against a neutral template, the gain-harmonised
gap-fill prediction of Section 3.9 with its stellar layer
subtracted, *T:* with *y* = (*g − S)/T* on source-free pixels,
the local clean level *k(r,ϕ*) is the median of *y* over tiles
that are neither census-flagged nor inside the current
scattered-light footprint, in 0.2<sup>◦</sup> *×* 30<sup>◦</sup> polar bins about
the star (empty bins inherit the nearest outer bin of
their sector); the scattered light is modelled as a smooth
non-negative field *E(r,ϕ),* the binned median of *y − k*
over all observed pixels, Gaussian-smoothed by one bin
and truncated at the outermost radius where the sector

$$
(3.7\times10^{7}
$$

$$
r_{i}-r_{j}
$$

$$
\left(\tau_{o}=0.30;\tau_{a}=0.30\right)
$$

$$
(c=2.5)
$$

$$
S'
$$

$$
R(\rho)
$$

$$
|o_{t}|>0.25
$$

$$
0.35^{\circ}
$$

$$
S'.
$$

$$
(>3R)
$$

$$
E(B-V)>0.15
$$

$$
S'
$$

$$
>4.5\sigma
$$

$$
(S'
$$

$$
2.5^{\circ}
$$

$$
0.5^{\circ}
$$

$$
0.7^{\circ}-1.7^{\circ}
$$

$$
>0.15\mathrm{dex}
$$

$$
3.6^{\circ}
$$

$$
y=(g-\bar{S})/T
$$

$$
k(r,\phi)
$$

$$
y
$$

$$
0.2^{\circ}\times30^{\circ}
$$

$$
E(r,\phi)
$$

$$
y-k
$$

still exceeds 0.01, plus per-tile constant and tangentplane gradient terms fitted jointly (exposure-weighted
tile footprints, ridge-damped, *2.5σ*-clipped) to the residual and kept only where significant (|c<sub>t</sub>| > 0.03 at > 4σ;
gradients > 0.03 across the tile at > 3σ). Positive tile
terms are allowed anywhere inside the footprint of *E,*
negative ones only on tiles the neighbour-pair rounds
had altered, so that a tile is returned to the local clean
level *k* rather than below it and no untouched clean tile
is changed; footprint, clean set, *k, E* and tile terms are
iterated three times. The smooth field carries a third of
the removed light (median peak amplitude 0.03/0.04 of
the template, up to 0.12/0.28 at the worst stars); the
tile terms (94/367 constants, 208/617 gradients; median
|c<sub>t</sub>| = 0.06/0.07) carry the visit-to-visit remainder. Tiles
whose modelled excess exceeds 0.5 of the template, or
0.10 with residual 14<sup>′</sup> structure above 0.10 rms after the
fit, are dominated by scattered light rather than sky and
are removed from the mosaic (5 FUV and 49 NUV tiles,
4.2/49.0deg<sup>2</sup>), to be filled by the fusion like any other
gap. Measured as the ratio of corrected to neighbouring
◦
uncorrected pixels, both divided by *T,* per 45 sector
within 3<sup>◦</sup> of each star, the tiles so treated sat at a median
of 1.04–1.06 (up to 1.2–2.4 in individual sectors) before
the correction and at 1.00–1.01 after it, with 22/125
(FUV) and 63/422 (NUV) sectors outside *±3* per cent
and 4/17 outside *±5* per cent, against 66/231 before.
Together the targeted passes change 2.2/4.1 per cent of
the *GALEX* pixels of each band by more than 1 per cent;
the removed pattern carries 0.04–0.1 (FUV) and 0.5–1.2
per cent (NUV) of the mosaic’s power at *ℓ* = 30–150
and is uncorrelated with *E(B −V*) and *Hα* (*|r|≤* 0.05
in the 1<sup>◦</sup> –3<sup>◦</sup> band), the band-passed correlation of the
mosaic itself with both templates rising slightly at each
pass. Two constraints protect the astrophysical content: the component of the offset map O = S<sup>′</sup> m on
scales > 2.5<sup>◦</sup> is projected out and its footprint mean
set to zero, so the isotropic level and the degree-scale
sky are untouched by construction; and *O* is replaced
by a local constant inside *r* = 0.75 *D<sub>25</sub>* + 0.42<sup>◦</sup> of 315
galaxies with D<sub>25</sub> > 5<sup>′</sup>, generous LMC/SMC/M31/M33
ellipses and 0.4<sup>◦</sup> of 147 globular clusters, while inside
21 bright-star ultraviolet scattering haloes (e.g. Spica)
the ridged solution is kept so that the halo, whatever
its nature, is left as observed. The *GALEX* layer is
g = G<sup>′</sup> −O (files galex_{fuv,nuv}_equalised); O, the
per-tile table (including the scattered-light and removal
flags) and the tile footprints are published. The rms
of *O* over *GALEX* pixels is 60 CU (FUV) and 157 CU
(NUV) outside the bright-star scattered-light regions
(70 and 202 CU with them); the equalisation reduces the
excess step at exposed tile edges of the mosaics by 60
per cent (FUV) and 77 per cent (NUV) and the number
of tiles with a residual disc contrast above 0.05 at > *5σ*
to 4.3/5.1 per cent of tiles, two fifths of which lie on
nebular/dusty tiles or in protected regions where the
correction is withheld by design; injection–recovery of
synthetic offsets and rims through the full procedure
returns slopes of 0.94/0.94 (offsets) and 0.89/0.92 (rims).

$$
(|c_{t}|>0.03
$$

$$
>4\sigma;
$$

$$
>0.03
$$

$$
>3\sigma)
$$

$$
|c_{t}|=0.06/0.07)
$$

$$
14'
$$

$$
4.2/49.0deg^2)
$$

$$
T,
$$

$$
45^{\circ}
$$

$$
3^{\circ}
$$

$$
\ell=30–150
$$

$$
E(B-V)
$$

$$
\left(|r|\leq0.05\right.
1^{\circ}-3^{\circ}
$$

$$
O=S'm
$$

$$
>2.5^{\circ}
$$

$$
r=0.75D_{25}+0.42^{\circ}
$$

$$
D_{25}>5'
$$

$$
\mathrm{LMC/SMC/M31/M33}
$$

$$
0.4^{\circ}
$$

$$
g=G^{\prime}-O
$$

$$
O,
$$

---

NUV before: (g-S)/S l=221 b=84 12deg

removed per-tile model: O/S l=221 b=84 12deg

after: (g-O-S)/S l=221 b=84 12deg

NUV before: (g-S)/S l=124 b=26 15deg

removed per-tile model: O/S l=124 b=26 15deg

after: (g-O-S)/S l=124 b=26 15deg

Polaris flare cirrus

Figure 7: Per-tile background equalisation in two NUV fields (top: Coma, *l,b* = 221<sup>◦</sup>,+84<sup>◦</sup>, 12<sup>◦</sup>across; bottom: Polaris-flare cirrus,
124<sup>◦</sup>,+26<sup>◦</sup>, 15<sup>◦</sup>): 2.5<sup>◦</sup>-high-pass relative residual of the destriped *GALEX* mosaic before (left), the fitted per-tile offset+rim+chord
model *O/S*<sup>′</sup>that is removed (middle), and the residual after (right), on a common ±0.12 stretch (sources masked white). The model
consists of discs, rims and chords only; the cirrus of the Polaris field is absent from it and unchanged in the corrected residual. The discs
that remain in the Coma field are pointings on NGC 4565, NGC 4559 and their neighbours, inside protected galaxy regions where *O* is
held at a local constant.

$$
l,b=221^{\circ},+84^{\circ},12^{\circ}
$$

$$
12\bar{4}^{\circ},+26^{\circ},
$$

$$
O/S'
$$

### 3.4 Artefact masks and bright-star sat- uration top-up

*Artefact masks.* Residual instrumental features of the
*GALEX* layer are down-weighted, never edited. Cells
at N<sub>side</sub> = 512 where |log<sub>10</sub>(g/P˜<sub>loc</sub>)| > 0.2 dex against
the local harmonised prediction are grouped into 8-
connected components; a component is classed as instrumental if extended (*≥* 3 cells), a positive excess within
12<sup>′</sup> of a V < 5 star, or touching the coverage edge, and
not coincident with an RC3 galaxy, globular cluster or
planetary nebula. A multiplier M<sub>A</sub>, zero inside each of
the 642 (FUV) and 433 (NUV) components and rising
to unity over 15<sup>′</sup>, multiplies the GALEX confidence
factor in the fusion so that the affected 123/119deg<sup>2</sup> are
filled by lower-priority layers; the NUV multiplier also
multiplies the confidence of the NUV-informed FUV
layer, which inherits the NUV artefacts (Section 3.9).
Maps and flags are published.

$$
N_{side}=512
$$

$$
|\log_{10}(g/\tilde{P}_{\operatorname{loc}})|>0.2
$$

$$
(\geq3cells)
$$

$$
12'
$$

$$
V<5
$$

$$
M_{\mathrm{A}}
$$

$$
123/119deg^2
$$

$$
15^{\prime}
$$

*TD-1 top-up of censored stars.* Bright stars above
*GALEX* ’s count-rate limits are absent or masked
by 6<sup>′</sup> holes. For TD-1 stars with f<sub>λ</sub> > 2 ×
10<sup>−11</sup> ergcm<sup>−2</sup> s<sup>−1</sup> Å<sup>−1</sup> inside the footprint whose 6<sup>′</sup>
*GALEX* aperture is deficient, the missing fraction of

$$
f_{\lambda}>2\times
6'
$$

$$
10^{-11}\mathrm{erg}\mathrm{cm}^{-2}\mathrm{s}^{-1}\mathrm{\AA}^{-1}
$$

$$
6'
$$

the TD-1 flux is deposited with the TD-1 point-spread
function into *g* (and into the imaged NUV used by
the NUV-informed FUV layer, Section 3.8); the median
log<sub>10</sub>(aperture/TD-1) of TD-1-bright stars changes from
−1.09 to 0.00 dex. The *GALEX* fusion layer *I<sub>1</sub>* is *g* plus
this top-up, undefined outside the footprint.

$$
g
$$

$$
\log_{10}
$$

$$
I_{1}
$$

### 3.5 The source layer: stars, galaxies and extragalactic points

$$
S=S_{\star}+E+L_{\mathrm{gal}}
$$

At 1.7<sup>′</sup> individual stars dominate the local intensity over
much of the sky, and no full-sky template localises them.
Discrete sources therefore enter the filled sky through
a catalogue-based source layer S = S<sub>⋆</sub> + E + L<sub>gal</sub> built
from the full depth of *Gaia* DR3, which also partitions
both maps into resolved sources and a diffuse remainder.
Validation figures are collected in Appendix C.2; the
photometric behaviour of sources in the published map
is tested in Section 4.5.

*Gaia harvest and ultraviolet photometry.* All 1.116 *×*
10<sup>8</sup> Gaia DR3 sources with G < 16.5 and the 7.2 ×
10<sup>6</sup> blue or hot sources (BP − RP < 0.6 or GSP-Phot
T<sub>eff</sub> > 7500 K) with 16.5 ≤ G < 19 were retrieved
in HEALPix-ordered chunks whose row counts were

$$
1.116\times
10^{8}
$$

$$
G<16.5
$$

$$
10^{6}
$$

$$
7.2\times
\left(\mathrm{BP}-\mathrm{RP}<0.6\right.
T_{eff}\;>\;7500K)
$$

$$
16.5\leq G<19
$$ verified against server-side counts, with the ESP-HS
hot-star parameters (2.4 × 10<sup>6</sup> stars), the white-dwarf
catalogue of Gentile Fusillo et al. (2021) (3.6 × 10<sup>5</sup>,
P<sub>WD</sub> > 0.75) and the hot-subdwarf lists of Culpan
et al. (2022) (6.8 × 10<sup>4</sup>). Ultraviolet photometry for
training and validation is GUVcat AIS (83.0 million
rows, 3.1 × 10<sup>7</sup> matched to Gaia within 3<sup>′′</sup>; Bianchi
et al. 2017), the *Swift* UVOTSSC and XMM-OM SUSS
catalogues (4.2 × 10<sup>6</sup> and 5.1 × 10<sup>6</sup> Gaia matches, 56
and 67 per cent outside the *GALEX* FUV footprint;
Yershov 2014; Page et al. 2012), UVIT DR1, SMC and
M31 photometry (Tandon et al. 2017)—never used in
training—and TD-1.

$$
(2.4\times10^{6}
$$

$$
P_{WD}\;>\;0.75)
$$

$$
(6.8\times10^{4})
$$

$$
3.1\times10^{7}
$$

$$
3'';
$$

$$
(4.2\times10^{6}
$$

$$
5.1\times10^{6}
$$

*Per-star predictors.* For normal stars the *GALEX*-
system AB magnitude is a physical backbone—synthetic
photometry of ATLAS9 spectra (Castelli & Kurucz 2003)
at the GSP-Phot T<sub>eff</sub>, logg, [M/H] and A<sub>0</sub>, anchored
to *G*—corrected by a histogram gradient-boosted regression of the GUVcat residual on *Gaia* colours, *MG,*
parallax quality, RUWE, flux excess, *Planck E(B −V*)
and latitude (84 per cent of stars; a colour-only regression otherwise). GUVcat is NUV-selected and only 5
per cent of clean training stars have an FUV detection,
so the FUV regression is a censoring-aware (Tobit-type)
expectation–maximisation fit in which non-detections in
FUV-exposed tiles enter as limits at the local 50 per cent
completeness magnitude, 17.6 + 1.6 log<sub>10</sub>(t<sub>FUV</sub>/s); out
of fold it places 99.8 per cent of 8.1 ×10<sup>6</sup> non-detections
fainter than that limit, where a detections-only fit is
biased bright by 1.3–2.5mag at 0.3 < BP *−* RP < 1.4.
White dwarfs (Koester DA grid), hot subdwarfs and
ESP-HS O/B/A stars (ATLAS9) use an analogous
physical-plus-residual quantile regression, the physical
value alone being adopted brightward of the *GALEX*
non-linearity limit. In five-fold cross-validation blocked
◦
on 7.3 cells the out-of-fold robust scatter for 8.6 × 10<sup>6</sup>
normal stars is 0.16mag in the NUV (0.10 at *G* = 11–13,
0.27 at 17–19; median offsets within ±0.02 mag in every
*G, |b|* and *E(B −V*) bin) and 0.38mag in the FUV for
depth-complete detections (0.35 at BP *−* RP < 0.3);
for ESP-HS B/A stars it is 0.13/0.10 (NUV) and
0.27/0.32mag (FUV), for white dwarfs 0.25/0.44 and for
known hot subdwarfs 0.13/0.22, with 63–71 per cent of
residuals inside the predicted *±1σ.* Blind tests outside
the *GALEX* footprint, where the layer is used, give for
normal stars NUV−UVM2 medians of −0.02 (UVOT,
1.07 × 10<sup>5</sup> stars) and +0.02 mag (XMM-OM, 6.7 × 10<sup>4</sup>)
with robust scatter 0.26/0.22mag and no plane bias
beyond ±0.07 mag up to line-of-sight *E(B−V*) > 3,
and FUV−UVIT F148W/F154W of +0.06 (0.45) and
−0.03 (0.33)mag; for hot stars, FUV/NUV−TD-1 of
−0.04 (0.25) / −0.10 (0.29)mag at *G* ≲ 10, FUV−UVIT
F148W of −0.06 (0.24)mag in Magellanic and plane
fields, and an NUV scatter against UVOT of 0.17–
◦
0.18mag at *|b|* < 10 (0.20 at *E(B −V*) > 1).

$$
T_{eff}
$$

$$
A_{0},
$$

$$
M_{G},
$$

$$
17.6+1.6\log_{10}(t_{\mathrm{FUV}}/\mathrm{s})
$$

$$
8.1\times10^{6}
$$

$$
0.3<\mathrm{BP}-\mathrm{RP}<1.4
$$

$$
7.3^{\circ}
$$

$$
8.6\times10^{6}
$$

$$
1.07\times10^{5}
$$

$$
6.7\times10^{4})
$$

$$
\pm0.07
$$

$$
E(B-V)>3
$$

$$
G\lesssim10
$$

$$
\left|b\right|<10^{\circ}
$$

$$
E(B-V)>1)
$$

stellar sources takes, per band, the first available of:
TD-1 at S/N ≥ 5, multiplied by 1.047 onto the GALEX

$$
S/N\geq5
$$

model flux for *GALEX*-saturated stars without TD-
1 (992/538), from a gradient-boosted model trained
on 4577/5256 TD-1-matched BSC stars that predicts
unmatched stars with a held-out scatter of 0.06–0.10 dex
for O–B stars; the measured GUVcat magnitude where
unsaturated and clean (7.7 × 10<sup>5</sup> / 1.17 × 10<sup>7</sup>); the
hot/degenerate prediction (2.5 × 10<sup>6</sup> / 2.1 × 10<sup>6</sup>); and
the normal-star prediction (1.16 × 10<sup>8</sup> / 1.05 × 10<sup>8</sup>). A
per-source *σ* and flux origin are recorded. The summed
stellar flux is 1.107 × 10<sup>5</sup> (FUV) and 9.04 × 10<sup>4</sup> (NUV)
photonscm<sup>−2</sup> s<sup>−1</sup> Å<sup>−1</sup>; stars fainter than G = 13 carry
1.1/2.2 per cent of it over the sky but 91 per cent (FUV;
hot subdwarfs and white dwarfs) and 31 per cent (NUV)
of the stellar flux of a median N<sub>side</sub> = 256 cell at |b| >
30<sup>◦</sup>. The 160099 *Gaia* quasar and galaxy candidates
without stellar astrometry form the extragalactic pointsource layer E (8.9/8.6 photonscm<sup>−2</sup> s<sup>−1</sup> Å<sup>−1</sup> in total).

$$
\left(7.7\times10^{5}/1.17\times10^{7}\right)
$$

$$
\left(2.5\times10^{6}\right.\left./2.1\times10^{6}\right)
$$

$$
\left(1.16\times10^{8}\;/\;1.05\times10^{8}\right)
$$

$$
\sigma
$$

$$
7\times10^{5}
$$

$$
9.04\times10^{4}(NUV)
$$

$$
G=13
$$

$$
1.1/2.2
$$

$$
\mathrm{cm^{-2}\:s^{-1}\:\mathring{A}^{-1}};
$$

$$
|b|>
$$

$$
30^{\circ}
$$

$$
N_{side}=256
$$

$$
\mathrm{n^{-2}\:s^{-1}\:\mathring{A}^{-1}}
$$

Galaxies and globular clusters L<sub>gal</sub>. The integrated
light of 14283 nearby galaxies (171 magnitudes predicted with 0.85/0.63mag rms) and 146 globular clusters (104 predicted with 0.69/0.41mag rms) is rendered
as exponential discs of scale length *D<sub>25</sub>/6.4* truncated
at *1.2R<sub>25</sub>* and as Plummer profiles, respectively; M31,
M33 and the Magellanic Clouds, which are imaged, are
excluded. Against *D<sub>25</sub>* photometry of 605 isolated galaxies the layer gives median *GALEX* /layer ratios of 0.94
in the FUV and 1.16 in the NUV. Its all-sky mean is
1.35/1.63 CU; inside *W*<sub>GALEX</sub>> 0.5 its contribution is
zero.

$$
L_{\mathrm{gal}}
$$

$$
D_{25}/6.4
$$

$$
1.2R_{25}
$$

$$
D_{25}
$$

$$
W_{GALEX}>0.5
$$

*Rendering.* Outside ultraviolet imaging sources are
deposited flux-conservingly as 2.0<sup>′</sup> Gaussians (3.0<sup>′</sup> for
TD-1/BSC stars, with 10 per cent in a 6<sup>′</sup> wing for the
brightest). Inside imaging the point-source response of
the published map, measured on 5854 isolated GUVcat
stars, is a pixel-limited core (σ = 0.20<sup>′</sup>) with 12.5 per
cent of the flux in a 6<sup>′</sup>-FWHM wing; aperture photometry of the map at GUVcat stars gives map/catalogue flux
ratios *κ* = 1.205 (FUV) and 1.213 (NUV), applied to
catalogue- and prediction-tier sources in the subtracted
(‘map-matched’) layer, TD-1/BSC stars entering unscaled. Per-pixel *1σ* maps σstarsare propagated from
lognormal per-star errors. In the Magellanic Clouds
◦ ◦
(8 /5 discs) the Galactic-calibrated predictions of ∼ 10<sup>6</sup>
members exceed the *GALEX* surface photometry, so
the stellar input to the fusion is capped cell-wise at a
reference level consistent with the surrounding imaging
(62deg<sup>2</sup>, the LMC bar and SMC body; Section 3.10).
Inside the footprint, stellar content follows the *GALEX*
mosaics, with *GALEX* non-linearity for NUV ≲ 15
stars inherited unflagged; very bright cool stars are the
population the hierarchy serves least well (Section 6).

$$
2.0^{\prime}
$$

$$
6'
$$

$$
(3.0^{\prime}
$$

$$
(\sigma=0.20^{\prime})
$$

$$
(8^{\circ}/5^{\circ}
$$

$$
\sim10^{6}
$$

$$
(62\deg^{2}
$$

$$
\mathit{GALEX}
$$

$$
\mathrm{NUV}\;\lesssim\;15
$$

*Partition and completeness.* The published diffuse
maps are the total minus the map-matched stellar layer,
E and L<sub>gal</sub>, provenance-weighted so that outside imaging they equal the harmonised diffuse prediction exactly (all-sky medians 1063/1140 CU; files diffuse_-
{b}). Stacked residuals at the positions of 14–18mag
layer stars are −4/−3 per cent (FUV/NUV) of the
star flux inside imaging and +12/−1.5 per cent outside, the FUV excess being FIMS/SPEAR-resolution

$$
L_{goal},
$$

$$
\tt d i f f u s e_{-}-
$$

$$
-4/-3
$$ starlight. Outside the Clouds 0.25 per cent (FUV) and
1.42 per cent (NUV) of N<sub>side</sub> = 2048 pixels lie below
*−2σ*<sub>stars</sub>(0.43/1.73 per cent negative at all; model-class
pixels 0.00–0.02 per cent), the NUV rate reflecting GU-
Vcat magnitude errors smaller than the true per-star
map/catalogue scatter; inside the Clouds 24 per cent
are negative and the region must be masked in diffuse analyses. Ninety-nine per cent of the layer flux
of a median N<sub>side</sub> = 256 cell is reached at G = 17.25
(FUV) / 16.5 (NUV), i.e. FUV 23.3 / NUV 21.8AB; a
power-law extrapolation of the differential source flux
to *m* = 23 leaves an unresolved stellar remainder, published at N<sub>side</sub> = 64, of median 4.2/14 CU over the sky,
◦
2.8/5.5 CU at *|b|* > 30 (*≤* 0.7 per cent of the diffuse
◦
level) and 13/242 CU at *|b|* < 10, so that only in the
NUV plane can unresolved starlight reach *∼* 10 per cent
of the diffuse map.

$$
N_{side}=2048
$$

$$
-2\sigma_{stars}(0.43/1.73
$$

$$
N_{side}=256
$$

$$
G=17.25
$$

$$
m=23
$$

$$
N_{side}=64
$$

$$
4.2/14CU
$$

$$
2.8/5.5\mathrm{CU}
$$

$$
|b|>30^{\circ}(\leq0.7
$$

$$
13/242CU
$$

$$
\left|b\right|<10^{\circ}
$$

### 3.6 Cross-calibration of UVOT and FIMS/SPEAR

*UVOT.* Count and exposure maps give a rate per filter,
and both UVOT and GALEX are smoothed to 1.7<sup>′</sup>.
For each UVOT filter and each *GALEX* band we fit
an isotonic regression of logg on logr<sub>UVOT</sub> over 0.7–
0.9 million common N<sub>side</sub> = 2048 pixels, with robust
scatter of 0.11 dex for the closest filter–band pairs and
0.25 dex for the FUV mapping. The transformation
is colour dependent: log(g<sub>FUV</sub>/UVOTFUV) drifts by
0.51 *±* 0.01 dex per decade of *E(B − V*) above *E(B −
V*) = 0.03 (Section 4.4). UVOT therefore receives a
capped weight (*c<sub>3</sub>* = 0.8), is gain-matched regionally
(Section 3.9), and is treated as a validation reference
of 0.1–0.2 dex precision; overlapping filter estimates are
combined with inverse-variance weights into the mapped
UVOT layer.

$$
1.7'
$$

$$
N_{side}=2048
$$

$$
\left(g_{\operatorname{FUV}}/\operatorname{UVOT}_{\operatorname{FUV}}\right)
$$

$$
0.51\pm0.01
$$

$$
E(B-V)
$$

$$
V)=0.03
$$

$$
(c_{3}=0.8)
$$

*FIMS/SPEAR.* Averaged to the local FIMS/SPEAR
resolution, g<sub>FUV</sub> and the FIMS/SPEAR starry map
have a median ratio of 0.97–1.08 by resolution tier with
0.13 dex robust scatter at 55<sup>′</sup>, and the ratio is flat in
*E(B−V*) to ±0.04 dex over 0.01 < *E(B−V*) < 1.2
(Section 4.4); no rescaling is applied beyond an empirical
global factor of 0.97 on the starless map. Because its
resolution is 10–30 times coarser than our grid, FIMS/
SPEAR is not injected as an image layer but constrains
the large-scale level of the FUV model layer far from
*GALEX;* the constraint is built in the fusion from the
*starless* map plus our own source layer, so that a bright
star inside the FIMS/SPEAR beam is not represented
twice (Section 3.9). The TD-1 scale is tied to the same
*OAO-2/IUE*-era flux scale as *GALEX* through whitedwarf standards and enters through the factor 1.047 of
Section 3.5.

$$
E(B-V)
$$

$$
0.01<E(B-V)<1.2
$$

$$
OAO-2/IUE-era
$$

### 3.7 The diffuse predictor

Where no ultraviolet imaging exists we require an estimate of the diffuse surface brightness that is unbiased,
retains true small-scale structure, and carries an empirical error model; discrete sources are added separately

through *S.* We cast this as supervised regression with
the star-subtracted *GALEX* sky as the training set, using histogram-based gradient-boosted regression trees
(HGB; Friedman 2001; Ke et al. 2017) in scikit-learn
(Pedregosa et al. 2011), minimising the squared error of
log<sub>10</sub> *I.* Trees were preferred over convolutional models
because the mapping from dust, gas and starlight tracers to ultraviolet intensity is local and non-linear, and
trees handle missing features natively and are cheap to
retrain for cross-validation. Their inability to extrapolate beyond the training feature range is relevant to the
inner Galactic plane (Section 4.2).

$$
\log_{10}I
$$

*Features.* The predictor uses the 90 features of Table 3:
pixel values of the dust, Hα, starlight and positional
templates, plus features encoding each pixel’s *neighbourhood*—Gaussian-smoothed templates at six scales with
their high-pass residuals, gradient amplitudes, texture,
coarser-scale source density, *τ<sub>353</sub>,* colour, and ecliptic
coordinates. The physical motivation is that ultraviolet intensity depends on the illuminating field, set by
sources and dust over degrees around it. All features
derive from full-sky templates; none uses the target or
ultraviolet data.

Target. The target y is defined at N<sub>side</sub> = 512 as the
mean of *g − S* (with *S* the map-matched source layer
of Section 3.5) over the fully imaged children of each
cell (*W*<sub>GALEX</sub>*≥* 0.99; children below *−5σ*<sub>stars</sub>dropped),
with the Magellanic Clouds masked and cells flagged
as non-positive, bright-star artefact or noisy removed,
leaving ≃ 2×10<sup>6</sup> cells per band. The explicit subtraction
makes the target the diffuse light as partitioned in the
published maps, rather than an order-statistic proxy for
it.

$$
N_{side}=512
$$

$$
g-S
$$

$$
-5\sigma_{stars}
$$

$$
\left(W_{GALEX}\geq0.99;\right.
\stackrel{\frown}{\simeq}2\times10^{6}
$$

*Training and cross-validation.* All performance figures are out-of-fold values from a six-fold spatial
block cross-validation, folds being unions of N<sub>side</sub> = 8
super-pixels (7.3 ) assigned by a fixed permutation<sup>◦</sup>
(151/121/122/120/130/124 super-pixels per fold); the
median held-out-to-training distance is 1.27<sup>◦</sup> (90th percentile 2.85 ; tail to 7<sup>◦</sup>.8 ). Held-out<sup>◦</sup> *GALEX* pixels are
re-weighted to the (*E(B −* V), *|b|*) distribution of the
filled sky (‘gap-matched’) for both training and scoring.
Each cross-validation model is trained on 5 × 10<sup>5</sup> pixels
per fold; the deployed model, which uses all folds, is
trained on 4×10<sup>5</sup> pixels entered three times, with sample
√
weights 0.3 + 0.7 clip( *w/w,¯* 0,10). Hyper-parameters
(800 iterations, learning rate 0.05, *≤* 127 leaves, *≥* 40
samples per leaf, L2 regularisation 1) were chosen on
the out-of-fold scatter, with tuning gain *≃* 0.002 (FUV)
and 0.003 dex (NUV) (Appendix B). The benchmark
file, with targets, folds and gap-matching weights, is
published for identical scoring of alternative models
(Appendix A); the same out-of-fold residuals feed the
uncertainty model (Section 4.6).

$$
N_{side}=8
$$

$$
(151/121/122/120/130/124
$$

$$
1.27^{\circ}
$$

$$
2.85^{\circ};
$$

$$
7.8^{\circ})
$$

$$
5\times10^{5}
$$

$$
4\times10^{5}
$$

$$
0.3+0.7\mathrm{clip}(\sqrt{w/\bar{w}},0,10)
$$

$$
0.05,\leq127
$$

$$
\geq40
$$

*Jitter-regularised positional features.* Axis-aligned
splits on coordinates can produce discontinuities along
coordinate lines. Each training pixel is entered with its
true templates plus two copies perturbed by Gaussian
◦ ◦
jitter (σ<sub>b</sub> = 2, σ<sub>l</sub> = 2.5 /cosb), and predictions are
averaged over six jittered draws. Across *b* = 0 the de-

$$
\left(\sigma_{b}=2^{\circ},\sigma_{l}=2.5^{\circ}/\cos b\right)
$$

---

Table 3: Features of the gap-fill predictor (90 in total), all evaluated from full-sky templates at *N*<sub>side</sub>= 512 for training and *N*<sub>side</sub>= 1024
for prediction. Sθdenotes Gaussian smoothing of FWHM θ; ‘high-pass’ is pixel value minus S<sub>θ</sub>. Logarithms of Hα and Gaia source
density are floored at *−1* (0.1R and 0.1 stars per pixel) and those of the *Gaia* fluxes at 0; the dust templates are strictly positive and
need no floor (the published feature matrix records the values used). The six templates marked *†* plus distance to data are the inputs of
the uncertainty model (Section 4.6).

$$
N_{side}=512
$$

$$
N_{side}=1024
$$

$$
\mathcal{S}_{\theta}
$$

$$
\theta;
$$

$$
\mathcal{S}_{\theta}
$$

| Group | Features |
| --- | --- |
| Base (12) | \(\log E(B-V)^{†}\), \(\log τ_{353}\), \(\log \mathcal{R}\) (Planck Collaboration XI 2014); \(\log I_{Hα}^{†}\) (Finkbeiner 2003); \(\log F_{BP}^{†}\), \(\log F_{RP}\), \(\log (F_{BP}/F_{RP})\), \(\log n_{*}^{†}\) (Gaia Collaboration 2023); \(\sin b\), \|b\|†, \(\cos l\), \(\sin l\) (jittered) |
| Multi-scale (48) | \(\mathcal{S}_{θ}[x]\) and \(x - \mathcal{S}_{θ}[x]\) for \(x \in \{\log E(B-V), \log \mathcal{R}, \log I_{Hα}, \log F_{BP}\}\) and θ = 0.5°, 1°, 2°, 4°, 8°, 16° |
| Gradients, texture (10) | \(\|∇ \mathcal{S}_{θ}[x]\|\) for \(x \in \{\log E(B-V), \log \mathcal{R}, \log I_{Hα}\}\), θ = 0.5°, 2°; standard deviation of the eight neighbours of \(\log E(B-V)\), \(\log \mathcal{R}\), \(\log I_{Hα}\), \(\log F_{BP}\) |
| Secondary scales (15) | \(\mathcal{S}_{θ}\) and high-pass of \(\log n_{*}\) and \(\log τ_{353}\) at θ = 1°, 4°, 16°, and \(\mathcal{S}_{θ}\) of BP/RP colour at 1°, 4° (plus high-pass at 1°) |
| Ratios (2) | \(\log \mathcal{R} - \log τ_{353}\) (dust temperature proxy), \(\log E(B-V) - \log τ_{353}\) |
| Ecliptic (3) | \|β\|, \(\sin λ\), \(\cos λ\) |

$$
\log E(B-V)^{\dagger}
$$

$$
I_{\mathrm{H}\alpha}^{\dagger}
$$

$$
F_{\mathrm{BP}}^{\dagger},
$$

$$
F_{\mathrm{RP}},
$$

$$
\mathrm{Collaboration~2023};\sin b,\left|b\right|^{\dagger},\cos l,
$$

$$
x-\mathcal{S}_{\theta}[x]
$$

$$
\left\{\log E(B-V),\log\mathcal{R},\log I_{\mathrm{H}\alpha},\log F_{\mathrm{BP}}\right\}
$$

$$
\mathcal{S}_{\theta}[x]
$$

$$
x\in
$$

$$
\theta=0.5^{\circ},1^{\circ},2^{\circ},4^{\circ},8^{\circ},16^{\circ}
$$

$$
\left|\nabla\mathcal{S}_{\theta}[x]\right|
$$

$$
x\in\left\{\log E(B-V),\log\mathcal{R},\log I_{\mathrm{H}\alpha}\right\},
$$

$$
\theta=0.5^{\circ},2^{\circ};
$$

$$
I_{\mathrm{H}\alpha},
$$

$$
F_{\mathrm{BP}}
$$

$$
\mathcal{S}_{\theta}
$$

$$
\theta=1^{\circ},4^{\circ},16^{\circ}
$$

$$
\mathcal{S}_{\theta}
$$

$$
\mathrm{BP/RP}
$$

$$
1^{\circ},4^{\circ}
$$

$$
E(B-V)-\log\tau_{353}
$$

$$
|\beta|,\sin\lambda,
$$

ployed prediction steps by −0.002/−0.003 dex between
the ±0.125<sup>◦</sup> bins and by 0.030/0.041 dex rms over eighteen longitude sectors, against 0.111 dex for the *Planck*
radiance itself: no seam. The cost in held-out scatter
is +0.0018 *±* 0.0003 dex (NUV, significant); we accept
it for removal of coordinate-aligned steps.

$$
\pm0.125^{\circ}
$$

$$
+0.0018\pm0.0003
$$

*Accuracy and deployment.* In the six-fold block crossvalidation the predictor scores an area-weighted/gapmatched robust scatter of 0.037/0.047 dex (FUV) and
0.047/0.057 dex (NUV) on the diffuse target, and
0.041/0.052 and 0.047/0.058 dex for prediction plus *S*
against the total *g;* part of this scatter is noise in the
target rather than prediction error (Section 4.2). Predictions are evaluated at N<sub>side</sub> = 1024 and interpolated
bilinearly in logI to N<sub>side</sub> = 2048, giving the diffuse
prediction P<sub>D</sub>.

$$
g;
$$

$$
N_{side}=1024
$$

$$
N_{side}=2048
$$

$$
P_{\mathrm{D}}
$$

### 3.8 The NUV-informed FUV layer

In the 11.9 per cent of the sky imaged by *GALEX*
in the NUV only, a 5<sup>′′</sup> ultraviolet image exists in the
neighbouring band, and the FUV/NUV colour of the
diffuse sky is a smooth, predictable function of dust
column, latitude and stellar content (Section 5). Stars,
by contrast, span colours from 10<sup>c</sup> ≪ 1 to NUV fluxes
10–60 times their FUV flux and cannot be transferred
with the diffuse colour. We therefore predict the *diffuse*
colour,
()
\

$$
5^{\prime\prime}
$$

$$
10^{c}\ll1
$$

$$
c_{\mathrm{d}}=\log_{10}\biggl(\frac{\widehat{g_{\mathrm{FUV}}-S_{\mathrm{FUV}}}}{g_{\mathrm{NUV}}-S_{\mathrm{NUV}}}\biggr),
$$

(3)

with an HGB regressor on the twelve base templates of
Table 3 plus logg<sub>NUV</sub>, trained on the star-subtracted

GALEX FUV and NUV at N<sub>side</sub> = 512 (six-fold crossvalidation robust scatter 0.0351 dex, *R<sup>2</sup>* = 0.966), evaluated at N<sub>side</sub> = 1024 and interpolated in log to
N<sub>side</sub> = 2048, and define wherever GALEX NUV exists

$$
N_{side}=512
$$

$$
R^{2}=0.966)
$$

$$
N_{side}=1024
$$

$$
I_{\mathrm{X}}=\left(g_{\mathrm{NUV}}-S_{\mathrm{NUV}}\right)_{\mathrm{L}}\times10^{\frac{c_{\mathrm{d}}}{}}+S_{\mathrm{FUV}},
$$

where S<sub>NUV</sub> and S<sub>FUV</sub> are the stellar layer S<sub>⋆</sub> of Section 3.5 in the two bands (catalogue fluxes deposited
with the model PSF), g<sub>NUV</sub> is the imaged NUV without the TD-1 top-up, and the subscript L denotes that
star cores (S<sub>NUV</sub> above the local star-free level L, the
N<sub>side</sub> = 512 median of the imaged NUV over pixels
with S<sub>NUV</sub> < 0.05L) and over-subtracted wings (residual < 0.5L where S<sub>NUV</sub> > 0.02L; together 6.6 per cent
of the NUV-imaged FUV-gap pixels) are replaced by
*L.* Stars in NUV-informed sky therefore enter the FUV
map with their catalogue (or predicted) FUV flux, exactly as in model sky, while the diffuse morphology is
that recorded by the NUV detector. Star-adjacent cold
pixels (> 0.3 dex below their eight-neighbour median,
two passes) are replaced by that median, reducing the
fraction of NUV point sources with an adjacent FUV
hole from 30.6 to 3.7 per cent.

(4)

$$
S_{\mathrm{NUV}}
$$

$$
S_{\mathrm{FUV}}
$$

$$
S_{\star}
$$

$$
(S_{NUV}
$$

$$
S_{\mathrm{NUV}}<0.05L)
$$

$$
S_{\mathrm{NUV}}>0.02L;
$$

$$
L.
$$

Stacks of isolated catalogue stars in the
layer alone are colour-flat (aperture/catalogue
0.86/0.86/0.85/0.91/1.10 at 15–17mag and
0.90/0.87/0.91/0.93/0.98 at 12–15mag for FUV−NUV
< *−0.5* / *−0.5*–0.5 / 0.5–1.5 / 1.5–3 / > 3, i.e. the
0.89 of the model PSF held by a 2<sup>′</sup> aperture), the
diffuse level of the layer against *GALEX* FUV (30th
percentile of N<sub>side</sub> = 256 cells in the overlap) is
−0.003 dex (−0.008/−0.007/+0.002 at *|b|* = 0–20/20–
40/40–90<sup>◦</sup>), and bright-star wings show no moats

$$
0.90/0.87/0.91/0.93/0.98\mathrm{at}12–15\mathrm{mag}\mathrm{for}\mathrm{FUV}\mathrm{-N U V}
$$

$$
<-0.5\quad-0.5-0.5\quad0.5-1.5\quad1.5-3\quad>3,
$$

$$
0.89
$$

$$
2'
$$

$$
N_{side}=256
$$

$$
(-0.008/-0.007/+0.002
$$

$$
|b|=0-20/20-
$$

$$
40/40-90^{\circ}
$$

---

((layer−S<sub>FUV</sub>)/background 1.55/1.35/1.06/1.04 at
0–2/5–6/8–10/10–14<sup>′</sup>, the residual being the imaged
*GALEX* halo beyond the model PSF wing). In
the published FUV map, stacks of isolated stars in
NUV-informed-class sky read 1.00 (4–8mag), 0.92
(8–12), 0.97 (12–15; blue/mid/red 0.91/0.97/1.02) and
1.10 (15–17; 1.20 at *|b|* < 10<sup>◦</sup> where faint imaged
neighbours fill the aperture, 1.10 above) of the
catalogue flux, against 0.92–0.97 for the model and
FIMS/SPEAR classes, which deposit the same PSF;
TD-1 stars in that sky read +0.006 dex against the
catalogue (8<sup>′</sup> apertures) and +0.04 dex against TD-1
itself, the adopted *Gaia*-based fluxes of these B/A stars
lying 0.027 dex above TD-1 in a direct star-by-star
comparison of the two flux scales. I<sub>X</sub> enters the fusion
as a survey-like layer with its own feather (*θ<sub>2</sub>* = 1.2<sup>◦</sup>)
and confidence cap *c<sub>2</sub>* = 0.9, below *GALEX* FUV and
above UVOT; it has non-zero weight over 19 per cent
of the sphere and is dominant over 9.7 per cent of
the FUV sky. Its status is dual: every structure was
recorded by the NUV detector, but its FUV intensity
is a prediction, and FUV/NUV colours or FUV-only
physics cannot be taken from it (Section 3.10). Inside
the Magellanic Clouds, where the layer carries the
capped stellar layer of Section 3.5, the median step
to adjacent *GALEX* FUV pixels is +0.036 (LMC)
and +0.031 dex (SMC); UVOT cannot arbitrate there
(coincidence loss), and the Clouds remain excluded
from quantitative use (Section 6).

$$
\left|b\right|<10^{\circ}
$$

$$
(8^{\prime}
$$

$$
I_{\mathrm{X}}
$$

$$
(\theta_{2}=1.2^{\circ})
$$

$$
c_{2}=0.9
$$

$$
UVOT;
$$

### 3.9 Fusion: gain harmonisation, the FIMS/SPEAR ratio field and priority-nested feathering

*Gain harmonisation and the model layer.* Even an unbiased regression has spatially correlated residuals at the
0.02–0.04 dex level on degree scales (Section 4.6), which
if uncorrected would produce steps where the prediction
meets the data. We therefore multiply the diffuse prediction by a smooth multi-scale gain field estimated from
the data/model ratio and extrapolated a short distance
into the gaps:

$$
\begin{aligned}\tilde{P}_{\mathrm{D}}=10^{\gamma}P_{\mathrm{D}},\qquad\gamma=\sum_{j=1}^{4}\zeta_{j}\gamma_{j},\qquad\\\gamma_{j}=\mathcal{S}_{\vartheta_{j}}\Bigl[\mathrm{med}_{c\in\mathrm{cells}_{j}}\bigl(\log I_{1}-\log P\bigr)\Bigr],\end{aligned}
$$

(5)

where the ratio is formed between the *GALEX* layer
*I<sub>1</sub>* and the complete model of the total intensity,
P = P<sub>D</sub> + S, and the hierarchy consists of medians
◦
in N<sub>side</sub> = 128 cells smoothed with ϑ<sub>1</sub> = 1.2, medians
◦ ◦
in N<sub>side</sub> = 64 cells smoothed with ϑ<sub>2</sub> = 4, a ϑ<sub>3</sub> = 15
smoothing, and the global median log ratio *γ<sub>4</sub>* (within 2
per cent of unity in both bands). Cell medians require
at least 30 (N<sub>side</sub> = 128) and 20 per cent (N<sub>side</sub> = 64)
valid *GALEX* children and are clipped to a factor of
∑
3. The coefficients ζ<sub>j</sub>(nˆ), ζ<sub>j</sub> = 1, select the finest
j
◦
level with adequate *GALEX* support, the 1.2 level rising from zero to unity between kernel-weighted valid

$$
I_{1}
$$

$$
P=P_{D}+S
$$

$$
\vartheta_{1}=1.2^{\circ}
$$

$$
N_{side}=128
$$

$$
N_{side}=64
$$

$$
\vartheta_{2}=4^{\circ},\mathrm{~a~}\vartheta_{3}=15^{\circ}
$$

$$
(N_{side}=128)
$$

$$
\gamma_{4}
$$

$$
(N_{side}=64)
$$

$$
\zeta_{j}(\hat{\mathbf{n}}),\sum_{j}\zeta_{j}=1
$$

fractions of 0.3 and 0.7, the 4<sup>◦</sup> level between 0.03 and
0.23. By construction *γ* has no structure below 1.2<sup>◦</sup>
and cannot imprint *GALEX* noise on the prediction.
Inside *enclosed* coverage holes (bright-star avoidance
zones and missing tiles; every non-*GALEX* component
smaller than 150deg<sup>2</sup>, 1316/1397 in FUV/NUV totalling
4400/4160deg<sup>2</sup>, i.e. everything except the Galactic-plane
gap) the harmonised diffuse prediction is additionally
anchored to the rim: the logarithm of the ratio of
the measured star-subtracted GALEX intensity to P˜<sub>D</sub>
is evaluated on 14<sup>′</sup> cells around each hole (artefactmasked cells excluded, 0.25<sup>◦</sup> smoothing), continued into
the hole as the harmonic (Laplace) interpolant of its
rim values—which carries the rim’s mean level and its
dipole (tilt) across the hole—plus, for the first degree
inward, a linear continuation of the radial gradient fitted to the rim cells within 1<sup>◦</sup> of the boundary (median
|d log C/dr| = 0.005 dex deg<sup>−1</sup> over the large holes, 0.03
at Spica in the FUV), and applied as a multiplicative
correction C to P˜<sub>D</sub> inside the hole and, tapering to
unity between 0.6<sup>◦</sup> and 1.2<sup>◦</sup>, on its surroundings, so
that the fill continues the measured brightness and its
trend across the coverage boundary in every direction
◦
rather than only on the 1.2 average of *γ.* The correction
has median *|C −* 1*|* = 0.015 and exceeds 8 per cent in
1–2 per cent of hole pixels; without it the extrapolation
of the predictor to the extreme stellar-density features
at the centres of bright-star holes leaves rim discontinuities of 5–15 per cent (e.g. around Spica and Achernar).
The mapped UVOT intensity is harmonised in the same
way against *GALEX* where available and against the
model layer elsewhere (global gains *≃* 1.7 FUV, *≃* 0.9
NUV). An island consistency test gives small isolated
*GALEX* regions (regional support below 20 per cent)
whose level disagrees with the harmonised model by
more than 0.10–0.25 dex a confidence multiplier falling
from unity to zero, preventing anomalous AIS tiles from
being propagated into the gaps.

$$
0.7,
$$

$$
4^{\circ}
$$

$$
\gamma
$$

$$
1.2^{\circ}
$$

$$
14'
$$

$$
\tilde{P}_{\mathrm{D}}
$$

$$
1^{\circ}
$$

$$
C/\mathrm{d}r|=0.005\mathrm{d}\mathrm{e}x\mathrm{d}\mathrm{e}\mathrm{g}^{-1}
$$

$$
\tilde{P}_{\mathrm{D}}
$$

$$
0.6^{\circ}
$$

$$
1.2^{\circ}
$$

$$
1.2^{\circ}
$$

$$
\gamma
$$

$$
|C-1|=0.015
$$

Because no template localises individual stars, a regression of the total intensity is smooth on arcminute
scales and recovers only 15–50 per cent of the flux of catalogued 11–15mag sources. The model layer separates
what the regression can and cannot predict:

$$
\mathcal{M}=10^{\gamma}P_{\mathrm{D}}+S_{\star}+E+L_{\mathrm{gal}},
$$

(6)

with P<sub>D</sub> the diffuse prediction and S<sub>⋆</sub>, E and L<sub>gal</sub> the
stellar, extragalactic point-source and galaxy/globularcluster layers of Section 3.5. *M* receives weight where
the data layers do not, so nothing is double counted inside the *GALEX,* NUV-informed and UVOT footprints;
it is published over the full sky as the gap-fill layer.

$$
S_{\star},E
$$

$$
P_{\mathrm{D}}
$$

$$
L_{\mathrm{gal}}
$$

*The FIMS/SPEAR-constrained layer.* In the FUV we
form the ratio of 0.97I<sub>FIMS,</sub>starless+S to M in N<sub>side</sub> = 32
(1.8<sup>◦</sup>) cells, excluding zones around the 76 stars brighter
than 150photonscm<sup>−2</sup> s<sup>−1</sup> Å<sup>−1</sup> whose FIMS/SPEAR
scattered-light haloes would raise it, smooth it above
4<sup>◦</sup>, clip it to [1/2,2], taper it, and multiply the *diffuse*

$$
0.97I_{FIMS,starless}+S
$$

$$
N_{side}=32
$$

$$
(1.8^{\circ})
$$

$$
^{-2}s^{-1}\text{Å }^{-1}
$$

$$
4^{\circ}
$$

$$
it,
$$ component of the model layer alone:

$$
\begin{aligned}&I_{\mathrm{F}}=C_{\mathrm{F}}10^{\gamma}P_{\mathrm{D}}+\big[\mathcal{M}-10^{\gamma}P_{\mathrm{D}}\big],\\&\quad C_{\mathrm{F}}=\mathrm{clip}\big(R,\ \frac{1}{2},\ 2\big)^{\eta_{\mathrm{F}}(\hat{\mathbf{n}})\:t(d)},\\&R=\mathcal{S}_{4^{\circ}}\bigg[\frac{\langle0.97I_{\mathrm{FIMS,starless}}+S\rangle_{32}}{\langle\mathcal{M}\rangle_{32}}\bigg],\\ \end{aligned}
$$

(7)

where S<sub>θ</sub> is mask-normalised Gaussian smoothing of
FWHM θ, η<sub>F</sub> ∈ [0,1] is a support taper, unity where
the smoothed ratio field is well supported and regional
*GALEX* support is weak, falling linearly to zero as the
support weight drops from 0.65 to 0.3 and as regional
*GALEX* coverage rises from 15 to 60 per cent, and
◦
*t(d*) is a distance taper, zero within 2 of the nearest
valid *GALEX* FUV pixel and rising linearly to unity at
◦
6. The distance taper reflects a measured property of
the constraint (Section 4.3): near *GALEX* the multiscale gain field already fixes the prediction level, and
the N<sub>side</sub> = 32 FIMS/SPEAR/prediction ratio there is
dominated by FIMS/SPEAR noise and residual stellar structure rather than prediction error, so applying
it increases the blind mock-gap error (68th percentile
0.067–0.077 versus 0.054–0.058 dex at 1<sup>◦</sup> –4<sup>◦</sup>); only be-
◦ ◦
yond *≃* 6 and at *|b|* < 10 does the ratio improve on
the harmonised prediction. With the taper the gapmatched blind mock-gap error of the FIMS/SPEARconstrained layer is 0.068 dex instead of 0.081 dex and
is nowhere worse. Before clipping, the N<sub>side</sub> = 32
log-ratio has 5–95 per cent range −0.14 to +0.19 dex;
1.4 per cent of supported cells sit at the upper clip
and 0.55 per cent at the lower, the W<sub>FIMS</sub>-weighted
upper-clipped cells forming three compact groups in
Vela/Gum, Carina–Crux and Upper Sco where residual FIMS/SPEAR stellar haloes survive the star mask;
without the clip the map there would be up to 0.16 dex
brighter, elsewhere unchanged (|∆| = 0.003 dex). I<sub>F</sub>
thus carries the measured diffuse FUV level above 4<sup>◦</sup> far
from *GALEX,* the template-predicted morphology below that scale, and catalogue sources at their catalogue
fluxes. In the *GALEX*-free plane the published map
reproduces the constraint quantity to a median of 0.99
with 0.07 dex scatter and r = 0.95 at 13.7<sup>′</sup>; its ratio to
the FIMS/SPEAR *starry* map is resolution dependent
(Section 4.4).

$$
\mathcal{S}_{\theta}
$$

$$
\theta,\eta_{\mathrm{F}}\in[0,1]
$$

$$
2^{\circ}
$$

$$
6^{\circ}
$$

$$
\hat{N}_{side}=32FIMS/SPEAR/
$$

$$
0.067--0.077
$$

$$
1^{\circ}-4^{\circ})
$$

$$
\simeq6^{\circ}
$$

$$
\left|b\right|<10^{\circ}
$$

$$
N_{side}=32
$$

$$
\widetilde{W_{FIMS}}-weight
$$

$$
\left(\left|\Delta\right|=0.003\mathrm{dex}\right)
$$

$$
I_{\mathrm{F}}
$$

$$
4^{\circ}
$$

*Priority-nested feathering.* The layers are combined
pixel by pixel as a convex combination; in decreasing
priority they are *k* = 1, *GALEX* (*I<sub>1</sub>,* i.e. *g* with the
TD-1 top-up); *k* = 2, the NUV-informed FUV layer
I<sub>X</sub> (FUV only); k = 3, the harmonised mapped UVOT
intensity; *k* = 4, the FIMS/SPEAR-constrained model
layer I<sub>F</sub> (FUV only); and finally the model layer M.
The fused intensity is

$$
(I_{1},
$$

$$
I_{\mathrm{X}}
$$

$$
I_{\mathrm{F}}
$$

$$
\begin{aligned}{I(\hat{\mathbf{n}})\;=\;}&{{}\sum_{k=1}^{4}w_{k}(\hat{\mathbf{n}})\thinspace I_{k}(\hat{\mathbf{n}})\;+\;w_{\operatorname{M}}(\hat{\mathbf{n}})\thinspace\mathcal{M}(\hat{\mathbf{n}}),}\\ {}&{{}\qquad w_{\operatorname{M}}=1-\sum_{k=1}^{4}w_{k}\;\geq\;0,}\\ \end{aligned}
$$

(8)

with priority-nested weights

$$
\begin{aligned}{}&{{}\quad w_{k}(\hat{\mathbf{n}})\;=\;c_{k}(\hat{\mathbf{n}})f_{k}(\hat{\mathbf{n}})\left(1-\sum_{j<k}w_{j}(\hat{\mathbf{n}})\right)_{+},}\\ {}&{{}\quad f_{k}\;=\;m_{k}\;\rho\bigg(\frac{\mathcal{S}_{\theta_{k}}[m_{k}]-\rho_{0}}{\rho_{1}-\rho_{0}}\bigg),\quad\rho(x)=\operatorname{clip}(x,0,1),}\\ {}&{{}\quad\omega}\\ \end{aligned}
$$

(9)

where m<sub>k</sub> ∈{0,1} is the validity mask of layer k and
S<sub>θ</sub><sub>k</sub>[m<sub>k</sub>] its coverage fraction smoothed with FWHM
θ<sub>k</sub>, so that f<sub>k</sub> vanishes where the local smoothed coverage is below *ρ<sub>0</sub>* and reaches unity above *ρ<sub>1</sub>,* and
c<sub>k</sub> ≤ 1 is a confidence factor. For GALEX we adopt
◦
*θ<sub>1</sub>* = 0.3 with (*ρ ,ρ<sub>1</sub>*<sub>0</sub>) = (0.45,0.95); for the NUV-
◦
informed layer, UVOT and FIMS/SPEAR θ<sub>k</sub> = 1.2 ,
◦ ◦
0.7 and 2.5 with (*ρ ,ρ<sub>1</sub>*<sub>0</sub>) = (0.5,0.95). The confidence factors are c<sub>1</sub> = M<sub>A</sub>, the artefact multiplier
of Section 3.4, times the island multiplier; *c<sub>2</sub>* = 0.9
times the NUV artefact multiplier, since I<sub>X</sub> inherits
the NUV artefacts; *c<sub>3</sub>* = 0.8; and *c<sub>4</sub>* a factor that is
unity where the regional *GALEX* coverage is below 20
per cent and falls to zero where it is complete. The
factors *c<sub>2</sub>* and *c<sub>3</sub>* encode the larger calibration-transfer
uncertainty of the colour-scaled NUV image and of
UVOT relative to *GALEX;* the source layer enters only
through I<sub>X</sub>, I<sub>F</sub> and M, because stars and galaxies inside the *GALEX* and UVOT footprints are already
∑
present there. The factor (1 −j<kw<sub>j</sub>)<sub>+</sub> nests the
weights by priority, so they sum to unity by construction
and a fully feathered-in *GALEX* pixel (*f<sub>1</sub>* = *c<sub>1</sub>* = 1)
has *w<sub>1</sub>* = 1 exactly; maxWGALEX= 1 in the published files. The *GALEX* feather scale *θ<sub>1</sub>* = 0.3<sup>◦</sup> balances fidelity to *GALEX* against seam roughness: the
90th percentile of *|*log<sub>10</sub>(map/g)*|* over the footprint is
0.0024 dex (FUV) and 87 per cent of imaged pixels have
WGALEX> 0.99. In the FUV *GALEX* dominates 61.9
per cent of the sky, the NUV-informed layer 9.7, UVOT
0.2, the FIMS/SPEAR-constrained model 23.7 and the
model 4.5 per cent; in the NUV *GALEX* dominates
73.3, UVOT 0.2 and the model 26.5 per cent. Pixels
with WGALEX+ W<sub>NUVX</sub> + W<sub>UVOT</sub> ≥ 0.5 cover 72.0/73.5
per cent of the sky.

$$
m_{k}\in\{0,1\}
$$

$$
\mathcal{S}_{\theta_{k}}[m_{k}]
$$

$$
\rho_{0}
$$

$$
f_{k}
$$

$$
\theta_{k},
$$

$$
\rho_{1}
$$

$$
c_{k}\leq1
$$

$$
\mathit{GALEX}
$$

$$
\theta_{1}\;=\;0.3^{\circ}
$$

$$
\left(\rho_{0},\rho_{1}\right)=\left(0.45,0.95\right)
$$

$$
2.5^{\circ}
$$

$$
0.7^{\circ}
$$

$$
\theta_{k}=1.2^{\circ}
$$

$$
\left(\rho_{0},\rho_{1}\right)=\left(0.5,0.95\right)
$$

$$
c_{1}=M_{A}
$$

$$
c_{2}\;=\;0.9
$$

$$
c_{3}=0.8;
$$

$$
I_{\mathrm{X}}
$$

$$
\mathit{GALEX}
$$

$$
c_{4}
$$

$$
c_{2}
$$

$$
c_{3}
$$

$$
I_{\mathrm{X}}
$$

$$
(1-\sum_{j<k}w_{j})_{-}
$$

$$
(f_{1}=c_{1}=1)
$$

$$
W_{GALEX}\;=\;1
$$

$$
w_{1}=1
$$

$$
\theta_{1}=0.3^{\circ}
$$

$$
\mathit{GALEX}
$$

$$
|\log_{10}(map/g)
$$

$$
W_{GALEX}>0.99
$$

$$
W_{GALEX}+W_{NUVX}+W_{UVOT}\geq0.5
$$

### 3.10 Post-fusion edits, products and pixel classes

*Post-fusion edits.* Four local edits are applied to the
fused maps. (i) Ultraviolet-bright stars that lack a valid
TD-1 flux and are censored in the imaging receive the
Bright Star Catalogue model deficit as a 2<sup>′</sup> Gaussian
(BSC stars earlier than F5; 306 FUV, 368 NUV stars),
and 56 (FUV) and 11 (NUV) very bright cool stars plus
Canopus receive an additive completion patch; all are deposited with weight 1− W<sub>FIMS</sub> − WMODEL− W<sub>NUVX</sub>, i.e.
only where the source layer does not already carry them.
(ii) Cold pixels (> 0.3 dex below their eight-neighbour
median, two passes) are replaced by that median in
FIMS/SPEAR-dominated FUV sky and in a handful of
*GALEX* and NUV pixels. (iii) One *GALEX* NUV edge-
2
reflection glint of ν And (0.4deg , peak 10<sup>5</sup> CU), which
the NUV-informed layer would copy into the FUV, is re-

$$
1-W_{\mathrm{FIMS}}-W_{\mathrm{MODEL}}-W_{\mathrm{NUVX}}
$$

$$
\left(\dot{0.4}\deg^2\right.
10^{5}CU)
$$ moved from both bands by harmonic inpainting of logI
from the boundary of a grown mask (915 pixels) with
compact sources protected and the original high-pass
texture retained; the boundary step after repair is 0.000
(NUV) and −0.010 dex (FUV). (iv) Inside the LMC (8<sup>◦</sup>)
◦
and SMC (5) discs, pixels with WMODEL+W<sub>FIMS</sub> > 0.5
are tied to the capped reference level of Section 3.5
through a smooth boundary-matching factor field, so
that un-imaged holes in the Clouds do not inherit the
over-predicted member starlight.

$$
(5^{\circ})
$$

$$
W_{MODEL}+W_{FIMS}>0.5
$$

*Products.* The published files (Section 7, Table 7)
are the two maps; the five weight columns W<sub>k</sub> ≡ w<sub>k</sub>
per band; an integer provenance code (the dominant
layer, Fig. 8); a ‘data-only’ variant in which pixels
with WGALEX+ W<sub>NUVX</sub> + W<sub>UVOT</sub> < 0.5 are blanked;
N<sub>side</sub> = 512 means of the sixteen children; the gapfill layer M (and I<sub>F</sub>) over the full sky; every source
layer separately, with the stellar, extragalactic, diffuse
and unresolved-remainder partition of Section 3.5 and
its per-source catalogue; and two-column uncertainty
maps. The per-pixel uncertainty of log<sub>10</sub> *I* (column
SIGMA, equation 10) is the weight-quadrature sum of
per-class terms—*GALEX* pixel noise, the colour-model
scatter for I<sub>X</sub>, an intensity-dependent UVOT term, an
empirical model term from a quantile regression of the
absolute out-of-fold residuals of Section 3.7 on the six
templates marked in Table 3 and distance to data, a
FIMS/SPEAR term from mock gaps, and the stellarlayer *σ*<sub>stars</sub>—with a separate systematic column SIGMA_-
SYS; its construction and calibration are given in Section 4.6.

$$
W_{k}\equiv w_{k}
$$

$$
W_{\operatorname{GALEX}}+W_{\operatorname{NUVX}}+W_{\operatorname{UVOT}}<0.5
$$

$$
N_{side}=512
$$

$$
\bar{I}_{\mathrm{F}})
$$

$$
\log_{10}I
$$

$$
I_{\mathrm{X}},
$$

$$
\sigma_{stars}—wi
$$

$$
\mathrm{SIGMA_{-}-}
$$

*Pixel classes: what the map is and is not.* The maps
combine direct imaging with template-based prediction
constrained by coarser data where imaging is absent,
fused seamlessly so that a pixel’s supported science
depends on its class rather than appearance. Table 4
gives the taxonomy: direct *GALEX* imaging sets 61.9
per cent of the FUV and 73.3 per cent of the NUV sky,
while predicted sub-degree morphology covers 28.0 per
cent (FUV) and 26.5 per cent (NUV).

Two of these classes need a sharper statement. First,
the NUV-informed FUV layer is imaging-derived—every
diffuse structure in it was recorded by the *GALEX* NUV
detector—but its FUV *intensity* is the star-subtracted
NUV intensity multiplied by a learned colour plus catalogue stars, so it is a prediction conditioned on an image.
Whether it counts as ‘data’ depends on the question: for
position and shape it does; for the FUV/NUV ratio, H<sub>2</sub>
fluorescence, or any FUV-specific quantity it does not, by
construction. We recommend a strictly data-only FUV
analysis select WGALEX+ W<sub>UVOT</sub> ≥ 0.5 (≃ 62 per cent
of the sky) rather than WGALEX+ W<sub>NUVX</sub> + W<sub>UVOT</sub> ≥
0.5 (72.0 per cent) used for the published ‘data-only’ file;
the header (SELECT) and Table 7 state it includes 9.7
per cent of sky whose FUV level and colour are transferred from the NUV. Second, in the FIMS/SPEARconstrained class the constraint is coarse: FIMS/SPEAR
fixes the mean level over ≳ 4<sup>◦</sup> (coherence falls below
0.5 at scales < 137<sup>′</sup>), while below a degree is template
prediction plus catalogue stars. Over the full 28 per cent

$$
W_{GALEX}+W_{UVOT}\geq0.5
$$

$$
W_{GALEX}+W_{NUVX}+W_{UVOT}
$$

$$
\gtrsim4^{\circ}
$$

$$
<137')
$$

of the FUV sky in these classes, sub-degree morphology
is predicted, not observed.

The practical rules by science case are as follows
(Appendix D). Photometry and morphology of diffuse
structure, structure functions and small-scale power:
*W*<sub>GALEX</sub>> 0.99 only. Cross-correlation with extragalactic tracers, EBL and offset studies: *W*<sub>GALEX</sub>> 0.99,
and remember the monopole is inherited (Section 5).
FUV/NUV colours and FUV-only physics: both bands
*GALEX*-dominated. Degree-scale level of the FUV
sky in the plane and in the *GALEX* gaps: all classes,
with the published uncertainty (0.05–0.10 dex per 7<sup>′</sup> in
WMODELand W<sub>FIMS</sub> sky from column SIGMA, plus the
0.10 dex coherent plane systematic of column SIGMA_-
SYS; Section 4.6). Bright-source avoidance: all classes,
noting that outside *GALEX* the stellar content is the
*Gaia*-based layer of Section 3.5, complete in flux to
*G ≃* 17 but with predicted rather than measured ultraviolet fluxes for most stars (0.16–0.4mag per star).
What no filled pixel can provide is new small-scale interstellar physics: a filament, shell or cloud edge seen
only in W<sub>FIMS</sub>/WMODELsky is templates re-projected
through the regression, with contrast biased low by the
conditional-mean nature of the predictor (Section 4.7).

$$
W_{GALEX}>0.99
$$

$$
W_{GALEX}>0.99
$$

$$
W_{MODEL}
$$

$$
7'
$$

$$
W_{\mathrm{FIMS}}/W_{\mathrm{MODEL}}
$$

## 4 Validation of the published maps

This section establishes, on the published files, (i) how
faithfully the fused map reproduces *GALEX* inside the
footprint; (ii) how well the diffuse predictor reproduces
*GALEX* where withheld, weighted as the sky actually
filled; (iii) how the complete fusion recovers mock gaps;
(iv) how the maps compare with independent instruments in sky *GALEX* never observed; (v) how discrete
sources behave by pixel class; (vi) whether the published uncertainty is calibrated; and (vii) what fraction
of the angular power the filled sky retains. We reserve
*validation* for tests against withheld *GALEX,* independent instruments (FIMS/SPEAR, UVOT, TD-1) and
catalogues; comparisons with dust, *Hα* and H i are *consistency checks* (Section 5, Appendix C). UVOT and
FIMS/SPEAR were never training features but were consulted during design. Every benchmark scatter quoted
for a single model carries a spatial block-bootstrap uncertainty of ±0.002 dex (FUV) and ±0.001 dex (NUV)
(Appendix B). Table 11 in Appendix B lists the source
of every headline number.

### 4.1 Fidelity to <em>GALEX</em> and seams

4.1 Fidelity to GALEX and seams

Where *W*<sub>GALEX</sub>*≥* 0.99 the map equals the destriped,
tile-equalised *GALEX* layer *g* identically. Over all pixels
with valid *GALEX* data, *|*log<sub>10</sub>(map/g)*|* has a 90th
percentile of 0.0023 dex (FUV) and 0.0004 dex (NUV)
and a 99th percentile of 0.064 and 0.073 dex. *W*<sub>GALEX</sub>>
0.99 for 87.0/89.3 per cent of valid pixels and *W*<sub>GALEX</sub><
0.5 for 4.1/3.4 per cent. In *W*<sub>GALEX</sub>> 0.9 sky the fusion
alters the *GALEX* angular power spectrum by at most

$$
W_{GALEX}\geq0.99
$$

$$
\log_{10}(map/g)
$$

$$
W_{GALEX}>
$$

$$
W_{GALEX}<
$$

$$
W_{\operatorname{GALEX}}>0.9
$$

---

0.02 dex 0.16

0.02 dex 0.16

Figure 8: Provenance and uncertainty of the published maps (Nside= 256 rendering of the published weight and uncertainty
files). (a) FUV dominant layer: *GALEX* (blue), NUV-informed FUV (light blue), UVOT (red), FIMS/SPEAR-constrained model
(yellow), model (grey). (b) NUV dominant layer, same colours. (c, d) The published *1σ* (68 per cent half-width) uncertainty of log<sub>10</sub>*I,*
column SIGMA of equation (10), FUV and NUV, on a common 0.02–0.16 dex scale; the systematic column SIGMA_SYS (0.10 dex in the
FIMS/SPEAR-constrained plane) is not shown.

$$
\left(N_{side}=256\right.
\log_{10}I,
$$

Table 4: Pixel classes of the published maps (dominant weight), their sky fractions, what is measured and what is predicted in each,
and the uses each supports. ‘Level’ means the ≳ 1<sup>◦</sup>mean intensity; ‘morphology’ means structure below that scale.

$$
\gtrsim1^{\circ}
$$

| Class (dominant weight) | FUV (%) | NUV (%) | Content | Supports / does not support |
| --- | --- | --- | --- | --- |
| GALEX imaging (WGALEX) | 61.9 | 73.3 | Destriped GR6/7 mosaic; level tied to UV-BKGD per 14' cell; native \(\simeq 2.5\)–3' structure and all sources | All uses: photometry of diffuse structure (WGALEX > 0.99), power spectra and cross-correlations, stacking, point sources. Not: an independent monopole (inherited zero point). |
| NUV-informed FUV (WNUVX) | 9.7 | — | star-subtracted GALEX NUV image × predicted diffuse FUV/NUV colour below 6.9', plus catalogue FUV stars (equation 4); level from the colour model | NUV-traced diffuse morphology at full resolution, source positions, large-scale FUV level to ~ 0.1 dex, stellar fluxes at the catalogue (predicted) level as in model sky. Not: FUV/NUV colours, FUV-only physics (H2, C IV, two-photon), measured FUV photometry of individual sources. |
| Swift-UVOT (W_{UVOT}) | 0.2 | 0.2 | UVOT UVM2/UVW2 imaging transformed to the GALEX bands (0.11–0.25 dex) | Level and morphology in small deep fields, mostly in the plane. Not: precise colours (filter transformation, red leak). |
| FIMS/SPEAR-constrained prediction (WFIMS) | 23.7 | — | Template prediction + catalogue stars, rescaled to the FIMS/SPEAR starless map on > 4° scales (\(\simeq 1^{°}\) data, 0.15–0.20 dex absolute) | Degree-scale FUV level in the GALEX-free sky including the plane; mission background planning; boundary conditions for RT models at \(\gtrsim 1^{°}\). Not: sub-degree ISM morphology, filament contrasts, small-scale power, new point sources. |
| Pure prediction (WMODEL) | 4.5 | 26.5 | Gradient-boosted diffuse prediction from dust, Hα, starlight and position templates + Gaia/TD-1/galaxy source layers | Expected background level (0.05–0.10 dex per 7') for planning and as a prior; continuity of all-sky statistics. Not: any measurement of ISM structure independent of the templates; EBL or offset studies; discovery of sources. |

$$
14'
$$

$$
\simeq2.5–3'
$$

$$
\left(W_{GALEX}>0.99\right)
$$

$$
>4^{\circ}
$$

$$
(\simeq1^{\circ}
$$

$$
\gtrsim1^{\circ}
$$

$$
7^{\prime})
$$

$$
Gaia/TD-1_{\triangle}
$$

---

1 per cent at any *ℓ ≤* 1536 (Section 4.7).

$$
\ell\leq1536
$$

Seams are measured with the coverage-boundary stack
of Appendix B: the median of log(map/S<sub>2</sub>◦ [map]) in
integer-pixel rings from the *GALEX* edge, the step being the mean over rings +1 to +3 minus rings *−1* to
−3. The sky-averaged step is +0.0031 dex (FUV) and
+0.0033 dex (NUV) at N<sub>side</sub> = 2048; across implementations differing in boundary-pixel selection it stays below
0.005 dex (FUV) and below 0.015 dex (NUV), while its
statistical error is ±0.0005 dex (block bootstrap), so
the range is definitional, not statistical. Locally, in
◦
3.7 boundary segments, the step has an rms of 0.022
(FUV) and 0.026 dex (NUV) against 0.012/0.013 dex
for the same estimator on random great circles inside
the footprint, i.e. an intrinsic local seam amplitude of
*≃* 0.02 dex rms, with 3–4 per cent of segments beyond
0.05 dex. A matched filter for coherent steps across internal HEALPix cell boundaries (N<sub>side</sub> = 8–256) finds no
excess over control lines in any pixel class (e.g. 0.0074
versus 0.0073 dex at N<sub>side</sub> = 32 in FUV model sky),
and a small-circle ring test finds no great-circle stripe
(Appendix C).

$$
\log(map/\mathcal{S}_{2^{\circ}}[map])
$$

$$
-3.
$$

$$
N_{side}=2048;
$$

$$
3.7^{\circ}
$$

$$
\simeq0.02
$$

$$
\left(N_{side}=8-256\right)
$$

$$
N_{side}=32
$$

### 4.2 Gap-fill accuracy: benchmark, dis- tance to data and covariate shift

*Headline.* Scored on the published benchmark (six-fold
◦
7.3-block spatial cross-validation at N<sub>side</sub> = 512; Appendix A) with importance weights matching held-out
pixels to the *(E(B−V), |b|)* distribution of the filled sky,
the out-of-fold diffuse prediction reproduces the withheld star-subtracted *GALEX* intensity with a robust
scatter of 0.037/0.047 dex (FUV, area-weighted/gapmatched) and 0.047/0.057 dex (NUV) per 6.9<sup>′</sup> element,
with 11/18 per cent of matched pixels beyond 0.1 dex;
prediction plus source layer scored against the *GALEX*
total gives 0.057/0.060 dex (matched, FUV/NUV). Reweighted to the (*E(B −V), |b|*) distribution of the sky
actually filled by each layer the scatter is 0.050–0.052
(FUV) and 0.061 dex (NUV). By column density it is
0.035–0.045 dex for *E(B−V*) < 0.2, 0.049/0.059 at
0.2–0.4, 0.065/0.074 at 0.4–0.8 and 0.095/0.088 dex at
0.8–1.6mag (FUV/NUV).

$$
7.3^{\circ}-block
$$

$$
N_{side}=512;
$$

$$
(E(B{-}V),|b|)
$$

$$
E(B-V)<0.2,\ 0.049<0.059
$$

*Baselines and the un-jittered variant.* Table 5 and
Fig. 9 compare the adopted predictor with featureless
baselines on the same folds, scored on the total intensity
against the destriped layer G<sup>′</sup> before tile equalisation;
on that truth the adopted predictor has a robust scatter
of 0.060± 0.002 dex (FUV) and 0.060± 0.001 dex (NUV)
(block bootstrap; fold-to-fold range 0.056–0.061 and
0.054–0.062), with 16 per cent of pixels beyond 0.1 dex
and a bias of +0.009/+0.010 dex. Tile equalisation of
the withheld *GALEX* lowers the diffuse-target scatter
by 0.004–0.007 dex, all but *≤* 0.001 dex of which is the removal of tile-offset noise from the truth itself, and leaves
the total-intensity scores unchanged within the bootstrap error. Featureless interpolation is competitive only
near data: the nearest *GALEX* pixel scores 0.042 dex
within 0.5<sup>◦</sup> but 0.17 dex beyond 3<sup>◦</sup>, and harmonic inpainting of logG<sup>′</sup> scores 0.064/0.085 dex overall. The

$$
G'
$$

$$
0.060\pm0.001
$$

$$
0.060\pm0.002
$$

$$
+0.009+0.010\mathrm{d}x
$$

$$
\leq0.001
$$

$$
0.5^{\circ}
$$

$$
3^{\circ}
$$

$$
G'
$$

same model without coordinate jitter scores marginally
*better* in the NUV (−0.0017 *±* 0.0002 dex); we publish
the jittered model nonetheless, for *b* = 0 continuity and
reproducibility, and state its NUV cost, 0.0017 dex in
0.060, here and in Section 6.

$$
\mathrm{NUV}\left(-0.0017\pm0.0002\mathrm{dex}\right)
$$

$$
b=0
$$

*Distance to data.* Spatial blocking guarantees a minimum separation, not one representative of the real gaps.
Held-out pixels lie at a median of 1.27<sup>◦</sup> from training
data, whereas FUV gap pixels lie at a median of 1.1<sup>◦</sup>
from *GALEX* but with a long tail: 25.5 per cent of the
filled FUV sky is beyond 3.65<sup>◦</sup>, 17.9 per cent beyond
◦ ◦
5 and 5 per cent beyond 8; the NUV gaps are benign
(median 0.50<sup>◦</sup>). Because smoothed features carry information across block edges, accuracy degrades with
distance (Fig. 9c,d): the matched scatter of the adopted
predictor is 0.048± 0.001 dex within 0.25<sup>◦</sup>, 0.053± 0.002
at 0.5<sup>◦</sup> –1<sup>◦</sup>, 0.063 *±* 0.003 at 1.5<sup>◦</sup> –2<sup>◦</sup>, 0.070 *±* 0.005 at
◦ ◦ ◦ ◦
3 –4 and 0.08 *±* 0.02 dex at 5 –8 (FUV), and 0.054,
0.070 and 0.08 *±* 0.02 dex in the NUV, i.e. slopes of
◦
+0.0066±0.0008 and +0.0048±0.0004 dex deg<sup>−1</sup> over 0 –
◦ ◦
5; beyond 5 the benchmark has no statistical power.
Two direct measurements reach farther: blind mock
gaps through the published FUV fusion out to 8.6<sup>◦</sup> (Section 4.3) give a 68th-percentile error of 0.046–0.057 dex
within 3<sup>◦</sup>, 0.068 at 3<sup>◦</sup> –5<sup>◦</sup> and 0.093 dex at 6<sup>◦</sup> –9<sup>◦</sup>, and a
29.3<sup>◦</sup>-block cross-validation re-weighted to the far FUV
gap gives 0.11 dex beyond 3.65<sup>◦</sup> and 0.16 dex beyond
◦
8. We therefore state the accuracy of the gap fill as
0.060 *±* 0.002 dex for the 97 per cent of the filled NUV
sky within 3.65<sup>◦</sup> of data and for the three quarters of the
◦
filled FUV sky within *≃* 3.7 of *GALEX;* 0.07–0.10 dex
for the quarter beyond; and 0.10–0.16 dex, poorly determined, for the innermost 5 per cent beyond 8<sup>◦</sup>, where
the map is checked against FIMS/SPEAR and UVOT
at their own 0.1–0.2 dex precision (Section 4.4). The
published uncertainty carries this distance dependence
explicitly (Section 4.6).

$$
1.27^{\circ}
$$

$$
1.1^{\circ}
$$

$$
3.65^{\circ}
$$

$$
5^{\circ}
$$

$$
8^{\circ};
$$

$$
0.048\pm0.001
$$

$$
0.5^{\circ}-1^{\circ},0.063\pm0.003
$$

$$
0.25^{\circ},0.053\pm0.002
$$

$$
1.5^{\circ}-2^{\circ}
$$

$$
0.08\pm0.02c
$$

$$
0.070\pm0.005
$$

$$
3^{\circ}-4^{\circ}
$$

$$
5^{\circ}-8^{\circ}
$$

$$
0.070
$$

$$
0.08\pm0.02
$$

$$
+0.0066{\pm}0.0
$$

$$
+0.0048{\pm}0.0004\operatorname{dex}\deg^{-1}
$$

$$
0^{\circ}-
$$

$$
5^{\circ};
$$

$$
5^{\circ}
$$

$$
8.6^{\circ}
$$

$$
3^{\circ}
$$

$$
3^{\circ}-5^{\circ}
$$

$$
6^{\circ}-9^{\circ}
$$

$$
29.3^{\circ}-b]
$$

$$
3.65^{\circ}
$$

$$
8^{\circ}.
$$

$$
97
$$

$$
0.060\pm0.002
$$

$$
3.65^{\circ}
$$

$$
\simeq3.7^{\circ}
$$

$$
8^{\circ}
$$

*Column-density regime and the limits of matching.*
Above *E(B−V*) = 1.6 only 110 held-out FUV pixels exist,
and of the filled FUV sky (WMODEL+ W<sub>FIMS</sub> > 0.5) 8.2
per cent lies at *E(B−V*) > 1.6 with no held-out *GALEX*
FUV analogue, where histogram trees return edge-bin
values set by FIMS/SPEAR. The importance weighting
is poorly conditioned in the FUV: its Kish effective
sample size is 4.5 × 10<sup>4</sup> of 2.0 × 10<sup>6</sup> pixels (2.2 per cent;
NUV 46 per cent), and the top 1 per cent of held-out
pixels carry 23 per cent of the weight. Matching on
(*E(B −V), |b|*) does not make held-out and filled pixels
exchangeable: a classifier two-sample test on the eight
non-positional templates separates matched held-out
from *W*<sub>MODEL</sub>> 0.5 pixels with AUC = 0.83 (FUV)
and 0.66 (NUV), driven by Hα, *Gaia* starlight and
source density, higher in the filled sky. Re-weighting on
the classifier score raises the matched scatter from 0.060
to 0.064–0.067 dex in the NUV and to 0.07–0.10 dex
for the 4.5 per cent model-filled FUV sky. We adopt
0.065 dex as the shift-corrected NUV figure.

$$
E(B-V)=1.6
$$

$$
\left(W_{MODEL}+W_{FIMS}>0.5\right)
$$

$$
E(B-V)>1.6
$$

$$
4.5\times10^{4}
$$

$$
2.0\times10^{6}
$$

$$
W_{MODEL}>0.5
$$

---

Table 5: Gap-fill benchmark (Appendix A): out-of-fold performance of the adopted predictor, its un-jittered variant and featureless
baselines in the six-fold 7.3<sup>◦</sup>-block cross-validation at *N* = 512, total-intensity target scored against the destriped *GALEX* layer *G*<sup>′</sup>
side
(before tile equalisation), with importance weights matching held-out pixels to the *(E(B−V), |b|)* distribution of the filled sky (‘matched’)
and area-weighted. *σ:* robust scatter (1.4826× weighted MAD) of log (pred/G<sup>′</sup>); *f* : fraction beyond *±0.1* dex. Block-bootstrap *1σ*
10 0.1
errors on single-model matched *σ* are 0.002–0.004 (FUV) and 0.001 (NUV). The last columns give the matched *σ* by distance to the
nearest training pixel (the > 3<sup>◦</sup>bin holds 1–2 per cent of the weight).

$$
N_{side}=512
$$

$$
G'
$$

$$
\sigma\mathrm{:}
$$

$$
\widetilde{\log}_{10}(\operatorname{pred}/G^{\prime})\widetilde{}
$$

$$
>3^{\circ}
$$

| Model | matched σ FUV | matched σ NUV | matched f0.1 FUV | matched f0.1 NUV | area σ FUV/NUV | FUV/NUV < 0.5° | matched σ by distance 1°–2° | matched σ by distance > 3° |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Featureless baselines |  |  |  |  |  |  |  |  |
| Planck-radiance power law per \|b\| band | 0.160 | 0.167 | 0.56 | 0.57 | 0.103/0.117 | 0.153/- | - | 0.141/- |
| Nearest observed GALEX pixel | 0.083 | 0.117 | 0.29 | 0.42 | 0.072/0.106 | 0.042/- | - | 0.169/- |
| Harmonic inpainting of \(\log G'\) (Nside=128) | 0.064 | 0.085 | 0.20 | 0.34 | 0.059/0.075 | 0.048/- | - | 0.112/- |
| Learned models |  |  |  |  |  |  |  |  |
| 90 features, HGB, no jitter | 0.059 | 0.058 | 0.17 | 0.15 | 0.048/0.052 | 0.047/0.051 | 0.060/0.058 | 0.069/0.070 |
| Adopted predictor (90 features, jitter, test-time averaging) | 0.060 | 0.060 | 0.16 | 0.16 | 0.049/0.053 | 0.050/0.054 | 0.061/0.060 | 0.071/0.071 |

$$
f_{0.1}
$$

$$
<0.5^{\circ}
$$

$$
1^{\circ}-2^{\circ}
$$

$$
>3^{\circ}
$$

$$
G'(N_{side}=128)
$$

$$
0.071/0.071
$$

| Category | gap-matched robust scatter (dex), 6-fold 7.3° spatial CV | Δ = paired difference |
| --- | --- | --- |
| A jittered (released) | 0.0599 | 0.0000±0.0000 |
| A unjittered | 0.0585 | 0.0014±0.0019 |
| champion 12-feat | 0.0684 | 0.0086±0.0018 |
| (i) kNN analogue | 0.0729 | 0.0131±0.0023 |
| (iii) two-scale HGB+kriged LS | 0.0653 | 0.0055±0.0020 |
| (ii) HGB quantile+monotone | 0.0671 | 0.0072±0.0025 |
| (iv) stack A+LS+kNN | 0.0594 | -0.0004±0.0006 |
| (iv) ens A/Ajit/kNN | 0.0585 | -0.0013±0.0015 |
| (iv) ens A/Ajit/E1/kNN | 0.0584 | -0.0015±0.0015 |

| Category | gap-matched robust scatter (dex), 6-fold 7.3° spatial CV |
| --- | --- |
| A jittered (released) | 0.0597 |
| A unjittered | 0.0579 |
| champion 12-feat | 0.0633 |
| (i) kNN analogue | 0.0675 |
| (iii) two-scale HGB+kriged LS | 0.0609 |
| (ii) HGB quantile+monotone | 0.0626 |
| (iv) stack A+LS+kNN | 0.0589 |
| (iv) ens A/Ajit/kNN | 0.0578 |
| (iv) ens A/Ajit/E1/kNN | 0.0572 |

Figure 9: Gap-fill benchmark scores of the adopted predictor, its un-jittered variant and alternative learners tested on the same
benchmark (Appendix A). (a) Gap-matched robust scatter with *N*<sub>side</sub>= 8 block-bootstrap errors, FUV and NUV. (b) Matched scatter
by *E(B −V*) regime. (c, d) Matched scatter versus distance to the nearest training pixel, FUV and NUV; all learned models share the
same rise with distance.

$$
E(B-V)
$$

$$
N_{side}=8
$$

$$
(c,d)
$$

$$
NUV;
$$

### 4.3 End-to-end test: mock gaps through the production fusion

The benchmark tests the regression; the published
map is the output of the whole fusion. We hid forty
N<sub>side</sub> = 8 super-pixels (5.2 per cent of the sky) from the
*GALEX* input and re-ran the production fusion code, reestimating every gain field, the FIMS/SPEAR ratio field
and the island penalty without the hidden data. Inside
the hidden blocks the diffuse prediction was replaced
by the adopted predictor’s spatial-block out-of-fold prediction, so neither the fusion nor the regression had
seen the hidden pixels. Against the destriped *GALEX*
truth (Fig. 10) the reconstruction has, at N<sub>side</sub> = 512, a
robust scatter of 0.049 dex (FUV) and 0.047 dex (NUV),
medians of −0.009 and +0.007 dex, and 8.3/10.6 per
cent of cells beyond 0.1 dex (against the tile-equalised
truth g, 0.046/0.043 dex); at N<sub>side</sub> = 2048 the scatter
is 0.059/0.062 dex with 0.8/3.6 per cent of pixels be-

$$
N_{side}=512
$$

$$
N_{side}=8
$$

$$
N_{side}=2048
$$

$$
\mathit{GALEX}
$$

$$
g,0.046/0.043\mathrm{d}x
$$

yond 0.3 dex. The block-to-block scatter of the forty
median offsets is 0.023 (FUV) and 0.011 dex (NUV).
The FUV error grows from 0.032 dex at the block edge
to 0.055 dex more than 2<sup>◦</sup> inside; keeping the NUVinformed layer flattens the profile to 0.037–0.038 dex
(*|*median*|≤* 0.004 dex), and the NUV profile is flat at
0.042–0.046 dex. Intra-block structure is recovered with
a Pearson correlation of 0.80 (FUV) and 0.78 (NUV) at
1.7<sup>′</sup>, rising to 0.90/0.93 at 27<sup>′</sup>. Model-only N<sub>side</sub> = 512
hindcasts of the six spatial folds (each sixth of the
*GALEX* sky rebuilt from the out-of-fold prediction, reestimated gain field and source layers, no ultraviolet
data inside) give 0.038/0.039 dex robust scatter against
*g,* medians +0.001/−0.005 dex, 0.030→0.042 dex from
the fold edge to > 2<sup>◦</sup> inside, block-to-block scatter
0.019/0.017 dex and intra-block structure correlations
◦
of 0.86/0.86 at 6.9<sup>′</sup> rising to 0.94/0.88 at 2. Using
the in-sample prediction instead of the out-of-fold one
changes the N<sub>side</sub> = 512 scatter by only 0.004/0.002 dex,

$$
2^{\circ}
$$

$$
\left(|median|\leq0.004dex\right)
$$

$$
N_{side}=512
$$

$$
1.7'
$$

$$
27'
$$

$$
g,
$$

$$
+0.001/-0.005
$$

$$
to>2^{\circ}
$$

$$
6.9^{\prime}
$$

$$
2^{\circ}
$$

$$
N_{side}=512
$$ a block 565: $/=62^\circ$, $b=-19^\circ$ (+0.006 dex)

block 147: $/=247^\circ$, $b=+36^\circ$ (-0.048 dex)

block 241: $/=322^\circ$, $b=+54^\circ$ (-0.024 dex)

destriped GALEX FUV $\Delta b$ (deg)

fusion, block hidden $\Delta b$ (deg)

log$_{10}$ CU

log$_{10}$ CU

log$_{10}$ recon./truth $\Delta b$ (deg)

$\Delta I$ (deg)

dex

$\Delta I$ (deg)

| log10 GALEX FUV truth (CU) | log10 reconstruction (CU) |
| --- | --- |
| 2.5 | 2.5 |
| 3.0 | 3.0 |
| 3.5 | 3.5 |
| 4.0 | 4.0 |
| 4.5 | 4.5 |

| log10 GALEX NUV truth (CU) | NUV gap, nside 512: -0.010 dex, σrob=0.045 dex (dex, nside 512) |
| --- | --- |
| 3.0 | 2.97 |
| 3.5 | 3.47 |
| 4.0 | 3.97 |
| 4.5 | 4.47 |

| distance inside block edge (deg) | FUV, pure gap (dex, nside 512) | FUV, NUV-inf. layer kept (dex, nside 512) | NUV (dex, nside 512) | FUV, pure gap median offset (dex, nside 512) | FUV, NUV-inf. layer kept median offset (dex, nside 512) | NUV median offset (dex, nside 512) |
| --- | --- | --- | --- | --- | --- | --- |
| 0.07 | 0.032 | 0.038 | 0.041 | -0.007 | -0.001 | -0.013 |
| 0.22 | 0.035 | 0.037 | 0.042 | -0.003 | -0.001 | -0.010 |
| 0.43 | 0.036 | 0.038 | 0.045 | -0.008 | -0.001 | -0.010 |
| 0.77 | 0.040 | 0.038 | 0.044 | -0.010 | -0.001 | -0.010 |
| 1.41 | 0.048 | 0.038 | 0.045 | -0.017 | -0.001 | -0.007 |
| 3.05 | 0.055 | 0.038 | 0.046 | -0.025 | -0.003 | -0.012 |

Figure 10: Mock-gap test of the production fusion. (a) Three of the forty hidden 7.3<sup>◦</sup>blocks (one per *|b|* stratum; outline,
hidden super-pixel; blank, no *GALEX):* destriped *GALEX* FUV truth, the production fusion re-run with the block hidden, and
log<sub>10</sub>(reconstruction/truth) on ±0.3 dex, with the per-block median offset. (b, c) Reconstruction versus truth at Nside= 512 for all
hidden FUV and NUV pixels; dashed, *±0.1* dex. (d) Robust scatter (solid) and median offset (dotted) versus distance inside the block
edge for the FUV pure gap, the FUV gap with the NUV-informed layer kept, and the NUV.

$$
7.3^{\circ}
$$

$$
GALEX)
$$

$$
\log_{10}(
$$

$$
N_{side}=512
$$ so the moderate column density of the hidden blocks,
not training-set membership, makes mock gaps easy (no
hidden block exceeds *E(B−V*) = 0.5). A larger blind experiment on the FIMS/SPEAR-constrained layer alone
(2.25 million hidden pixels, distances to 8.6<sup>◦</sup>) gives 68thpercentile errors of 0.046, 0.051, 0.054, 0.057, 0.068 and
0.093 dex from 0<sup>◦</sup> to 9<sup>◦</sup>, U-shaped in *E(B−V*) (0.078 at
*E(B−V*) < 0.02, 0.058 at 0.06–0.2, 0.10–0.13 dex above
0.4).

$$
E(B-V)=0.5
$$

$$
8.6^{\circ})
$$

$$
0^{\circ}
$$

$$
9^{\circ}
$$

$$
E(B-V)<0.02,0.058
$$

### 4.4 Independent instruments in the <em>GALEX</em>-free sky

## GALEX -free sky

All preceding tests use *GALEX* as truth and cannot
reach the high-column plane. Two instruments observed
it (Fig. 11). *FUV versus FIMS/SPEAR.* In strictly
GALEX-free 27.5<sup>′</sup> pixels (n = 45 354, median |b| =
8.7<sup>◦</sup>, median *E(B −V*) = 0.36) the published map has
a median ratio to the FIMS/SPEAR *starry* map of
0.92 with 0.19 dex robust scatter and *r* = 0.80 in logI,
against 1.05, 0.15 dex and *r* = 0.83 for destriped *GALEX*
itself versus FIMS/SPEAR starry where both observed:
the filled FUV sky agrees with FIMS/SPEAR to within
1.3 times the scatter that *GALEX* shows, at a level
slightly below the *GALEX* /FIMS/SPEAR scale. This
comparison is not independent of FIMS/SPEAR above
◦ ◦ ◦
4 in W<sub>FIMS</sub> sky, but the sub-4 structure (4-highpass *r* = 0.67) is untouched by the ratio field, and the
reference differs from the constraint quantity (starry
versus 0.97×starless plus catalogue stars, which the
map matches to 0.99, 0.07 dex, *r* = 0.95). Against
FIMS/SPEAR starless the *diffuse* prediction with no
ultraviolet input has, in the same sky, a median ratio
of 0.86 with 0.135 dex scatter and *r* = 0.84—as high
a correlation as between the two instruments where
both observed—and a genuine low bias reaching 20 per
cent in the mid-plane, which is what the FIMS/SPEAR
constraint corrects. NUV versus UVOT. In 13.7<sup>′</sup> pixels
◦
with no *GALEX* NUV (*n* = 5362, median *|b|* = 0.7,
median *E(B −V*) = 5.2) the published NUV map has a
median ratio to UVM2-calibrated UVOT of 1.005 with
◦
0.146 dex scatter and *r* = 0.77 (1.03, 0.12 dex at *|b|* < 1;
1.01, 0.21 dex in pixels with W<sub>UVOT</sub> < 0.05, which are
strictly independent).

$$
n=45354
$$

$$
\left|b\right|=
8.7^{\circ}
$$

$$
E(B-V)=0.36
$$

$$
r=0.80
$$

$$
r=0.83
$$

$$
I,
$$

$$
4^{\circ}
$$

$$
W_{FIMS}
$$

$$
(4^{\circ}-high-
$$

$$
r=0.67)
$$

$$
r\;=\;0.95)
$$

$$
r=0.84—as
$$

$$
(n=5362
$$

$$
E(B-V)=5.2
$$

$$
\left|b\right|=0.7^{\circ}
$$

$$
r=0.77
$$

$$
|b|<1^{\circ};
$$

$$
W_{UVOT}<0.05
$$

*The FUV plane bracket.* In the same *GALEX*-free
plane the published FUV map is 1.37/1.41/1.23 times
the UVOT-derived FUV at *|b|* = 0<sup>◦</sup> –5, 5–10 and 10<sup>◦</sup> –
20<sup>◦</sup>, while it is 0.87–0.95 times FIMS/SPEAR: the two
references bracket the adopted level by *≃* 0.2 dex either side (Fig. 12). We adopt the FIMS/SPEAR-based
level: FIMS/SPEAR measures the 1350–1710Å band
directly and its ratio to *GALEX* is flat to ±0.04 dex
over 0.01 < *E(B −V*) < 1.2, whereas the UVOT ‘FUV’
is an extrapolation from 1930–2250Å across the steepest
part of the extinction curve, whose ratio to *GALEX*
drifts by 0.51 *±* 0.01 dex per decade of *E(B −V).* Half
the bracket, 0.10 dex, equal also to the FIMS/SPEAR
absolute calibration uncertainty (Seon et al. 2011), is
recorded as the coherent systematic SIGMA_SYS of the
FIMS/SPEAR-constrained plane (Section 4.6).

$$
1.37/1.41/1.23
$$

$$
20^{\circ}
$$

$$
|b|=0^{\circ}-5
$$

$$
10^{\circ}-
$$

$$
0.01<E(B-V)<1.2
$$

$$
0.51\pm0.01
$$

### 4.5 Discrete sources by pixel class

Aperture photometry of GUVcat, XMM-SUSS and
UVOT serendipitous-source stars recovers catalogue
fluxes with a median ratio of 0.98 in *GALEX*-dominated
pixels (Appendix C). In *GALEX*-free, model-filled NUV
′
sky, aperture photometry (*r* < 2 ) of 3191 isolated
*Swift* UVOTSSC sources recovers half the UVM2 flux at
UVM2≃ 19.3, within a factor of two for 0.57 of sources
at 17–18mag and 0.42 at 18–19, with an NMAD of the
ratio of 0.18–0.43 at 13–17mag; beyond UVM2≃ 19 a 2<sup>′</sup>
aperture on a ∼ 10<sup>3</sup> CU background has S/N< 1 and the
test loses power. TD-1 stars outside the *GALEX* FUV
footprint (*n* = 2828) return a median log(ap/1.047 TD1)
◦
of +0.008 dex (Section 3.6; 24 stars at *|b|* < 11 read
> 0.3 dex high because *Gaia* neighbours enter the
6<sup>′</sup> aperture, and +0.006 dex against the catalogue in
NUV-informed sky), and every *V* < 4.5 hot BSC star
(*n* = 209) is reproduced to within 1 dex. A stack of
143154 Milliquas quasars, each referenced to a matched
control position, detects them at +13.1 *±* 0.5 CU (FUV)
and +26 CU (NUV) in *GALEX* pixels. These catalogued quasars are inputs of the extragalactic point
layer and are recovered in every filled class as well
(+13.0 *±* 1.1 CU FUV over all filled pixels; +10.6 *±* 0.9
FIMS/SPEAR-, +17±6 NUV-informed- and +17±7 CU
model-dominated), so the null test is made with 98513
fainter quasars (19.8 < *R* < 21.5) absent from every input layer: they are detected at +1.1 *±* 0.5 CU
in *GALEX* FUV pixels and give *−0.5 ±* 1.1 CU over
filled FUV sky (0.0 *±* 0.8 FIMS/SPEAR-dominated,
*−6 ±* 5 model-dominated, +6 *±* 6 CU NUV-informed,
where NUV imaging legitimately carries them): objects
absent from the inputs leave no imprint, as required
(Appendix C). Stacked 15–17mag input stars return
0.99–1.09 (median) of their catalogue flux within 2<sup>′</sup> in
*GALEX*-, FIMS/SPEAR- and model-dominated sky; in
NUV-informed FUV sky (W<sub>NUVX</sub> > 0.5) stacks return
1.00, 0.92, 0.97 and 1.10 of the catalogue flux at 4–8, 8–
12, 12–15 and 15–17mag, colour-flat (0.91/0.97/1.02 for
blue/intermediate/red stars at 12–15mag; Section 3.8).

$$
\left(r\;<\;2^{\prime}\right)
$$

$$
\mathrm{UVM2}\simeq19.3
$$

$$
2'
$$

$$
S/N<1
$$

$$
\ldotp\sim10^{3}
$$

$$
(n=2828)
$$

$$
>0.3\mathrm{dex}
$$

$$
6'
$$

$$
|b|<11^{\circ}
$$

$$
V<4.5
$$

$$
(n=209)
$$

$$
+13.1\pm0.5CU
$$

$$
(+13.0\pm1.1\mathrm{CU}
$$

$$
+10.6\pm0.9
$$

$$
\pm17\pm7\mathrm{CU}
$$

$$
(19.8<R<21.5)
$$

$$
+1.1\pm0.5CU
$$

$$
-0.5\pm1.1CU
$$

$$
(0.0\pm0.8
$$

$$
-6\pm5
$$

$$
+6\pm6\mathrm{CU}
$$

$$
\left(W_{NUVX}>0.5\right)
$$

### 4.6 Uncertainty model and its calibra- tion

The published uncertainty file carries two columns per
N<sub>side</sub> = 512 pixel. SIGMA is the half-width u of the
central 68 per cent interval of log<sub>10</sub> *I,*
∑

$$
N_{side}=512
$$

$$
\log_{10}I,
$$

$$
\begin{aligned}{u^{2}}&{{}=\frac{\sum_{k}W_{k}\:\sigma_{k}^{2}}{\sum_{k}W_{k}}+\big(W_{\operatorname{FIMS}}+W_{\operatorname{MODEL}}+W_{\operatorname{NUVX}}\big)\:\sigma_{\star}^{2},}\\ {\sigma_{\operatorname{G}}}&{{}=\sigma_{\operatorname{G}}(I,\:t_{\exp}),\qquad\sigma_{\operatorname{U}}=\sigma_{\operatorname{U}}(I),}\\ {\sigma_{\operatorname{X}}}&{{}=\sigma_{\operatorname{X}}(E(B\mathop{-}V),\:|b|,\:I_{\operatorname{NUV}}),}\\ {\sigma_{\operatorname{M}}}&{{}=\sigma_{\operatorname{M}}(\hat{\mathbf{n}}),}\\ {\sigma_{\operatorname{F}}}&{{}=\operatorname*{m a x}\bigl[S(d,E(B\mathop{-}V)),\;\sigma_{\operatorname{M}},\;0.046\bigr],}\\ \end{aligned}
$$

(10)

in dex, with u ≥ 0.02 and the weights of the fusion. σ<sub>G</sub>
is the empirical 6.9<sup>′</sup> noise of g, a monotone quantile-
0.68 regression of the eight-neighbour scatter of the
star-subtracted diffuse map on intensity and effective

$$
u\geq0.02
$$

$$
6.9^{\prime}
$$

---

| log10 FIMS/SPEAR starry FUV (CU) | log10 FUV, this work (CU) |
| --- | --- |
| 2.0 | 1.9 |
| 2.5 | 2.5 |
| 3.0 | 3.0 |
| 3.5 | 3.5 |
| 4.0 | 4.0 |
| 4.5 | 4.5 |

| log10 UVOT UVM2 → NUV (CU) | Series 1 (unlabelled) (log10 NUV, this work, CU) | Series 2 (unlabelled) (log10 NUV, this work, CU) |
| --- | --- | --- |
| 3.25 | 3.21 | 3.51 |
| 3.50 | 3.48 | 3.79 |
| 3.75 | 3.75 | 4.07 |
| 4.00 | 4.01 | 4.35 |
| 4.25 | 4.27 | 4.63 |
| 4.50 | 4.51 | 4.91 |

|  | b | (deg) | FUV map / FIMS stary (log10 this work / reference, dex) | NUV map / UVOT UVM2 (log10 this work / reference, dex) | NUV gap-fill layer / UVM2 (log10 this work / reference, dex) |
| --- | --- | --- | --- | --- | --- |
| 0.3 |  | 0.00 | 0.00 |  |  |
| 0.6 | -0.07 | 0.01 | -0.01 |  |  |
| 0.9 | -0.07 | 0.02 | 0.01 |  |  |
| 1.3 | -0.06 | 0.11 | 0.05 |  |  |
| 2.3 | -0.05 | 0.08 | 0.05 |  |  |
| 3.0 | -0.04 | 0.03 | 0.00 |  |  |
| 5.0 | -0.02 | -0.03 | -0.03 |  |  |
| 6.0 | -0.02 | 0.03 | 0.00 |  |  |
| 7.5 | -0.03 | -0.04 | -0.04 |  |  |
| 10 | -0.04 | -0.06 | -0.06 |  |  |
| 13 | -0.04 | -0.03 | -0.03 |  |  |
| 17 | -0.05 | -0.04 | -0.05 |  |  |
| 20 | -0.05 | -0.04 | -0.06 |  |  |
| 23 | -0.05 | -0.06 | -0.07 |  |  |
| 26 | -0.04 | -0.20 | -0.23 |  |  |
| 50 | -0.08 |  |  |  |  |
| 53 | -0.08 |  |  |  |  |

Figure 11: The published maps against independent instruments in sky with no *GALEX* coverage in the band concerned. (a) Published
FUV versus the FIMS/SPEAR starry map in 27.5<sup>′</sup> pixels (n = 45 354, median |b| = 8.7◦). (b) Published NUV versus Swift-UVOT
UVM2, calibrated to CU on *GALEX*-covered *E(B−V*) > 0.1 pixels, in 13.7<sup>′</sup>pixels (*n* = 5362, median *|b|* = 0.7<sup>◦</sup>, median *E(B−V*) = 5.2).
Dashed, factor of two. (c) Median and 16–84 per cent band of log(map/reference) versus *|b|*; dotted, the NUV gap-fill layer alone.

$$
(n=45354
$$

$$
\left|b\right|=8.7^{\circ})
$$

$$
E(B-V)>0.1
$$

$$
13.7'
$$

exposure (FUV 0.019 dex in AIS tiles below 120s to
0.008 dex above 5ks, at the 0.02 floor over 80 per cent
of *GALEX* sky; NUV 0.029/0.020/0.011 dex at < 120 s,
120–200s and 0.4–1.5ks, median 0.026). The model
term σ<sub>M</sub> is a quantile-0.68 gradient-boosted regression
of the adopted predictor’s out-of-fold *|*log<sub>10</sub>(pred/g)*|* on
the total intensity (1.75 × 10<sup>6</sup>/2.12 × 10<sup>6</sup> pixels; heldout p68 0.044/0.053 dex) on logE(B −V), |b|, logI<sub>Hα</sub>,
logn<sub>⋆</sub>, logF<sub>BP</sub> and distance to training data, the latter
recomputed per fold and capped at 5<sup>◦</sup>, the range the
◦
7.3 blocks can represent; it is evaluated all-sky with the
distance-to-*GALEX* map and extended linearly beyond:
◦ ◦
+0.0825 dex deg<sup>−1</sup> from 5 to 6 (which brings the 2500
held-out FUV pixels at 5<sup>◦</sup> –8<sup>◦</sup>, p68 0.113 dex, from a
coverage of 0.43 to 0.71) and +0.0133 dex deg<sup>−1</sup> beyond
(the weighted slope of the held-out p68 over 3<sup>◦</sup> –7<sup>◦</sup>) in
◦
the FUV, +0.0056 dex deg<sup>−1</sup> beyond 5 in the NUV.
σ<sub>X</sub> is a quantile-0.68 regression of the diffuse colour
model’s cross-validation residuals (p68 0.036 dex; Section 3.8) on *E(B −V), |b|* and NUV intensity (median
0.046 dex in W<sub>NUVX</sub> > 0.5 sky, 90th percentile 0.121);
σ<sub>U</sub> is the measured scatter of the UVOT layer against g
after removal of 3.7<sup>◦</sup> medians, interpolated in intensity
(0.077/0.107/0.188 dex FUV and 0.056/0.090/0.114 dex
NUV at logI = 2.5/3.25/3.75). The FIMS/SPEAR
term σ<sub>F</sub> is the quantile-0.68 surface S of the blind
mock-gap error of Section 4.3 on distance to *GALEX*
and logE(B *−V),* monotone non-decreasing in distance
(Fig. 13): 0.040–0.066 dex at *E(B − V*) < 0.2 within
◦ ◦ ◦
1, 0.048–0.069 at 1.5, 0.049–0.084 at 3, and 0.106–
0.134 dex for *E(B − V*) *≥* 0.6 beyond 1.5<sup>◦</sup>, saturating
at 0.134 dex beyond 5<sup>◦</sup>; the floor 0.046 dex is the measured error within 0.5<sup>◦</sup> of data. The stellar-layer term is
σ<sub>⋆</sub> = log<sub>10</sub>(1 + s<sub>512</sub>/I) from the per-pixel 1σ map of the
source layer (stars_sigma; median 0.013/0.006 dex in
filled sky, raising *u* by more than 10 per cent in 11/7 per
cent of filled pixels). In FIMS/SPEAR-dominated sky
the median *u* is 0.052, 0.076 and 0.113 dex at 0<sup>◦</sup> –1, 1–2
◦ ◦
and 2 –4 from *GALEX,* and 0.19 dex (90th percentile

out p68 0.044/0.053 dex) on log E ( B −V ), |b| , log I Hα ,

$$
|b|=0.7^{\circ}
$$

$$
E(B-V)=5.2
$$

$$
at<120s,
$$

$$
\sigma_{M}
$$

$$
|\log_{10}(pred/g)
$$

$$
(1.75\times10^{6}/2.12\times10^{6})
$$

$$
\mathrm{p}68~0.044/0.053\mathrm{dex}
$$

$$
E(B-V)
$$

$$
I_{\mathrm{H}\alpha},
$$

$$
F_{\mathrm{BP}}
$$

$$
n_{\star}
$$

$$
5^{\circ}
$$

$$
7.3^{\circ}
$$

$$
5^{\circ}
$$

$$
+0.0825\deg^{-1}
$$

$$
6^{\circ}
$$

$$
5^{\circ}-8^{\circ}
$$

$$
\mathrm {F U V}, + 0. 0 0 5 6 \mathrm {d e x} \deg^ {- 1}
$$

$$
3^{\circ}-7^{\circ})
$$

$$
5^{\circ}
$$

$$
\sigma_{U}
$$

$$
3.7^{\circ}
$$

$$
(0.077/0.107/0
$$

$$
I=2.5/3.25/3.75
$$

$$
\sigma_{\mathrm{F}}
$$

$$
1^{\circ},\;0.048--0.069
$$

$$
E(B-V)<0.2
$$

$$
1.5^{\circ},0.049--0.084
$$

$$
E(B-V)\geq0.6
$$

$$
0.5^{\circ}
$$

$$
3^{\circ}
$$

$$
1.5^{\circ}.
$$

$$
\sigma_{\star}=\log_{10}(1+s_{512}/I)
$$

$$
2^{\circ}-4^{\circ}
$$

$$
0^{\circ}-1
$$

0.27) in the 4.5 per cent of the sky that is FUV-filled
◦ ◦
more than 6 from *GALEX;* beyond 8 (1.8 per cent of
the sky) σ<sub>M</sub> is a linear extrapolation without held-out
support. In NUV model sky the medians are 0.058, 0.084
and 0.100 dex at 0<sup>◦</sup> –1, 1–2 and 2<sup>◦</sup> –4<sup>◦</sup>. All-sky, SIGMA
has 10th/50th/90th percentiles of 0.020/0.022/0.118
(FUV) and 0.021/0.029/0.083 dex (NUV); by provenance
the medians are 0.020/0.026 (*GALEX),* 0.058 (NUVinformed FUV), 0.068 (FIMS/SPEAR), 0.072/0.065
(model) and 0.19/0.11 dex (UVOT). SIGMA_SYS is
◦
0.10 dex where W<sub>FIMS</sub> > 0 at |b| < 10 (13.0 per cent
of the sky) and 0.05 dex where W<sub>UVOT</sub> > 0.5, flagged
in both bands; it is not to be added in quadrature per
*√*
pixel nor divided by *N* when averaging. The per-tile
offsets removed by the tile equalisation amount to p68
0.018/0.021 dex over *GALEX* pixels and are recovered
with slope 0.947 in injection tests, so the un-removed
tile residual is *≃* 0.001 dex (keyword TILESYS) and is
not added; the inherited additive foreground zero point
is in neither column.

$$
6^{\circ}
$$

$$
8^{\circ}
$$

$$
\sigma_{M}
$$

$$
0^{\circ}-1
$$

$$
2^{\circ}-4^{\circ}
$$

$$
\left|b\right|<10^{\circ}
$$

$$
W_{FIMS}>0
$$

$$
W_{UVOT}>0.5,
$$

$$
\sqrt{N}
$$

$$
0.947
$$

$$
\simeq0.001
$$

*Calibration.* In cross-half validation the coverage
of |r| < σ<sub>M</sub> on held-out pixels is 0.677/0.678 areaweighted and 0.656/0.675 gap-matched (FUV/NUV),
0.67–0.69 in every distance bin to 8<sup>◦</sup> (FUV, with the
far extension; NUV to 6<sup>◦</sup>), 0.65–0.68 in every *E(B −V*)
bin below 0.8mag, and 0.52/0.66 at *E(B − V*) = 0.8–
1.6 (2600/36000 pixels), where FUV users should inflate SIGMA by ≃ 1.6. The held-out coverage of σ<sub>X</sub>
is 0.684/0.671. On the test half of the 2.25 million
blind mock-gap pixels σ<sub>F</sub> covers 0.71 overall, 0.70–0.73
by distance, 0.69–0.77 by *E(B − V*) and 0.68–0.74 by
*|b|*, where a flat 0.10 dex would cover 0.90 overall but
0.63–0.73 at *E(B − V*) > 0.4 (Table 10, Appendix B).
The residuals are heavy-tailed and mildly skewed:
|r|/σ<sub>M</sub> has 95.45th/99.73rd percentiles of 2.52/15.3
(FUV) and 2.46/19.2 (NUV) area-weighted, 2.69/7.1 and
2.49/13.3 gap-matched (2.23/5.24 for the NUV-informed
term; header keywords MULT954, MULT997, MULT954G,
MULT997G), and 92 per cent of residuals lie within *2u*

$$
|r|<\sigma_{M}
$$

$$
0.67-0.69
$$

$$
8^{\circ}
$$

$$
E(B-V)
$$

$$
E(B-V)=0.8
$$

$$
\sigma_{X}
$$

$$
\simeq1.6.
$$

$$
\sigma_{\mathrm{F}}
$$

$$
E(B-V)
$$

$$
E(B-V)>0.4
$$

$$
|r|/\sigma_{\mathrm{M}}
$$

$$
\mathrm{(FUV)}
$$

---

**a**

| Planck E(B−V) [mag] | GALEX / UVOT-derived FUV (log10 ratio, dex) | GALEX / FIMS starry (55' cells) (log10 ratio, dex) | released map / UVOT-FUV, no GALEX (log10 ratio, dex) |
| --- | --- | --- | --- |
| 0.013 | -0.48 | 0.00 |  |
| 0.016 | -0.47 | -0.02 |  |
| 0.02 | -0.44 | 0.02 |  |
| 0.024 | -0.43 | 0.03 |  |
| 0.028 | -0.40 | 0.03 |  |
| 0.032 | -0.37 | 0.04 |  |
| 0.036 | -0.34 | 0.04 |  |
| 0.04 | -0.31 | 0.04 |  |
| 0.044 | -0.28 | 0.04 |  |
| 0.048 | -0.25 | 0.04 |  |
| 0.052 | -0.22 | 0.04 |  |
| 0.056 | -0.20 | 0.04 |  |
| 0.06 | -0.18 | 0.04 |  |
| 0.064 | -0.16 | 0.04 |  |
| 0.068 | -0.14 | 0.04 |  |
| 0.072 | -0.12 | 0.04 |  |
| 0.076 | -0.10 | 0.04 |  |
| 0.08 | -0.08 | 0.04 |  |
| 0.084 | -0.06 | 0.04 |  |
| 0.088 | -0.04 | 0.04 |  |
| 0.092 | -0.02 | 0.04 |  |
| 0.096 | 0.00 | 0.04 |  |

| Galactic latitude bin (GALEX-free pixels, 13.7) | map / UVOT-FUV (E(B−V)≥0.1 cal.) (median log10 ratio, dex) | map / UVOT-FUV (E(B−V)-dependent cal.) (median log10 ratio, dex) | map / FIMS starry (median log10 ratio, dex) | map / (0.97 FIMS starless + cat. stars) (median log10 ratio, dex) | (0.97 FIMS starless + stars) / UVOT-FUV (median log10 ratio, dex) |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  | b | <5° | 0.13 | -0.01 | -0.12 | -0.02 | 0.18 |
| 5-10° | 0.15 | 0.05 | -0.08 | -0.01 | 0.18 |  |  |
| 10-20° | 0.08 | 0.08 | -0.09 | -0.01 | 0.10 |  |  |
| >20° | -0.16 | 0.05 | -0.07 | -0.01 | -0.10 |  |  |

**Where the tension lives: log10(released map / UVOT-derived FUV) in GALEX-free pixels (n = 29474, nside 256)**

| Galactic longitude l (deg) (increasing to the left) | b (deg) | dex |
| --- | --- | --- |
| 210 | 0 | 0.00 |
| 260 | 0 | 0.00 |
| 310 | 0 | 0.00 |
| 0 | 0 | 0.00 |
| 50 | 0 | 0.00 |
| 100 | 0 | 0.00 |
| 150 | 0 | 0.00 |

$$
C_{F}
$$

$$
W_{F}>0.1
$$

**d. mostly Vela / Upper Sco**

| l (deg) | b (deg) |
| --- | --- |
| -200 | 15 |
| -200 | -60 |
| -180 | -60 |
| -180 | -68 |
| -160 | -68 |
| -160 | -60 |
| -140 | -60 |
| -140 | -68 |
| -120 | -68 |
| -120 | -60 |
| -100 | -60 |
| -100 | -68 |
| -80 | -60 |
| -80 | -68 |
| -60 | -60 |
| -60 | -68 |
| -40 | -60 |
| -40 | -68 |
| -20 | -60 |
| -20 | -68 |
| 0 | -60 |
| 0 | -68 |
| 20 | -60 |
| 20 | -68 |
| 40 | -60 |
| 40 | -68 |
| 60 | -60 |
| 60 | -68 |
| 80 | -60 |
| 80 | -68 |
| 100 | -60 |
| 100 | -68 |
| 120 | -60 |
| 120 | -68 |
| 140 | -60 |
| 140 | -68 |
| 160 | -60 |
| 160 | -68 |
| 180 | -60 |
| 180 | -68 |
| 200 | -60 |
| 200 | -68 |
| 220 | -60 |
| 220 | -68 |
| 240 | -60 |
| 240 | -68 |
| 260 | -60 |
| 260 | -68 |
| 280 | -60 |
| 280 | -68 |
| 300 | -60 |
| 300 | -68 |
| 320 | -60 |
| 320 | -68 |
| 340 | -60 |
| 340 | -68 |
| 360 | -60 |
| 360 | -68 |
| 380 | -60 |
| 380 | -68 |
| 400 | -60 |
| 400 | -68 |
| 420 | -60 |
| 420 | -68 |
| 440 | -60 |
| 440 | -68 |
| 460 | -60 |
| 460 | -68 |
| 480 | -60 |
| 480 | -68 |
| 500 | -60 |
| 500 | -68 |
| 520 | -60 |
| 520 | -68 |
| 540 | -60 |
| 540 | -68 |
| 560 | -60 |
| 560 | -68 |
| 580 | -60 |
| 580 | -68 |
| 600 | -60 |
| 600 | -68 |
| 620 | -60 |
| 620 | -68 |
| 640 | -60 |
| 640 | -68 |
| 660 | -60 |
| 660 | -68 |
| 680 | -60 |
| 680 | -68 |
| 700 | -60 |
| 700 | -68 |
| 720 | -60 |
| 720 | -68 |
| 740 | -60 |
| 740 | -68 |
| 760 | -60 |
| 760 | -68 |
| 780 | -60 |
| 780 | -68 |
| 800 | -60 |
| 800 | -68 |
| 820 | -60 |
| 820 | -68 |
| 840 | -60 |
| 840 | -68 |
| 860 | -60 |
| 860 | -68 |
| 880 | -60 |
| 880 | -68 |
| 900 | -60 |
| 900 | -68 |
| 920 | -60 |
| 920 | -68 |
| 940 | -60 |
| 940 | -68 |
| 960 | -60 |
| 960 | -68 |
| 980 | -60 |
| 980 | -68 |
| 1000 | -60 |
| 1000 | -68 |
| 1020 | -60 |
| 1020 | -68 |
| 1040 | -60 |
| 1040 | -68 |
| 1060 | -60 |
| 1060 | -68 |
| 1080 | -60 |
| 1080 | -68 |
| 1100 | -60 |
| 1100 | -68 |
| 1120 | -60 |
| 1120 | -68 |
| 1140 | -60 |
| 1140 | -68 |
| 1160 | -60 |
| 1160 | -68 |
| 1180 | -60 |
| 1180 | -68 |
| 1200 | -60 |
| 1200 | -68 |
| 1220 | -60 |
| 1220 | -68 |
| 1240 | -60 |
| 1240 | -68 |
| 1260 | -60 |
| 1260 | -68 |
| 1280 | -60 |
| 1280 | -68 |
| 1300 | -60 |
| 1300 | -68 |
| 1320 | -60 |
| 1320 | -68 |
| 1340 | -60 |
| 1340 | -68 |
| 1360 | -60 |
| 1360 | -68 |
| 1380 | -60 |
| 1380 | -68 |
| 1400 | -60 |
| 1400 | -68 |
| 1420 | -60 |
| 1420 | -68 |
| 1440 | -60 |
| 1440 | -68 |
| 1460 | -60 |
| 1460 | -68 |
| 1480 | -60 |
| 1480 | -68 |
| 1500 | -60 |
| 1500 | -68 |
| 1520 | -60 |
| 1520 | -68 |
| 1540 | -60 |
| 1540 | -68 |
| 1560 | -60 |
| 1560 | -68 |
| 1580 | -60 |
| 1580 | -68 |
| 1600 | -60 |
| 1600 | -68 |
| 1620 | -60 |
| 1620 | -68 |
| 1640 | -60 |
| 1640 | -68 |
| 1660 | -60 |
| 1660 | -68 |
| 1680 | -60 |
| 1680 | -68 |
| 1700 | -60 |
| 1700 | -68 |
| 1720 | -60 |
| 1720 | -68 |
| 1740 | -60 |
| 1740 | -68 |
| 1760 | -60 |
| 1760 | -68 |
| 1780 | -60 |
| 1780 | -68 |
| 1800 | -60 |
| 1800 | -68 |
| 1820 | -60 |
| 1820 | -68 |
| 1840 | -60 |
| 1840 | -68 |
| 1860 | -60 |
| 1860 | -68 |
| 1880 | -60 |
| 1880 | -68 |
| 1900 | -60 |
| 1900 | -68 |
| 1920 | -60 |
| 1920 | -68 |
| 1940 | -60 |
| 1940 | -68 |
| 1960 | -60 |
| 1960 | -68 |
| 1980 | -60 |
| 1980 | -68 |
| 2000 | -60 |
| 2000 | -68 |
| 2020 | -60 |
| 2020 | -68 |
| 2040 | -60 |
| 2040 | -68 |
| 2060 | -60 |
| 2060 | -68 |
| 2080 | -60 |
| 2080 | -68 |
| 2100 | -60 |
| 2100 | -68 |
| 2120 | -60 |
| 2120 | -68 |
| 2140 | -60 |
| 2140 | -68 |
| 2160 | -60 |
| 2160 | -68 |
| 2180 | -60 |
| 2180 | -68 |
| 2200 | -60 |
| 2200 | -68 |
| 2220 | -60 |
| 2220 | -68 |
| 2240 | -60 |
| 2240 | -68 |
| 2260 | -60 |
| 2260 | -68 |
| 2280 | -60 |
| 2280 | -68 |
| 2300 | -60 |
| 2300 | -68 |
| 2320 | -60 |
| 2320 | -68 |
| 2340 | -60 |
| 2340 | -68 |
| 2360 | -60 |
| 2360 | -68 |
| 2380 | -60 |
| 2380 | -68 |
| 2400 | -60 |
| 2400 | -68 |
| 2420 | -60 |
| 2420 | -68 |
| 2440 | -60 |
| 2440 | -68 |
| 2460 | -60 |
| 2460 | -68 |
| 2480 | -60 |
| 2480 | -68 |
| 2500 | -60 |
| 2500 | -68 |
| 2520 | -60 |
| 2520 | -68 |
| 2540 | -60 |
| 2540 | -68 |
| 2560 | -60 |
| 2560 | -68 |
| 2580 | -60 |
| 2580 | -68 |
| 2600 | -60 |
| 2600 | -68 |
| 2620 | -60 |
| 2620 | -68 |
| 2640 | -60 |
| 2640 | -68 |
| 2660 | -60 |
| 2660 | -68 |
| 2680 | -60 |
| 2680 | -68 |
| 2700 | -60 |
| 2700 | -68 |
| 2720 | -60 |
| 2720 | -68 |

| unclipped log10 (0.97 FIMS starless + stars)32 / (model)32 | all FIMS ratio cells (n=10059) (cells per 0.02 dex) | cells with mean W_F>0.5 (n=2544) (cells per 0.02 dex) |
| --- | --- | --- |
| -1.00 | 10 |  |
| -0.75 | 1 |  |
| -0.50 | 2 | 1 |
| -0.25 | 15 | 5 |
| 0.00 | 1000 | 200 |
| 0.25 | 30 | 15 |
| 0.50 | 6 | 3 |
| 0.75 | 1 |  |

Figure 12: The FUV Galactic-plane level bracket. (a) Ratio of the published FUV map to the UVOT-derived FUV and to the
FIMS/SPEAR starry map in *GALEX*-free 13.7<sup>′</sup>pixels versus *|b|*, for the dust-matched UVOT gain and for an *E(B −V*)-dependent gain.
(b) log(G<sup>′</sup>/UVOTFUV) and log(G<sup>′</sup>/FIMS) versus *E(B −V*) in *GALEX*-covered pixels: the UVOT transfer drifts by 0.5 dex per
FUV FUV
decade of *E(B −V),* the FIMS/SPEAR ratio is flat. (c) Sky distribution of the map/UVOT ratio in the *GALEX*-free plane.

$$
13.7'
$$

$$
\mathrm{g}(G_{\mathrm{FUV}}^{\prime}/\mathrm{UVOT}_{\mathrm{FUV}})
$$

$$
E(B-V)
$$

$$
(G_{\mathrm{FUV}}^{\prime}/\mathrm{FIMS}
$$

$$
E(B-V)
$$

$$
\stackrel{\mathrm{e}}{E}(B-V)
$$

---

$$
\sigma_{F},
$$

$$
\sigma_ {F},
$$

$$
\sigma_{F}
$$

| distance to un-hidden GALEX (deg) | flat 0.10 (earlier) (dex) | interim σF (dex) | Series 1 (unlabelled) (dex) |
| --- | --- | --- | --- |
| 0.3 | 0.050 | 0.050 | 0.046 |
| 0.7 | 0.056 | 0.057 | 0.052 |
| 1.5 | 0.066 | 0.066 | 0.056 |
| 3.0 | 0.077 | 0.073 | 0.058 |
| 5.0 | 0.076 | 0.077 | 0.069 |
| 7.2 | 0.094 | 0.095 | 0.093 |

| E(B−V) (mag) | final σ_F (test half) | Series 1 (unlabelled) | Series 2 (unlabelled) |
| --- | --- | --- | --- |
| 0.01 | 0.062 | 0.079 | 0.078 |
| 0.02 | 0.060 | 0.075 | 0.074 |
| 0.03 | 0.058 | 0.073 | 0.071 |
| 0.04 | 0.053 | 0.065 | 0.062 |
| 0.05 | 0.050 | 0.062 | 0.060 |
| 0.06 | 0.047 | 0.062 | 0.058 |
| 0.07 | 0.046 | 0.062 | 0.057 |
| 0.08 | 0.049 | 0.062 | 0.058 |
| 0.09 | 0.052 | 0.062 | 0.059 |
| 0.1 | 0.053 | 0.062 | 0.060 |

|  | b | (deg) | Series 1 (unlabelled) | Series 2 (unlabelled) | Series 3 (unlabelled) | Series 4 (unlabelled) | Series 5 (unlabelled) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 10 | 0.087 | 0.088 | 0.098 | 0.085 | 0.097 |  |  |
| 15 | 0.053 | 0.058 | 0.065 | 0.060 | 0.064 |  |  |
| 25 | 0.054 | 0.055 | 0.066 | 0.060 | 0.066 |  |  |
| 35 | 0.052 | 0.052 | 0.065 | 0.065 | 0.065 |  |  |
| 50 | 0.057 | 0.057 | 0.067 | 0.068 | 0.067 |  |  |
| 70 | 0.055 | 0.059 | 0.074 | 0.067 | 0.073 |  |  |

$$
\sigma_{F}
$$

| distance to un-hidden GALEX (deg) | Series 1 (unlabelled) coverage P( | err | < σ) | Series 2 (unlabelled) coverage P( | err | < σ) | Series 3 (unlabelled) coverage P( | err | < σ) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0.2 | 0.73 | 0.75 | 0.93 |  |  |  |  |  |  |
| 0.8 | 0.71 | 0.74 | 0.915 |  |  |  |  |  |  |
| 1.5 | 0.70 | 0.78 | 0.90 |  |  |  |  |  |  |
| 3.0 | 0.71 | 0.81 | 0.88 |  |  |  |  |  |  |
| 5.0 | 0.73 | 0.77 | 0.83 |  |  |  |  |  |  |
| 7.2 | 0.72 | 0.74 | 0.74 |  |  |  |  |  |  |

| E(B−V) (mag) | final σF (test half) | Series 1 (unlabelled) | Series 2 (unlabelled) | Series 3 (unlabelled) |
| --- | --- | --- | --- | --- |
| 0.01 | 0.70 | 0.79 | 0.88 |  |
| 0.02 | 0.69 | 0.78 | 0.89 |  |
| 0.03 | 0.69 | 0.77 | 0.90 |  |
| 0.04 | 0.70 | 0.76 | 0.92 |  |
| 0.05 | 0.73 | 0.78 | 0.92 |  |
| 0.06 | 0.73 | 0.77 | 0.91 |  |
| 0.07 | 0.71 | 0.75 | 0.88 |  |
| 0.08 | 0.71 | 0.75 | 0.85 |  |
| 0.09 | 0.70 | 0.75 | 0.83 |  |
| 0.1 | 0.68 | 0.75 | 0.81 |  |
| 0.2 | 0.71 | 0.75 | 0.73 |  |
| 0.3 | 0.70 | 0.75 | 0.65 |  |
| 0.4 | 0.71 | 0.75 | 0.63 |  |
| 0.5 | 0.73 | 0.76 | 0.62 |  |
| 0.6 | 0.74 | 0.76 | 0.62 |  |
| 0.7 | 0.76 | 0.77 | 0.63 |  |
| 0.8 | 0.77 | 0.79 | 0.64 |  |
| 0.9 | 0.78 | 0.80 | 0.65 |  |
| 1 | 0.79 | 0.80 | 0.66 |  |
| 2 | 0.77 | 0.79 | 0.68 |  |

|  | b | (deg) | Series 1 (unlabelled) | Series 2 (unlabelled) | Series 3 (unlabelled) | Series 4 (unlabelled) |
| --- | --- | --- | --- | --- | --- | --- |
| 10 | 0.72 | 0.76 | 0.74 | 0.68 |  |  |
| 17 | 0.74 | 0.78 | 0.88 | 0.68 |  |  |
| 25 | 0.72 | 0.77 | 0.89 | 0.68 |  |  |
| 37 | 0.71 | 0.78 | 0.91 | 0.68 |  |  |
| 52 | 0.68 | 0.75 | 0.90 | 0.68 |  |  |
| 71 | 0.71 | 0.80 | 0.91 | 0.68 |  |  |

Figure 13: The empirical error of the FIMS/SPEAR-constrained FUV layer from 2.25 million blind mock-gap pixels (withheld
GALEX), and the published uncertainty. (a–c) 68th-percentile absolute error at 6.9<sup>′</sup> of the published, distance-tapered layer (solid)
and of an untapered variant (dashed grey) versus distance to the nearest un-hidden *GALEX* pixel, *E(B −V*) and *|b|*, with the median
assigned σF(open squares). (d–f) Coverage of the tapered-layer errors by a flat 0.10 dex, by a surface calibrated on the untapered
variant, and by the published *σ*<sub>F</sub>(test half; target 0.68, grey line).

$$
\mathit{GALEX}
$$

$$
6.9^{\prime}
$$

$$
\sigma_{\mathrm{F}}
$$

$$
E(B-V)
$$

$$
|b|,
$$

$$
\sigma_{\mathrm{F}}
$$

and 97 per cent within 3u, so 95.4 or 99.7 per cent
intervals need these multipliers rather than *2u* and 3u.
Residuals are spatially correlated on ≳ 1<sup>◦</sup> scales and
between bands (*r* = 0.52 for block-mean residuals), so
*√*
*u* must not be divided by *N* when averaging.

$$
3u,
$$

$$
\gtrsim1^{\circ}
$$

### 4.7 Angular power spectra and small- scale power by pixel class

On GALEX pixels with WGALEX> 0.9, the pseudo-C<sub>ℓ</sub>
of log<sub>10</sub> *I* of the published map is within 1.00–1.01 of
that of the *GALEX* layer at every *ℓ ≤* 1536 (Fig. 14d,e).
The gap-fill layer (harmonised prediction plus source
layer) evaluated on the same pixels retains the large
scales (r<sub>ℓ</sub> > 0.99 at ℓ < 100) and 0.87/0.86/0.80 (FUV,
*|b|* = 20<sup>◦</sup> –40/40–60/60<sup>◦</sup> –90) and 1.03/1.09/1.12 (NUV)
of the log-power over ℓ = 100–1500, with r<sub>ℓ</sub> = 0.87–
0.94 averaged over that range (Fig. 14f): a conditionalmean predictor built from 5<sup>′</sup>–6<sup>′</sup> templates is necessarily
smoother than the truth, and the *diffuse* prediction

$$
W_{GALEX}>0.9
$$

$$
\log_{10}I
$$

$$
(r_{\ell}>0.99
$$

$$
C_{\ell}
$$

$$
\ell\leq1536
$$

$$
\left|b\right|=20^{\circ}-40/40-60/60^{\circ}-90
$$

$$
r_{\ell}=0.87
$$

$$
5^{\prime}-6^{\prime}
$$

retains 0.75, 0.54 and 0.36 of the true diffuse power at
*ℓ ≃* 100, 300 and 700, while the discrete-source layer
restores the small-scale power of the *total* intensity to
within *≃* 20 per cent of *GALEX* on identical pixels.
Users of filled pixels for diffuse structure statistics should
treat filament contrasts, structure functions and smallscale power there as lower limits, smoothed by 13–40 per
cent between *ℓ* = 100 and 700 (Section 6). Within the
*GALEX* footprint itself the tile equalisation removes the
tile-diameter bump at *ℓ ≃* 150–400 (Section 3.3); users
comparing power spectra with un-equalised *GALEX*
mosaics should expect this difference.

$$
\ell\simeq150–400
$$

## 5 Physical content and absolute level

We do not perform component separation of the ultraviolet sky, but the reported zero points, dust and *Hα*
slopes, and colours have a physical reading, and users

---

**a FUV: /b/ 40°, E(B−V) 0.08**

| E(B−V) (mag) | GALEX: 231±12 CU (P30 FUV, CU, nside 128) | this map: 232±12 CU (P30 FUV, CU, nside 128) | gap-fill layer: 250±12 CU (P30 FUV, CU, nside 128) |
| --- | --- | --- | --- |
| 0.00 | 220 | 220 | 220 |
| 0.02 | 400 | 390 | 410 |
| 0.04 | 560 | 550 | 570 |
| 0.06 | 720 | 710 | 730 |
| 0.08 | 880 | 860 | 890 |

**b. NUV: /b/ 40°, E(B−V) 0.08**

| E(B−V) (mag) | GALEX: 564±12 CU (P30 NUV, CU, nside 128) | this map: 561±10 CU (P30 NUV, CU, nside 128) | gap-fill layer: 575±10 CU (P30 NUV, CU, nside 128) |
| --- | --- | --- | --- |
| 0.00 | 564 | 561 | 575 |
| 0.02 | 660 | 655 | 670 |
| 0.04 | 755 | 750 | 760 |
| 0.06 | 850 | 845 | 855 |
| 0.08 | 945 | 940 | 950 |

| Category | isotropic intercept (CU) |
| --- | --- |
| GALEX | 220 |
| map (GALEX px) | 220 |
| gap-fill (GALEX px) | 245 |
| map (filled px) | 220 |
| gap-fill (filled px) | 220 |
| FUV | 570 |
| NUV | 570 |
| unlabelled 1 | 570 |
| unlabelled 2 | 570 |

**f. gap-fill–GALEX coherence**

| Category | Series 1 (unlabelled) | Series 2 (unlabelled) | Series 3 (unlabelled) | Series 4 (unlabelled) | Series 5 (unlabelled) | Series 6 (unlabelled) |
| --- | --- | --- | --- | --- | --- | --- |
| unlabelled 1 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 |
| unlabelled 2 | 0.98 | 0.97 | 0.96 | 0.95 | 0.94 | 0.93 |
| unlabelled 3 | 0.95 | 0.93 | 0.91 | 0.89 | 0.87 | 0.85 |
| unlabelled 4 | 0.9 | 0.88 | 0.85 | 0.82 | 0.79 | 0.76 |
| unlabelled 5 | 0.8 | 0.77 | 0.74 | 0.71 | 0.68 | 0.65 |
| unlabelled 6 | 0.65 | 0.63 | 0.61 | 0.59 | 0.57 | 0.55 |
| unlabelled 7 | 0.58 | 0.56 | 0.54 | 0.52 | 0.50 | 0.48 |
| unlabelled 8 | 0.57 | 0.55 | 0.53 | 0.51 | 0.49 | 0.47 |

| multipole ℓ | 20°< | b | <40° (C_L / C_GALEX of log10) | 40°< | b | <60° (C_L / C_GALEX of log10) |  | b | >60° (C_L / C_GALEX of log10) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 10 | 1.0 | 1.0 | 1.05 |  |  |  |  |  |  |
| 20 | 1.0 | 1.0 | 1.05 |  |  |  |  |  |  |
| 30 | 1.0 | 1.0 | 1.05 |  |  |  |  |  |  |
| 40 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 50 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 60 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 70 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 80 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 90 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 100 | 1.0 | 1.0 | 0.85 |  |  |  |  |  |  |
| 200 | 0.75 | 0.65 | 0.35 |  |  |  |  |  |  |
| 300 | 0.55 | 0.5 | 0.35 |  |  |  |  |  |  |
| 400 | 0.4 | 0.4 | 0.4 |  |  |  |  |  |  |
| 500 | 0.35 | 0.35 | 0.45 |  |  |  |  |  |  |
| 600 | 0.35 | 0.35 | 0.45 |  |  |  |  |  |  |
| 700 | 0.35 | 0.35 | 0.45 |  |  |  |  |  |  |
| 800 | 0.35 | 0.35 | 0.45 |  |  |  |  |  |  |
| 900 | 0.35 | 0.35 | 0.45 |  |  |  |  |  |  |
| 1000 | 0.35 | 0.35 | 0.5 |  |  |  |  |  |  |

| multipole ℓ | NUV: map (solid) (G_L / C_L^ALEX, log10) | NUV: gap-fill (dashed) (G_L / C_L^ALEX, log10) | Series 1 (unlabelled) (G_L / C_L^ALEX, log10) | Series 2 (unlabelled) (G_L / C_L^ALEX, log10) |
| --- | --- | --- | --- | --- |
| 10 | 1.0 | 1.0 | 1.0 | 1.05 |
| 20 | 1.0 | 1.0 | 1.0 | 1.0 |
| 30 | 1.0 | 1.0 | 1.0 | 1.0 |
| 40 | 1.0 | 1.0 | 1.0 | 1.0 |
| 50 | 1.0 | 1.0 | 1.0 | 1.0 |
| 60 | 1.0 | 1.0 | 1.0 | 1.0 |
| 70 | 1.0 | 1.0 | 1.0 | 1.0 |
| 80 | 1.0 | 1.0 | 1.0 | 1.0 |
| 90 | 1.0 | 1.0 | 1.0 | 1.0 |
| 100 | 1.0 | 1.0 | 1.0 | 0.95 |
| 200 | 1.0 | 1.0 | 0.6 | 0.6 |
| 300 | 1.0 | 1.0 | 0.6 | 0.75 |
| 400 | 1.0 | 1.0 | 0.6 | 0.8 |
| 500 | 1.0 | 1.0 | 0.6 | 0.8 |
| 600 | 1.0 | 1.0 | 0.6 | 0.8 |
| 700 | 1.0 | 1.0 | 0.6 | 0.8 |
| 800 | 1.0 | 1.0 | 0.6 | 0.8 |
| 900 | 1.0 | 1.0 | 0.6 | 0.8 |
| 1000 | 1.0 | 1.0 | 0.65 | 0.85 |

| multipole ℓ | 20°< | b | <40° (r_f) | 40°< | b | <60° (r_f) |  | b | >60° (r_f) | FUV (r_f) | NUV (r_f) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 10 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 20 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 30 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 40 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 50 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 60 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 70 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 80 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 90 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 100 | 1.0 | 1.0 | 1.0 | 1.0 | 1.0 |  |  |  |  |  |  |
| 200 | 0.85 | 0.75 | 0.65 | 0.70 | 0.60 |  |  |  |  |  |  |
| 300 | 0.70 | 0.60 | 0.55 | 0.60 | 0.50 |  |  |  |  |  |  |
| 400 | 0.60 | 0.55 | 0.55 | 0.55 | 0.50 |  |  |  |  |  |  |
| 500 | 0.55 | 0.52 | 0.55 | 0.52 | 0.50 |  |  |  |  |  |  |
| 600 | 0.52 | 0.50 | 0.55 | 0.50 | 0.50 |  |  |  |  |  |  |
| 700 | 0.50 | 0.50 | 0.55 | 0.50 | 0.50 |  |  |  |  |  |  |
| 800 | 0.50 | 0.50 | 0.55 | 0.50 | 0.50 |  |  |  |  |  |  |
| 900 | 0.50 | 0.50 | 0.55 | 0.50 | 0.50 |  |  |  |  |  |  |
| 1000 | 0.50 | 0.50 | 0.55 | 0.50 | 0.50 |  |  |  |  |  |  |
| 2000 | 0.50 | 0.55 | 0.55 | 0.50 | 0.50 |  |  |  |  |  |  |

Figure 14: Zero point and angular power spectra of the published maps and of the gap-fill layer on identical *GALEX* pixels. (a, b) *P*<sub>30</sub>
intensity in *N* = 128 cells versus *Planck E(B −V*) at *|b|* > 40<sup>◦</sup>, *E(B −V*) < 0.08 (hexbin, destriped *GALEX*) with the least-squares
side
lines of *GALEX,* the published map and the gap-fill layer, FUV and NUV. (c) Intercepts by layer and pixel set (dotted, the *GALEX*
value). (d, e) Ratio of the pseudo-*C*<sup>ℓ</sup>of log<sup>10</sup>*I* to that of destriped *GALEX* for the published map (solid, *≃* 1) and for the gap-fill layer
(dashed) in three *|b|* bands, *N* = 1024, *W* > 0.9 pixels, *1◦*-apodised masks. (f) Cross-correlation coefficient *r* of the gap-fill
side GALEX ℓ
layer with *GALEX.*

$$
E(B-V)
$$

$$
\left|b\right|>40^{\circ}
$$

$$
N_{side}=128
$$

$$
E(B-V)<0
$$

$$
\mathit{GALEX},
$$

$$
C_{\ell}
$$

$$
N_{side}=1024
$$

$$
W_{GALEX}>0.9
$$

$$
r_{\ell}
$$

must know which emission processes each band integrates before comparing the maps with models. This
section summarises the physical content of the two bands
from the literature before placing our measurements in
context. A line of intensity 10<sup>4</sup> photonscm<sup>−2</sup> s<sup>−1</sup> sr<sup>−1</sup>
(LU) inside the *GALEX* FUV band contributes *≃* 25–
40 CU of band-averaged intensity, depending on where
it falls in the response (Morrissey et al. 2007).

$$
10^{4}\mathrm{\stackrel{\circ}{p h o t o n s}\mathrm{cm}^{-2}\mathrm{s}^{-1}\mathrm{sr}^{-1}}
$$

$$
\simeq25-
$$

### 5.1 What the FUV and NUV bands con- tain

*Dust-scattered starlight.* Away from resolved stars the
dominant signal in both bands is the diffuse Galactic
light (DGL), starlight from O and B stars scattered by
interstellar grains. Its intensity scales with dust column
in the optically thin regime and saturates at *τ* ≳ 1–2.
Radiative-transfer analyses of wide-field FUV data give
a moderate albedo and strongly forward-throwing phase
function: *a* = 0.45 *±* 0.05, *g* = 0.68 *±* 0.10 (Witt et al.
1997); *a* = 0.55 *±* 0.10, *g* = 0.75 *±* 0.10 (Schiminovich
et al. 2001); *a* = 0.62 *±* 0.04, *g* = 0.78 *±* 0.05 (Hamden
et al. 2013); 0.3 < *a* < 0.5, *g* < 0.6 (Murthy 2016); and
*a* = 0.4 *±* 0.1, *g* = 0.8 *±* 0.1 (FUV), 0.5 *±* 0.1 (NUV)
(Akshaya et al. 2019), bracketing *a ≃* 0.4, *g ≃* 0.6–0.7
(Weingartner & Draine 2001; Draine 2003b), though
*a* and *g* are degenerate for any single geometry. Two
model-independent facts matter: the albedo minimum
across the 2175Å feature, inside the *GALEX* NUV band,

$$
\tau\gtrsim1-2
$$

$$
a=0.45\pm0.05,\;g=0.68\pm0.10
$$

$$
1997);a=0.55\pm0.10,g=0.75\pm0.10
$$

$$
g=0.78\pm0.05
$$

$$
a=0.4\pm0.1,g=0.8\pm0.1(FUV),0.5\pm0.1
$$

$$
a\simeq0.4,g\simeq0.6-0.7
$$

means scattered light should be at least as blue as the
illuminating field even though A<sub>FUV</sub> ≃ A<sub>NUV</sub> per unit
*E(B −V);* and turbulence enhances scattered intensity
on low-density and suppresses it on high-density sight
lines relative to column (Seon & Witt 2013), limiting
pixel-by-pixel DGL prediction (Section 4.7).

$$
E(B-V)
$$

$$
A_{\mathrm{FUV}}\simeq A_{\mathrm{NUV}}
$$

$$
\mathrm{H_2}
$$

*Molecular-hydrogen fluorescence (FUV only).* Lymanand Werner-band fluorescence of H<sub>2</sub> pumped by 912–
1108Å photons emits a forest of lines between 1350 and
1700Å, within the *GALEX* FUV and FIMS/SPEAR L
bands and absent from the NUV. The FIMS/SPEAR allsky map (Jo et al. 2017) shows fluorescence correlated
with *E(B −* V), reaching *∼* 10 per cent of total FUV
intensity in Taurus–Perseus–Auriga (Lim et al. 2013),
up to 30–50 per cent in the Sandage nebulosity (Sujatha
et al. 2009), and detectable at the Galactic poles (Akshaya et al. 2018): a dust-correlated, FUV-only excess
of a few per cent at high latitude to 10–40 per cent on
illuminated surfaces.

*Atomic and ionic lines (mostly FUV).* FIMS/SPEAR
detects C ivλ1549, Si ivλ1398, Si ii<sup>∗</sup> λ1533, He iiλ1640,
O iii]λ1663 and Al iiλ1671 from the warm and hot
ionised media at high latitude (Welsh et al. 2007; Korpela et al. 2006; Edelstein et al. 2006). Their summed
intensity is of order 10<sup>4</sup> LU, a few tens of CU, or ≲ 10
per cent of the darkest FUV sky, larger towards superbubble walls. The strongest line in raw *GALEX* FUV
data is geocoronal O iλ1356, not interstellar. The NUV
band contains only weak interstellar lines (C iii]λ1909,

$$
10^{4}LU
$$

$$
\mathrm{CU},\mathrm{or}\lesssim10
$$

---

[O ii]λ2470).

*Two-photon continuum.* Every population of the hydrogen *2s* level decays by two-photon emission, peaking at *≃* 1480 Å and extending through the NUV. In
photoionised gas a fraction P<sub>2s</sub> ≃ 0.32 of case-B recombinations passes through 2s, so the continuum scales
with Hα. Integrating the *2s → 1s* distribution of Nussbaumer & Schmutz (1984) with the case-B *Hα* emissivity (Draine 2011) yields 57 CU R<sup>−1</sup> (FUV, 1350–1750Å)
and 34 CU R<sup>−1</sup> (NUV, 1750–2800Å) per Rayleigh of
in-situ *Hα* at *T* = 8000 K, FUV/NUV ratio 1.67. At
the Galactic poles (I<sub>Hα</sub> ≃ 0.4–0.6R; Finkbeiner 2003)
this yields *≃* 25–35 CU in the FUV and 15–20 CU in the
NUV.

$$
P_{2s}\simeq0.32
$$

$$
\mathrm{R}^{-1}
$$

$$
5\overset{\cdot}{7}\operatorname{CU}\operatorname{R}^{-1}
$$

$$
T=8000K
$$

$$
(I_{\mathrm{H}\alpha}\simeq0.4–0.6\mathrm{R};
$$

*Extragalactic background.* The integrated light of
galaxies (IGL) from *GALEX, HST* and ground-based
counts converges to 1.45 *±* 0.27 (FUV) and 3.15 *±*
0.45 nWm<sup>−2</sup> sr<sup>−1</sup> (NUV), i.e. 60–87 and 136–181 CU
(Xu et al. 2005; Driver et al. 2016). The tomographic
measurement of Chiang et al. (2019) gives 89<sup>+28</sup><sub>−16</sub> (FUV)
and 172<sup>+40</sup><sub>−21</sub> CU (NUV), consistent with the IGL, leaving
little room for a diffuse component. In a 1.7<sup>′</sup>-pixel map
most IGL is unresolved, residing in the ‘diffuse’ level.

0 . 45 nWm −2 sr −1 (NUV), i.e. 60–87 and 136–181 CU

$$
1.45\pm0.27
$$

$$
3.15\ \pm
$$

$$
0.45\mathrm{nWm^{-2}sr^{-1}}
$$

$$
89_{-16}^{+28}(FUV)
$$

$$
172_{-21}^{+40}
$$

*The isotropic ‘offset’.* Every *GALEX* and FIMS/
SPEAR analysis finds the DGL regression against dust
extrapolates to *≃* 230–300 CU in the FUV and *≃* 400–
600 CU in the NUV at zero column (Hamden et al. 2013;
Murthy 2016; Akshaya et al. 2018, 2019). Henry et al.
(2015) argue the FUV remainder is a genuine ‘mystery’
component, while Kulkarni (2022) shows two thirds,
arguably all, can be supplied by conventional sources.
Murthy (2016)’s model requires offsets of 100 (FUV)
and 200 CU (NUV) at the poles rising to 200–400 CU
at lower latitude—probably not isotropic.

*Zodiacal light (NUV only).* Sunlight scattered by
interplanetary dust is negligible shortward of 2000Å
but the largest term in the NUV sky. Murthy (2014b)
finds the *GALEX* NUV zodiacal signal proportional to
the Leinert et al. (1998) visible-light distribution with
a UV/optical colour of 0.65, subtracted by UV-BKGD
and hence here. The *≃* 330 CU pole-to-plane amplitude
implies *≃* 140–270 CU subtracted at the poles; a 30 per
cent colour error maps onto ±40–80 CU. The 250 CU
difference between Murthy et al. (2010) and Murthy
(2014a) NUV levels matches this size.

$$
\simeq330\mathrm{CU}
$$

$$
\simeq140–270\mathrm{CU}
$$

*Airglow (both bands, FUV-dominated).* From
low Earth orbit the FUV band receives geocoronal/thermospheric emission (O iλλ1304,1356, N<sub>2</sub>
Lyman–Birge–Hopfield bands, Lyman-*β*-pumped twophoton continuum). In *GALEX* data airglow separates
into a Sun-angle-dependent baseline and a component
depending only on time from local midnight, tens to
*≃* 200 CU per visit in the FUV (Murthy 2014b,a). This
visit-dependent term produces tile striping and is what
the destriping transfers (median 46 CU FUV, 480 CU
NUV; Section 3.2).

$$
N_2
$$

$$
\simeq200\mathrm{CU}
$$

### 5.2 Our measurements in this context

Zero points of the published maps. At N<sub>side</sub> = 256,
*|b|* > 40 ,<sup>◦</sup> *E(B−V*) < 0.08, a linear fit of intensity against *Planck E(B − V*) gives, for the destriped
*GALEX* pixels, total-intensity intercepts of 301 *±* 24
(FUV) and 727 *±* 16 CU (NUV) and diffuse *(P<sub>30</sub>)* intercepts of 264 *±* 24 and 579 *±* 11 CU; the Murthy (2014a)
maps fitted identically give 262 *±* 23 and 573 *±* 11 CU,
i.e. our diffuse level differs by +2/+4 CU. Alternative
estimators move the intercept by 30–40 CU, less than
the inherited systematic. The published map reproduces the *GALEX* intercept to +0.2 *±* 0.2/−3 *±* 3 CU;
*GALEX*-free pixels give 223 *±* 82/557 *±* 65 CU (*P<sub>30</sub>* estimator on 27<sup>′</sup> cells: GALEX 233 ± 12/567 ± 10 CU
with slopes 7.8/5.2kCU mag<sup>−1</sup>, map−GALEX on identical cells +0.4 *±* 0.2/−0.5 *±* 0.5 CU, gap-*fill−GALEX*
+11.9 *±* 0.4/+1.4 *±* 0.7 CU, *GALEX*-free cells 232 *±*
62/552 *±* 43 CU) (Fig. 14a–c). The excess of total over
diffuse intercept, 37±34 (FUV) and 148±19 CU (NUV),
is resolved-source light at 1.7 , consistent with galaxy<sup>′</sup>
counts (Xu et al. 2005; Driver et al. 2016) plus field stars,
less the *P<sub>30</sub>* order-statistic offset. Users comparing with
diffuse-background work should use the published diffuse
prediction or subtract the total-minus-diffuse increment;
no constant has been subtracted from either band.

$$
N_{side}\;=\;256
$$

$$
|b|>40^{\circ},\;E(B-V)<0.08,
$$

$$
301\pm24
$$

$$
727\pm16CU
$$

$$
(\mathcal{P}_{30})
$$

$$
264\pm24
$$

$$
579\pm11CU;
$$

$$
+2/+4CU
$$

$$
573\pm11\mathrm{CU}
$$

$$
+0.2\pm0.2-3\pm3\mathrm{CU};
$$

$$
223\pm82\times557\pm65\mathrm{CU}(P_{30}
$$

$$
27'
$$

$$
233\pm12/567\pm10\mathrm{CU}
$$

$$
+0.4\pm0.2-0.5\pm0.5
$$

$$
+11.9\pm0.4+1.4\pm0.7\mathrm{~CU}
$$

$$
232\pm
$$

$$
62/552\pm43CU)
$$

$$
148\pm19\mathrm{CU}(\mathrm{NUV})
$$

$$
1.7'
$$

$$
\mathcal{P}_{30}
$$

*The isotropic level is inherited, and what it would
contain.* These intercepts are not a measurement of the
extragalactic or ‘offset’ background. Equation (2) ties
the 14<sup>′</sup> diffuse level of every cell to the UV-BKGD
per-visit foreground model, so the monopole of our
maps *is* the monopole of Murthy (2014a) and carries
that model’s airglow and zodiacal-colour systematics
unchanged; we add only the total-minus-diffuse increment and the spatial structure of the residual. In the
FUV (Table 6, Section 5.1), identified terms (IGL 60–87,
two-photon continuum 25–35, interstellar lines 20–40,
faint stars ≲ 10 CU) sum to *≃* 105–170 CU against a diffuse intercept of 230–265 CU, leaving 60–160 CU shared
between DGL, residual airglow, and any extragalactic
term; this is the 120–180 CU ‘unidentified’ component
of Akshaya et al. (2018). In the NUV, identified terms
leave 340–430 CU of the 560–580 CU diffuse intercept
unexplained, as found by Akshaya et al. (2018). The
diffuse FUV/NUV ratio, 264/579 = 0.46, equals that of
the IGL (0.44–0.48) and the tomographic EBL (0.52);
the ratio of the unexplained parts, *≃* 0.15–0.45, is redder
than the IGL and any Galactic diffuse process—only
a zodiacal or extragalactic spectrum is that red. A
NUV foreground residual at 100–200 CU is thus the
most economical reading, and it cannot be settled with
any *GALEX*-based product.

$$
14'
$$

$$
\lesssim10CU)
$$

$$
264/579=0.46
$$

*The structure of the offset is new information.* The
monopole is inherited, but its anisotropy is measured:
after linear *E(B−V*) detrending, the *|b|* > 30<sup>◦</sup> sky
retains a dipole of 0.042 dex (FUV) and 0.038 dex
(NUV) towards *(l,b) ≃* (290 –310<sup>◦ ◦</sup>, −26 ), a Galac-<sup>◦</sup>
tic quadrupole, and the northern hemisphere 6 per
cent fainter than the southern at fixed *E(B −* V). At
◦
*|b|* > 50, *E(B − V*) < 0.03, the FUV diffuse residual
rises by *≃* 130 CU from the ecliptic plane to *|β|* = 70<sup>◦</sup>

$$
E(B-V)
$$

$$
\left|b\right|>30^{\circ}
$$

$$
\left(l,b\right)\simeq\left(290^{\circ}-310^{\circ},-26^{\circ}\right)
$$

$$
|b|>50^{\circ},E(B-V)<0.03
$$

$$
by\simeq130CU
$$

$$
|\beta|=70^{\circ}
$$

---

Table 6: Illustrative budget of the isotropic (zero-dust, *|b|* > 40<sup>◦</sup>) diffuse intensity. Our intercepts are inherited from the Murthy
(2014a) foreground model and carry a common ±50–100 CU additive systematic; component estimates are from the literature cited in
Section 5.1 except the two-photon term (this work, case B, *T* = 8000 K, *IHα=* 0.4–0.6R).

$$
|b|>40^{\circ}
$$

$$
I_{\mathrm{H}\alpha}=0.4-0.6\mathrm{R}
$$

| Term [CU] | FUV | NUV |
| --- | --- | --- |
| Diffuse intercept, this work (P30; estimator range) | 230-265 | 560-580 |
| Total-intensity intercept, this work (Huber) | 291 ± 8 | 742 ± 9 |
| Integrated galaxy light (Xu et al. 2005; Driver et al. 2016) (tomographic EBL, Chiang et al. 2019) | 60-87 (89-16+28) | 136-181 (172-21+40) |
| WIM two-photon continuum (recombination) | 25-35 | 15-20 |
| Interstellar line emission (C IV, Si II*, ...) | 20-40 | < 10 |
| Unresolved Galactic stars below P30 | \(\lesssim 10\) | \(\lesssim 20\) |
| Sum of identified terms | 105-170 | 150-230 |
| Remainder (DGL at zero template column, residual airglow/near-Earth two-photon, zodiacal colour, diffuse EBL) | 60-160 | 340-430 |
| of which dust-template zero level (± 5 mmag) | ∓ 35 | ∓ 25 |
| of which zodiacal model at 30 per cent | — | ± 40-180 |

$$
230–265
$$

$$
(\mathcal{P}_{30};
$$

$$
560–580
$$

$$
291\pm8
$$

$$
742\pm9
$$

$$
(\mathrm{C}.\mathrm{IV},.\mathrm{Si}.\mathrm{II}^*,\ldots)
$$

$$
\mathcal{P}_{30}
$$

$$
\pm40–180
$$

(+1.82± 0.14 CU deg<sup>−1</sup>), with an NUV counterpart consistent with zero (+0.00 ± 0.12 CU deg<sup>−1</sup>). The FUV
pattern appears at +4.1 ± 0.4 CU deg<sup>−1</sup> in the independent FIMS/SPEAR starless map, matching low-order
Galactic harmonics as well as ecliptic latitude and the
polar asymmetries of Akshaya et al. (2018, 2019) and the
Henry et al. (2015) component, pointing to the FUVspecific terms of Section 5.1. Users fitting isotropic
backgrounds to these maps must model this *ℓ ≤* 2 FUV
structure; a template is distributed for characterisation.

$$
\left(+1.82\pm0.14\mathrm{CU}\deg^{-1}\right)
$$

$$
\left(+0.00\pm0.12\mathrm{CU}\deg^{-1}\right)
$$

$$
+4.1\pm0.4\mathrm{CU}\mathrm{deg^{-1}}
$$

$$
\ell\leq2\mathrm{FUV}
$$

*The FUV–Hα slope and two-photon emission.* The
diffuse FUV of *GALEX*-dominated pixels rises with Hα,
but *Hα* and *E(B − V*) are correlated. Regressing the
N<sub>side</sub> = 512 intensity on both gives partial Hα slopes of
◦
86±3 (FUV) and 75±3 CU R<sup>−1</sup> (NUV) at |b| > 25 and
◦
68±3 / 53±3 CU R<sup>−1</sup> at |b| > 40. The recombination
two-photon expectation of 57 (FUV) and 34 CU R<sup>−1</sup>
(NUV) supplies 65–85 per cent of the *Hα*-correlated
FUV and 45–65 per cent of the *Hα*-correlated NUV; the
remainder is plausibly scattered starlight and WIM line
emission. The measured slope is thus largely accounted
for by WIM two-photon emission, without requiring
an additional *Hα*-correlated source. Two corollaries
follow: the sub-linear scaling I<sub>FUV</sub> ∝ I<sub>H</sub><sup>0.α4</sup> (Appendix C)
reflects a transition from dust-dominated to two-photonplus-dust regimes, and *Hα*-bright filled pixels inherit a
physically motivated FUV excess via the *Hα* feature of
the predictor.

86 ± 3 (FUV) and 75 ± 3 CU R −1 (NUV) at |b| > 25 ◦ and

$$
E(B-V)
$$

$$
N_{side}=512
$$

$$
86\pm\bar{3}\;\mathrm{(F U V)}
$$

$$
\left|b\right|>25^{\circ}
$$

$$
\left|b\right|>40^{\circ}
$$

$$
68\pm3\quad53\pm3\mathrm{CUR^{-1}}
$$

$$
34\mathrm{CU}\mathrm{R}^{-1}
$$

$$
I_{\mathrm{FUV}}\propto I_{\mathrm{H}\alpha}^{0.4}
$$

*The FUV/NUV colour–E(B−V*) *relation.* Three
regimes of the FUV/NUV colour–*E(B−V*) relation
map onto the components above. (1) The red plateau,
FUV/NUV = 0.58 at *E(B−V*) < 0.02 (0.46 for
the diffuse intercepts), is the colour of the isotropic
term of Table 6 and carries no information on grains.
(2) The blueing between *E(B−V*) = 0.02 and 0.3
at d log(FUV/NUV)/d logE(B−V) *≃* 0.30 reflects
the growing weight of the DGL, whose photon colour
is set by the dust slope ratio, 7760/4720 = 1.64 at
◦
*|b|* > 40. In the optically thin, single-scattering limit
dI<sub>λ</sub>/dE(B−V) ∝ a<sub>λ</sub> (τ<sub>λ</sub>/E(B−V)) Φ<sub>λ</sub>(g) J<sub>λ</sub>, so the

$$
E(B-V)<0.02
$$

$$
\mathrm{FUV/NUV=~0.58}
$$

$$
\left|b\right|>40^{\circ}
$$

$$
7760/4720=1.64
$$

$$
\mathrm{d}I_{\lambda}/\mathrm{d}E(B-V)\propto a_{\lambda}\left(\tau_{\lambda}/E(B-V)\right)\Phi_{\lambda}(g)J_{\lambda},
$$

DGL colour measures (aΦ)FUV/(aΦ)NUV times the illuminating field colour (1.0–1.2 for the map’s own stellar layers, *≃* 1.8 for the Draine (1978) field), giving
(aΦ)FUV/(aΦ)NUV *≃* 0.9–1.6: equal albedos or a lower
NUV albedo are admitted, as expected if the 2175Å
feature is pure absorption (Draine 2003b), disfavouring
a scattering population markedly more reflective at 2300
than at 1500Å. Since H<sub>2</sub> fluorescence and FUV lines
are dust-correlated and FUV-only, 5–15 per cent of the
FUV dust slope may be fluorescence; the FUV-blue excursions of Upper Sco/Ophiuchus and Orion–Eridanus
are equally consistent with a harder field, enhanced
H<sub>2</sub> fluorescence (Jo et al. 2017; Lim et al. 2013), or
C iv/two-photon emission (Kregenow et al. 2006; Jo
et al. 2012). (3) The turnover at *E(B−V*) = 0.38± 0.06
(τ<sub>FUV</sub> ≃ 2.8) marks saturation of scattered intensity
plus decline of H<sub>2</sub> fluorescence; the higher peak colour
(1.27) than the toy model’s (1.1) is expected near OB
associations dominating the *E(B −V*) *≃* 0.3–1 sky.

$$
(a\Phi)_{\mathrm{FUV}}/(a\Phi)_{\mathrm{NUV}}
$$

$$
(a\Phi)_{\mathrm{FUV}}/(a\Phi)_{\mathrm{NUV}}\simeq0.9-1.6;
$$

$$
\mathrm{H_2}
$$

$$
\mathrm{H_2}
$$

$$
(\tau_{\mathrm{FUV}}\simeq2.8)
$$

$$
E(B-V)=0.38\pm0.06
$$

$$
\mathrm{H_2}
$$

$$
E(B-V)\simeq0.3-1
$$

*Albedo and phase function: what can and cannot
be inferred.* The high-latitude dust slope constrains
only a combination of *a* and *g.* For a thin layer illuminated by the Draine (1978) field, single scattering with a Henyey–Greenstein phase function reproduces S<sub>FUV</sub> = 7760 CU mag<sup>−1</sup> for (a,g) = (0.26,0.6),
(0.39,0.7) or (0.65,0.8). Breaking the degeneracy requires the anisotropy of the radiation field, multiple
scattering and the three-dimensional dust distribution,
via a Monte-Carlo transfer calculation as in Witt et al.
(1997), Murthy (2016) and Akshaya et al. (2019). Such
a fit—embedding the observer in a three-dimensional
*Gaia*-based dust map illuminated by the full-depth stellar source layer with measured or calibrated ultraviolet
fluxes and parallax distances, and fitting the albedo
and phase-function asymmetry in both bands to the
point-source-free *GALEX* sky together with isotropic
and *Hα* terms—is left to future work; here we quote
only model-independent slopes and colours, not grain
constants.

$$
S_{\mathrm{FUV}}=7760\mathrm{CU}\mathrm{mag^{-1}}
$$

$$
(a,g)=(0.26,0.6)
$$

---

### 5.3 Foreground residuals: a UV-blind test and the FUV high-latitude structure

The foreground field *F* removed from the NUV mosaic
follows ecliptic geometry, its median falling from *≃*
650 CU on the ecliptic to *≃* 360 CU at *|β|≃* 60<sup>◦</sup>, with
anomalous airglow tiles superposed; a two-component
model (airglow term plus Leinert et al. 1998 template)
explains 77 per cent of its variance. The FUV term
(46 CU median) instead rises towards the ecliptic poles
by +1.08 ± 0.08 CU deg<sup>−1</sup>.

$$
\simeq
$$

$$
to\simeq360CU
$$

$$
\left|\beta\right|\simeq60^{\circ}
$$

$$
+1.08\pm0.08\mathrm{CU\deg^{-1}}
$$

A test of the residual that does not use the trained
prediction as reference is available and decisive. At
*|b|* > 50<sup>◦</sup>, *E(B−V*) < 0.03 we regress the diffuse
level on *E(B−V*) alone and bin the residual in *|β|*
(Fig. 15). In the NUV the destriped residual slope is
+0.25± 0.12 CU deg<sup>−1</sup> (+0.00± 0.12 with a csc |b| term)
where the raw mosaic gives *−3.2 ±* 0.3: the zodiacal
removal is complete to < 10 CU over 70<sup>◦</sup> of ecliptic
latitude. In the FUV the destriped residual is *not* flat:
it rises by +1.82 ± 0.14 CU deg<sup>−1</sup>, ≃ 130 CU from the
ecliptic to *|β|* = 70<sup>◦</sup>, in both Galactic hemispheres.
Four facts identify this as sky rather than foreground.
(i) The same estimator on the FIMS/SPEAR starless
map gives +4.1 ± 0.4 CU deg<sup>−1</sup>, steeper, and the FIMS/
SPEAR and *GALEX* residual fields correlate at *r* = 0.74.
(ii) The trend is present in the raw mosaic (+2.5 *±* 0.2)
and destriping *reduced* it; a zodiacal residual would
have the opposite sign and be *≥* 10 times larger in the
NUV. (iii) The residual has no dependence on *GALEX*
depth at fixed sky (+0.02 *±* 0.05 CU per CU of pixel
noise). (iv) The published diffuse prediction, which
contains no ultraviolet data, reproduces 81 per cent of
the trend (+1.48 *±* 0.12), so the prediction-referenced
variant of this test returns +0.05 CU deg<sup>−1</sup> and must
not be quoted as a foreground bound. Ecliptic latitude is moreover degenerate with Galactic longitude at
◦
*|b|* > 50 : adding harmonics in *β* and *λ* plus csc *|b|* and
*Hα* reverses the sign of the partial *|β|* slope (−0.9 *±* 0.5
*GALEX, −3.2 ±* 1.2 FIMS/SPEAR). The removed FUV
foreground correlates with *GALEX* depth (−90 *±* 5 CU
per decade of fractional pixel noise) more than with *|β|*
(+1.0 CU deg<sup>−1</sup>), accounting for its polewards rise; the
amplitude matches the FUV-only isotropic-component
variations discussed by Henry et al. (2015) and Akshaya
et al. (2018). We therefore make no correction; the
possible foreground share is ≲ 0.4 CU deg<sup>−1</sup> (≲ 30 CU
◦
over 70) and is listed among the additive systematics
of Section 3.2.

$$
|b|>50^{\circ},\;E(B-V)<0.03
$$

$$
E(B-V)
$$

$$
+0.25\pm0.12\mathrm{C U\:d e g^{-1}(+0.00\pm0.12}
$$

$$
-3.2\pm0.3;
$$

$$
\flat\:<\:10\mathrm{CU}
$$

$$
70^{\circ}
$$

$$
+1.82\pm0.14\mathrm{CU}\deg^{-1},\simeq130\mathrm{CU}
$$

$$
|\beta|=70^{\circ}
$$

$$
+4.1\pm0.4\mathrm{CU}\mathrm{d e g^{-1}}
$$

$$
(+2.5\pm0.2)
$$

$$
\geq10
$$

$$
(+0.02\pm0.05CU
$$

$$
(+1.48\pm0.12)
$$

$$
\left|b\right|>50^{\circ}
$$

$$
\beta
$$

$$
(-0.9\pm0.5
$$

$$
GALEX,-3.2\pm1.2\mathrm{FMS/SPEAR}
$$

$$
(-90\pm5CU
$$

$$
(+1.0\mathrm{CU}\mathrm{deg}^{-1})
$$

$$
70^{\circ})
$$

## 6 Limitations

$$
\lesssim0.4\mathrm{CU\deg^{-1}}\quad(\lesssim30\mathrm{CU})
$$

The fused maps are heterogeneous by construction; the
weight and uncertainty maps are the primary tools for
using them responsibly (Section 3.10). The specific
limitations, each with its measured size, are as follows.

(i) *The FUV inner Galactic plane is an extrapolation constrained only by FIMS/SPEAR and UVOT.* No
*GALEX* FUV pixel exists above *E(B − V*) = 1.6 (8.2

per cent of the filled FUV sky), and 25 per cent of that
sky is farther from *GALEX* than 3.65 . Above 4<sup>◦ ◦</sup> scales
the level is set by FIMS/SPEAR and is 1.4–1.6 times
the UVOT-derived FUV, a 0.2 dex instrument bracket,
half of which is recorded as SIGMA_SYS = 0.10 dex. The
filled sky is brighter in *Hα* and starlight than held-out
sky, inflating error of the 4.5 per cent pure-model FUV
sky to 0.07–0.10 dex. Near *γ<sup>2</sup>* Vel and *δ/β* Sco the
FIMS/SPEAR-calibrated level lies 0.25–0.3 dex above
the unconstrained prediction and 0.5–0.65 dex below the
FIMS/SPEAR starry map.

$$
4^{\circ}
$$

$$
3.65^{\circ}
$$

$$
\gamma^{2}
$$

$$
\delta/\beta
$$

(ii) *The NUV plane is validated over less than 3
per cent of its area.* 26.5 per cent of the NUV sky
is model layer, validated against held-out *GALEX* NUV
to 0.060 *±* 0.001 dex (0.065 dex after correcting for the
Hα/starlight covariate shift), with narrow gaps (median 0.5<sup>◦</sup>). The only imaging test, against *Swift*-UVOT
(map/UVOT = 1.01, 0.14 dex), covers 0.8 per cent of
the sky with a non-random pointing distribution.

$$
\mathrm{H}\alpha/
$$

$$
0.060\pm0.001
$$

$$
0.5^{\circ})
$$

(iii) *Sub-degree diffuse structure in filled sky is reduced
in amplitude* by 13, 27 and 40 per cent at *ℓ ≃* 100, 300
and 700 (Section 4.7). The effective beam of predicted
diffuse structure is ≃ 4<sup>′</sup>, FIMS/SPEAR information
enters only above *≃* 1<sup>◦</sup>, and a residual 20–30 per cent
excess of pixel differences across N<sub>side</sub> = 1024 cell edges
remains. A scale-dependent structure-amplification (debias) correction is available but is not applied to the
published maps.

$$
\ell\simeq100
$$

$$
\simeq\;4^{\prime},
$$

$$
\simeq1^{\circ}
$$

$$
N_{side}=1024
$$

(iv) *Dependence on positional features.* Removing position raises the gap-matched scatter by
+0.017/+0.010 dex, as the model captures the regional
*GALEX* sky level via smooth functions of coordinates
and *≥* 4<sup>◦</sup>-smoothed templates, with error growing with
distance (+0.0066±0.0008 / +0.0048±0.0004 dex deg<sup>−1</sup>).
The UV-blind test of Section 5.3 bounds such residuals
in the NUV but not the FUV beyond ≲ 30 CU.

$$
+0.017/+0.010
$$

$$
\mathit{GALEX}
$$

$$
+0.0048\pm0.0004\mathrm{d}\mathrm{e}x\mathrm{d}\mathrm{e}\mathrm{g}^{-1}
$$

$$
\lesssim30CU
$$

(v) *The adopted predictor is not the best-scoring configuration of its own benchmark.* The un-jittered model
and a four-model ensemble score 3–4 per cent better in
the NUV (−0.0017 *±* 0.0002 and −0.0025 *±* 0.0002 dex);
the single jittered model is adopted for *b* = 0 continuity
and simplicity (Section 4.2). Hyper-parameters were
tuned on the reporting folds, and UVOT, FIMS/SPEAR
and the folds were consulted during development: there
is no blind test.

$$
(-0.0017\pm0.0002
$$

$$
-0.0025\pm0.0002\mathrm{d}\mathrm{e}\mathrm{x});
$$

$$
b=0
$$

(vi) *Stellar content of the filled sky.* Outside ultraviolet imaging every *Gaia* DR3 source to *G* = 16.5 (blue
and hot sources to *G* = 19, plus catalogued white dwarfs
and hot subdwarfs) is individually present, and 99 per
cent of the stellar flux of a median N<sub>side</sub> = 256 cell is
reached at *G* = 17.25/16.5 (FUV/NUV 23.3/21.8AB;
per-cell limits are published); but for 97 per cent (FUV)
/ 88 per cent (NUV) of these stars the flux is a prediction with a per-star uncertainty of 0.16mag (NUV)
to 0.4mag (FUV, hot stars) and ≳ 1 mag for the chromospheric FUV of cool dwarfs, so individual filled-sky
sources are expectations, not measurements, and 8.9
◦
per cent of TD-1 stars at *|b|* < 11 read > 0.3 dex high
in a 6<sup>′</sup> aperture because of predicted neighbours. The
unresolved remainder below the layer limit is 3–6 CU at

$$
N_{side}=256
$$

$$
G=17.25/16.5
$$

$$
\gtrsim1
$$

$$
\left|b\right|<11^{\circ}
$$

$$
6'
$$

---

**FUV: every product rises towards the ecliptic poles**

|  | GALEX raw, pre-destripping: +2.54±0.18 (FUV residual, diffuse − (I_0 + S_E(B−V)) (CU)) | Murphy 2014 UV-BKGD: +1.82±0.14 (FUV residual, diffuse − (I_0 + S_E(B−V)) (CU)) | prediction (no UV data): +1.48±0.12 (FUV residual, diffuse − (I_0 + S_E(B−V)) (CU)) | FIMS/SPEAR starless: +4.08±0.35 (FUV residual, diffuse − (I_0 + S_E(B−V)) (CU)) | GALEX destripped (P30): +1.82±0.14 (FUV residual, diffuse − (I_0 + S_E(B−V)) (CU)) |
| --- | --- | --- | --- | --- | --- |
| 5 | -10 | -10 | -12 | -60 | -15 |
| 10 | -30 | -30 | -35 | -120 | -40 |
| 15 | -45 | -45 | -35 | -100 | -45 |
| 20 | -55 | -55 | -30 | -80 | -50 |
| 25 | -40 | -40 | -20 | -70 | -30 |
| 30 | -25 | -25 | -10 | -70 | -20 |
| 35 | -15 | -15 | 0 | -40 | -10 |
| 40 | -5 | -5 | 10 | -10 | 0 |
| 45 | 10 | 10 | 20 | 20 | 10 |
| 50 | 35 | 35 | 30 | 30 | 30 |
| 55 | 80 | 80 | 40 | 130 | 55 |
| 60 | 90 | 90 | 45 | 120 | 60 |
| 65 | 120 | 120 | 55 | 160 | 70 |

**NUV: zodiacal gradient removed, residual = 0**

|  | GALEX raw, pre-destripping: -3.22±0.25 (NUV residual, CU) | prediction (no UV data): +0.17±0.09 (NUV residual, CU) | Murphy 2014 UV-BKGD: +0.25±0.13 (NUV residual, CU) | GALEX destripped (P30): +0.25±0.12 (NUV residual, CU) |
| --- | --- | --- | --- | --- |
| unlabelled 1 | 175 |  |  |  |
| unlabelled 2 | 175 |  |  |  |
| unlabelled 3 | 112 |  |  |  |
| unlabelled 4 | 55 |  |  |  |
| unlabelled 5 | 22 |  |  |  |
| unlabelled 6 |  |  |  | 15 |
| unlabelled 7 |  |  | 35 | 25 |
| unlabelled 8 |  |  | 38 | 42 |

|  | GALEX destripped, north Gal.: +1.60±0.17 | GALEX destripped, south Gal.: +2.15±0.19 | FIMS/SPEAR starless, north Gal.: +3.65±0.39 | FIMS/SPEAR starless, south Gal.: +5.36±0.93 |
| --- | --- | --- | --- | --- |
| 2 | -5 | -35 | -95 | -35 |
| 4 | -15 | -40 | -130 | -30 |
| 6 | -30 | -45 | -130 | -150 |
| 8 | -40 | -35 | -130 | -150 |
| 10 | -50 | -30 | -130 | -150 |
| 12 | -50 | -30 | -130 | 50 |
| 14 | -45 | -30 | -110 | 70 |
| 16 | -50 | -30 | -110 | -20 |
| 18 | -50 | -30 | -115 | -20 |
| 20 | -50 | -30 | -115 | -20 |
| 22 | -45 | -30 | -115 | -25 |
| 24 | -40 | -30 | -90 | -25 |
| 26 | -35 | -30 | -85 | -25 |
| 28 | -30 | -30 | -90 | -25 |
| 30 | -30 | -30 | -105 | -20 |
| 32 | -30 | -30 | -80 | -15 |
| 34 | -25 | -30 | -60 | -10 |
| 36 | -20 | -30 | -45 | -5 |
| 38 | -15 | -30 | -30 | 0 |
| 40 | -10 | -30 | -20 | 5 |
| 42 | -5 | -30 | -10 | 10 |
| 44 | 0 | -30 | 0 | 15 |
| 46 | 5 | -30 | 10 | 20 |
| 48 | 10 | -30 | 20 | 25 |
| 50 | 15 | -30 | 30 | 30 |
| 52 | 20 | -30 | 40 | 35 |
| 54 | 30 | -30 | 50 | 40 |
| 56 | 40 | -30 | 60 | 45 |
| 58 | 50 | -30 | 70 | 50 |
| 60 | 60 | -30 | 80 | 55 |
| 62 | 65 | -30 | 90 | 55 |
| 64 | 70 | -30 | 100 | 55 |
| 66 | 75 | -30 | 110 | 55 |
| 68 | 80 | -30 | 120 | 55 |
| 70 | 85 | -30 | 130 | 55 |

|  | F FUV: +1.08±0.08 CU deg⁻¹ (r=0.43) | F NUV: -4.52±0.21 CU deg⁻¹ (r=-0.70) |
| --- | --- | --- |
| unlabelled 1 |  | 620 |
| unlabelled 2 |  | 475 |
| unlabelled 3 |  | 400 |
| unlabelled 4 |  | 345 |

|  | GALEX raw, pre-dstriping: -3.22±0.25 (slope, CU deg⁻¹) | prediction (no UV data): +0.17±0.09 (slope, CU deg⁻¹) | Murthy 2014 UV-BKGD: +0.25±0.13 (slope, CU deg⁻¹) | GALEX destripped (P30): +0.25±0.12 (slope, CU deg⁻¹) |
| --- | --- | --- | --- | --- |
| 3 | 172 | -15 |  | 0 |
| 6 | 172 | -10 |  | 3 |
| 8 | 125 | -5 |  | 7 |
| 11 | 112 | -3 |  | 5 |
| 14 | 112 | -3 |  | 5 |
| 17 | 55 | -2 |  | -2 |
| 20 | 30 | -3 |  | -10 |
| 23 | 15 | -3 |  | -18 |
| 26 | 5 | -2 |  | -10 |
| 29 | -5 | -2 |  | -12 |
| 32 | -15 | -1 |  | -15 |
| 35 | -40 | -1 |  | -20 |
| 38 | -45 | -1 |  | -12 |
| 41 | -40 | 0 |  | -8 |
| 44 | -60 | 0 |  | -15 |
| 47 | -65 | 0 |  | -18 |
| 50 | -55 | 0 |  | -5 |
| 53 | -55 | 0 |  | 5 |
| 56 | -60 | 0 |  | 10 |
| 59 | -55 | 0 |  | 18 |
| 62 | -45 | 0 | 30 | 25 |
| 65 | -35 | 5 |  | 35 |

**d. Removed FUV offset F already rises polewards**

|  | F FUV: +1.08±0.08 CU deg⁻¹ (r=0.43) (foreground offset F removed in destripping, CU) | F NUV: -4.52±0.21 CU deg⁻¹ (r=-0.70) (foreground offset F removed in destripping, CU) |
| --- | --- | --- |
|  | 25 | 620 |
|  | 22 | 475 |
|  | 38 | 400 |
|  | 52 | 365 |
|  | 68 | 75 |

Figure 15: A UV-blind foreground-residual test at *|b|* > 50<sup>◦</sup>, *E(B −V*) < 0.03 (*N* = 256 cells): the diffuse intensity minus a linear
side
dust regression *I*<sub>0</sub>+ *SE(B−V),* binned in ecliptic latitude *|β|*, for the destriped *GALEX* FUV and NUV, the raw (foreground-inclusive)
mosaics, the removed Murthy (2014a) foreground itself, the independent FIMS/SPEAR starless FUV map, and the published diffuse
prediction (no ultraviolet input). Slopes with 7.3<sup>◦</sup>block-bootstrap errors are given in the legend. The FUV trend is steeper in
FIMS/SPEAR than in *GALEX* and absent in the NUV: it is celestial.

$$
|b|>50^{\circ},E(B-V)<0.03(N_{\mathrm{side}}=256
$$

$$
I_{0}+S E(B-V)
$$

$$
|\beta|,
$$

$$
7.3^{\circ}
$$

◦
*|b|* > 30 but *≃* 240 CU (up to *∼* 800 CU if extrapolated
to *m* = 25) in the NUV plane, i.e. up to 10–25 per cent
◦
of the NUV diffuse partition at *|b|* < 10 can still be
starlight. Three specific caveats apply. (a) In the most
◦
crowded inner-plane *Swift*-UVOT tiles (*|b|* < 5) the
model layer is 1.47/1.42 times the UVOT-derived intensity; the published map there is UVOT itself and no
edge step results, but the level of the surrounding filled
plane may be 0.06–0.08 dex high if the *Gaia*-based fluxes
rather than the UVM2 photometry are at fault. (b) In
the Magellanic Clouds the Galactic-calibrated stellar
predictions are inconsistent with the *GALEX* surface
photometry: the stellar input is capped, so un-imaged
holes in the LMC bar and SMC core (12.7/10.9deg<sup>2</sup>)
retain a fixed intensity level, 24 per cent of diffusepartition pixels within the discs are negative, and the
Clouds must be masked in any use of the partition.
(c) Very bright cool stars: the hierarchy adopts TD-1
*F<sub>2365</sub>* alone for the NUV, which is within ±0.25 dex of an
unsaturated-*GALEX* calibration for 14 of 18 test stars,
and it had no FUV tier for a few *V* < 2 G/K stars; 56
FUV and 11 NUV stars (among them Arcturus, Dubhe,
Polaris, Pollux, *α* Cen and the B9 star HD 225132 in
a *GALEX* mask hole) therefore received additive completion deposits totalling 32/6.9 photonscm<sup>−2</sup> s<sup>−1</sup> Å<sup>−1</sup>
(FUV/NUV, summed over the patched stars) so that
no BSC *V* < 4.5 star outside imaging reads below half
its expected flux, at the price of *≃* 0.5 dex-uncertain
typical rather than measured cool-star FUV fluxes. In-

$$
\left|b\right|>30^{\circ}
$$

$$
m=25)
$$

$$
\left|b\right|<10^{\circ}
$$

$$
\left(\left|b\right|<5^{\circ}\right)
$$

$$
(12.7/10.9deg^2)
$$

$$
F_{2365}
$$

$$
V<2\ G/K
$$

$$
-2s^{-1}\xrightarrow{\circ}-1
$$

$$
V<4.5
$$

side the footprint the *GALEX* bright-star non-linearity
(NUV ≲ 15) is inherited unflagged, the layer restates
the resolved sources of *GALEX* itself (*κ* = 1.21 aperture
scale), and *G* < 8 stars are over-subtracted by 7–11 per
cent in the diffuse partition.

$$
\left(\kappa=1.21\right.
\left(NUV\lesssim15\right)
$$

*Zero point and foreground.* The absolute level is inherited from Murthy (2014a) with a ±50–100 CU additive
systematic, plus a +13 CU order-statistic term and a
possible ≲ 30 CU ecliptic FUV term, none in the uncertainty maps (Sections 3.2, 5.3); the total-intensity
NUV zero point (727 *±* 16 CU) exceeds the diffuse one
(579 *±* 11 CU); the diffuse NUV level contains a 340–
430 CU unexplained component; and the high-latitude
FUV sky carries *≃* 130 CU of structure not traced by
dust. Intensities are observed, not extinction-corrected.

$$
a\pm50–100\mathrm{CU}
$$

$$
\lesssim30CU
$$

$$
\left(727\pm16CU\right)
$$

$$
\left(579\pm11\mathrm{CU}\right)
$$

$$
\simeq130\mathrm{CU}
$$

(viii) *Residual GALEX artefacts.* Bright-star ghosts
and halos, edge reflections and inconsistent AIS tiles
survive below repair thresholds (0.2 dex, three cells);
light scattered into the field by avoided ultraviolet-bright
stars is modelled around 30 FUV and 74 NUV stars
and 5/49 tiles are discarded (Section 3.3), leaving the
treated tiles within *±3* per cent of their untouched
neighbours in five sectors out of six, but scattered light
around stars below the census thresholds, visit-to-visit
structure finer than a tile gradient, and sub-tile structure
on *Hα*-bright tiles where flattening is withheld (e.g.
6 per cent in the NUV 3<sup>◦</sup> south of Spica), remain;
inside bright-star holes the fill is tied to the rim level
and gradient (Section 3.9) and is continuous with the

$$
3^{\circ}
$$ adjacent observed mosaic to 5–10 per cent per 30<sup>◦</sup> sector,
but the interior is an extrapolation: a genuine dustscattered halo of the avoided star (cf. Murthy 2014a)
rising inside the rim, or residual instrumental scattered
light in the rim tiles that sets the continued gradient,
cannot be distinguished there; the artefact census flags
*|*log(GALEX/prediction)*|* > 0.25 dex in 0.30/0.90 per
cent of covered cells. Edge-reflection glints of *V* ≲ 5
stars are propagated into the FUV by the NUV-informed
layer; one (*ν* And, 0.4deg<sup>2</sup>) was removed, others may
remain.

$$
30^{\circ}
$$

$$
V\lesssim5
$$

(ix) *Colour of filled pixels and the NUV-informed FUV
level.* The FUV/NUV ratio of model-dominated pixels
falls 0.1–0.3 below the *GALEX* locus at *E(B−V)≳0.2*
(Appendix C); in NUV-informed FUV sky (9.7 per cent)
the diffuse colour is the predicted one below 6.9<sup>′</sup>, the
absolute FUV level is uncertain at *∼* 0.1 dex (0.06 dex
median SIGMA) and point sources carry their cataloguepredicted FUV fluxes (Section 3.8); 15–17mag stacks
◦
in that sky still read 1.1–1.2 at *|b|* < 10 from imaged
faint neighbours; FUV boundaries facing that sky step
by −0.02 dex.

$$
E(B-V)\gtrsim0.2
$$

$$
\left|b\right|<10^{\circ}
$$

(x) *Products not provided.* The uncertainty is valid
at N<sub>side</sub> ≤ 512 only and is heavy-tailed (95.4 per cent
at *≃* 2.5u, 99.7 per cent at 7–19u), rests on a mock-gap
calibration for the FIMS/SPEAR term and on a linear
extrapolation beyond 8<sup>◦</sup> from *GALEX* (Section 4.6);
no exposure or photon-noise map is provided, no perprovenance window function beyond the transfer function of Fig. 14, and no dust-template alternative: only
the *Planck* 2013 model has been used, with cosmicinfrared-background leakage (Chiang & Ménard 2019)
entering filled high-latitude sky with positive sign at ≲
few per cent of *E(B−V*) at *E(B−V*) < 0.02. A residual
◦
dipole of 0.04 dex remains at *|b|* > 30 after *E(B −V*)
detrending. The UVOT contribution to the FUV is
capped at weight 0.8 (0.2 per cent of the sky). Ultraviolet inputs span 1972–2017, with variable sources at an
ill-defined mean epoch. Equatorial copies are bilinearly
resampled in logI; stellar photometry should use the
Galactic files.

$$
N_{side}\leq512
$$

$$
\simeq2.5u,99.7
$$

$$
8^{\circ}
$$

$$
E(B-V)
$$

$$
E(B-V)<0.02
$$

$$
\lesssim
$$

$$
\left|b\right|>30^{\circ}
$$

$$
I;
$$

## 7 Data products, code and repro- ducibility

Figures 1–3 show the published maps and Table 7 lists
the principal files by their distributed names; the complete manifest (manifest.json, with SHA-256 checksums, HEALPix metadata and per-file descriptions) and
a browsable landing page accompany them. All maps are
HEALPix NESTED, Galactic, single precision, in CU;
missing data are IEEE NaN in the destriped *GALEX*
layers and the data-only maps; the fused maps have no
missing or non-positive pixel (medians 1120/1359 CU;
all-sky means 10590/8726 CU). Every FITS header
carries BAND, the HEALPix keywords and, for the dataonly and provenance files, SELECT, BADVAL and the class
fractions; the numerical constants of the pipeline (unit
conversions of equation 1, global gains, feather parame-

ters, FIMS/SPEAR constants, jitter, TD-1 cap, the ordered feature list, the uncertainty recipe and its interval
multipliers) are tabulated in header_addendum.json
distributed with the maps and are written as HIERARCH
cards by the published script add_header_keywords.py.
The uncertainty files carry the model description and
the 2.4u/9–13u multipliers as header comments. HiPS
tile trees for the published maps and derived products,
for use in Aladin/ipyaladin, are built by hips_lib.py.

$$
2.4u/9–13u
$$

*Code and reproducibility.* The data set is accompanied by pipeline_code.tar.gz and PIPELINE.md,
which states what can and cannot be re-run from the
distributed material. Two stages are fully scripted and
reproduce the published files from published inputs: the
diffuse predictor (feature construction given the template matrices, cross-validation, training, prediction;
challengerA_gapfill.py) and the fusion (fusion.py,
run as run_fusion_fuv.py for the FUV and run_fusion_nuv.py for the NUV, with the configuration file,
the fusion input archives fusein_FUV.npz/fusein_-
NUV.npz and the FIMS/SPEAR star mask; a re-run
reproduces the published weights bit for bit, Section 4.3).
The stellar layer is scripted end to end: the *Gaia*
harvest (gaia_harvest_code.tar.gz), the two persource predictors with their trained models and inference code, the rendering and partition, and the training of the diffuse predictor (run_cv.py, run_full.py);
the two post-fusion patches (Magellanic holes, bright
cool stars) are published as additive/multiplicative layers with their scripts. The artefact repair is partly
scripted. HiPS ingestion, template regridding, destriping, cross-calibration, the source layer, the NUVinformed layer and the derived products are produced
interactively; for these the algorithms are specified
in this paper and in PIPELINE.md to the level of every constant, and their *outputs* (offset fields, crosscalibration curves, every source layer with its perobject catalogue, I<sub>X</sub>, multipliers) are published, so
every downstream stage can be re-run and every upstream stage checked against its product, but not
regenerated from raw HiPS tiles by a single command. The gap-fill benchmark (gapfill_benchmark_-
nside512.npz; the out-of-fold predictions oof_challengerA_jit_{FUV,NUV}_{total,diffuse}.npy; the
scorer score_benchmark.py) allows any alternative gapfill model to be scored under the rules of Table 5. validation_statistics.json tabulates the headline numbers of this paper with their provenance (Table 11).

$$
I_{\mathrm{X}},
$$

*Licence, citation and archive.* The data products
are distributed under CC-BY-4.0 and the code under
the MIT licence (LICENCE.md); CITATION.cff gives the
citation and ACKNOWLEDGEMENTS.md the acknowledgements inherited from the public input archives: *GALEX*
GR6/7 and the UV-BKGD high-level science product (https://archive.stsci.edu/prepds/uv-bkgd/;
no DOI is registered) from MAST; *GALEX* and *Gaia*
DR3 HiPS from the CDS; *Swift*-UVOT images from
HEASARC *SkyView;* FIMS/SPEAR maps from the
MAST MCCM collection; the *Planck* 2013 dust model
from the Planck Legacy Archive; the *Hα* composite from

---

Table 7: Principal published files, by their distributed names ({b} stands for fuv or nuv). The complete list with checksums is
manifest.json.

| File | Content |
| --- | --- |
| Primary products {b}_total_n2048_gal.fits | Fused, foreground-subtracted total surface brightness, equations (8)-(9), combining the tile-equalised GALEX layer with the model layer (full-depth stellar layer plus diffuse prediction) and, in the FUV, the NUV-informed layer of equation (4); Magellanic-hole and bright-cool-star patches; full sky, processing recorded in HISTORY. Also at Nside = 512 (n512, exact 16:1 mean). Blending weights WGALEX, WNUVX, WUVOT, WFIMS, WMODEL as five columns (WNUVX, WFIMS zero in the NUV); sum unity to 1.2 × 10-7. |
| {b}_weights_n2048_gal.fits | Calibration combining GALEX, model, NUV-informed, UVOT and stellar-layer terms (held-out coverage keywords COVAREA, COVGAPM, COVNUVX). Two columns: SIGMA, the 68 per cent half-width of \(\log_{10} I\), equation (10); SIGMA_SYS, the coherent systematic (0.10 dex FUV where WFIMS > 0 at \|b\| 6; 0.05 dex NUV where WUVOT > 0.5). Header keywords MULT954/MULT997 (2.51/15.2 FUV, 2.45/19.2 NUV; gap-matched MULT954G/MULT997G 2.73/7.0, 2.47/13.2) give the multipliers for 95.4/99.7 per cent intervals; DKNEE, DSLOPE1/2, DBREAK the far-distance extension and TILESYS the un-added tile residual. Neither column includes photon noise or the additive zero-point systematic. |
| {b}_provenance_n2048_gal.fits | Dominant layer code (uint8: 0 GALEX, 1 NUV-informed FUV, 2 UVOT, 3 FIMS/SPEAR-constrained model, 4 model; 61.9/9.7/0.2/23.7/4.5 per cent FUV, 73.2/0/0.2/0/26.6 per cent NUV; keywords CODE0-4, FRAC0-4). |
| {b}_dataonly_n2048_gal.fits, fuv_dataonly_strict_n2048_gal.fits | Fused map where WGALEX + WNUVX + WUVOT ≥ 0.5, NaN elsewhere (72.0/73.4 per cent kept; header SELECT). The FUV file includes the 9.7 per cent of sky whose FUV level and colour are transferred from NUV imaging; the strict file keeps WGALEX + WUVOT ≥ 0.5 only (61.6 per cent). |
| tile_offset_map_{b}_n2048_gal.fits, tile_offsets_{b}_{parquet, galex_{b}_{equalised_n2048_gal.fits, scattered_light_model_b}_{n2048_gal.fits, scattered_light_census_tiles.csv}_{b}_{model_prediction_n2048_gal.fits, hole_anchor_b}_{n2048_gal.fits, nuvx_fuv_from_nuv_n2048_gal.fits, nuvx_colour_log10_fuvdiff_over_nuvdiff_n1024_gal.fits, crosscal_json, gapfill_benchmark_nside512.npz, y_targets_nside512.npz_galex_{b}_{destriped_n2048_gal.fits_stellar_and_source_layer_and_diffuse_partition_stars_all_{b}_{n2048_gal.fits_stars_sigma_{b}_{n2048_gal.fits_stars_count_n2048_gal.fits_extragalactic_points_{b}_{n2048_gal.fits_diffuse_negfrac_n256_gal.fits_unresolved_remainder_n64_gal.fits_stellar_layer_aux_n256_gal.fits_stellar_layer_, parquet (27 files), stellar_layer_catalogue_manifest_json | Per-tile GALEX background model O (CU; zero outside the footprint; includes the scattered-light model), the per-tile table (offset, rim, gradient, chord, flags, scattered-light terms, interior residuals), the bright-star scattered-light model T(E + ct + ...) on its own, the census and the corrected destriped GALEX layer g on which the fusion is based (Section 3.3). |
| {b}_model_prediction_n2048_gal.fits, nuvx_fuv_from_nuv_n2048_gal.fits_stars_sigma_{b}_{n2048_gal.fits_stars_count_n2048_gal.fits_extragalactic_points_{b}_{n2048_gal.fits_diffuse_negfrac_n256_gal.fits_unresolved_remainder_n64_gal.fits_stellar_layer_aux_n256_gal.fits_stellar_layer_, parquet (27 files), stellar_layer_catalogue_manifest_json | Component layers used in the fusion: the gap-fill layer prediction (rim-anchored) and its hole-anchor field C (EXTNAME = GAPFILL; the full-depth stellar, extragalactic and galaxy layers plus the diffuse prediction), the NUV-informed FUV layer and its colour model, the UVOT crosscalibration, and the gap-fill benchmark on the tile-equalised targets (out-of-fold predictions; scorer score_benchmark.py). |
| {b}_destriped_n2048_gal.fits_stars_all_{b}_{n2048_gal.fits_stars_square_g_{b}_{n2048_gal.fits_stars_count_n2048_gal.fits_diffuse_negfrac_n256_gal.fits_unresolved_remainder_n64_gal.fits_stellar_layer_aux_n256_gal.fits_stellar_layer_, parquet (27 files), stellar_layer_catalogue_manifest_json | Destriped GALEX GR6/7 layer with TD-1 top-up (gc); NaN outside the footprint (BADVAL). (Section 3.5) Full-depth stellar layer (118 932 137 sources at catalogue scale, CU), its propagated 1σ, and source counts per pixel; stars_mapmatched_{b} is the κ-scaled variant subtracted in the diffuse partition. The 160 099 Gaia quasar/galaxy candidates, and the diffuse partition (fused map minus the κ-scaled map-matched stars, extragalactic points and Lgal inside imaging, minus the fusion's deposited source layer outside imaging, weighted by w = WGALEX + WNUVX + WUVOT; equal to the diffuse prediction outside imaging); pixels below -2κσstars masked (0.25/1.42 per cent per cell fractions in the negfrac file), as are the Magellanic discs. |
| {b}_ulster_n2048_gal.fits_stellar_layer_catalogue_manifest_json | Extrapolated unresolved stellar surface brightness below the layer limit (SB_MISSING, L0/H1, L025); per-cell counts, Gg9/G90 completeness magnitudes and negative-pixel fractions. |
| {b}_ulster_n2048_gal.fits_stellar_layer_catalogue_manifest_json | Per-source catalogue: Gaia source_id, position, adopted FUV/NUV flux and σ, fuv-origin/nuv origin tier code, population, flags; predictor tables uvpred_ms_gaia_*.parquet, uvpred_hot_predictions_parquet with models and inference code (uvpred_ms_model.pk1, uvpred_hot_models.pk1, uvpred_ms_hot_infer.py, stellar_uv_model.py). |
| {b}_ulster_n2048_gal.fits_stellar_layer_catalogue_manifest_json | Additive bright-cool-star completion patch of Section 6 (56/11 stars) and its 5001-row candidate table with expectations, origins and deposits. Diffuse-predictor target, out-of-fold predictions, fitted models and cross-validation scores (Section 3.7). |

$$
W_{GALEX}
$$

$$
N_{side}=512(\_n512\_
$$

$$
W_{\operatorname{FIMS}}
$$

$$
W_{\operatorname{MODEL}}
$$

$$
1.2\times10^{-7}.
$$

$$
(W_{\operatorname{NUVX}}
$$

$$
W_{\operatorname{UVOT}}>0.5)
$$

$$
W_{FIMS}>0
$$

$$
T(E+c_{t}+\ldots)
$$

$$
G a i a
$$

$$
L_{goal}
$$

$$
w=\stackrel{\mathrm{g}\mathrm{m}}{W}_{\mathrm{GALEX}}+\stackrel{\mathrm{G}}{W}_{\mathrm{NUVX}}+W_{\mathrm{UVOT}};
$$

$$
-2\kappa\sigma_{stars}
$$

$$
(0.25/1.42
$$

$$
G_{99}/G_{90}
$$

---

Table 7: – *continued:* component layers, benchmark, code and documentation.

| File | Content |
| --- | --- |
| Component layers and auxiliary maps galex_nuv_informed_fuv_n2048_gal.fits | The NUV-informed FUV layer IX of equation (3), with top-up, cold-pixel repair and glint removal. Artefact-repair multiplier MA (1 = GALEX untouched, 0 = replaced); {b_-qualityflags_n512_gal.fits, artefact-census flags. Lgal, the galaxy/globular-cluster layer, with galgc_layer_catalogue.csv. Additive foreground field F of equation (2) at its native Nside = 256, with the UV-BKGD coverage (inpainting) flag. |
| galex_artefact_repair_mult_{b}_n2048_gal.fits | The FIMS/SPEAR starless input and exposure map, and the bright-star exclusion mask of equation (7). |
| galgc_layer_{b}_n2048_gal.fits | UVOT rate maps and the isotonic cross-calibration curves. |
| foreground_offset_{b}_n256_gal.fits | Raw GALEX mosaics, regridded UV-BKGD reference and predictor templates as used. |
| fims_fuv_starless_n2048_gal.fits, fims_exposure_n2048_gal.fits, fims_ratio_starmask.npy | UVOT rate maps and the isotonic cross-calibration curves. |
| uvot_{uvw2,uvm2,uvw1}_{rate,coverage}_n2048_gal.fits, uvot_galex_crosscal.json | Raw GALEX mosaics, regridded UV-BKGD reference and predictor templates as used. |
| galex_{b}_n{2048,4096}_gal.fits, murthy_{b}_diffuse_n2048_gal.fits, ebv_planck2013_n2048_gal.fits, templates_gaia_halpha_n2048.npz | Raw GALEX mosaics, regridded UV-BKGD reference and predictor templates as used. |
| Benchmark, code and documentation |  |
| gapfill_benchmark_nside512.npz, oof_-challengerA_jit_{fUV,NUV}_{total,diffuse}_npy, score_benchmark.py, challengerA_gapfill.py | The gap-fill benchmark of Appendix A: features, fold assignment, targets, gap-matching weights; out-of-fold predictions of the diffuse predictor; the scorer; the predictor code. Fusion code (FUV/NUV entry points) and its configuration. |
| fusion.py, run_fusion_fuv.py, run_fusion_nuv.py, config/ |  |
| pipeline_code.tar.gz, PIPELINE.md | All project scripts and the stage-by-stage pipeline specification with its scripted/not-scripted statement. Pipeline constants as keyword tables and the script that writes them as HIERARCH cards. |
| header_addendum.json, add_header_keywords.py | Column-level data model with an executable worked example; user guide; manifest; headline numbers with provenance; citation; licences; inherited acknowledgements; landing page. |
| DATAMODEL.md, README.md, manifest.json, validation_statistics.json, CITATION.css, LICENCE.md, ACKNOWLEDGEMENTS.md, index.html |  |

$$
I_{\mathrm{X}}
$$

$$
M_{\mathrm{A}}
$$

$$
L_{\mathrm{gal}}
$$

$$
N_{side}=256,
$$

LAMBDA; HI4PI, TD-1, Bright Star Catalogue, GU-
Vcat and galaxy and globular-cluster catalogues (Bai
et al. 2015; Gil de Paz et al. 2007; de Vaucouleurs
et al. 1991; Harris 2010; Dalessandro et al. 2012) from
VizieR; *Gaia* DR3 photometry from the *Gaia* archive;
and z0MGS from IRSA.

## 8 Summary

This work provides HEALPix N<sub>side</sub> = 2048 maps of the
total FUV and NUV surface brightness over the full
sky, built from a destriped and tile-equalised *GALEX*
GR6/7 layer whose foreground and zero point are inherited from Murthy (2014a), an NUV-informed FUV
layer, *Swift*-UVOT, a FIMS/SPEAR large-scale constraint, a full-depth Gaia DR3 stellar layer of 1.19×10<sup>8</sup>
sources with measured (TD-1, GUVcat) or predicted
ultraviolet fluxes and nearby galaxies, and—where no ultraviolet imaging exists—a 90-feature gradient-boosted
prediction of the diffuse light from dust, Hα, starlight
and position, trained on the star-subtracted *GALEX*
sky, harmonised to the data and feathered into it with
published per-pixel weights; both bands are also published partitioned into resolved sources and diffuse light.
Direct *GALEX* imaging sets 61.9/73.2 per cent of the
FUV/NUV sky and is reproduced to < 0.003 dex over 90
per cent of the footprint; predicted sub-degree morphology covers 28.0/26.5 per cent. The gap fill is accurate
to 0.052/0.058 dex (FUV/NUV, prediction plus stars)
and 0.047/0.057 dex (diffuse partition) per 7<sup>′</sup> in gapmatched block cross-validation against the tile-equalised
*GALEX* sky, degrading to 0.07–0.10 dex several degrees

$$
N_{side}=2048
$$

$$
1.19\times10^{8}
$$

$$
7'
$$

from data and in the pure-model FUV sky; blind mock
gaps through the fusion are recovered to 0.05 dex. In the
*GALEX*-free plane the maps agree with FIMS/SPEAR
(0.90, 0.19 dex) and UVOT (1.01, 0.14 dex). In filled
sky stars are individually present to *G ≃* 17 and isolated UVOT sources are half recovered at UVM2 = 19.3.
The published two-column uncertainty is empirical in
every pixel class and calibrated to 0.68–0.71 coverage,
with heavy-tail multipliers of 2.3–*2.4u* and 9–13u. The
isotropic level (264/579 CU) is inherited, not measured;
its FUV–*Hα* slope is largely two-photon emission, its
colour–*E(B −V*) relation reflects the changing mix of
a red isotropic term and blue scattered light, and a
130 CU FUV-only high-latitude structure shared by
FIMS/SPEAR is celestial. The principal limitations are
the weakly constrained FUV inner plane, the sparsely
tested NUV plane, the 13–40 per cent amplitude deficit
of sub-degree diffuse structure in predicted sky, the
positional-feature dependence, the inherited zero point,
the predicted (not measured) fluxes of most filled-sky
stars, a heavy-tailed uncertainty whose FIMS/SPEAR
term rests on mock-gap experiments, and the Magellanic Clouds, where the stellar layer is capped and the
partition invalid.

These maps are likely to remain the only full-sky FUVplus-NUV surface-brightness product until the deeper
full-sky UVEX survey (Kulkarni et al. 2021) becomes
available; until then they supply the background, confusion and stellar environment needed by SPHEREx (Doré
et al. 2014; Crill et al. 2020), ULTRASAT (Shvartzvald
et al. 2024), the CSST survey camera (Zhan 2021),
UVIT (Tandon et al. 2017), CASTOR (Côté et al. 2019)

---

and UVEX planning itself. The scientifically richest
interim uses are those needing *4π* rather than depth
(Appendix D): full-sky radiative-transfer fits of the DGL
with *Gaia*-based three-dimensional dust, a directionresolved empirical FUV radiation field, the anisotropy
of the high-latitude offset, and clustering-redshift tomography over two to three times the sky now usable
for such analyses. In every application the weight and
uncertainty maps should be consulted first.

## Acknowledgements

This work is based on observations made with the NASA
*Galaxy Evolution Explorer; GALEX* was operated for
NASA by the California Institute of Technology under
NASA contract NAS5-98034, and its data were obtained
from the Mikulski Archive for Space Telescopes (MAST)
at the Space Telescope Science Institute, operated by the
Association of Universities for Research in Astronomy,
Inc., under NASA contract NAS5-26555; the UV-BKGD
and FIMS-SPEAR high-level science products were likewise obtained from MAST. We acknowledge the use of
public data from the *Swift* data archive and of NASA’s
HEASARC; we acknowledge the use of NASA’s *SkyView*
facility (http://skyview.gsfc.nasa.gov) located at
NASA Goddard Space Flight Center. This research
made use of the HiPS and MOC services (*hips2fits,*
MocServer, Aladin) and of the VizieR catalogue access tool (DOI: 10.26093/cds/vizier) of the Centre de
Données astronomiques de Strasbourg (Fernique et al.
2015; Boch & Fernique 2014), of ESASky, developed
by the ESAC Science Data Centre, of the NASA/IPAC
Infrared Science Archive, which is funded by NASA and
operated by the California Institute of Technology, and
of the HyperLEDA database. Based on observations
obtained with *Planck* (http://www.esa.int/Planck),
an ESA science mission with instruments and contributions directly funded by ESA Member States, NASA
and Canada. This work has made use of data from
the European Space Agency mission *Gaia* (https:
//www.cosmos.esa.int/gaia), processed by the *Gaia*
Data Processing and Analysis Consortium (DPAC);
funding for the DPAC has been provided by national
institutions, in particular those participating in the *Gaia*
Multilateral Agreement. HI4PI is based on observations
with the 100-m telescope of the MPIfR at Effelsberg
and the Parkes Radio Telescope, part of the Australia
Telescope National Facility funded by the Commonwealth of Australia for operation as a National Facility
managed by CSIRO. We acknowledge the use of the
Legacy Archive for Microwave Background Data Analysis (LAMBDA), part of the High Energy Astrophysics
Science Archive Center (HEASARC), a service of the
Astrophysics Science Division at NASA Goddard Space
Flight Center. The Wisconsin H-Alpha Mapper, the
Virginia Tech Spectral-Line Survey and the Southern
H-Alpha Sky Survey Atlas are supported by the National Science Foundation. FIMS/SPEAR was a joint
project of the Korea Astronomy and Space Science Institute, the Korea Advanced Institute of Science and

Technology and the University of California, Berkeley,
supported by the Korean Ministry of Science and Technology and NASA. This research made use of healpy
and HEALPix (Górski et al. 2005; Zonca et al. 2019),
NaMaster (Alonso et al. 2019), numpy (Harris et al.
2020), scipy (Virtanen et al. 2020), astropy (Astropy
Collaboration 2022), matplotlib (Hunter 2007) and
scikit-learn (Pedregosa et al. 2011).

## Data availability

The maps, code bundle, benchmark and documentation
described in Section 7 are available at menard.pha.jhu.
edu/uvmap. All inputs are public (Section 7).

---

## A The gap-fill benchmark, base- lines and feature ablation

*Benchmark definition.* The published benchmark (gapfill_benchmark_nside512.npz) contains,
at N<sub>side</sub> = 512: the feature matrix (the twelve base features plus, for reference, the withdrawn FIMS/SPEAR
and *AKARI* columns; the adopted predictor uses the
columns listed in clean_cols and derives its remaining
78 features from the published templates with challengerA_gapfill.py); the fold assignment (six folds,
unions of N<sub>side</sub> = 8 super-pixels assigned by a fixed
pseudo-random permutation, 151/121/122/120/130/124
super-pixels); the total and diffuse targets (destriped
GALEX at N<sub>side</sub> = 512; NaN where no GALEX); the
importance weights weight_matched_{FUV,NUV} that
match held-out pixels to the (*E(B − V), |b|*) distribution of the filled sky in 10 *×* 6 cells *(E(B −V)* edges 0,
0.02, 0.035, 0.06, 0.1, 0.2, 0.4, 0.8, 1.6, 3.2; *|b|* edges 0,
10, 20, 30, 45, 60, 90<sup>◦</sup>; cells with fewer than 50 heldout pixels unused; the FUV target population is the
no-*GALEX* mask, the NUV target the *W*<sub>MODEL</sub>> 0.5
sky at N<sub>side</sub> = 2048); E(B −V), |b|, l; and the out-offold predictions of the adopted predictor (oof_challengerA_jit_*). The scorer computes the weighted
robust scatter as 1.4826 times the weighted median absolute deviation of log<sub>10</sub>(pred/G<sup>′</sup>), the weighted median
taking the lower sample at ties (which shifts the matched
scatter by ≃ 10<sup>−4</sup> dex relative to an interpolating quantile). Table 5 in the main text gives all scores; Table 8
here gives a single-fold feature ablation.

$$
\widehat{N_{side}=512};
$$

$$
N_{side}=8
$$

$$
\mathit{GALEX}
$$

$$
N_{side}=512
$$

$$
10\times6
$$

$$
W_{MODEL}>0.5
$$

$$
N_{\underline{side}}=2048)
$$

$$
A_{-}jit_{-}*)
$$

$$
\log_{10}(pred/G')
$$

$$
\simeq10^{-4}
$$

*Feature ablation.* Table 8 retrains the adopted feature set on held-out fold 0. Permutation and retrainablation importances agree that *Planck* dust features
dominate the FUV prediction and *Gaia* starlight the
NUV, that position is the second most valuable group
(+0.017/+0.010 dex gap-matched scatter when removed
from the published set), and that *Hα* is worth *≃*
0.002 dex.

$$
(+0.017/+0.010
$$

$$
\simeq
$$

## B Statistical details and sources of the headline numbers

*Block bootstrap and paired comparisons.* Residuals are
spatially correlated on *≥* 1<sup>◦</sup> scales and the FUV importance weights concentrate on few super-pixels, so
pixel-level bootstrap errors (±0.0003/±0.0001 dex) understate the uncertainty of the matched scatter by a
factor of 6–8. Resampling the 708 (FUV) and 765
(NUV) N<sub>side</sub> = 8 super-pixels that carry held-out data
(1000 resamples) gives ±0.0023 and ±0.0009 dex (95 per
cent intervals 0.055–0.064 and 0.058–0.062); resampling
N<sub>side</sub> = 16 super-pixels ±0.0018/±0.0006; the fold-tofold range is 0.056–0.061 and 0.054–0.062 dex. Paired
differences between models scored on identical pixels
with the same resamples are determined to ±0.0015–
0.002 (FUV) and ±0.0002–0.0005 dex (NUV); single-fold
ablation differences below *≃* 0.004 (FUV) and 0.002 dex
(NUV), including the individual rows of Table 8 and

$$
\geq1^{\circ}
$$

$$
\left(\pm0.0003/\pm0.0001\mathrm{dex}\right)
$$

$$
6{-}8.
$$

$$
N_{side}=8
$$

$$
\pm0.0023
$$

$$
N _ {\mathrm {s i d e}} = 1 6 \text {s u p e r - p i x e l s} \pm 0. 0 0 1 8 / \pm 0. 0 0 0 6;
$$

$$
0.056--0.061
$$

$$
\pm0.0015-
$$

Table 8: Retrain ablation of the adopted feature set on heldout fold 0 (gap-matched robust scatter, dex; single fold; paired
differences below *≃* 0.004 dex FUV and 0.002 dex NUV are not
significant, Appendix B). ‘Base’ is the twelve-feature set of the
earlier predictor; ‘full’ the 89-feature development set including *AKARI;* the published 90-feature set equals *‘full−AKARI’*
plus the 16<sup>◦</sup>scale and the secondary-scale block, and scores
0.0597/0.0597 (FUV/NUV) on the same fold, 0.0575/0.0589 with
the importance-based sample weights.

$$
\mathit{AKARI};
$$

$$
\mathrm{full}-\mathrm{AKARI}
$$

$$
16^{\circ}
$$

| Feature set | nfeat | FUV | NUV |
| --- | --- | --- | --- |
| base | 12 | 0.0641 | 0.0616 |
| base + multi-scale | 61 | 0.0637 | 0.0611 |
| base + ecliptic | 15 | 0.0645 | 0.0611 |
| base + ratios | 14 | 0.0634 | 0.0624 |
| base + AKARI | 35 | 0.0676 | 0.0632 |
| full | 89 | 0.0611 | 0.0606 |
| full - AKARI | 66 | 0.0612 | 0.0606 |
| full - multi-scale | 40 | 0.0672 | 0.0620 |
| full - ecliptic | 86 | 0.0614 | 0.0614 |
| full - position | 82 | 0.0781 | 0.0706 |

$$
n_{\mathrm{feat}}
$$

Table 9: Notation.

| Symbol | Meaning |
| --- | --- |
| G, G' | raw and destriped GALEX intensity (Nside = 2048) |
| F | additive foreground offset field, equation (2) |
| M | UV-BKGD diffuse intensity (Murthy 2014a) |
| \(\mathcal{P}_{30}\) | 30th percentile of the Nside = 2048 children of a cell (diffuse proxy) |
| PD, P | diffuse prediction; complete model of the total used for the gain |
| \(g, g_j, ζ_j, \vartheta_j\) | gain field, its levels, mixing coefficients and smoothing scales, equation (5) |
| \(\mathcal{M}\) | model layer, equation (6) |
| IX, c | NUV-informed FUV layer and predicted colour, equation (3) |
| IF, CF, ηF, t(d) | FIMS/SPEAR-constrained layer, ratio field, support and distance tapers, equation (7) |
| \(w_k \equiv W_k, f_k, c_k, m_k, θ_k, ρ_{0,1}\) | weights, feather, confidence, mask, feather scale and thresholds, equation (9) |
| MA | artefact-repair multiplier |
| u, σk, S(d, E(B-V)) | published SIGMA, per-layer terms and the FIMS/SPEAR mock-gap surface, equation (10) |
| σ (benchmark) | robust scatter, 1.4826 × weighted MAD of \(\log_{10}(pred/G')\) |

$$
\bar{G},G'
$$

$$
F
$$

$$
M
$$

$$
\mathcal{P}_{30}
$$

$$
(N_{side}=2048)
$$

$$
P_{D},P
$$

$$
N_{side}=2048
$$

$$
g,\;g_{j},\;\zeta_{j},\;\vartheta_{j}
$$

$$
I_{\mathrm{X}},c
$$

$$
I_{\mathrm{F}},C_{\mathrm{F}},\eta_{\mathrm{F}},t(d)
$$

$$
w_{k}\equiv W_{k},f_{k},c_{k},
$$

$$
m_{k},\theta_{k},\rho_{0,1}
$$

$$
M_{\mathrm{A}}
$$

$$
u,\;\sigma_{k},
$$

$$
S(d,E(B-V))
$$

$$
\sigma (\mathrm {b e n c h m a r k})
$$

$$
\log_{10}(\operatorname{pred}/G^{\prime})
$$

---

**a. Cumulative feature ladder**

| Category | FUV, all sky (R² of log10 intensity, test fold) | NUV, all sky (R² of log10 intensity, test fold) | FUV, | b | <20° (R² of log10 intensity, test fold) | NUV, | b | <20° (R² of log10 intensity, test fold) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| position only | 0.847 | 0.737 | 0.647 | 0.571 |  |  |  |  |
| + Planck dust | 0.871 | 0.738 | 0.684 | 0.585 |  |  |  |  |
| + Gaia | 0.912 | 0.913 | 0.810 | 0.832 |  |  |  |  |
| + Hα | 0.916 | 0.916 | 0.821 | 0.840 |  |  |  |  |
| + AKARI | 0.916 | 0.916 | 0.816 | 0.840 |  |  |  |  |
| + FIMS | 0.920 | 0.918 | 0.829 | 0.846 |  |  |  |  |

**b. Learning curve, all 16 features**

| Category | Series 1 (unlabelled) (R² of log10 intensity, test fold) | Series 2 (unlabelled) (R² of log10 intensity, test fold) |
| --- | --- | --- |
| unlabelled 1 | 0.887 | 0.902 |
| unlabelled 2 | 0.911 | 0.914 |
| unlabelled 3 | 0.921 | 0.920 |
| unlabelled 4 | 0.924 | 0.923 |

**Leave one feature group out (stem = loss vs. full model)**

| Category | Series 1 (unlabelled) | Series 2 (unlabelled) | Series 3 (unlabelled) | Series 4 (unlabelled) |
| --- | --- | --- | --- | --- |
| position | 0.75 | 0.75 | 0.47 | 0.42 |
| Planck dust | 0.75 | 0.75 | 0.55 | 0.57 |
| Gaia | 0.60 | 0.35 | 0.20 | 0.16 |
| Hα | 0.75 | 0.75 | 0.55 | 0.57 |
| AKARI | 0.75 | 0.75 | 0.55 | 0.57 |
| FIMS | 0.75 | 0.75 | 0.55 | 0.57 |

**c. Boosting vs. simpler predictors (numbers: R²)**

| Category | FUV (R²) | NUV (R²) |
| --- | --- | --- |
| unlabelled 1 | 0.75 | 0.61 |
| unlabelled 2 | 0.82 | 0.81 |
| unlabelled 3 | 0.88 | 0.89 |

| Training pixels (inside 512, folds 1-5) | FUV, all sky (R² of log10 intensity, test fold) | NUV, all sky (R² of log10 intensity, test fold) | FUV, | b | <20° (R² of log10 intensity, test fold) | NUV, | b | <20° (R² of log10 intensity, test fold) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 5k | 0.887 | 0.901 | 0.745 | 0.786 |  |  |  |  |
| 20k | 0.909 | 0.912 | 0.805 | 0.819 |  |  |  |  |
| 80k | 0.919 | 0.918 | 0.822 | 0.836 |  |  |  |  |
| 250k | 0.922 | 0.921 | 0.829 | 0.845 |  |  |  |  |

| Category | FUV (R²) | NUV (R²) |  |  |
| --- | --- | --- | --- | --- |
| E(B−V)× | b | lookup | 0.75 | 0.61 |
| ridge (linear) | 0.82 | 0.81 |  |  |
| ridge (quadratic) | 0.88 | 0.89 |  |  |
| gradient boosting | 0.92 | 0.92 |  |  |

Figure 16: Feature ablation on the held-out spatial fold (7.3◦blocks; 4.0 × 10<sup>5</sup> FUV and 4.9 × 10<sup>5</sup> NUV test pixels at N = 512,
side
2.5×10<sub>5</sub>training pixels), using the sixteen-feature development model so that the withdrawn groups can be assessed. (a) Left: cumulative
feature ladder, *R*<sup>2</sup>(log *I*) on the test fold as groups are added in the order position, *Planck* dust, *Gaia,* Hα, *AKARI,* FIMS/SPEAR,
10
for all sky (filled) and *|b|* < 20<sub>◦</sub>(open); right: leave-one-group-out *R*<sub>2</sub>(stems show the loss relative to the full model). (b) Learning
curve: test-fold *R*<sup>2</sup>versus number of training pixels. (c) Median absolute log residual of the gradient-boosted model compared with an
*E(B −V*) *×|b|* median lookup table, linear ridge and quadratic ridge regression on the same features; numbers above the bars are *R*<sup>2</sup>.

$$
(7.3^{\circ}
$$

$$
4.0\times10^{5}
$$

$$
4.9\times10^{5}NUV
$$

$$
2.5\times10^{5}
$$

$$
N_{side}=512,
$$

$$
R^{2}(\log_{10}^{'}I)
$$

$$
\left|b\right|<20^{\circ}
$$

$$
R^{2}
$$

$$
R^{2}
$$

$$
E(B-V)\times|b|
$$

$$
R^{2}
$$

statements such as ‘ecliptic and ratio features are worth
*≤* 0.001 dex each’, are not significant. Hyper-parameters
were tuned on the reporting folds; the tuning gain available within the fold-to-fold standard deviation is *≃* 0.002
(FUV) and 0.003 dex (NUV), which bounds the resulting
optimism.

$$
\leq0.001\text{dex each}
$$

*Importance weights and covariate shift.* Kish effective
sample sizes, weight concentration, the sensitivity of
the matched scatter to the cell definition (±0.0015 dex
FUV, up to +0.002 dex NUV) and the classifier twosample test (HGB classifier on eight non-positional templates, 200k versus 200k pixels, two-fold cross-fitted with
the split on N<sub>side</sub> = 16 super-pixels; AUC 0.83/0.66;
single-feature AUCs logI<sub>Hα</sub> 0.64, logF<sub>BP</sub> 0.63, sinb
0.63, logn<sub>⋆</sub> 0.58, E(B −V) 0.54 in the FUV) are those
quoted in Section 4.2. Between 15 and 49 per cent of
model-filled FUV pixels have a classifier score above
the 99th percentile of held-out scores, i.e. essentially no
analogue; the three shift-inflation estimators (top-decile
scatter, density-ratio re-weighting with weights capped
at the 99th percentile, linear extrapolation of *|r|* on
score) give 0.065–0.086, 0.097± 0.020 (effective sample 2
per cent) and +2 to +6 per cent for the FUV and 0.073,
0.064 *±* 0.001 and +2 to +6 per cent for the NUV.

$$
N_{side}=16
$$

$$
I_{\mathrm{H}\alpha}
$$

$$
\mathrm{AUCs}
$$

$$
0.097\pm0.020
$$

$$
0.064\pm0.001
$$

*Seam estimator and its null.* The stacked step is
the mean of the ring medians of log(map/S<sub>2</sub>◦ [map])
at +1 to +3 pixels outside the *GALEX* coverage edge

$$
\log(\operatorname{map}/\mathcal{S}_{2^{\circ}}[\operatorname{map}])
$$

minus that at *−1* to *−3* inside. Placed on random
great circles through pure-*GALEX* sky the estimator returns 0.0000 *±* 0.0024 (FUV) and +0.0002 *±* 0.0021 dex
per circle and a white-noise floor of ±0.0002 dex at
the length of the real boundary; block bootstrap of
the real-boundary statistic gives ±0.0005/±0.0006 dex.
Three implementations (published code at N<sub>side</sub> = 2048:
+0.0011/+0.0054; an independent pair-selection variant: 0.003–0.004/0.015–0.016; a re-implementation at
N<sub>side</sub> = 512 with the benchmark coverage: +0.0041 ±
0.0005/+0.0138 *±* 0.0006 dex) span the ranges quoted
in Section 4.1. Per 3.7<sup>◦</sup> segment (*≥* 100 ring pixels
each side; 1601/1548 segments) the local step has rms
0.022/0.026 dex, 90th percentile of *|*step*|* 0.031/0.039
and 2.6/3.6 per cent of segments beyond 0.05 dex; the
great-circle null gives rms 0.012/0.013, 90th percentile
0.021 and 0.0/0.3 per cent.

$$
+0.0002\pm0.0021
$$

$$
0.0000\pm0.0024
$$

$$
\pm0.0002
$$

$$
N_{side}=2048;
$$

$$
+0.0011/+0.0054
$$

$$
0.003-0.004/0.015-0.016;
$$

$$
(\geq100
$$

$$
+0.0041\pm
$$

$$
N_{side}=512
$$

$$
0.0005+0.0138\pm0.0006\mathrm{d}x
$$

$$
3.7^{\circ}
$$

*Uncertainty calibration by stratum.* Table 10 gives the
coverage of *±u, ±2u* and *±3u* by *E(B−V),* distance and
latitude stratum for the model term (benchmark, crosshalf) and the FIMS/SPEAR term (blind mock gaps,
test half), with the conventions stated in the caption.

---

Table 10: Coverage of the published uncertainty on withheld data (target 0.68 for ±1u). Model term *σ*<sub>M</sub>: cross-half validation on the
benchmark, adopted predictor’s out-of-fold residuals, area-weighted (gap-matched in parentheses). FIMS/SPEAR term *σ*<sub>F</sub>: test half of
the 2.25 million blind mock-gap pixels. Multipliers: percentiles of *|r|/σ* (model term: with the far-distance extension applied).

$$
\sigma_{M}\colon
$$

$$
\sigma_{F}\text{:}
$$

| Stratum | σM FUV | σM NUV | σF FUV |
| --- | --- | --- | --- |
| All | 0.68 (0.66) | 0.68 (0.68) | 0.71 (0.73) |
| E(B-V) < 0.1 / 0.1-0.4 / 0.4-0.8 | 0.66-0.68 | 0.67-0.68 | 0.69-0.73 / 0.71-0.73 / 0.71 |
| E(B-V) 0.8-1.6 / > 1.6 | 0.54 / 0.81 | 0.66 / 0.67 | 0.73 / 0.77 |
| d < 1° / 1-3 / 3-5 / 5°-9° | 0.68 / 0.68 / 0.67 / 0.71 | 0.68 / 0.68 / 0.67 / 0.67 | 0.71-0.73 / 0.70-0.71 / 0.73 / 0.72 |
| \|b\|  30° | — | — | 0.72 / 0.72-0.74 / 0.68-0.72 |
| σ-decile range | 0.65-0.72 | 0.66-0.70 | — |
| within ± 2u / ± 3u | 0.921 / 0.968 | 0.925 / 0.971 | 0.937 / 0.972 |
| \|r\|/σ at 95.45 / 99.73 per cent | 2.51 / 15.2 | 2.45 / 19.2 | 2.29 / 13.5 |

$$
\sigma_{M}
$$

$$
\sigma_{\mathrm{F}}
$$

$$
\sigma_{M}
$$

$$
E(B-V)<0.1\quad0.1-0.4\quad0.4-0.8
$$

$$
E(B-V)0.8-1.6>1.6
$$

$$
0.72\mid0.72-0.74\mid0.68-0.72
$$

$$
d<1^{\circ}\quad1-3\quad3-5\quad5^{\circ}-9^{\circ}
$$

$$
|b|<10^{\circ}/10-30/>30^{\circ}
$$

Table 11: Sources of the quantitative claims of the abstract and summary. ‘File’ entries are published maps from which the number is
recomputed by the recipe given; named .json/.csv files are published tables (also collected in validation_statistics.json); ‘Table’
entries point to this paper.

| Claim | Value | Source |
| --- | --- | --- |
| Dominant-layer sky fractions | 61.9/9.7/0.2/23.7/4.5; 73.3/0.2/26.5 per cent | File: argmax over columns of {b}_weights_n2048_gal.fits; header FRAC0-4 of the provenance files |
| Stellar layer size and completeness | 1.189 × 108 stars; G99 = 17.25/16.5 | stellar_layer_key_numbers.json; stellar_layer_aux_n256_gal.fits |
| Predicted sub-degree morphology | 28.0/26.5 per cent | File: WFIMS + WMODEL dominant; equivalently NaN fraction of the data-only files |
| GALEX fidelity | p90 \(\|\log(map/G')\| = 0.0024/0.0005\) dex; WGALEX > 0.99 on 87.0/89.3 per cent | File: fused map versus galex_{b}_destriped_n2048_gal.fits on its non-NaN pixels; weight file |
| Sky-averaged seam step | +0.0011 dex FUV; < 0.005/0.005-0.015 across implementations | File + seam_metrics.py; S5_seam_null_test.csv; Appendix B |
| Local seam rms | 0.022/0.026 versus null 0.012/0.013 dex | S5_seam_null_test.csv; Appendix B |
| Gap-matched CV scatter | 0.060 ± 0.002 / 0.060 ± 0.001 dex | File: score_benchmark.py on gapfill_benchmark_nside512.npz with oof_challengerA_jit_{b}_total.npy; bootstrap over Nside = 8 super-pixels (S1_bootstrap_matchedMAD_CIs.csv) |
| Shift-corrected NUV scatter | 0.065 dex | S3_covariate_shift_c2st.csv, S3b_shift_reweighted_uncertainty.csv |
| Distance dependence | 0.048 → 0.08 (FUV), 0.053 → 0.08 dex (NUV) at < 0.25° → 5-8° | File: benchmark + dist_to_training_champion_folds_{b}_npy; S6_distance_binned_MAD_v26.csv |
| Far-gap fractions | 25.5 per cent of filled FUV sky beyond 3.65°; 5 per cent beyond 8° | File: dist_gap_to_galex_FUV.npy (any-valid-child rule, Nside = 512) |
| Mock-gap recovery | 0.049/0.047 dex at Nside = 512; structure r = 0.80-0.93 | e2e_validation_v26.json (T1, variant C); fusion_v35.py |
| Blind FIMS/SPEAR-layer mock gaps | p68 0.046→0.093 dex with distance; gap-matched 0.068 | fims_mockgap_error_table.csv, fims_sky_uncertainty.json |
| FUV versus FIMS/SPEAR (GALEX-free) | 0.90, 0.19 dex, r = 0.80; 0.87-0.95 by \|b\| | File: fused FUV versus fims_fuv_starry_n2048_gal.fits at Nside = 128, zero GALEX children; e2e_validation_v26.json (T2) |
| NUV versus UVOT (GALEX-free) | 1.01, 0.14 dex, r = 0.78 | File: fused NUV versus uvot_uvm2_rate calibrated by uvot_galex_crosscal.json; e2e_validation_v26.json (T2) |
| Uncertainty coverage | 0.68/0.68 (σM); 0.70 (σF); multipliers 2.4, 9-12 | model_term_v27_calibration.csv, fims_sky_uncertainty.json; File 10; FITS header comments |
| Zero points | 291/742 (Huber, total); 301/727 and 264/579 CU (jackknife, total/diffuse) | File: destriped GALEX + ebv_planck2013 at Nside = 256, \|b\| > 40°, E(B-V) < 0.08; nuv_zeropoint_key_numbers.json,≠2e_validation_v26.json (T3) |
| Identity to UV-BKGD | +2/+4 CU (fit); -4/-6 CU (pixel median) | File: murthy_{b}_diffuse_n2048_gal.fits versus P30 of G' |
| FUV-Hα partial slope | 86 ± 3 / 68 ± 3 UCU R-1 | halpha_partial_slopes.json; two-photon 57/34 CU R-1 computed from Nussbaumer &amp; Schmutz (1984) |
| FUV high-latitude \|β\| structure | +1.82 ± 0.14 (GALEX), +4.1 ± 0.4 (FIMS/SPEAR), +0.25 ± 0.12 CU deg-1 (NUV) | File: G', FIMS/SPEAR starless, E(B-V) at Nside = 256, \|b\| > 50°, E(B-V) < 0.03; ecliptic_residual_numbers.json; template file header |
| P30 depth term | +0.34 ± 0.01 σpix; +13 CU median | p30_bias_results.json; raw Nside = 4096 mosaics |
| Power retained by gap fill | \(C_{\ell}\) ratio 0.47-0.60/0.68-0.70 at \(\ell = 100-1500\); diffuse 0.75/0.54/0.36 | e2e_validation_v26.json (T4), structure_amplitude_heldout_v26.csv; Fig. 14 |
| Foreground removed | median 46/480 CU; inpainted cells 9.4/8.2 per cent | File: foreground_offset_{b}_n256_gal.fits weighted by GALEX pixel counts; UV-BKGD coverage at Nside = 256 |
| Map extrema and medians | min 107/254 CU; medians 1120/1359 CU | File: fused maps (audit_key_numbers.json, integrity) |

$$
1.189\times10^{8}
$$

$$
G_{99}=17.25/16.5
$$

$$
\mathrm{p}90\mid\log(\mathrm{map}/G^{\prime})\mid=
W_{\mathrm{FIMS}}+W_{\mathrm{MODEL}}
$$

$$
W_{GALEX}>0.99
$$

$$
0.060\pm0.002/0.060\pm0.001\mathrm{d}x
$$

$$
N_{side}=8
$$

$$
0.048{\to}0.\dot{0}8~\mathrm{(F U V)},
$$

$$
<0.25^{\circ}\rightarrow5^{-}8^{\circ}
$$

$$
3.65^{\circ};5
$$

$$
N_{side}=512)
$$

$$
N_{side}=512;
$$

$$
N_{side}=128,
$$

$$
\left|b\right|>40^{\circ}
$$

$$
N_{side}=256,
$$

$$
86\pm3\dot{/}68\pm3\mathrm{CUR^{-1}}
$$

$$
\mathcal{P}_{30}
$$

$$
+1.82\pm0.14(GALEX),+4.1\pm0.4
$$

$$
G'
$$

$$
\mathcal{P}_{30}
$$

$$
\operatorname{(F I M S/S P E A R)},
$$

$$
+0.25\pm0.12\mathrm{CU\deg^{-1}}
$$

$$
\mathrm{R}^{-1}
$$

$$
G^{\prime},
$$

$$
N_{side}=256,
$$

$$
|b|>50^{\circ}
$$

$$
N_{side}=4096
$$

$$
+0.34\pm0.01\sigma_{\mathrm{pix}};
$$

$$
GALEX
$$

$$
\overset{\rightharpoonup}{N}_{side}=\overset{\rightharpoonup}{2}56
$$

---

## C Additional validation

This appendix collects validation material that supports
statements of the main text but is not required to follow
it: a quasar null test by fusion-layer provenance, recovery
of stellar and galaxy photometry, and further internalconsistency checks on the published maps.

### C.1 Quasar null test and point-source photometry

Extragalactic point sources test for spurious extended
signal and for the degree to which the diffuse predictor
imprints extragalactic objects through its templates,
independently of which layer supplies the flux at a given
pixel. Figure 17 stacks Milliquas quasars (Flesch 2023)
on the published maps, split by the provenance of the
pixel: the point-source term is detected where the pixel
is *GALEX*-dominated and where the FUV is NUVinformed (both carry real NUV imaging), and is null
where the FUV is FIMS/SPEAR-constrained or modelfilled, i.e. the predictor does not synthesise a spurious
quasar signal where no ultraviolet photon has been
measured.

The map is trained on, and represents, total surface
brightness; whether individual sources are photometrically preserved depends on provenance. In *GALEX*-
dominated pixels (*W*<sub>GALEX</sub>> 0.9) stellar flux is recovered essentially one to one: r = 3<sup>′</sup> aperture photometry
with a 6<sup>′</sup>–10<sup>′</sup> median-annulus background at the positions of 27110 isolated GUVcat AIS stars (Bianchi
et al. 2017) with NUV < 14 (8126 with FUV < 14.5)
gives median recovered/catalogue photon-flux ratios
of 1.13 (16–84 per cent range 1.03–1.29, robust scatter 0.025 dex, *n* = 1098) in FUV and 1.15 (0.84–1.45,
0.08 dex, *n* = 19 910) in NUV, flat with magnitude and
identical in the destriped *GALEX*-only map, the 13–
15 per cent excess being consistent with PSF wings
and blends entering the aperture relative to the GU-
Vcat Kron-type magnitudes. Ratios above unity for
stars brighter than NUV *≃* 14 reflect the known nonlinearity of *GALEX* bright-star photometry, which
the map inherits; an independent check with isolated
12 < NUV < 16.5 AIS stars gives destriped-*GALEX*
4<sup>′</sup>-aperture flux/catalogue ratios of +0.02 to +0.08 dex,
i.e. the CU scale of the *GALEX* layer is correct to
≲ 20 per cent for unsaturated stars. Brighter still, the
stars censored by *GALEX* are restored from TD-1 and
the BSC (Section 3.5): TD-1-bright stars inside the
footprint have a median map/TD-1 aperture ratio of
0.00 dex in both bands.

$$
\left(W_{GALEX}>0.9\right)
$$

$$
r=3'
$$

$$
6'-10'
$$

$$
\mathrm{NUV}<14
$$

$$
\mathrm{FUV}<14.5)
$$

$$
n=1098)
$$

$$
n=19910)
$$

$$
12<NUV<16.5
$$

$$
4'\cdot
+0.02to+0.08dx
$$

$$
\mathit{GALEX}
$$

$$
\lesssim20
$$

### C.2 Galaxy and globular-cluster layer validation

The galaxy and globular-cluster layer L<sub>gal</sub> has galaxy
fluxes validated directly against *GALEX* aperture photometry to 0.94 (FUV) and 1.16 (NUV) with 0.08–
0.09 dex scatter (Fig. 18); for the 104 globular clusters
without measured integrated ultraviolet magnitudes the

$$
L_{\mathrm{gal}}
$$

predicted FUV flux carries a 0.7mag uncertainty, and
their cores should be regarded as recovered only partially.

### C.3 Further consistency checks

A matched filter for coherent steps across HEALPix
N<sub>side</sub> = 8–256 cell boundaries of the published maps,
compared with identical control lines offset by half a
cell, finds no excess at any scale in any provenance
class (median *|*step*|* along boundary segments equal to
control to < 0.001 dex; e.g. FUV model-class N<sub>side</sub> = 32
boundaries 0.0074 versus 0.0073 dex, FIMS/SPEARclass N<sub>side</sub> = 16 0.0064 versus 0.0063). A Hough-style
ring test (mean 1 -high-pass along 0<sup>◦</sup>.23 -wide small<sup>◦</sup>
circles about 6144 poles) is dominated by the Galactic
plane for all poles; at *|b|* > 25<sup>◦</sup> the maximum over
400 poles is 13.6/16.3 against a smooth distribution of
mean 11.2 *±* 0.8/13.1 *±* 0.8, i.e. no outlier pole and no
great-circle stripe.

$$
N_{side}=8-256
$$

$$
N_{side}=32
$$

$$
N_{side}=16\ 0.006
$$

$$
0.23^{\circ}
$$

$$
\left|b\right|>25^{\circ}
$$

$$
11.2\pm0.8/13.1\pm0.8
$$

## D Measurements enabled by a full-sky two-band ultraviolet map

We list, without elaboration, measurements that the
published maps make newly possible or substantially
easier, with the pixel class each requires (Section 3.10).
*Diffuse Galactic light and dust* — full-sky Monte-Carlo
radiative-transfer fits of the DGL with *Gaia*-based threedimensional dust and the resolved hot-star census as
sources, closing the albedo–*g* degeneracy that singlegeometry fits leave open (WGALEXsky as data, filled
sky as boundary condition); a direction-resolved empirical FUV interstellar radiation field at the cloud surfaces
mapped by *Planck;* the FUV/NUV colour of the DGL
as a function of environment (*W*<sub>GALEX</sub>in both bands);
H -fluorescence and two-photon excesses as FUV-only2
residuals against dust and *Hα* (*W*<sub>GALEX</sub>). *The isotropic
component* — the anisotropy (dipole, quadrupole, hemispheric asymmetry, the *ℓ ≤* 2 FUV structure of Section 5.3) of the high-latitude offset over twice the
sky now usable, and its cross-correlation with H i velocity components, soft X-rays and synchrotron templates (*W*<sub>GALEX</sub>; monopole inherited). *Extragalactic* —
clustering-redshift tomography and broadband intensity mapping (Chiang et al. 2019) over the full *GALEX*
footprint with a characterised Galactic component and
transfer function; cross-correlation with CMB lensing
and the cosmic infrared background; stacking on clusters, filaments and quasar sightlines for halo ultraviolet
light and circumgalactic dust (*W*<sub>GALEX</sub>> 0.99, transfer
function of Fig. 14 for anything else). *Mission planning
and calibration* — background, confusion and brightobject maps over *4π* for ULTRASAT, UVEX, CSST,
UVIT and CASTOR field selection and exposure-time
calculators (all classes, with SIGMA and SIGMA_SYS);
inter-mission zero-point cross-checks (the harmonisation
gains are themselves measurements: *UVOT→GALEX*

$$
\mathit{Diffuse}
$$

$$
\left(W_{GALEX}\right.
\left(W_{GALEX}\right.
\left(W_{\mathrm{GALEX}}\right)
$$

$$
\ell\leq2\mathrm{FUV}
$$

$$
H_{I}
$$

$$
\left(W_{\mathrm{GALEX}};\right.
\left(W_{GALEX}>0.99\right.
$$

---

| radius (arcmin) | GALEX sky: 26.1±2.4 CU (n=40k) (QSO − control, CU) | filled sky: -0.2±9.8 CU (n=14k) (QSO − control, CU) |
| --- | --- | --- |
| 0.75 | 27 | -0.8 |
| 3.0 | 0 | 3.9 |
| 5.0 | -3.5 | -3.5 |
| 7.0 | -0.6 | 1 |
| 9.0 | -2.2 | 3.5 |
| 12.5 | 0.9 | -2.4 |

| radius (arcmin) | GALEX sky: 13.0±0.5 CU (n=40k) (QSO − control, CU) | filled sky: 5.0±1.1 CU (n=19k) (QSO − control, CU) |
| --- | --- | --- |
| 1.0 | 11.5 | 2.1 |
| 3.3 | 0.3 | 0.4 |
| 5.0 | 0.5 | -0.9 |
| 7.0 | 0.0 | 0.0 |
| 9.0 | 0.1 | -0.4 |
| 12.5 | 0.0 | 0.1 |

| Category | NUV (net stacked signal, r<2' (CU)) | FUV (net stacked signal, r<2' (CU)) |
| --- | --- | --- |
| GALEX | 25.5 | 13.5 |
| mixed | 21.5 | 7.5 |
| filled (all) | 0 | 5.5 |
| filled: model | -1.5 | 1.5 |
| filled: NUV-inf. | 17 | 17 |
| filled: FIMS | 1 | 1 |

Figure 17: Quasar null test on the published maps by provenance class (143154 Milliquas quasars, 0.3 < *z* < 2.5, *R* < 19.5; each
object referenced to a control position of the same class 0.4<sup>◦</sup>–2<sup>◦</sup>away). (a, b) Stacked radial excess in *GALEX*-dominated and in filled
pixels of the published NUV and FUV maps. (c) Net r < 2<sup>′</sup> excess by class: detected in GALEX pixels and in NUV-informed FUV
pixels (which are NUV imaging), null in FIMS/SPEAR-constrained and model-filled sky.

$$
0.3<z<2.5,
$$

$$
R<19.5;
$$

$$
0.4^{\circ}-2^{\circ}
$$

$$
r<2'
$$

| Layer aperture flux (ph cm⁻² s⁻¹ Å⁻¹) | FUV (N=602) (GALEX map aperture flux, ph cm⁻² s⁻¹ Å⁻¹) | NUV (N=605) (GALEX map aperture flux, ph cm⁻² s⁻¹ Å⁻¹) |
| --- | --- | --- |
| 0.00007 | 0.000085 |  |
| 0.00008 | 0.000075 |  |
| 0.00009 |  | 0.00022 |
| 0.0001 | 0.000003 |  |
| 0.00011 | 0.00014 |  |
| 0.00012 |  | 0.0016 |
| 0.00013 | 0.00016 |  |
| 0.00014 | 0.00007 |  |

| log10 (GALEX map / layer) aperture flux | FUV: median 0.94, σrob 0.08 dex (Number of galaxies) | NUV: median 1.16, σrob 0.09 dex (Number of galaxies) |
| --- | --- | --- |
| -0.6 | 0 | 0 |
| -0.5 | 0 | 0 |
| -0.4 | 1 | 0 |
| -0.3 | 2 | 0 |
| -0.2 | 3 | 1 |
| -0.1 | 10 | 2 |
| 0 | 64 | 34 |
| 0.1 | 32 | 40 |
| 0.2 | 4 | 11 |
| 0.3 | 1 | 3 |
| 0.4 | 0 | 1 |
| 0.5 | 0 | 2 |
| 0.6 | 0 | 0 |

| D25 (arcmin) | Series 1 (unlabelled) (log10 GALEX map / layer) | Series 2 (unlabelled) (log10 GALEX map / layer) |
| --- | --- | --- |
| 3 | 0.0 | 0.0 |
| 4 | 0.05 | -0.02 |
| 5 | 0.05 | -0.03 |
| 6 | 0.05 | -0.02 |
| 7 | 0.05 | -0.01 |
| 8 | 0.04 | -0.01 |
| 9 | 0.03 | -0.01 |
| 10 | 0.04 | -0.02 |
| 11 | 0.04 | -0.03 |
| 12 | 0.04 | -0.04 |
| 13 | 0.04 | -0.03 |
| 14 | 0.05 | -0.02 |
| 15 | 0.05 | -0.01 |
| 16 | 0.05 | -0.01 |
| 17 | 0.05 | -0.01 |
| 18 | 0.05 | -0.01 |
| 19 | 0.05 | -0.02 |
| 20 | 0.05 | -0.02 |
| 21 | 0.05 | -0.02 |
| 22 | 0.05 | -0.02 |
| 23 | 0.05 | -0.02 |
| 24 | 0.05 | -0.02 |
| 25 | 0.05 | -0.02 |
| 26 | 0.05 | -0.02 |
| 27 | 0.05 | -0.02 |
| 28 | 0.05 | -0.02 |
| 29 | 0.05 | -0.02 |

Figure 18: Validation of the galaxy and globular-cluster layer *L*<sup>gal</sup>against *D*<sup>25</sup>-aperture photometry of the destriped *GALEX* maps for
605 isolated galaxies with 3<sub>′</sub>*≤ D ≤* 30<sub>′</sub>at *|b|* > 20<sub>◦</sub>and full *GALEX* coverage (aperture *1.2R* + 1.5<sub>′</sub>, background from a 1.6–*2.5R*
25 25 25
annulus). (a) *GALEX* aperture flux versus layer aperture flux, FUV (blue) and NUV (red). (b) Distribution of log<sub>10</sub>(GALEX /layer)
with medians and robust scatters. (c) The same ratio versus *D*<sub>25</sub>with running medians.

$$
L_{\mathrm{gal}}
$$

$$
D_{25^{\circ}}
$$

$$
3^{\prime}\leq D_{25}\leq30^{\prime}
$$

$$
\overset{\circ}{GA}LEX
$$

$$
\left|b\right|>20^{\circ}
$$

$$
1.2R_{25}+1.5'
$$

$$
a 1. 6 - 2. 5 R _ {2 5}
$$

$$
\mathit{GALEX}
$$

$$
\log_{10}(\mathit{GALEX}/\mathit{layer})
$$

$$
D_{25}
$$

NUV 0.88, FIMS/SPEAR/GALEX FUV 0.97–1.05); a
reference background for transient searches in archival
*GALEX* /UVOT data. *Foreground by-products* — the
zodiacal ultraviolet colour and ecliptic profile at 2300Å
and airglow statistics versus orbital geometry from the
published per-cell offsets. *Stellar populations* — an
all-sky ultraviolet-bright star census cross-matched to
*Gaia* including the plane (imaging classes only; filled-sky
stellar fluxes are predictions), and integrated photometry of objects larger than a *GALEX* tile on a uniform
background. *Methodological* — the frozen public gapfill benchmark, on which any inpainting method can
be scored under identical rules, and the priority-nested
feathering with per-pixel provenance and empirical uncertainty as a template for other multi-mission sky fusions.

---

## References

Akshaya M. S., Murthy J., Ravichandran S., Henry
R. C., Overduin J., 2018, ApJ, 858, 101

Akshaya M. S., Murthy J., Ravichandran S., Henry
R. C., Overduin J., 2019, MNRAS, 489, 1120

Alonso D., Sanchez J., Slosar A., LSST Dark Energy
Science Collaboration, 2019, MNRAS, 484, 4127

Astropy Collaboration, 2022, ApJ, 935, 167

Bai Y., Zou H., Liu J., Wang S., 2015, ApJS, 220, 6

Bianchi L., Conti A., Shiao B., 2014, Advances in Space
Research, 53, 900

Bianchi L., Shiao B., Thilker D., 2017, ApJS, 230, 24

Boch T., Fernique P., 2014, in Manset N., Forshay P.,
eds, ASP Conf. Ser. Vol. 485, Astronomical Data
Analysis Software and Systems XXIII. Astron. Soc.
Pac., San Francisco, p. 277

Bowyer S., 1991, ARA&A, 29, 59

Bowyer S. et al., 1993, ApJ, 415, 875

Cantat-Gaudin T. et al., 2020, A&A, 640, A1

Chiang Y.-K., 2023, ApJ, 958, 118

Castelli F., Kurucz R. L., 2003, in Piskunov N., Weiss
W. W., Gray D. F., eds, IAU Symp. 210, Modelling of
Stellar Atmospheres. Astron. Soc. Pac., San Francisco,
p. A20

Culpan R., Geier S., Reindl N., Pelisoli I., Gentile Fusillo
N., Vorontseva A., 2022, A&A, 662, A40

Gentile Fusillo N. P. et al., 2021, MNRAS, 508, 3877

Page M. J. et al., 2014, in Proceedings of Swift: 10
Years of Discovery, PoS(SWIFT 10)037

Chiang Y.-K., Ménard B., 2019, ApJ, 870, 120

Chiang Y.-K., Ménard B., Schiminovich D., 2019, ApJ,
877, 150

Chiang Y.-K., Makiya R., Ménard B., Komatsu E., 2020,
ApJ, 902, 56

Côté P. et al., 2019, CASTOR: A Flagship Canadian Space Telescope, Canadian Long Range Plan
for Astronomy and Astrophysics White Paper,
arXiv:1910.00557

Crill B. P. et al., 2020, in Proc. SPIE, Vol. 11443, Space
Telescopes and Instrumentation 2020: Optical, Infrared, and Millimeter Wave, 114430I

Dalessandro E., Schiavon R. P., Rood R. T., Ferraro
F. R., Sohn S. T., Lanzoni B., O’Connell R. W., 2012,
AJ, 144, 126

de Vaucouleurs G., de Vaucouleurs A., Corwin H. G. Jr,
Buta R. J., Paturel G., Fouqué P., 1991, Third Reference Catalogue of Bright Galaxies. Springer, New
York

de Zeeuw P. T., Hoogerwerf R., de Bruijne J. H. J.,
Brown A. G. A., Blaauw A., 1999, AJ, 117, 354

Doi Y. et al., 2015, PASJ, 67, 50

Doré O. et al., 2014, arXiv e-prints, arXiv:1412.4872

Draine B. T., 1978, ApJS, 36, 595

Draine B. T., 2003, ARA&A, 41, 241

Draine B. T., 2003b, ApJ, 598, 1017

Draine B. T., 2011, Physics of the Interstellar and Intergalactic Medium. Princeton University Press

Driver S. P. et al., 2016, ApJ, 827, 108

Edelstein J. et al., 2006, ApJ, 644, L153

Edenhofer G., Zucker C., Frank P., Saydjari A. K.,
Speagle J. S., Finkbeiner D., Enßlin T. A., 2024,
A&A, 685, A82

Fernique P. et al., 2015, A&A, 578, A114

Finkbeiner D. P., 2003, ApJS, 146, 407

Flesch E. W., 2023, Open J. Astrophys., 6, 49

Friedman J. H., 2001, Annals of Statistics, 29, 1189

Gaia Collaboration, Prusti T. et al., 2016, A&A, 595,
A1

Gaia Collaboration, Vallenari A. et al., 2023, A&A, 674,
A1

Gil de Paz A. et al., 2007, ApJS, 173, 185

Giordano F. et al., 2018, Astronomy and Computing,
24, 97

Górski K. M., Hivon E., Banday A. J., Wandelt B. D.,
Hansen F. K., Reinecke M., Bartelmann M., 2005,
ApJ, 622, 759

Habing H. J., 1968, Bull. Astron. Inst. Netherlands, 19,
421

Hamden E. T., Schiminovich D., Seibert M., 2013, ApJ,
779, 180

Harris W. E., 2010, arXiv:1012.3224

Harris C. R. et al., 2020, Nature, 585, 357

Henry R. C., Murthy J., Overduin J., Tyler J., 2015,
ApJ, 798, 14

HI4PI Collaboration, Ben Bekhti N. et al., 2016, A&A,
594, A116

Hoffleit D., Jaschek C., 1991, The Bright Star Catalogue,
5th rev. edn. Yale University Observatory, New Haven

---

Hunter J. D., 2007, Computing in Science & Engineering,
9, 90

Jo Y.-S. et al., 2012, ApJ, 756, 38

Jo Y.-S., Seon K.-I., Min K.-W., Edelstein J., Han W.,
2017, ApJS, 231, 21

Jo Y.-S., Seon K.-I., Min K.-W., Edelstein J., Han W.,
2021, MNRAS, 502, 3200

Ke G., Meng Q., Finley T., Wang T., Chen W., Ma
W., Ye Q., Liu T.-Y., 2017, in Advances in Neural Information Processing Systems 30. Curran Associates,
p. 3146

Korpela E. J. et al., 2006, ApJ, 644, L163

Kregenow J. et al., 2006, ApJ, 644, L167

Kulkarni S. R., 2022, PASP, 134, 084302
(arXiv:2107.09585)

Kulkarni S. R. et al., 2021, arXiv:2111.15608

Leinert C. et al., 1998, A&AS, 127, 1

Lenz D., Hensley B. S., Doré O., 2017, ApJ, 846, 38

Leroy A. K. et al., 2019, ApJS, 244, 24

Lim T.-H., Min K.-W., Seon K.-I., 2013, ApJ, 765, 107

Madau P., 1995, ApJ, 441, 18

Martin D. C. et al., 2005, ApJ, 619, L1

Mathis J. S., Mezger P. G., Panagia N., 1983, A&A,
128, 212

Ménard B., Scranton R., Fukugita M., Richards G.,
2010a, MNRAS, 405, 1025

Ménard B., Kilbinger M., Scranton R., 2010b, MNRAS,
406, 1815

Ménard B., Scranton R., Schmidt S., Morrison C., Jeong
D., Budavari T., Rahman M., 2013, arXiv:1303.4722

Morgan D. H., Nandy K., Thompson G. I., 1976, MN-
RAS, 177, 531

Morrissey P. et al., 2007, ApJS, 173, 682

Murthy J., 2014, ApJS, 213, 32

Murthy J., 2014b, Ap&SS, 349, 165

Murthy J., 2016, MNRAS, 459, 1710

Murthy J. et al., 1999, ApJ, 522, 904

Murthy J., Henry R. C., Sujatha N. V., 2010, ApJ, 724,
1389

Murthy J., Akshaya M. S., Ravichandran S., 2019, arXiv
e-prints, arXiv:1909.05325

Nussbaumer H., Schmutz W., 1984, A&A, 138, 495

Page M. J. et al., 2012, MNRAS, 426, 903

Pedregosa F. et al., 2011, Journal of Machine Learning
Research, 12, 2825

Peek J. E. G., Ménard B., Corrales L., 2015, ApJ, 813,
7

Planck Collaboration XI, 2014, A&A, 571, A11

Poole T. S. et al., 2008, MNRAS, 383, 627

Roming P. W. A. et al., 2005, Space Sci. Rev., 120, 95

Schiminovich D. et al., 2001, ApJ, 563, L161

Schlegel D. J., Finkbeiner D. P., Davis M., 1998, ApJ,
500, 525

Seon K.-I., Witt A. N., 2012, ApJ, 758, 109

Seon K.-I., Witt A. N., 2013, ApJ, 778, L40

Seon K.-I. et al., 2011, ApJS, 196, 15

Shvartzvald Y. et al., 2024, ApJ, 964, 74

Sujatha N. V. et al., 2005, ApJ, 633, 257

Sujatha N. V. et al., 2009, ApJ, 692, 1333

Takita S. et al., 2015, PASJ, 67, 51

Tandon S. N. et al., 2017, AJ, 154, 128

Thompson G. I., Nandy K., Jamar C., Monfils A., Houziaux L., Carnochan D. J., Wilson R., 1978, Catalogue
of Stellar Ultraviolet Fluxes (TD-1): A Compilation
of Absolute Stellar Fluxes Measured by the Sky Survey Telescope (S2/68) Aboard the ESRO Satellite
TD-1. Science Research Council, London

Virtanen P. et al., 2020, Nature Methods, 17, 261

Weingartner J. C., Draine B. T., 2001, ApJ, 548, 296

Welsh B. Y. et al., 2007, A&A, 472, 509

Witt A. N., Friedmann B. C., Sasseen T. P., 1997, ApJ,
481, 809

Witt A. N., Gold B., Barnes F. S., DeRoo C. T., Vijh
U. P., Madsen G. J., 2010, ApJ, 724, 1551

Xu C. K. et al., 2005, ApJ, 619, L11

Yershov V. N., 2014, Ap&SS, 354, 97

Zhan H., 2021, Chinese Science Bulletin, 66, 1290

Zonca A., Singer L., Lenz D., Reinecke M., Rosset C.,
Hivon E., Górski K., 2019, Journal of Open Source
Software, 4, 1298