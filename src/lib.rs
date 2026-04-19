use worker::*;

#[event(fetch)]
async fn fetch(
    _req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let img_url = "http://goplaywithcarrot.com/rabbit.png";
    env.assets("ASSETS")?.fetch(img_url, None).await
}
