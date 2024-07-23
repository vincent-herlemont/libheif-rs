use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::io::Write;

use exif::parse_exif;

use libheif_rs::{
    check_file_type, color_profile_types, Chroma, ChromaDownsamplingAlgorithm, ChromaUpsamplingAlgorithm, ColorPrimaries, ColorProfile, ColorSpace, CompressionFormat, DecodingOptions, EncoderQuality, EncodingOptions, FileTypeResult, HeifContext, ImageHandle, ItemId, LibHeif, MatrixCoefficients, Result, RgbChroma, StreamReader, TransferCharacteristics
};

#[test]
fn read_from_file() -> Result<()> {
    let ctx = HeifContext::read_from_file("./data/IMG_0832.HEIC")?;
    let handle = ctx.primary_image_handle()?;
    assert_eq!(handle.width(), 4032);
    assert_eq!(handle.height(), 3024);

    Ok(())
}

#[test]
fn convert_to_avif() -> Result<()> {
    let read_ctx = HeifContext::read_from_file("./data/IMG_0832.HEIC")?;
    let handle = read_ctx.primary_image_handle()?;

    let lib_heif = LibHeif::new();
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;

    let mut write_context = HeifContext::new()?;
    let mut encoder = lib_heif.encoder_for_format(CompressionFormat::Av1)?;
    write_context.encode_image(&image, &mut encoder, None)?;

    let buf = write_context.write_to_bytes()?;

    let mut file = File::create("./data/IMG_0832.avif").unwrap();
    file.write_all(&buf).unwrap();


    Ok(())
}


fn convert_to_jpg() -> Result<()> {
    let read_ctx = HeifContext::read_from_file("./data/IMG_0832.HEIC")?;
    let handle = read_ctx.primary_image_handle()?;

    let lib_heif = LibHeif::new();
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;

    let mut write_context = HeifContext::new()?;
    let mut encoder = lib_heif.encoder_for_format(CompressionFormat::Jpeg)?;
    encoder.set_quality(EncoderQuality::LossLess)?;
    let encoding_options = EncodingOptions::new().unwrap();
    write_context.encode_image(&image, &mut encoder, Some(encoding_options))?;

    let buf = write_context.write_to_bytes()?;

    let mut file = File::create("./data/IMG_0832.jpg").unwrap();
    file.write_all(&buf).unwrap();

    Ok(())
}

fn convert_to_jpg2000() -> Result<()> {
    let read_ctx = HeifContext::read_from_file("./data/IMG_0832.HEIC")?;
    let handle = read_ctx.primary_image_handle()?;

    let lib_heif = LibHeif::new();
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;

    let mut write_context = HeifContext::new()?;
    let mut encoder = lib_heif.encoder_for_format(CompressionFormat::Jpeg2000)?;
    let encoding_options: EncodingOptions = EncodingOptions::new().unwrap();
    write_context.encode_image(&image, &mut encoder, None)?;

    let buf = write_context.write_to_bytes()?;

    let mut file = File::create("./data/IMG_0832.jpg").unwrap();
    file.write_all(&buf).unwrap();

    Ok(())
}

#[test]
fn convert_to_jpg_with_image_crate() -> Result<()> {
    let read_ctx = HeifContext::read_from_file("./data/IMG_0832.HEIC")?;
    let handle = read_ctx.primary_image_handle()?;

    let lib_heif = LibHeif::new();
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    let planes = image.planes();
    let interleaved_plane = planes.interleaved.unwrap();
    assert_eq!(interleaved_plane.width, 4032);
    assert_eq!(interleaved_plane.height, 3024);
    assert_eq!(interleaved_plane.stride, 12096);
    
    let buffer: &[u8] = interleaved_plane.data;
    let rgb_image = image::RgbImage::from_raw(
        interleaved_plane.width, 
        interleaved_plane.height, 
        buffer.to_vec(),
    ).unwrap();
    let dyn_image = image::DynamicImage::ImageRgb8(rgb_image);  

    let mut file = File::create("./data/IMG_0832.jpg").unwrap();
    dyn_image.write_to(&mut file, image::ImageFormat::Jpeg).unwrap();

    Ok(())
}