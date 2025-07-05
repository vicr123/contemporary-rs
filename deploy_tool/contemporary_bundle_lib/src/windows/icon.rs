use std::io;
use std::io::{Cursor, Write};
use crate::windows::group_icon::GroupIconEntry;

pub struct Icon {
    width: u32,
    height: u32,
    planes: u16,
    bits_per_pixel: u16,
    compression: u32,
    x_pels_per_meter: i32,
    y_pels_per_meter: i32,
    clr_used: u32,
    clr_important: u32,
    image_data: Vec<u8>,

    pub icon_id: u16
}

impl Icon {
    pub fn new(width: u32, height: u32, icon_id: u16, image_data: Vec<u8>) -> Self {
        Icon {
            width,
            height,
            planes: 1,
            bits_per_pixel: 32,
            compression: 0,
            x_pels_per_meter: 0,
            y_pels_per_meter: 0,
            clr_used: 0,
            clr_important: 0,
            image_data,
            icon_id
        }
    }

    pub fn new_from_rgba(width: u32, height: u32, icon_id: u16, rgba_pixels: Vec<u8>) -> Self {
        let mut data = Vec::new();

        // Color data (RGBA pixels, must be bottom-up)
        for y in (0..height).rev() {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                data.push(rgba_pixels[idx + 2]); // B
                data.push(rgba_pixels[idx + 1]); // G
                data.push(rgba_pixels[idx]);     // R
                data.push(rgba_pixels[idx + 3]); // A
            }
        }

        Icon::new(width, height, icon_id, data)
    }

    pub fn encode(&self) -> io::Result<Vec<u8>> {
        let mut buffer = Cursor::new(Vec::new());

        let stride = ((self.width + 31) / 32) * 4;
        let mask_size = stride * self.height;

        // Write the header
        buffer.write_all(&40u32.to_le_bytes())?;
        buffer.write_all(&self.width.to_le_bytes())?;
        buffer.write_all(&(self.height * 2).to_le_bytes())?;
        buffer.write_all(&self.planes.to_le_bytes())?;
        buffer.write_all(&self.bits_per_pixel.to_le_bytes())?;
        buffer.write_all(&self.compression.to_le_bytes())?;
        buffer.write_all(&((self.width * self.height * 4) + mask_size).to_le_bytes())?;
        buffer.write_all(&self.x_pels_per_meter.to_le_bytes())?;
        buffer.write_all(&self.y_pels_per_meter.to_le_bytes())?;
        buffer.write_all(&self.clr_used.to_le_bytes())?;
        buffer.write_all(&self.clr_important.to_le_bytes())?;

        buffer.write_all(&*self.image_data.clone())?;

        // AND mask (all zeros for 32-bit icons)
        buffer.write_all(&*vec![0u8; mask_size as usize])?;

        Ok(buffer.into_inner())
    }

    pub fn group_icon_entry(&self) -> io::Result<GroupIconEntry> {
        Ok(GroupIconEntry {
            width: self.width as u8,
            height: self.height as u8,
            color_count: 0,
            reserved: 0,
            planes: self.planes,
            bit_count: self.bits_per_pixel,
            bytes_in_res: self.encode()?.len() as u32,
            id: self.icon_id,
        })
    }
}