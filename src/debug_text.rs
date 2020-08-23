use ggez::{Context, GameResult, nalgebra as na};
use ggez::graphics::{self, Text, TextFragment, Color, Font, Scale, DrawParam, FilterMode};

pub struct DebugText {}

impl DebugText {
    pub fn new() -> DebugText {
        DebugText {}
    }

    pub fn draw(&mut self,
                context: &mut Context,
                state_text_content: String,
                state_frame_text_content: String,
                x_velocity_text_content: String,
                y_velocity_text_content: String,
                x_axis_text_content: String,
                y_axis_text_content: String,
                text_x: f32,
                text_y: f32) -> GameResult {

        let color = Some(Color::new(1.0, 1.0, 1.0, 1.0));
        let font = Some(Font::default());
        let scale = Some(Scale::uniform(30.0));

        let state_text = Text::new(TextFragment {text: state_text_content, color: color, font: font, scale: scale});
        let state_frame_text = Text::new(TextFragment {text: state_frame_text_content, color: color, font: font, scale: scale});
        let x_velocity_text = Text::new(TextFragment {text: x_velocity_text_content, color: color, font: font, scale: scale});
        let y_velocity_text = Text::new(TextFragment {text: y_velocity_text_content, color: color, font: font, scale: scale});
        let x_axis_text = Text::new(TextFragment {text: x_axis_text_content, color: color, font: font, scale: scale});
        let y_axis_text = Text::new(TextFragment {text: y_axis_text_content, color: color, font: font, scale: scale});

        graphics::queue_text(context, &state_text, na::Point2::new(text_x, text_y), None);
        graphics::queue_text(context, &state_frame_text, na::Point2::new(text_x + 200.0, text_y), None);
        graphics::queue_text(context, &x_velocity_text, na::Point2::new(text_x - 200.0, text_y), None);
        graphics::queue_text(context, &y_velocity_text, na::Point2::new(text_x - 200.0, text_y - 25.0), None);
        graphics::queue_text(context, &x_axis_text, na::Point2::new(text_x - 340.0, text_y), None);
        graphics::queue_text(context, &y_axis_text, na::Point2::new(text_x - 340.0, text_y - 25.0), None);

        graphics::draw_queued_text(
            context,
            DrawParam::default(),
            None,
            FilterMode::Linear,
        )?;

        Ok(())
    }
}
