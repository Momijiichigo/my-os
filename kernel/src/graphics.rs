// use bootloader_api::info::FrameBuffer;
// use embedded_graphics::{
//     draw_target::DrawTarget,
//     geometry::Dimensions,
//     mono_font::{ascii::FONT_6X10, MonoTextStyle},
//     pixelcolor::BinaryColor,
//     prelude::*,
//     text::{Alignment, Text},
// };
// pub struct FrameBufferDisplay {
//     
// }
// 
// impl DrawTarget for FrameBufferDisplay {
//     type Color = BinaryColor;
//     type Error = core::convert::Infallible;
// 
// }
// 
// impl FrameBufferDisplay {
//     fn new() -> Self {
//         FrameBufferDisplay {  }
//     }
// }
// 
// pub fn print_something() -> Result<(), core::convert::Infallible> {
//     let mut display = FrameBufferDisplay::new();
//     
//     let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
//     // Draw centered text.
//     let text = "embedded-graphics";
//     Text::with_alignment(
//         text,
//         display.bounding_box().center() + Point::new(0, 15),
//         character_style,
//         Alignment::Center,
//     )
//     .draw(&mut display)?;
// 
//     Ok(())
// }
