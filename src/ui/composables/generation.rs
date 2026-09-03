use crate::core::{controller, models::image::ImageData};

pub fn get_history() -> Result<Vec<ImageData>, String> {
    controller::get_history()
}
