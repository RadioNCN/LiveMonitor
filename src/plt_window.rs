use std::collections::HashMap;
use eframe::egui;
use eframe::egui::{Button, ComboBox, DragValue, SelectableLabel, Window};
use egui_plotter::EguiBackend;
use plotters::prelude::*;

pub(crate) struct Plotpara {
    pub(crate) settings: bool,
    pub(crate) x_min:f64, pub(crate) x_max:f64,
    pub(crate) x_rescale: bool,
    pub(crate) y_min:f64, pub(crate) y_max:f64,
    pub(crate) y_rescale: bool,
    pub(crate) addplots: [usize; 4],
    pub(crate) plot_mode: PlotMode
}
pub(crate) enum PlotMode {
    Scatter, Line
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
    let mut other_keys = vec![];
        other_keys.push("Unselected".to_string());
    for k in data.keys(){
        if k != key{
            other_keys.push(k.clone());
        }
    }

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

                let root = EguiBackend::new(ui).into_drawing_area();
                let mut chart = ChartBuilder::on(&root)
                    // .margin(50)
                    .margin_top(20)
                    .margin_left(10)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(parameters.x_min..parameters.x_max, parameters.y_min..parameters.y_max)
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
                for addplot_index in parameters.addplots{
                    if data.contains_key(&other_keys[addplot_index]){
                        chart.draw_series(
                            data[&other_keys[addplot_index]].iter().map(|&(x, y)| {
                                Circle::new((x, y), 2, MandelbrotHSL.get_color(addplot_index as f64 / (data.keys().len() as f64)).filled())
                            })
                        ).unwrap();
                    }
                }
            }
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
