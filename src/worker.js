export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    var discord_request = false;
    const r2 = env.MY_BUCKET;
    const default_img = await r2.get('512x512_tile64.png');

    console.log("URL: " + url);
    let user_agent = request.headers.get("user-agent");
    if (user_agent)
    {
      console.log("User-Agent: " + user_agent);
    }

    if (discord_request)
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
    </body>`;

    return new Response(html, {
      headers: {
        "content-type": "text/html;charset=UTF-8",
      },
    });
  },
};