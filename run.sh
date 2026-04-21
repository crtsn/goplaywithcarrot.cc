sed s'/"command": .*/"watch_dir": "build"/' <wrangler.jsonc >sub_wrangler.jsonc
FORCE_COLOR=1 npx wrangler dev --local --host localhost --port 8787  --inspector-port 9229 |& sed 's/^/LOCALHOST: /' &
FORCE_COLOR=1 npx wrangler dev --local -c sub_wrangler.jsonc --host xn--4o8h.localhost --port 8788  --inspector-port 9230 |& sed 's/^/RABBIT.LOCALHOST: /' &
FORCE_COLOR=1 npx wrangler dev --local -c sub_wrangler.jsonc --host xn--yn8h.localhost --port 8789  --inspector-port 9231 |& sed 's/^/RABBIT2.LOCALHOST: /' &
FORCE_COLOR=1 npx wrangler dev --local -c sub_wrangler.jsonc --host xn--dp8h.localhost --port 8790  --inspector-port 9232 |& sed 's/^/FROG.LOCALHOST: /' &
FORCE_COLOR=1 npx wrangler dev --local -c sub_wrangler.jsonc --host xn--kt9h.localhost --port 8791  --inspector-port 9233 |& sed 's/^/HEDGEHOG.LOCALHOST: /' &

wait

