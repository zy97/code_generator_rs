#[derive(Debug, Default)]
pub struct App {
    can_exit: bool,
    is_exiting: bool,
}

impl eframe::App for App {
    // fn on_exit_event(&mut self) -> bool {
    //     self.is_exiting = true;
    //     self.can_exit
    // }
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // egui::CentralPanel::default().show(ctx, |ui| ui.heading("Try to close window"));

        custom_window_frame(ctx, frame, "egui with custom frame", |ui| {
            ui.label("This is just the content of the window");
            ui.horizontal(|ui| {
                ui.label("egui theme:");
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
        // if self.is_exiting {
        //     egui::Window::new("Do you want to quit?")
        //         .collapsible(false)
        //         .resizable(false)
        //         .show(ctx, |ui| {
        //             ui.horizontal(|ui| {
        //                 if ui.button("Not yet").clicked() {
        //                     self.is_exiting = false;
        //                 }
        //                 if ui.button("Yes!").clicked() {
        //                     self.can_exit = true;
        //                     frame.quit()
        //                 }
        //             })
        //         });
        // }
    }
}
fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    use egui::*;
    let text_color = ctx.style().visuals.text_color();

    let height = 28.0;

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            painter.rect(
                rect.shrink(1.0),
                10.0,
                ctx.style().visuals.window_fill(),
                Stroke::new(1.0, text_color),
            );
            painter.text(
                rect.center_top() + vec2(0.0, height / 2.0),
                Align2::CENTER_CENTER,
                title,
                FontId::proportional(height - 2.0),
                text_color,
            );
            painter.line_segment(
                [
                    rect.left_top() + vec2(2.0, height),
                    rect.right_top() + vec2(-2.0, height),
                ],
                Stroke::new(1.0, text_color),
            );
            let close_response = ui.put(
                Rect::from_min_size(rect.left_top(), Vec2::splat(height)),
                Button::new(RichText::new("❌").size(height - 4.0)).frame(false),
            );
            if close_response.clicked() {
                frame.quit();
            }
            let title_bar_rect = {
                let mut rect = rect;
                rect.max.y = rect.min.y + height;
                rect
            };
            let title_bar_response =
                ui.interact(title_bar_rect, Id::new("title_bar"), Sense::drag());
            if title_bar_response.drag_started() {
                frame.drag_window();
            }
            let content_rect = {
                let mut rect = rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);
        });
}