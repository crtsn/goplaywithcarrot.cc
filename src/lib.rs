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
        static MAP_RAW: &[u8] = include_bytes!("../public/map.raw");
        static RABBIT_RAW: &[u8] = include_bytes!("../public/rabbit.raw");
        static RABBIT2_RAW: &[u8] = include_bytes!("../public/rabbit2.raw");
        static FROG_RAW: &[u8] = include_bytes!("../public/frog.raw");
        static HEDGEHOG_RAW: &[u8] = include_bytes!("../public/hedgehog.raw");

        let char_data = match subdomain {
          Some("xn--4o8h") => RABBIT_RAW,
          Some("xn--yn8h") => RABBIT2_RAW,
          Some("xn--dp8h") => FROG_RAW,
          _ => HEDGEHOG_RAW,
        };
        let mut canvas = MAP_RAW.to_vec();
        fast_raw_overlay(&mut canvas, 72, &char_data, 72, 72, 10, 10);
        let bmp_bytes = encode_bmp_fast(&canvas, 72, 72);
        let bmp_size = bmp_bytes.len();

        return Ok(Response::from_bytes(bmp_bytes)?.with_headers(headers_with_bmp(bmp_size)));
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


fn fast_raw_overlay(base: &mut [u8], base_w: u32, top: &[u8], top_w: u32, top_h: u32, ox: u32, oy: u32) {
    for y in 0..top_h {
        for x in 0..top_w {
            let top_idx = ((y * top_w + x) * 4) as usize;
            let base_idx = (((y + oy) * base_w + (x + ox)) * 4) as usize;

            let alpha = top[top_idx + 3] as f32 / 255.0;

            if alpha >= 1.0 {
                // Fully opaque: Direct copy
                base[base_idx..base_idx + 3].copy_from_slice(&top[top_idx..top_idx + 3]);
            } else if alpha > 0.0 {
                // Alpha blend
                for i in 0..3 {
                    base[base_idx + i] = ((top[top_idx + i] as f32 * alpha) +
                                         (base[base_idx + i] as f32 * (1.0 - alpha))) as u8;
                }
            }
            // If alpha is 0.0, we do nothing (transparent)
        }
    }
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

fn headers_with_bmp(bmp_size: usize) -> Headers {
    let mut h = Headers::new();
    h.set("Content-Type", "image/bmp").unwrap();
    h.set("Content-Length", &bmp_size.to_string()).unwrap();
    h
}

