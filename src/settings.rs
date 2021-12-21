use lazy_static::lazy_static;

use barn::fonts::font_details::FontDetails;

pub const DEBUG: bool = true;
pub const TITLE: &str = "Mind's Eye";

lazy_static! {
    pub static ref FONT_DETAILS: FontDetails = FontDetails{path: "res/fonts/VeniceClassic.ttf", size: 19};
}
