mod server;
mod plt_window;
mod guide;

use tokio::{self, time};
use std::collections::HashMap;
use tokio::sync::{mpsc};
use rayon::prelude::*;
use eframe::egui::{self, SidePanel, CentralPanel, Visuals, Window, Button};
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
    plotpara: HashMap<String,plt_window::Plotpara>,
    keys_for_plots: HashMap<String, bool>,
    enGuide: bool
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

        Self{data_rx: rx,
            data_db: HashMap::new(), keys_for_plots: HashMap::new(),
            plotpara: HashMap::new(),
            enGuide: false}
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
                        self.data_db.insert(id.clone(), vec![data]);
                        self.plotpara.insert(id, plt_window::Plotpara{x_min:0., x_max:0., x_rescale:true,
                        y_min:0., y_max:0., y_rescale:true});
                    }
                }
                Err(e) => {}
            }
        }

        CentralPanel::default().show(ctx, |ui| {
            for key in self.keys_for_plots.keys(){
                plt_window::new(ctx, key, &self.data_db, &mut self.plotpara)
            }
        });
        SidePanel::left("Data Channels")
            .frame(egui::Frame::new().fill(egui::Color32::from_rgb(0,100,180)))
            .show(ctx, |ui| {
                if ui.add(Button::new("Guide")).clicked(){
                    self.enGuide ^= true;
                }
                if self.enGuide {
                    guide::new(ctx)
                }

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

