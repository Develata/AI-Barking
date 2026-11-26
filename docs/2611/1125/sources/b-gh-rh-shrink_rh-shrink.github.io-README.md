# RH proof size

Size of the Lean source of [leanprover-community/riemann-hypothesis](https://github.com/leanprover-community/riemann-hypothesis)
per commit on `main`: <https://rh-shrink.github.io>. Unofficial.

- `collect.py` counts lines, declarations, files and import statements with regular expressions over the
  file contents in git (no Lean). The page lists exactly what is counted.
- `.github/workflows/update.yml` runs it every 15 minutes and deploys `site/` to GitHub Pages.
- `site/earlier.json`: `main` before it was replaced on 2026-11-25 by the readable edition of
  [grh-lean](https://github.com/lsernau/grh-lean), up to the commit `EARLIER` in the workflow (the
  branch `alpoge` goes on from there). The page shows it before `main`, with an arrow at the
  jump; the workflow measures it again only when `collect.py`'s VERSION changes.
  `site/graph-earlier.json`: the import graph of its first commit, the original (2.5 min, 1.5 GB),
  computed again only when `graph.py`'s VERSION changes.
