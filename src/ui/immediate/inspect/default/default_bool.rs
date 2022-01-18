use crate::ui::immediate::*;

use super::*;

impl InspectRenderDefault<bool> for bool {
    fn render(data: &[&bool], label: &'static str, ui: &mut Ui, _args: &InspectArgsDefault) {
        if data.is_empty() {
            // Values are inconsistent
            let label = format!("{}: ", label);
            // TODO: seems should be RED color
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
        data: &mut [&mut bool],
        label: &'static str,
        ui: &mut Ui,
        _args: &InspectArgsDefault,
    ) -> bool {
        let same_or_none_value = same_or_none_mut(data);

        // Some reasonable default
        let mut value = same_or_none_value.unwrap_or(false);

        let style_token = if same_or_none_value.is_none() {
            // If values are inconsistent, push a style
            Some(color::YELLOW_5)
        } else {
            None
        };

        let mut changed = false;
        let label = format!("{}", label);
        // if ui.check(label.as_str(), &mut value) {
        //     for d in data {
        //         **d = value;
        //         changed = true;
        //     }
        // }

        // if let Some(style_token) = style_token {
        //     style_token.pop(ui);
        // }

        changed
    }
}
