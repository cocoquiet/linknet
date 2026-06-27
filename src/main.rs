mod crawler;
mod url;

fn main() -> eframe::Result<()> {
    let mut initial_url = String::new();

    let options = eframe::NativeOptions::default();
    eframe::run_ui_native("LINKNET", options, move |ui, _frame| {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Node Graph for all links in a website");
            ui.horizontal(|ui| {
                let initial_url_label = ui.label("Initial url: ");
                ui.text_edit_singleline(&mut initial_url)
                    .labelled_by(initial_url_label.id);
            });
            if ui.button("Crawl Links").clicked() {
                if !url::validate_url(&initial_url) {
                    eprintln!("Invalid URL: {}", initial_url);
                    std::process::exit(1);
                }

                let url_node = url::Url::new(&initial_url);
                println!("Valid URL: {}", url_node);

                crawler::crawl_recursive(&url_node);
            }
        });
    })
}
