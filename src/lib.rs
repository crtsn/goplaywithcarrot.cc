use worker::*;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let url = req.url().unwrap();
    let host = req.headers().get("host").unwrap().unwrap();
    let is_local = host.ends_with("localhost");
    let host_to_split = host.clone();
    let host_parts: Vec<_> = host_to_split.split('.').collect();
    let mut only_image = false;
    let mut subdomain = None;
    let mut base_host = host;

    if (is_local && host_parts.len() == 2) || (host_parts.len() == 3) {
        subdomain = Some(host_parts[0]);
        base_host = host_parts[1..].join(".");
    }

    if let Ok(Some(user_agent)) = req.headers().get("user-agent") {
        if user_agent.contains("Discordbot") {
            only_image = true;
        }
    }

    if let Some(query) = url.query() {
        if query.contains("i=1") {
            only_image = true;
        }
    }

    console_log!("only_image: {only_image}");
    if only_image {
        let img_url = match subdomain {
          Some("xn--4o8h") => "rabbit.png",
          Some("xn--yn8h") => "rabbit2.png",
          Some("xn--dp8h") => "frog.png",
          _ => "hedgehog.png",
        };
        let img_request = format!("http://{base_host}/{img_url}");
        return env.assets("ASSETS")?.fetch(img_request, None).await;
    }

    let html = format!(
        r#"<!DOCTYPE html>
    <body>
      <a href="http://{base_host}">{base_host}</a>
      <p>URL used: {url}</p>
      <div style="display: flex; align-items: center;">
        <h1>Meet players:</h1>
        <div style="display: flex; align-items: center;">
          <a href="http://{char1}.{base_host}">{char1}</a>
          <a href="http://{char2}.{base_host}">{char2}</a>
          <a href="http://{char3}.{base_host}">{char3}</a>
          <a href="http://{char4}.{base_host}">{char4}</a>
        </div>
      </div>
	  <img src="?i=1">
    </body>
    "#,
        char1 = "\u{1F430}",
        char2 = "\u{1F407}",
        char3 = "\u{1F438}",
        char4 = "\u{1F994}"
    );
    Response::from_html(html)
}
