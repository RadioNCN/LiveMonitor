use std::collections::HashMap;
use eframe::egui;
use eframe::egui::{Button, DragValue, Window};
use egui_plotter::EguiBackend;
use plotters::prelude::*;

pub(crate) struct Plotpara {
    pub(crate) settings: bool,
    pub(crate) x_min:f64, pub(crate) x_max:f64,
    pub(crate) x_rescale: bool,
    pub(crate) y_min:f64, pub(crate) y_max:f64,
    pub(crate) y_rescale: bool,
}

pub(crate) fn new(ctx: &egui::Context, key: &String,
                  data: &mut HashMap<String, Vec<(f64, f64)>>,
                  para: &mut HashMap<String, Plotpara>) {

    let (x_min, x_max) = data[key].iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &(x, _)|
        (min.min(x), max.max(x)));
    let (y_min, y_max) = data[key].iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &(_, y)|
        (min.min(y), max.max(y)));

    if let Some(parameters) = para.get_mut(key){
        if parameters.x_rescale == false {
            parameters.x_min = parameters.x_min.min(x_min);
            parameters.x_max = parameters.x_max.max(x_max);
        }else {
            parameters.x_min = x_min;
            parameters.x_max = x_max;
        }
        if parameters.y_rescale == false {
            parameters.y_min = parameters.y_min.min(y_min);
            parameters.y_max = parameters.y_max.max(y_max);
        }else {
            parameters.y_min = y_min;
            parameters.y_max = y_max;
        }
    }
    Window::new(key)
        .default_open(true)
        .default_pos((100., 20.))
        .show(ctx, |ui| {
            ui.set_height(ui.available_height());
            ui.set_width(ui.available_width());

            if let Some(parameters) =para.get_mut(key) {
                if ui.add(Button::new("Settings")).clicked() {
                    parameters.settings ^= true
                }
            }

            let root = EguiBackend::new(ui).into_drawing_area();
            let mut chart = ChartBuilder::on(&root)
                // .margin(50)
                .margin_top(20)
                .margin_left(10)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(para[key].x_min..para[key].x_max, para[key].y_min..para[key].y_max)
                .unwrap();

            chart.configure_mesh()
                .x_label_style(("sans-serif", 15).into_font().color(&BLACK))
                .y_label_style(("sans-serif", 15).into_font().color(&BLACK))
                .draw().unwrap();
            chart.draw_series(
                data[key].iter().map(|&(x, y)| {
                    Circle::new((x, y), 2, MandelbrotHSL.get_color(0. / (data.keys().len() as f64)).filled())
                })
            ).unwrap();
        });

    if para[key].settings {
        Window::new(key.to_string()+" | Settings")
            .default_open(true)
            .show(ctx, |ui| {
                // ui.set_height(ui.available_height());
                ui.set_width(ui.available_width());
                if let Some(parameters) =para.get_mut(key) {
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
                    });
                    ui.horizontal(|vui| {
                        if vui.add(Button::new("Empty data")).clicked() {
                            if let Some(val) = data.get_mut(key) {
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
