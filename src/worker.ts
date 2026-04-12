interface Env {
  IMAGES: ImagesBinding;
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
          img_url = "rabbit.png";
          break;
        case "xn--yn8h":
          img_url = "rabbit2.png";
          break;
        case "xn--dp8h":
          img_url = "frog.png";
          break;
        default:
          img_url = "hedgehog.png";
      }
      let img_request = new Request(`http://${baseHost}/${img_url}`);
      const character_resp = await env.ASSETS.fetch(img_request);
      if (!character_resp.ok) return new Response(`NOT FOUND: http://${baseHost}/${img_url}`, { status: 404 });
      const characterStream = character_resp.body;

      let map_request = new Request(`http://${baseHost}/map.png`);
      const map_resp = await env.ASSETS.fetch(map_request);
      if (!map_resp.ok) return new Response(`NOT FOUND: http://${baseHost}/map.png`, { status: 404 });
      const mapStream = map_resp.body;
      return (
        await env.IMAGES.input(mapStream)
          .draw(characterStream, { top: 250 / 2 - 72 / 2, left: 250 / 2 - 72 / 2 })
          .output({ format: "image/png" })
        ).response();
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
