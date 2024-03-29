use lazy_static::lazy_static;

use barn::fonts::font_details::FontDetails;

pub const DEBUG: bool = false;
pub const TITLE: &str = "Mind's Eye";
pub const ENABLE_SOUND: bool = true;

lazy_static! {
    pub static ref FONT_DETAILS: FontDetails = FontDetails{path: "res/fonts/VeniceClassic.ttf", size: 19};
}
