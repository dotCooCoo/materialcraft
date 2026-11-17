use crate::{FormatError, Result};
use craft_texture::{Image, Kind, Limits, MAX_SIDE, srgb_to_linear};

pub(crate) fn decode(bytes: &[u8], linear: bool) -> Result<Image> {
    let error = |message: String| FormatError::Image(message);
    let mut limits = craft_texture_dds::Limits::default();
    limits.max_side = MAX_SIDE;
    let decoded = craft_texture_dds::dds::decode_first(bytes, &limits).map_err(|e| match e {
        craft_texture_dds::dds::DecodeError::Image(craft_texture_dds::ImageError::BadSize(_, _)) => error(format!("DDS dimensions must be 1..={MAX_SIDE}")),
        e => error(e.to_string()),
    })?;
    let mut data = decoded.rgba;
    if !linear && !decoded.source_precision_is_f32 {
        for pixel in data.as_chunks_mut::<4>().0 {
            for value in pixel.iter_mut().take(3) {
                *value = srgb_to_linear(*value);
            }
        }
    }

    Image::from_data(decoded.width, decoded.height, Kind::Color, data, &Limits::default()).map_err(|e| error(e.to_string()))
}
