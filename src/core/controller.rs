use crate::core::ai_models;
use crate::core::database;
use crate::core::encoder;
use crate::core::generator;

use crate::core::models::image::ImageData;
use diffusion_rs::api::Progress;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub fn generate(
    text: String,
    image: Option<ImageData>,
    key_str: String,
    progress_tx: Sender<Progress>,
    progress_rx: &Receiver<Progress>,
    result_tx: Sender<std::result::Result<Vec<u8>, std::string::String>>,
    result_rx: &Receiver<std::result::Result<Vec<u8>, std::string::String>>,
) {
    generator::generate(
        text,
        image,
        key_str,
        progress_tx,
        progress_rx,
        result_tx,
        result_rx,
    );
}

pub fn base64_encode(data: &[u8]) -> String {
    encoder::base64_encode(data)
}

pub fn model_list() -> HashMap<String, String> {
    ai_models::model_list()
}

pub fn get_history() -> Result<Vec<ImageData>, String> {
    database::Database::get_db().get_conversations()
}
