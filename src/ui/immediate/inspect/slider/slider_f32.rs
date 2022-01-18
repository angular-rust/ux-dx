use crate::ui::immediate::*;

use super::*;

impl InspectRenderSlider<f32> for f32 {
    fn render(data: &[&Self], label: &'static str, ui: &mut Ui, _args: &InspectArgsSlider) {
        if data.is_empty() {
            let label = format!("{}: None", label);
            ui.text(label.as_str(), Align::Left, None);
            return;
        }

        let label = format!("{}: {}", label, data[0]);
        ui.text(label.as_str(), Align::Left, None);
    }

    fn render_mut(
        data: &mut [&mut Self],
        label: &'static str,
        ui: &mut Ui,
        args: &InspectArgsSlider,
    ) -> bool {
        let same_or_none_value = same_or_none_mut(data);

        // Some reasonable default
        let mut value = same_or_none_value.unwrap_or(0.0);

        let style_token = if same_or_none_value.is_none() {
            // If values are inconsistent, push a style
            Some(color::YELLOW_5)
        } else {
            None
        };

        let mut min = -100.0;
        let mut max = 100.0;
        if let Some(min_value) = args.min_value {
            min = min_value;
        }

        if let Some(max_value) = args.max_value {
            max = max_value;
        }

        let mut changed = false;
        let label = format!("{}", label);
        // if ui.slider(label.as_str()))
        //     .range(std::ops::RangeInclusive::new(min, max))
        //     .build(ui, &mut value)
        // {
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
