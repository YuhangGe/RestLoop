use std::sync::Arc;

use eframe::egui::{Context, FontData, FontDefinitions, FontFamily};

pub fn setup_fonts(ctx: &Context) {
  let mut fonts = FontDefinitions::default();

  let font_data = FontData::from_owned(include_bytes!("./opposans.ttf").to_vec());
  // Insert the Chinese font
  fonts
    .font_data
    .insert("chinese".to_owned(), Arc::new(font_data));

  // Configure font families
  fonts
    .families
    .entry(FontFamily::Proportional)
    .or_default()
    .insert(0, "chinese".to_owned());
  fonts
    .families
    .entry(FontFamily::Monospace)
    .or_default()
    .insert(0, "chinese".to_owned());

  // Apply the font configuration
  ctx.set_fonts(fonts);
}
