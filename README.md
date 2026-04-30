Dump of things that I found and that are usefull:

```
"user-agent": "Mozilla/5.0 (compatible; Discordbot/2.0; +https://discordapp.com)"
contains("$workers.event.request.headers.user-agent", "discord")
```

Running wrangler dev tools in firefox when port set to 9229
found it with LLM, because it suggested for me to look to http://localhost:9229/json/list to check if debug server is running and there was this link
```
https://devtools.devprod.cloudflare.dev/js_app?theme=systemPreferred&debugger=true&ws=localhost:9229/ws
```
