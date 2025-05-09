use std::vec::Vec;
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
    pub(crate) x_min:f64, pub(crate) x_max:f64,
    pub(crate) x_rescale: bool,
    pub(crate) y_min:f64, pub(crate) y_max:f64,
    pub(crate) y_rescale: bool,
    pub(crate) addplots: [usize; 4],
    pub(crate) plot_mode: PlotMode
}
impl Default for Plotpara {
    fn default() -> Self {
        Plotpara {
        x_min: 0., x_max: 0., x_rescale:true,
        y_min:0., y_max:0., y_rescale:true,
        settings: false, legend: false,
        addplots: [0,0,0,0], plot_mode: PlotMode::Line
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
    if data.get(key).unwrap().len()>0 {
        let (x_min, x_max) = data.get(key).unwrap().iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &(x, _)|
            (min.min(x), max.max(x)));
        let (y_min, y_max) = data.get(key).unwrap().iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &(_, y)|
            (min.min(y), max.max(y)));



        if let Some(mut parameters) = para.get_mut(key) {
            if parameters.x_rescale == false {
                parameters.x_min = parameters.x_min.min(x_min);
                parameters.x_max = parameters.x_max.max(x_max);
            } else {
                parameters.x_min = x_min;
                parameters.x_max = x_max;
            }
            if parameters.y_rescale == false {
                parameters.y_min = parameters.y_min.min(y_min);
                parameters.y_max = parameters.y_max.max(y_max);
            } else {
                parameters.y_min = y_min;
                parameters.y_max = y_max;
            }
        }
    }
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
            // if let Some(mut parameters) =para.get_mut(key) {
            //     ui.horizontal(|hui| {
            //         if hui.add(Button::new("Settings")).clicked() {
            //             parameters.settings ^= true
            //         }
            //         if hui.add(Button::new("Legend")).clicked() {
            //             parameters.legend ^= true
            //         }
            //     });
            // }

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

            let n =10;
            let m =5;
            let mut matrix: Vec<Vec<f64>> = vec![vec![]];
            if let Some(value) = data.get(key) {
                let mut k =0;
                for i in 0..n {
                    let mut row:Vec<f64> =vec![];
                    for j in 0..m{
                        row.push(value[k].0);
                        k+=1;
                    }
                    matrix.push(row);
                }
                let len =value.value().len()as f64;
                chart.draw_series(
                    matrix.iter().enumerate().flat_map(|(y, row)| {
                        row.iter().enumerate().map(move |(x, val)| {
                            let color = MandelbrotHSL.get_color(val / len);
                            Rectangle::new(
                                [(x, y), (x + 1, y + 1)],
                                color.filled(),
                            )

                        })
                    }),
                ).unwrap();
            }
        });

    if para.get(key).unwrap().settings {
        Window::new(key.to_string()+" | Settings")
            .default_open(true)
            .show(ctx, |ui| {
                ui.set_width(ui.available_width());
                if let Some(mut parameters) =para.get_mut(key) {
                    ui.horizontal(|hui| {
                        hui.vertical(|vui| {
                            vui.checkbox(&mut parameters.x_rescale, "Rescale: X");
                            vui.horizontal(|hui| {
                                hui.label("X min:");
                                hui.add(DragValue::new(&mut parameters.x_min));
                                hui.label("X max:");
                                hui.add(DragValue::new(&mut parameters.x_max));
                            })
                        });
                        hui.vertical(|vui| {
                            vui.checkbox(&mut parameters.y_rescale, "Rescale: Y");
                            vui.horizontal(|hui| {
                                hui.label("Y min:");
                                hui.add(DragValue::new(&mut parameters.y_min));
                                hui.label("Y max:");
                                hui.add(DragValue::new(&mut parameters.y_max));
                            })
                        });
                        ComboBox::from_id_salt("PlotMode")
                            .selected_text(format!("{}", &parameters.plot_mode))
                            .show_ui(hui, |dui| {
                                dui.selectable_value(&mut parameters.plot_mode, PlotMode::Scatter, "Scatter");
                                dui.selectable_value(&mut parameters.plot_mode, PlotMode::Line, "Line");
                            });
                    });
                    ui.horizontal(|vui| {
                        if vui.add(Button::new("Empty data")).clicked() {
                            if let Some(mut val) = data.get_mut(key) {
                                val.clear()
                            }
                        }
                        ComboBox::from_id_salt("Graph 1")
                            .selected_text(&other_keys[parameters.addplots[0]])
                            .show_ui(vui, |dui| {
                                for (index, k) in other_keys.iter().enumerate(){
                                    if dui.selectable_label(parameters.addplots[0]==index, k).clicked(){
                                        parameters.addplots[0]= index;
                                    }
                                }
                            });
                        ComboBox::from_id_salt("Graph 2")
                            .selected_text(&other_keys[parameters.addplots[1]])
                            .show_ui(vui, |dui| {
                                for (index, k) in other_keys.iter().enumerate(){
                                    if dui.selectable_label(parameters.addplots[1]==index, k).clicked(){
                                        parameters.addplots[1]= index;
                                    }
                                }
                            });
                        ComboBox::from_id_salt("Graph 3")
                            .selected_text(&other_keys[parameters.addplots[2]])
                            .show_ui(vui, |dui| {
                                for (index, k) in other_keys.iter().enumerate(){
                                    if dui.selectable_label(parameters.addplots[2]==index, k).clicked(){
                                        parameters.addplots[2]= index;
                                    }
                                }
                            });
                        ComboBox::from_id_salt("Graph 4")
                            .selected_text(&other_keys[parameters.addplots[3]])
                            .show_ui(vui, |dui| {
                                for (index, k) in other_keys.iter().enumerate(){
                                    if dui.selectable_label(parameters.addplots[3]==index, k).clicked(){
                                        parameters.addplots[3]= index;
                                    }
                                }
                            });
                    });
                    if ui.add(Button::new("Exit Settings")).clicked() {
                        parameters.settings = false
                    }
                }
            });
    }
}
