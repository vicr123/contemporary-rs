use std::io;
use std::io::{Cursor, Write};

pub struct GroupIcon {
    id_reserved: u16,
    id_type: u16,
    entries: Vec<GroupIconEntry>,
}

pub struct GroupIconEntry {
    pub width: u8,
    pub height: u8,
    pub color_count: u8,
    pub reserved: u8,
    pub planes: u16,
    pub bit_count: u16,
    pub bytes_in_res: u32,
    pub id: u16,
}

impl Default for GroupIcon {
    fn default() -> Self {
        GroupIcon {
            id_reserved: 0,
            id_type: 1,
            entries: Vec::new(),
        }
    }
}

impl GroupIcon {
    pub fn push_icon(&mut self, icon: GroupIconEntry) {
        self.entries.push(icon);
    }

    pub fn encode(&self) -> io::Result<Vec<u8>> {
        let mut buffer = Cursor::new(Vec::new());

        buffer.write_all(&self.id_reserved.to_le_bytes())?;
        buffer.write_all(&self.id_type.to_le_bytes())?;
        buffer.write_all(&(self.entries.len() as u16).to_le_bytes())?;

        for entry in &self.entries {
            buffer.write_all(&entry.width.to_le_bytes())?;
            buffer.write_all(&entry.height.to_le_bytes())?;
            buffer.write_all(&entry.color_count.to_le_bytes())?;
            buffer.write_all(&entry.reserved.to_le_bytes())?;
            buffer.write_all(&entry.planes.to_le_bytes())?;
            buffer.write_all(&entry.bit_count.to_le_bytes())?;
            buffer.write_all(&entry.bytes_in_res.to_le_bytes())?;
            buffer.write_all(&entry.id.to_le_bytes())?;
        }

        Ok(buffer.into_inner())
    }
}
