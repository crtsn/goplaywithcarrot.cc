use worker::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Position {
    x: u32,
    y: u32,
}

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
        let mut x = 0;
        let mut y = 0;
        let db = env.d1("game_db")?;
        let char_data;
        let char_id;
        match subdomain {
          Some("xn--4o8h") => {
              char_data = RABBIT_RAW;
              char_id = 0;
          }
          Some("xn--yn8h") => {
              char_data = RABBIT2_RAW;
              char_id = 1;
          }
          Some("xn--dp8h") => {
              char_data = FROG_RAW;
              char_id = 2;
          }
          _ => {
              char_data = HEDGEHOG_RAW;
              char_id = 3;
          }
        };

        let prep = db.prepare("UPDATE players SET x = x + 1 WHERE x < 250 AND id = ? RETURNING x, y");
        let mut canvas = MAP_RAW.to_vec();

        let row = db.batch(vec![
            prep.bind(&[char_id.into()])?
        ]).await?;

        let results: Result<Vec<Position>, _> = row[0].results();
        if let Ok([Position{x: new_x, y: new_y}]) = results.as_deref() {
            console_log!("{:?}", results);
            x = *new_x;
            y = *new_y;
        }
        fast_raw_overlay(&mut canvas, 250, &char_data, 72, 72, x, y);
        let png_bytes = encode_png_manual(&canvas, 250, 250);
        let png_size = png_bytes.len();
        return Ok(Response::from_bytes(png_bytes)?.with_headers(headers_with_png(png_size)));
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

fn encode_png_manual(rgba_pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut png = Vec::with_capacity(rgba_pixels.len() + 256);

    // 1. PNG Signature
    png.extend_from_slice(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);

    // 2. IHDR Chunk
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]); // 8bit, RGBA, Deflate, No Filter, No Interlace
    write_chunk(&mut png, b"IHDR", &ihdr);

    // 3. IDAT Chunk (The Pixel Data)
    // We add a '0' byte before every row (Filter Type: None)
    let mut raw_data = Vec::with_capacity((width * height * 4 + height) as usize);
    for row in rgba_pixels.chunks_exact((width * 4) as usize) {
        raw_data.push(0);
        raw_data.extend_from_slice(row);
    }

    // Zlib Wrapper (No Compression mode)
    let mut idat = Vec::new();
    idat.extend_from_slice(&[0x78, 0x01]); // Zlib Header (No compression)

    let mut pos = 0;
    while pos < raw_data.len() {
        let chunk_size = std::cmp::min(raw_data.len() - pos, 65535);
        let last_block = if pos + chunk_size >= raw_data.len() { 1 } else { 0 };

        idat.push(last_block); // BFINAL and BTYPE (00)
        idat.extend_from_slice(&(chunk_size as u16).to_le_bytes());
        idat.extend_from_slice(&(!(chunk_size as u16)).to_le_bytes());
        idat.extend_from_slice(&raw_data[pos..pos + chunk_size]);
        pos += chunk_size;
    }

    // Adler-32 Checksum for Zlib
    let mut a: u32 = 1; let mut b: u32 = 0;
    for &byte in &raw_data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    idat.extend_from_slice(&((b << 16) | a).to_be_bytes());

    write_chunk(&mut png, b"IDAT", &idat);

    // 4. IEND Chunk
    write_chunk(&mut png, b"IEND", &[]);

    png
}

fn write_chunk(png: &mut Vec<u8>, name: &[u8; 4], data: &[u8]) {
    png.extend_from_slice(&(data.len() as u32).to_be_bytes());
    png.extend_from_slice(name);
    png.extend_from_slice(data);

    // CRC-32 (Standard PNG polynomial)
    let mut c = 0xFFFFFFFFu32;
    for b in name.iter().chain(data.iter()) {
        c ^= *b as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 { (c >> 1) ^ 0xEDB88320 } else { c >> 1 };
        }
    }
    png.extend_from_slice(&(c ^ 0xFFFFFFFFu32).to_be_bytes());
}

fn headers_with_png(png_size: usize) -> Headers {
    let h = Headers::new();
    h.set("Content-Type", "image/png").unwrap();
    h.set("Content-Length", &png_size.to_string()).unwrap();
    h
}

