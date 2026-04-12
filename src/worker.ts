interface Env {
  ASSETS: Fetcher;
}

export default {
  async fetch(
    request: Request,
    env: Env,
    ctx: ExecutionContext
  ): Promise<Response> {
    const url = new URL(request.url);
    var only_image = false;
    const isLocal = url.hostname.endsWith("localhost");
    const host = url.host;
    const host_parts = url.host.split('.')
    var baseHost = host;
    var subdomain = null
    if ((isLocal && host_parts.length == 2) || (host_parts.length == 3))
    {
      subdomain = host_parts[0]
      baseHost = host_parts.slice(1).join('.')
    }

    // for (let key in request) {
    //   console.log(key, request[key])
    // }
    // console.log([...request.headers]);

    let user_agent = request.headers.get("user-agent");
    only_image = user_agent && user_agent.includes("Discordbot");
    only_image = only_image || (url.searchParams.get("i") != null);

    if (only_image)
    {
      let img_url;

      switch(subdomain) {
        case "xn--4o8h":
          img_url = "img/rabbit.png";
          break;
        case "xn--yn8h":
          img_url = "img/rabbit2.png";
          break;
        case "xn--dp8h":
          img_url = "img/frog.png";
          break;
        default:
          img_url = "img/hedgehog.png";
      }
      let img_request = new Request(`http://${baseHost}/${img_url}`);
      const assetResponse = await env.ASSETS.fetch(img_request);
      return assetResponse;
    }

    const html = `<!DOCTYPE html>
    <body>
      <a href="http://${baseHost}">${baseHost}</a>
      <p>URL used: ${url}</p>
      <div style="display: flex; align-items: center;">
        <h1>Meet players:</h1>
        <div style="display: flex; align-items: center;">
          <a href="http://\u{1F430}.${baseHost}">\u{1F430}</a>
          <a href="http://\u{1F407}.${baseHost}">\u{1F407}</a>
          <a href="http://\u{1F438}.${baseHost}">\u{1F438}</a>
          <a href="http://\u{1F994}.${baseHost}">\u{1F994}</a>
        </div>
      </div>
	    <img src="?i=1">
    </body>`;

    return new Response(html, {
      headers: {
        "content-type": "text/html;charset=UTF-8",
      },
    });
  },
} satisfies ExportedHandler<Env>;
