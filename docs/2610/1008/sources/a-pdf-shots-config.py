# 配置：SHOTS = {目标文件名: [(pdf键, 页码, 起始锚点, 结束锚点[, 结束行是否含入])...]}
SHOTS = {
 # 质疑论文（arXiv 2610.08144v1）
 "03-crit-example31.png": [("crit",4,"A. BASTOUNIS","A. BASTOUNIS"), ("crit",4,"3.1. When the NL paper declares",None),
                           ("crit",6,"A. BASTOUNIS","A. BASTOUNIS"), ("crit",6,"The proof of Lemma 8.6 opens","closest resemblance")],
 "04-crit-figure3.png": [("crit",5,None,None)],
 "05-crit-example33.png": [("crit",8,"A. BASTOUNIS","A. BASTOUNIS"), ("crit",8,"Example 3.3 (The NL","hard to comp"),
                           ("crit",10,"A. BASTOUNIS","A. BASTOUNIS"), ("crit",10,"The discrepancy in (3.5) above arises","qualifier")],
 "06-crit-disclaimer.png": [("crit",2,"A. BASTOUNIS","A. BASTOUNIS"), ("crit",2,"NL proof could be wrong while","about mistranslations into Lean")],
 # OpenAI 原论文（Finite time blowup for Navier–Stokes）
 "07-oai-819.png": [("oai",95,"FINITE TIME BLOWUP","FINITE TIME BLOWUP"), ("oai",95,"Lemma 8.6. Let N",None),
                    ("oai",96,"OPENAI","OPENAI"), ("oai",96,"Proof. The divisor bound","Let P",False)],
 "08-oai-1019.png": [("oai",122,"OPENAI","OPENAI"), ("oai",122,"Pressure flux.",None),
                     ("oai",123,"FINITE TIME BLOWUP","FINITE TIME BLOWUP"), ("oai",123,"Young’s convolution","Difference energy",False)],
}
