use crate::ui::immediate::*;

use super::*;

impl InspectRenderDefault<String> for String {
    fn render(data: &[&String], label: &'static str, ui: &mut Ui, _args: &InspectArgsDefault) {
        if data.is_empty() {
            // Values are inconsistent
            // TODO: seems should be RED color
            let label = format!("{}: ", label);
            ui.text(label.as_str(), Align::Left, None);
            return;
        }

        match same_or_none(data) {
            Some(_v) => {
                // Values are consistent
                let label = format!("{}: {}", label, data[0]);
                ui.text(label.as_str(), Align::Left, None);
            }
            None => {
                // Values are inconsistent
                // TODO: seems should be YELLOW color
                let label = format!("{}: ", label);
                ui.text(label.as_str(), Align::Left, None);
            }
        }
    }

    fn render_mut(
        data: &mut [&mut String],
        label: &'static str,
        ui: &mut Ui,
        _args: &InspectArgsDefault,
    ) -> bool {
        let same_or_none_value = same_or_none_mut(data);

        let style_token = if same_or_none_value.is_none() {
            // If values are inconsistent, push a style
            Some(color::YELLOW_5)
        } else {
            None
        };

        let value = match same_or_none_value {
            Some(v) => v,
            None => "".to_string(), // Some reasonable default
        };

        let mut changed = false;
        let mut value = format!("{}", value);
        let label = format!("{}", label);

        // TODO:

        // if ui
        //     .text_input(label.as_str(), &mut value)
        //     .resize_buffer(true)
        //     .build()
        // {
        //     for d in data {
        //         **d = value.to_string();
        //         changed = true;
        //     }
        // }

        // if let Some(style_token) = style_token {
        //     style_token.pop(ui);
        // }

        changed
    }
}
