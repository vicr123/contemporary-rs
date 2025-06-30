use anyhow::Error;
use std::fs;
use std::iter::repeat_n;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const HFS_EPOCH: u64 = 2082844800;
const BASE_LENGTH: u16 = 150;

pub struct Alias {
    version: i32,
    volume_name: String,
    volume_created: SystemTime,
    parent_inode: u32,
    target_inode: u32,
    target_filename: String,
    target_created: SystemTime,
    target_type: i32, // file = 0
    extras: Vec<(i32, Vec<u8>)>,
}

impl Alias {
    pub fn alias_for(path: PathBuf, mount_point: PathBuf) -> Result<Self, Error> {
        let parent_dir = path.parent().unwrap();

        let target_metadata = fs::metadata(&path)?;
        let parent_metadata = fs::metadata(parent_dir)?;
        let volume_metadata = fs::metadata(&mount_point)?;

        let volume_name = Self::volume_name(&mount_point)?;
        let volume_created = volume_metadata.created()?;

        let parent_inode = parent_metadata.ino() as u32;

        let target_inode = target_metadata.ino() as u32;
        let target_filename = path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("No filename"))?
            .to_str()
            .unwrap()
            .to_string();
        let target_created = target_metadata.created()?;

        // Add extras
        let mut extras = Vec::new();
        extras.push((
            0,
            parent_dir
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("No parent dirname"))?
                .to_str()
                .unwrap()
                .as_bytes()
                .into(),
        ));

        extras.push((1, parent_inode.to_be_bytes().to_vec()));

        // UTF-16BE encoding of target filename
        let target_filename_utf16: Vec<u8> = target_filename
            .encode_utf16()
            .flat_map(|c| c.to_be_bytes().to_vec())
            .collect();
        let mut target_filename_data = (target_filename.len() as u16).to_be_bytes().to_vec();
        target_filename_data.extend(target_filename_utf16);
        extras.push((14, target_filename_data));

        // UTF-16BE encoding of volume name
        let volume_name_utf16: Vec<u8> = volume_name
            .encode_utf16()
            .flat_map(|c| c.to_be_bytes().to_vec())
            .collect();
        let mut volume_name_data = (volume_name.len() as u16).to_be_bytes().to_vec();
        volume_name_data.extend(volume_name_utf16);
        extras.push((15, volume_name_data));

        // Relative path from volume
        let relative_path = path.strip_prefix(&mount_point)?;
        extras.push((
            18,
            format!("/{}", relative_path.to_str().unwrap()).into_bytes(),
        ));

        // Volume path
        extras.push((19, mount_point.to_str().unwrap().as_bytes().into()));

        Ok(Alias {
            version: 2,
            volume_name,
            volume_created,
            parent_inode,
            target_inode,
            target_filename,
            target_created,
            target_type: 0,
            extras,
        })
    }

    pub fn data(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        // Calculate extra length with padding
        let extra_length: u16 = self
            .extras
            .iter()
            .map(|(_, data)| {
                let padding = data.len() % 2;
                4 + data.len() + padding
            })
            .sum::<usize>() as u16;

        let trailer_length: u16 = 4;
        let total_length = BASE_LENGTH + extra_length + trailer_length;

        buffer.reserve(total_length as usize);

        // Write header
        buffer.extend_from_slice(&0u32.to_be_bytes());
        buffer.extend_from_slice(&total_length.to_be_bytes());
        buffer.extend_from_slice(&(self.version as u16).to_be_bytes());
        buffer.extend_from_slice(&(self.target_type as u16).to_be_bytes());

        // Write volume info
        buffer.push(self.volume_name.len() as u8);
        Self::write_padded_string(&mut buffer, &self.volume_name, 27);

        let volume_time = self
            .volume_created
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            + HFS_EPOCH;
        buffer.extend_from_slice(&(volume_time as u32).to_be_bytes());

        buffer.extend_from_slice(b"H+");
        buffer.extend_from_slice(&5u16.to_be_bytes()); // Type: other

        // Write inode and filename info
        buffer.extend_from_slice(&self.parent_inode.to_be_bytes());
        buffer.push(self.target_filename.len() as u8);
        Self::write_padded_string(&mut buffer, &self.target_filename, 63);
        buffer.extend_from_slice(&self.target_inode.to_be_bytes());

        let target_time = self
            .target_created
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            + HFS_EPOCH;
        buffer.extend_from_slice(&(target_time as u32).to_be_bytes());

        // Write fixed fields
        buffer.extend_from_slice(&[0u8; 8]);
        buffer.extend_from_slice(&(-1i16).to_be_bytes());
        buffer.extend_from_slice(&(-1i16).to_be_bytes());
        buffer.extend_from_slice(&0x00000D02u32.to_be_bytes());
        buffer.extend_from_slice(&0u16.to_be_bytes());
        buffer.extend_from_slice(&[0u8; 10]);

        // Write extras
        for (type_code, data) in &self.extras {
            buffer.extend_from_slice(&(*type_code as i16).to_be_bytes());
            buffer.extend_from_slice(&(data.len() as u16).to_be_bytes());
            buffer.extend_from_slice(data);
            if data.len() % 2 == 1 {
                buffer.push(0);
            }
        }

        // Write trailer
        buffer.extend_from_slice(&(-1i16).to_be_bytes());
        buffer.extend_from_slice(&0u16.to_be_bytes());

        buffer
    }

    fn write_padded_string(buffer: &mut Vec<u8>, s: &str, length: usize) {
        let bytes = s.as_bytes();
        buffer.extend_from_slice(bytes);
        if bytes.len() < length {
            buffer.extend(repeat_n(0, length - bytes.len()));
        }
    }

    fn volume_name(volume_path: &Path) -> anyhow::Result<String> {
        Ok(volume_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("No volume name"))?
            .to_string_lossy()
            .into_owned())
    }
}
