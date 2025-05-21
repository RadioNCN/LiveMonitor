use std::collections::HashMap;
use eframe::egui;
use eframe::egui::{Button, ComboBox, DragValue, SelectableLabel, Window};
use egui_plotter::EguiBackend;
use plotters::prelude::*;
use std::string::ToString;
use dashmap::DashMap;
use strum_macros::Display;
use crate::pltGraph;

pub(crate) struct Plotpara {
    pub(crate) settings: bool,
    pub(crate) legend: bool,
    pub(crate) x_num:i32,
    pub(crate) y_num:i32,
}
impl Default for Plotpara {
    fn default() -> Self {
        Plotpara {
        x_num: 5, y_num: 10,
        settings: false, legend: false,
    }}
}
#[derive(PartialEq, Display, Copy, Clone)]
pub(crate) enum PlotMode{
    #[strum(serialize = "Scatter")] Scatter,
    #[strum(serialize = "Line")] Line
}
pub(crate) fn new(ctx: &egui::Context, key: &String,
                  data: &DashMap<String, Vec<(f64, f64)>>,
                  para: &DashMap<String, Plotpara>) {
    let mut other_keys = vec![];
    other_keys.push("Unselected".to_string());
    data.iter().for_each(|entry| {
        if entry.key() != key {
            other_keys.push(entry.key().clone());
        }
    });

    Window::new(key)
        .default_open(true)
        .default_pos((100., 20.))
        .show(ctx, |ui| {
            ui.set_height(ui.available_height());
            ui.set_width(ui.available_width());
            if let Some(mut parameters) =para.get_mut(key) {
                ui.horizontal(|hui| {
                    if hui.add(Button::new("Settings")).clicked() {
                        parameters.settings ^= true
                    }
                    // if hui.add(Button::new("Legend")).clicked() {
                    //     parameters.legend ^= true
                    // }
                });
            }

            let root = EguiBackend::new(ui).into_drawing_area();
            let mut chart = ChartBuilder::on(&root)
                // .margin(50)
                .margin_top(20)
                .margin_left(10)
                .margin_right(20)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(0usize..5usize, 0usize..10usize)
                .unwrap();
            if let Some(parameters) =para.get(key) {
                let mut matrix: Vec<Vec<f64>> = vec![vec![]];
                if let Some(value) = data.get(key) {
                    let mut k = 0;
                    for i in 0..parameters.y_num {
                        let mut row: Vec<f64> = vec![];
                        for j in 0..parameters.x_num {
                            row.push(value[k].0);
                            k += 1;
                        }
                        matrix.push(row);
                    }
                    chart.draw_series(
                        matrix.iter().enumerate().flat_map(|(y, row)| {
                            row.iter().enumerate().map(move |(x, val)| {
                                let color = MandelbrotHSL.get_color(*val);
                                Rectangle::new(
                                    [(x, y), (x + 1, y + 1)],
                                    color.filled(),
                                )
                            })
                        }),
                    ).unwrap();
                }
            }
        });

    if para.get(key).unwrap().settings {
        Window::new(key.to_string()+" | Settings")
            .default_open(true)
            .show(ctx, |ui| {
                ui.set_width(ui.available_width());
                let data_len_f64 =data.get(key).unwrap().len()as f64;
                if let Some(mut parameters) =para.get_mut(key) {
                    ui.horizontal(|hui| {
                        hui.vertical(|vui| {
                            vui.horizontal(|hui| {
                                hui.label("X:");
                                let y = parameters.y_num as f64;
                                hui.add(DragValue::new(&mut parameters.x_num).range(0..=(data_len_f64/y)as usize));
                            })
                        });
                        hui.vertical(|vui| {
                            vui.horizontal(|hui| {
                                hui.label("Y:");
                                let x = parameters.x_num as f64;
                                hui.add(DragValue::new(&mut parameters.y_num).range(0..=(data_len_f64/x)as usize));
                            })
                        });
                    });
                    ui.horizontal(|vui| {
                        if vui.add(Button::new("Empty data")).clicked() {
                            if let Some(mut val) = data.get_mut(key) {
                                val.clear()
                            }
                        }
                    });
                    if ui.add(Button::new("Exit Settings")).clicked() {
                        parameters.settings = false
                    }
                }
            });
    }
}
