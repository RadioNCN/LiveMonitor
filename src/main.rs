#![windows_subsystem = "windows"]
mod server;
mod pltGraph;
mod pltHeatmap;
mod guide;

use tokio::{self, time, runtime};
use std::collections::HashMap;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use rayon::prelude::*;
use eframe::egui::{self, SidePanel, CentralPanel, TopBottomPanel, Visuals, Window, Button, DragValue, RichText, Color32};
use plotters::prelude::*;
use std::time::Instant;


fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Monitor",
        native_options,
        Box::new(|cc| Ok(Box::new(Monitor::new(cc)))),
    )
    .unwrap();
}


struct Monitor {
    /// tokio runtime
    rt: runtime::Runtime,
    /// database for the actual data
    data_db: Arc<DashMap<String,Vec<(f64,f64)>>>,
    /// capacity of data points in the database per channel
    data_cap: Arc<Mutex<usize>>,
    /// for if no lock on data_cap can be acquired
    data_cap_old: usize,
    /// maximum time until a repaint of the gui is requested
    time_delay: usize,
    /// parameter database for graph like plots
    graphpara: Arc<DashMap<String, pltGraph::Plotpara>>,
    /// parameter database for the heatmap plots
    heatmpara: Arc<DashMap<String, pltHeatmap::Plotpara>>,
    /// list of data channels stored in database
    keys_for_plots: HashMap<String, bool>,
    /// is the guide enabled?
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
        // context.set_visuals(Visuals::light());
        let rt = runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        let data_db =Arc::new(DashMap::new());
        let g_para_db =Arc::new(DashMap::new());
        let h_para_db =Arc::new(DashMap::new());
        let cap_init = 1000;
        let data_cap =Arc::new(Mutex::new(cap_init));

        let db =Arc::clone(&data_db);
        let para= Arc::clone(&g_para_db);
        let cap = Arc::clone(&data_cap);
        rt.spawn(server::GraphServer(db, para, cap));

        let db =Arc::clone(&data_db);
        let para= Arc::clone(&h_para_db);
        rt.spawn(server::HeatmapServer(db, para));

        Self{rt:rt,
            data_db: data_db, keys_for_plots: HashMap::new(),
            data_cap: data_cap, data_cap_old: cap_init, time_delay: 30,
            graphpara: g_para_db, heatmpara: h_para_db,
            enGuide: false}
    }
}

impl eframe::App for Monitor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let latency = Instant::now();
        ctx.request_repaint_after(time::Duration::from_millis(self.time_delay as u64));

        CentralPanel::default()
            .show(ctx, |ui| {
            for key in self.keys_for_plots.keys(){
                if self.graphpara.contains_key(key){
                    pltGraph::new(ctx, key, &self.data_db, &self.graphpara)
                }
                if self.heatmpara.contains_key(key){
                    pltHeatmap::new(ctx, key, &self.data_db, &self.heatmpara)
                }
            }
        });
        let mut zbt_blue = Color32::from_rgb(0,100,180);
        if ctx.theme() == egui::Theme::Dark{
            zbt_blue = Color32::from_rgb(0,10,18);
        }
        SidePanel::left("Data Channels")

            .frame(egui::Frame::new().fill(zbt_blue))
            .show(ctx, |ui| {
                if ui.add(Button::new("Guide")).clicked(){
                    self.enGuide ^= true;
                }
                if self.enGuide {
                    guide::new(ctx)
                }
                ui.label(RichText::new("Data Capacity:"));
                match self.data_cap.try_lock() {
                    Ok(mut cap_lock) => {
                        *cap_lock = self.data_cap_old;
                        ui.add(DragValue::new(&mut *cap_lock).speed(10));
                        self.data_cap_old = *cap_lock;
                    }
                    Err(e) => {ui.add(DragValue::new(&mut self.data_cap_old).speed(10));}
                }
                ui.label(RichText::new("Frame Time:"));
                ui.add(DragValue::new(&mut self.time_delay).speed(1));
                ui.label(RichText::new("Data Channels:"));
                self.data_db.iter().for_each(|entry| {
                    if ui.add(Button::new(entry.key())).clicked() {
                        if self.keys_for_plots.contains_key(entry.key()) {
                            self.keys_for_plots.remove(entry.key());
                        } else {
                            self.keys_for_plots.insert((*entry.key()).clone(), true);
                        }
                    }
                });

            });
        TopBottomPanel::bottom("AppInfo")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("Version: {}", env!("CARGO_PKG_VERSION")));
                    ui.label(format!("TCP: {}", "127.0.0.1:7800"));
                    ui.label(format!("Latency: {:?}", latency.elapsed()))
                })
            });


    }
}

