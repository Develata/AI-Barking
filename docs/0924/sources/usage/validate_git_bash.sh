cd /e/gitclone/AI-Barking/0924/sources/usage
wc -l x_raw.jsonl x_coded.jsonl x_blind20.jsonl
python -c "import json;[json.loads(l) for f in ['x_raw.jsonl','x_coded.jsonl','x_blind20.jsonl'] for l in open(f,encoding='utf-8')];print('jsonl ok')"
grep -c verdict x_blind20.jsonl || true
git -C /e/gitclone/AI-Barking status --short
