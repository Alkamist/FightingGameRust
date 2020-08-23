use ggez::{Context, GameResult, nalgebra as na};
use ggez::graphics::{self, Text, TextFragment, Color, Font, Scale, DrawParam, FilterMode};

pub struct DebugText {
    state_text: Text,
    state_frame_text: Text,
    x_velocity_text: Text,
    y_velocity_text: Text,
    x_axis_text: Text,
    y_axis_text: Text,
    text_color: Color,
    text_font: Font,
    text_scale: Scale,
}

impl DebugText {
    pub fn new() -> DebugText {
        DebugText {
            state_text: Text::new(""),
            state_frame_text: Text::new(""),
            x_velocity_text: Text::new(""),
            y_velocity_text: Text::new(""),
            x_axis_text: Text::new(""),
            y_axis_text: Text::new(""),
            text_color: Color::new(1.0, 1.0, 1.0, 1.0),
            text_font: Font::default(),
            text_scale: Scale::uniform(30.0),
        }
    }

    pub fn update_text(&mut self,
                       state_text_content: String,
                       state_frame_text_content: String,
                       x_velocity_text_content: String,
                       y_velocity_text_content: String,
                       x_axis_text_content: String,
                       y_axis_text_content: String) {
        let color = Some(self.text_color);
        let font = Some(self.text_font);
        let scale = Some(self.text_scale);
        self.state_text = Text::new(TextFragment {text: state_text_content, color: color, font: font, scale: scale});
        self.state_frame_text = Text::new(TextFragment {text: state_frame_text_content, color: color, font: font, scale: scale});
        self.x_velocity_text = Text::new(TextFragment {text: x_velocity_text_content, color: color, font: font, scale: scale});
        self.y_velocity_text = Text::new(TextFragment {text: y_velocity_text_content, color: color, font: font, scale: scale});
        self.x_axis_text = Text::new(TextFragment {text: x_axis_text_content, color: color, font: font, scale: scale});
        self.y_axis_text = Text::new(TextFragment {text: y_axis_text_content, color: color, font: font, scale: scale});
    }

    pub fn draw(&mut self, context: &mut Context, text_x: f32, text_y: f32) -> GameResult {
        graphics::queue_text(context, &self.state_text, na::Point2::new(text_x, text_y), None);
        graphics::queue_text(context, &self.state_frame_text, na::Point2::new(text_x + 200.0, text_y), None);
        graphics::queue_text(context, &self.x_velocity_text, na::Point2::new(text_x - 200.0, text_y), None);
        graphics::queue_text(context, &self.y_velocity_text, na::Point2::new(text_x - 200.0, text_y - 25.0), None);
        graphics::queue_text(context, &self.x_axis_text, na::Point2::new(text_x - 340.0, text_y), None);
        graphics::queue_text(context, &self.y_axis_text, na::Point2::new(text_x - 340.0, text_y - 25.0), None);

        graphics::draw_queued_text(
            context,
            DrawParam::default(),
            None,
            FilterMode::Linear,
        )?;

        Ok(())
    }
}
