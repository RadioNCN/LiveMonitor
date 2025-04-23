mod server;
mod plt_window;

use tokio::{self, time};
use std::collections::HashMap;
use tokio::sync::{mpsc, Mutex};
use rayon::prelude::*;
use eframe::egui::{self, SidePanel, CentralPanel, Visuals, Window, Button};
use egui_plotter::EguiBackend;
use plotters::prelude::*;

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Monitor",
        native_options,
        Box::new(|cc| Ok(Box::new(Monitor::new(cc)))),
    )
    .unwrap();
}


struct Monitor {
    data_rx: mpsc::Receiver<(String,(f64,f64))>,
    data_db: HashMap<String,Vec<(f64,f64)>>,
    keys_for_plots: HashMap<String, bool>
}

impl Monitor {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Disable feathering as it causes artifacts
        let context = &cc.egui_ctx;

        context.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });

        // Also enable light mode
        context.set_visuals(Visuals::light());
        let (tx, mut rx) = mpsc::channel(100);
        tokio::spawn(server::ConnectionManager(tx));

        Self{data_rx: rx, data_db: HashMap::new(), keys_for_plots: HashMap::new()}
    }
}

impl eframe::App for Monitor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while self.data_rx.is_empty() == false {
            match self.data_rx.try_recv() {
                Ok((id, data)) => {
                    if self.data_db.contains_key(&id){
                        if let Some(x) = self.data_db.get_mut(&id){
                            x.push(data);
                            if x.len() > 1000 {
                                x.remove(0);
                            }
                        }
                    }else{
                        self.data_db.insert(id, vec![data]);
                    }
                }
                Err(e) => {}
            }
        }
        CentralPanel::default().show(ctx, |ui| {
            for key in self.keys_for_plots.keys(){
                plt_window::new(ctx, key, self.data_db.clone())
            }
        });
        SidePanel::left("Data Channels")
            .frame(egui::Frame::new().fill(egui::Color32::from_rgb(0,100,180)))
            .show(ctx, |ui| {
                // ui.set_height(ui.available_height());
                // ui.set_width(ui.available_width());
                for key in self.data_db.keys(){
                    if ui.add(Button::new(key)).clicked(){
                        if self.keys_for_plots.contains_key(key){
                            self.keys_for_plots.remove(key);
                        }else{
                            self.keys_for_plots.insert((*key).clone(), true);
                        }
                    }
                };
            });
        ctx.request_repaint_after(time::Duration::from_millis(30));
    }
}

