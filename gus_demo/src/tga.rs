#[derive(Serialize, Deserialize)]
pub struct TgaHeader {
    id_length: u8,
    color_map_type: u8,
    image_type: u8,
    color_map: [u8; 5],
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    pixel_depth: u8,
    image_descriptor: u8,
}

impl TgaHeader {
    pub fn rgb(width: usize, height: usize) -> Self {
        TgaHeader {
            id_length: 0,
            color_map_type: 0,
            image_type: 2,
            color_map: [0, 0, 0, 0, 0],
            x_origin: 0,
            y_origin: 0,
            width: width as _,
            height: height as _,
            pixel_depth: 24,
            image_descriptor: 0
        }
    }
}
