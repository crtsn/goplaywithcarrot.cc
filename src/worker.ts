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

    console.log("URL: " + url);
    let user_agent = request.headers.get("user-agent");
    only_image = user_agent && user_agent.includes("Discordbot");
    only_image = only_image || (url.searchParams.get("i") != null);

    if (only_image)
    {
      if (default_img) {
        return new Response(default_img.body, {
          headers: {
            'Content-Type': default_img.httpMetadata.contentType || 'image/png',
            'Cache-Control': 'public, max-age=31536000',
          },
        });
      }
    }

    const html = `<!DOCTYPE html>
    <body>
      <h1>Meet players:</h1>
      <p>URL used: ${url}</p>
      <ul><li><a href="http://🐰.goplaywithcarrot.cc">🐰</a></li></ul>
      <ul><li><a href="http://🐇.goplaywithcarrot.cc">🐇</a></li></ul>
      <ul><li><a href="http://🐸.goplaywithcarrot.cc">🐸</a></li></ul>
      <ul><li><a href="http://🦔.goplaywithcarrot.cc">🦔</a></li></ul>
	  <img src="?i=1">
    </body>`;

    return new Response(html, {
      headers: {
        "content-type": "text/html;charset=UTF-8",
      },
    });
  },
} satisfies ExportedHandler<Env>;
