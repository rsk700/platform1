use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    render::texture::BevyDefault,
};

fn color_f32_to_u8(c: f32) -> u8 {
    if c == 1.0 {
        255
    } else {
        (256.0 * c) as u8
    }
}

pub fn image(width: usize, height: usize, color: Color) -> Image {
    let mut bytes = Vec::with_capacity(width * height * 4);
    let color = color.as_rgba_f32();
    let (r, g, b, a) = (
        color_f32_to_u8(color[0]).to_le_bytes(),
        color_f32_to_u8(color[1]).to_le_bytes(),
        color_f32_to_u8(color[2]).to_le_bytes(),
        color_f32_to_u8(color[3]).to_le_bytes(),
    );
    let texture_format = TextureFormat::bevy_default();
    let pixel = match texture_format {
        TextureFormat::Rgba8UnormSrgb => [r, g, b, a].concat(),
        TextureFormat::Bgra8UnormSrgb => [b, g, r, a].concat(),
        _ => unimplemented!("unsuported TextureFormat"),
    };
    for _ in 0..width * height {
        bytes.extend(pixel.clone());
    }
    Image::new(
        Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        bytes,
        texture_format,
    )
}

pub fn image_1px(color: Color) -> Image {
    let mut bytes = Vec::with_capacity(4);
    let color = color.as_rgba_f32();
    let (r, g, b, a) = (
        color_f32_to_u8(color[0]).to_le_bytes(),
        color_f32_to_u8(color[1]).to_le_bytes(),
        color_f32_to_u8(color[2]).to_le_bytes(),
        color_f32_to_u8(color[3]).to_le_bytes(),
    );
    let texture_format = TextureFormat::bevy_default();
    match texture_format {
        TextureFormat::Rgba8UnormSrgb => bytes.extend([r, g, b, a].concat()),
        TextureFormat::Bgra8UnormSrgb => bytes.extend([b, g, r, a].concat()),
        _ => unimplemented!("unsuported TextureFormat"),
    }
    Image::new(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        bytes,
        texture_format,
    )
}
