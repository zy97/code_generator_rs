#[derive(Debug, Default)]
pub struct App {
    can_exit: bool,
    is_exiting: bool,
}

impl eframe::App for App {
    fn on_exit_event(&mut self) -> bool {
        self.is_exiting = true;
        self.can_exit
    }
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| ui.heading("Try to close window"));

        if self.is_exiting {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Not yet").clicked() {
                            self.is_exiting = false;
                        }
                        if ui.button("Yes!").clicked() {
                            self.can_exit = true;
                            frame.quit()
                        }
                    })
                });
        }
    }
}
