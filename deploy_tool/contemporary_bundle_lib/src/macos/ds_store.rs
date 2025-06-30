use plist::{to_writer_binary, Dictionary, Value};

pub struct DSStore {
    entries: Vec<DSStoreEntry>,
}

pub struct DSStoreEntry {
    file_name: String,
    structure_id: &'static str,
    data_type: &'static str,
    buffer: Vec<u8>,
}

impl DSStore {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let mut modified = Vec::new();
        let p = 0_i32;
        let count = self.entries.len() as i32;

        // Write P and count in big-endian
        modified.extend_from_slice(&p.to_be_bytes());
        modified.extend_from_slice(&count.to_be_bytes());

        // Write all entry buffers
        for entry in &self.entries {
            modified.extend_from_slice(&entry.get_bytes());
        }

        // Read the template DSStore file from resources
        let template_dsstore = include_bytes!("../../assets/DSStore-clean");
        let mut final_buffer = template_dsstore.to_vec();

        // Modify the count at offset 76 (4-byte big-endian)
        final_buffer[76..80].copy_from_slice(&count.to_be_bytes());

        // Write the modified buffer at offset 4100
        final_buffer[4100..4100 + modified.len()].copy_from_slice(modified.as_slice());

        final_buffer
    }

    pub fn push_entry(&mut self, entry: DSStoreEntry) -> &Self {
        self.entries.push(entry);
        self
    }
}

impl DSStoreEntry {
    pub fn new_v_srn(file_name: &str, value: u32) -> Self {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&value.to_be_bytes());
        DSStoreEntry {
            file_name: file_name.into(),
            structure_id: "vSrn",
            data_type: "long",
            buffer,
        }
    }

    pub fn new_iloc(file_name: &str, x: u32, y: u32) -> Self {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&12_i32.to_be_bytes());
        buffer.extend_from_slice(&x.to_be_bytes());
        buffer.extend_from_slice(&y.to_be_bytes());
        buffer.extend_from_slice(b"\xFF\xFF\xFF\x00".as_ref());

        DSStoreEntry {
            file_name: file_name.into(),
            structure_id: "Iloc",
            data_type: "blob",
            buffer,
        }
    }

    pub fn new_bwsp(file_name: &str, x: u32, y: u32, width: u32, height: u32) -> Self {
        let mut buffer = Vec::new();

        let mut plist_dictionary = Dictionary::new();
        plist_dictionary.insert("ContainerShowSidebar".into(), Value::Boolean(true));
        plist_dictionary.insert("ShowPathbar".into(), Value::Boolean(false));
        plist_dictionary.insert("ShowSidebar".into(), Value::Boolean(true));
        plist_dictionary.insert("ShowStatusBar".into(), Value::Boolean(false));
        plist_dictionary.insert("ShowTabView".into(), Value::Boolean(false));
        plist_dictionary.insert("ShowToolbar".into(), Value::Boolean(false));
        plist_dictionary.insert("SidebarWidth".into(), Value::Integer(0.into()));
        plist_dictionary.insert(
            "WindowBounds".into(),
            Value::String(format!("{{{{{x},{y}}},{{{width},{height}}}}}").to_string()),
        );

        let mut plist_buffer = Vec::new();
        to_writer_binary(&mut plist_buffer, &plist_dictionary).unwrap();
        buffer.extend_from_slice(&(plist_buffer.len() as u32).to_be_bytes());
        buffer.extend_from_slice(&plist_buffer);

        DSStoreEntry {
            file_name: file_name.into(),
            structure_id: "bwsp",
            data_type: "blob",
            buffer,
        }
    }

    pub fn new_icvp(file_name: &str, icon_size: u32, background_alias: Vec<u8>) -> Self {
        let mut buffer = Vec::new();

        let mut plist_dictionary = Dictionary::new();
        plist_dictionary.insert("backgroundType".into(), Value::Integer(2.into()));
        plist_dictionary.insert("backgroundColorRed".into(), Value::Real(1.0));
        plist_dictionary.insert("backgroundColorGreen".into(), Value::Real(1.0));
        plist_dictionary.insert("backgroundColorBlue".into(), Value::Real(1.0));
        plist_dictionary.insert("showIconPreview".into(), Value::Boolean(true));
        plist_dictionary.insert("showItemInfo".into(), Value::Boolean(false));
        plist_dictionary.insert("textSize".into(), Value::Integer(12.into()));
        plist_dictionary.insert("iconSize".into(), Value::Integer(icon_size.into()));
        plist_dictionary.insert("viewOptionsVersion".into(), Value::Integer(1.into()));
        plist_dictionary.insert("gridSpacing".into(), Value::Real(100.0));
        plist_dictionary.insert("gridOffsetX".into(), Value::Real(0.0));
        plist_dictionary.insert("gridOffsetY".into(), Value::Real(0.0));
        plist_dictionary.insert("labelOnBottom".into(), Value::Boolean(true));
        plist_dictionary.insert("arrangeBy".into(), Value::String("none".into()));
        plist_dictionary.insert("backgroundImageAlias".into(), Value::Data(background_alias));

        let mut plist_buffer = Vec::new();
        to_writer_binary(&mut plist_buffer, &plist_dictionary).unwrap();
        buffer.extend_from_slice(&(plist_buffer.len() as u32).to_be_bytes());
        buffer.extend_from_slice(&plist_buffer);

        DSStoreEntry {
            file_name: file_name.into(),
            structure_id: "icvp",
            data_type: "blob",
            buffer,
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&(self.file_name.len() as u32).to_be_bytes());

        for c in self.file_name.encode_utf16() {
            buffer.extend_from_slice(&c.to_be_bytes());
        }

        buffer.extend_from_slice(self.structure_id.as_bytes());
        buffer.extend_from_slice(self.data_type.as_bytes());
        buffer.extend_from_slice(&self.buffer);

        buffer
    }
}
