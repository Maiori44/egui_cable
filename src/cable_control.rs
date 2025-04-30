use egui::{Order, Pos2, Widget};

use crate::{cable::CableId, custom_widget::CustomWidget, state::State};

#[derive(Debug)]
pub struct CableControl {
    pub(crate) id: CableId,
    pub(crate) pos: Pos2,
    pub(crate) widget: CustomWidget,
}

impl Widget for CableControl {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut state = State::get_cloned(ui);
        let size = state.cable_control_size(&self.id);
        egui::Area::new(egui::Id::new((self.id, "cable_control")))
            // must be top-left of the widget
            .current_pos(if let Some(size) = size {
                self.pos - size / 2.0
            } else {
                self.pos
            })
            .order(Order::Foreground)
            .show(ui.ctx(), |ui| {
                if size.is_none() {
                    // hide cable control for first rendering.
                    ui.set_invisible();
                }
                // should be displayed on cable bezier
                ui.ctx().move_to_top(ui.layer_id());

                // cable control has click sense for make cable active, and drag sense for bezier deforming.
                let response = self.widget.ui(ui);

                // update cable control size for calculate the next position of this area
                state.update_cable_control_size(self.id, response.rect.size());
                state.store_to(ui);

                response
            })
            .inner
    }
}
