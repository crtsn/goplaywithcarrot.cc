interface Env {
  MY_BUCKET: R2Bucket;
  test: string;
}

export default {
  async fetch(
    request: Request,
    env: Env,
    ctx: ExecutionContext
  ): Promise<Response> {
    const url = new URL(request.url);
    var only_image = false;
    const r2 = env.MY_BUCKET;
    const default_img = await r2.get('512x512_tile64.png');
    const isLocal = url.hostname.endsWith("localhost");
    const host = url.host;

    // for (let key in request) {
    //   console.log(key, request[key])
    // }
    // console.log([...request.headers]);

    console.log("URL: " + url);
    console.log("HOST: " + host);
    let user_agent = request.headers.get("user-agent");
    only_image = user_agent && user_agent.includes("Discordbot");
    only_image = only_image || (url.searchParams.get("i") != null);

    console.log(`only_image: ${only_image}`)
    if (only_image)
    {
      if (default_img) {
        return new Response(default_img.body, {
          headers: {
            'Content-Type': default_img.httpMetadata.contentType || 'image/png',
            'Cache-Control': 'public, max-age=3600',
          },
        });
      }
    }

    const html = `<!DOCTYPE html>
    <body>
      <a href="http://${host}">${host}</a>
      <p>URL used: ${url}</p>
      <div style="display: flex; align-items: center;">
        <h1>Meet players:</h1>
        <div style="display: flex; align-items: center;">
          <a href="http://\u{1F430}.${host}">\u{1F430}</a>
          <a href="http://\u{1F407}.${host}">\u{1F407}</a>
          <a href="http://\u{1F438}.${host}">\u{1F438}</a>
          <a href="http://\u{1F994}.${host}">\u{1F994}</a>
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
