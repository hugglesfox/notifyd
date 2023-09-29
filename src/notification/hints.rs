use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::zvariant::{DeserializeDict, SerializeDict, Type};

#[derive(Copy, Clone, Serialize_repr, Deserialize_repr, Type)]
#[repr(u8)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

#[derive(Clone, Serialize, Deserialize, Type)]
pub struct ImageData {
    width: i32,
    height: i32,
    rowstride: i32,
    has_alpha: bool,
    bits_per_sample: i32,
    channels: i32,
    image_data: Vec<u8>,
}

#[derive(Clone, SerializeDict, DeserializeDict, Type)]
#[zvariant(signature = "dict")]
pub struct Hints {
    #[zvariant(rename = "action-icons")]
    action_icons: Option<bool>,
    category: Option<String>,
    #[zvariant(rename = "desktop-entry")]
    desktop_entry: Option<String>,
    #[zvariant(rename = "image-data")]
    image_data: Option<ImageData>,
    #[zvariant(rename = "image-path")]
    image_path: Option<String>,
    resident: Option<bool>,
    #[zvariant(rename = "image-file")]
    sound_file: Option<String>,
    #[zvariant(rename = "sound-name")]
    sound_name: Option<String>,
    #[zvariant(rename = "suppress-sound")]
    suppress_sound: Option<bool>,
    transient: Option<bool>,
    x: Option<i32>,
    y: Option<i32>,
    urgency: Option<Urgency>,
}
