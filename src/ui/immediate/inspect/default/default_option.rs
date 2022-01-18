use crate::ui::immediate::*;

use super::*;

impl<T: InspectRenderDefault<T>> InspectRenderDefault<Option<T>> for Option<T> {
    fn render(data: &[&Option<T>], label: &'static str, ui: &mut Ui, args: &InspectArgsDefault) {
        if data.is_empty() {
            let label = format!("{}: None", label);
            ui.text(label.as_str(), Align::Left, None);
            return;
        }

        let d = data[0];
        match d {
            Some(value) => <T as InspectRenderDefault<T>>::render(&[value], label, ui, args),
            None => {
                let label = format!("{}: None", label);
                ui.text(label.as_str(), Align::Left, None);
            }
        };
    }

    fn render_mut(
        data: &mut [&mut Option<T>],
        label: &'static str,
        ui: &mut Ui,
        args: &InspectArgsDefault,
    ) -> bool {
        if data.is_empty() {
            let label = format!("{}: None", label);
            ui.text(label.as_str(), Align::Left, None);
            return false;
        }

        let d = &mut data[0];
        match d {
            Some(value) => {
                <T as InspectRenderDefault<T>>::render_mut(&mut [value], label, ui, args)
            }
            None => {
                let label = format!("{}: None", label);
                ui.text(label.as_str(), Align::Left, None);
                false
            }
        }
    }
}
