use worker::*;
use tiny_skia::*;
use image::load_from_memory;

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
        let character_resp = env.assets("ASSETS")?.fetch(img_request, None).await;
        if let Ok(mut character) = character_resp {
            let character_stream = character.bytes().await?;

            let map_request = format!("http://{base_host}/map.png");
            let map_resp = env.assets("ASSETS")?.fetch(map_request, None).await;

            if let Ok(mut map) = map_resp {
                let map_stream = map.bytes().await?;

                let bg_decoded = load_from_memory(&map_stream)
                                    .map_err(|e| worker::Error::from(e.to_string()))?
                                    .to_rgba8();
                let (width, height) = (bg_decoded.width(), bg_decoded.height());
                let mut pixmap = Pixmap::from_vec(
                    bg_decoded.into_raw(),
                    IntSize::from_wh(width, height).unwrap()
                ).unwrap();
                let overlay_data = Pixmap::decode_png(&character_stream).unwrap();
                let paint = Paint::default();
                pixmap.draw_pixmap(
                    10, 10,
                    overlay_data.as_ref(),
                    &PixmapPaint::default(),
                    Transform::identity(),
                    None,
                );
                let bmp_bytes = encode_bmp_fast(pixmap.data(), width, height);

                return Ok(Response::from_bytes(bmp_bytes)?
                    .with_headers(headers_with_bmp()));
            } else {
                return Response::error(format!("NOT FOUND: http://{base_host}/map.png"), 404);
            }
        } else {
            return Response::error(format!("NOT FOUND: http://{base_host}/{img_url}"), 404);
        }
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

// Minimal BMP encoder to save CPU (No compression)
fn encode_bmp_fast(rgba_pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
        let pixel_data_size = rgba_pixels.len();
    let file_size = 54 + pixel_data_size;
    let mut bmp = Vec::with_capacity(file_size);

    // --- FILE HEADER (14 bytes) ---
    bmp.extend_from_slice(b"BM");          // Signature
    bmp.extend_from_slice(&(file_size as u32).to_le_bytes()); // File size
    bmp.extend_from_slice(&[0, 0, 0, 0]);  // Reserved
    bmp.extend_from_slice(&54u32.to_le_bytes()); // Pixel data offset

    // --- INFO HEADER (40 bytes) ---
    bmp.extend_from_slice(&40u32.to_le_bytes()); // Header size
    bmp.extend_from_slice(&(width as i32).to_le_bytes());
    // Use negative height to indicate "Top-Down" order (standard for RGBA buffers)
    bmp.extend_from_slice(&(-(height as i32)).to_le_bytes());
    bmp.extend_from_slice(&1u16.to_le_bytes());  // Planes
    bmp.extend_from_slice(&32u16.to_le_bytes()); // Bits per pixel (32 = RGBA)
    bmp.extend_from_slice(&0u32.to_le_bytes());  // Compression (0 = None)
    bmp.extend_from_slice(&(pixel_data_size as u32).to_le_bytes()); // Image size
    bmp.extend_from_slice(&[0; 16]);             // Resolution & Colors (ignored)

    // --- PIXEL DATA ---
    // BMP expects BGRA. We must swap R and B from our RGBA buffer.
    let mut bgra = rgba_pixels.to_vec();
    for i in (0..bgra.len()).step_by(4) {
        bgra.swap(i, i + 2);
    }

    bmp.extend_from_slice(&bgra);
    bmp
}

fn headers_with_bmp() -> Headers {
    let mut h = Headers::new();
    h.set("Content-Type", "image/bmp").unwrap();
    h
}

