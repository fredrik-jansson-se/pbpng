use clap::Parser;

#[derive(Parser)]
struct Opts {
    /// Path to image file
    image_file: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let mut cb = arboard::Clipboard::new()?;
    let cb_img = cb.get_image()?;

    let img = image::RgbaImage::from_raw(
        cb_img.width as _,
        cb_img.height as _,
        cb_img.bytes.to_vec(),
    ).ok_or(anyhow::anyhow!("Failed to store image"))?;

    img.save_with_format(&opts.image_file, image::ImageFormat::Png)?;

    Ok(())
}
