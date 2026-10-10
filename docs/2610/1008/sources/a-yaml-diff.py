"""1008 A 组：比较 openai/math 的 lean/formalization.yaml 新旧版本（旧 = 1006 存档 = 初始提交 adc7f12；新 = 合并提交 fd4aeeb），
并按论文目录名精确核对三篇撤稿是否出现在任一版本的 sources / main_results / CONTENTS 中。只用标准库。
用法：python a-yaml-diff.py <old.yaml> <new.yaml> [old_contents.md new_contents.md]"""
import re, sys, json

def parse(path):
    t = open(path, encoding="utf-8").read()
    src = []
    for m in re.finditer(r'^  - title: "(.*?)"\n    authors: .*\n    id: (\S+)\n', t, re.M):
        src.append((m.group(1), m.group(2)))
    mr_block = t.split("main_results:", 1)[1].split("\nautomation:", 1)[0]
    mr = re.findall(r'comparator_config: (\S+)\n\s+declaration: (\S+)\n\s+file: (\S+)', mr_block)
    return t, src, mr

def dirname(i):
    # ../preprints/<dir>/<file>.pdf -> <dir>
    m = re.match(r'\.\./preprints/([^/]+)/', i)
    return m.group(1) if m else i

WITHDRAWN = {
 "Algebraicity-of-Weil-classes-on-split-abelian-eightfolds-September-18-2026",
 "Algebraicity-of-Kuga-Satake-Correspondences-for-K3-Surfaces-October-3-2026",
 "The-rational-Hodge-conjecture-for-products-of-K3-surfaces-October-4-2026",
}
if __name__ == "__main__":
    ot, osrc, omr = parse(sys.argv[1]); nt, nsrc, nmr = parse(sys.argv[2])
    od = {dirname(i): t for t, i in osrc}; nd = {dirname(i): t for t, i in nsrc}
    print("sources entries: old", len(osrc), "new", len(nsrc), "| unique dirs old", len(od), "new", len(nd))
    print("main_results entries: old", len(omr), "new", len(nmr))
    print("unique main_results files: old", len({x[2] for x in omr}), "new", len({x[2] for x in nmr}))
    add = sorted(set(nd) - set(od)); rem = sorted(set(od) - set(nd))
    print("\n== sources added (%d) ==" % len(add)); [print(" +", d, "|", nd[d]) for d in add]
    print("\n== sources removed (%d) ==" % len(rem)); [print(" -", d, "|", od[d]) for d in rem]
    same_dir_changed = [d for d in od if d in nd and od[d] != nd[d]]
    print("\n== same dir, title changed (%d) ==" % len(same_dir_changed)); [print(" ~", d) for d in same_dir_changed]
    oc = {x[0] for x in omr}; nc = {x[0] for x in nmr}
    print("\n== main_results added (%d) ==" % len(nc - oc)); [print(" +", x) for x in sorted(nc - oc)]
    print("== main_results removed (%d) ==" % len(oc - nc)); [print(" -", x) for x in sorted(oc - nc)]
    print("\n== withdrawn dirs (exact dir-name match) ==")
    for w in sorted(WITHDRAWN):
        print(w, "| in old sources:", w in od, "| in new sources:", w in nd,
              "| substring in old yaml text:", w in ot, "| in new yaml text:", w in nt)
    # loose keyword check (informational only)
    for kw in ["eightfold", "Kuga", "K3", "Hodge", "abelian", "Weil"]:
        print("keyword", kw, "old:", len(re.findall(kw, ot)), "new:", len(re.findall(kw, nt)))
    if len(sys.argv) > 4:
        oc_, nc_ = open(sys.argv[3], encoding="utf-8").read(), open(sys.argv[4], encoding="utf-8").read()
        print("\n== CONTENTS.md ==")
        for w in sorted(WITHDRAWN):
            print(w, "| in old CONTENTS:", w in oc_, "| in new CONTENTS:", w in nc_)
        for tt in ["Algebraicity of Weil classes on split abelian eightfolds", "Algebraicity of Kuga", "The rational Hodge conjecture for products of K3 surfaces"]:
            print(repr(tt), "old CONTENTS:", oc_.count(tt), "new CONTENTS:", nc_.count(tt))
