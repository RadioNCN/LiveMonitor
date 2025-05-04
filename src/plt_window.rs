use std::collections::HashMap;
use eframe::egui;
use eframe::egui::{Button, ComboBox, DragValue, SelectableLabel, Window};
use egui_plotter::EguiBackend;
use plotters::prelude::*;
use std::string::ToString;
use dashmap::DashMap;
use strum_macros::Display;


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
#[derive(PartialEq, Display, Copy, Clone)]
pub(crate) enum PlotMode {
    #[strum(serialize = "Scatter")] Scatter,
    #[strum(serialize = "Line")] Line
}
pub(crate) fn new(ctx: &egui::Context, key: &String,
                  data: &mut DashMap<String, Vec<(f64, f64)>>,
                  para: &mut HashMap<String, Plotpara>) {

    let (x_min, x_max) = data.get(key).unwrap().iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &(x, _)|
        (min.min(x), max.max(x)));
    let (y_min, y_max) = data.get(key).unwrap().iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &(_, y)|
        (min.min(y), max.max(y)));
    let mut other_keys = vec![];
        other_keys.push("Unselected".to_string());
    data.iter().for_each(|entry|{
        if entry.key() != key{
            other_keys.push(entry.key().clone());
        }
    });

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
                ui.horizontal(|hui| {
                    if hui.add(Button::new("Settings")).clicked() {
                        parameters.settings ^= true
                    }
                    if hui.add(Button::new("Legend")).clicked() {
                        parameters.legend ^= true
                    }
                });
            }
            if data.iter().count() > 0 {
                let root = EguiBackend::new(ui).into_drawing_area();
                let mut chart = ChartBuilder::on(&root)
                    // .margin(50)
                    .margin_top(20)
                    .margin_left(20)
                    .margin_right(10)
                    .x_label_area_size(30)
                    .y_label_area_size(30)
                    .build_cartesian_2d(para.get(key).unwrap().x_min..para.get(key).unwrap().x_max, para.get(key).unwrap().y_min..para.get(key).unwrap().y_max)
                    .unwrap();

                chart.configure_mesh()
                    .x_label_style(("sans-serif", 15).into_font().color(&BLACK))
                    .y_label_style(("sans-serif", 15).into_font().color(&BLACK))
                    .draw().unwrap();

                let color = MandelbrotHSL.get_color(0 as f64 / (data.iter().count() as f64));

                match para.get(key).unwrap().plot_mode {
                    PlotMode::Scatter => {
                        chart.draw_series(
                            data.get(key).unwrap().iter().map(|&(x, y)| {
                                Circle::new((x, y), 2, color.filled())
                            })
                        ).unwrap()
                            .label(key)
                            .legend(move |(x, y)| {
                                Rectangle::new([(x - 15, y + 1), (x, y)], color)
                            });
                    }
                    PlotMode::Line => {
                        chart.draw_series(
                            LineSeries::new(
                                data.get(key).unwrap().iter().map(|&(x, y)| {
                                    (x, y)
                                }),
                                color.filled())
                        ).unwrap()
                            .label(key)
                            .legend(move |(x, y)| {
                                Rectangle::new([(x - 15, y + 1), (x, y)], color)
                            });
                    }
                }

                for addplot_index in para.get(key).unwrap().addplots {
                    if let Some(parameters) = para.get(&other_keys[addplot_index]) {
                        match parameters.plot_mode {
                            PlotMode::Scatter => {
                                if data.contains_key(&other_keys[addplot_index]) {
                                    let color = MandelbrotHSL.get_color(addplot_index as f64 / (data.iter().count() as f64));
                                    chart.draw_series(
                                        data.get(&other_keys[addplot_index]).unwrap().iter().map(|&(x, y)| {
                                            Circle::new((x, y), 2, color.filled())
                                        })
                                    ).unwrap().label(format!("{}", &other_keys[addplot_index]))
                                        .legend(move |(x, y)| {
                                            Rectangle::new([(x - 15, y + 1), (x, y)], color)
                                        });
                                }
                            }
                            PlotMode::Line => {
                                if data.contains_key(&other_keys[addplot_index]) {
                                    let color = MandelbrotHSL.get_color(addplot_index as f64 / (data.iter().count() as f64));
                                    chart.draw_series(
                                        LineSeries::new(
                                            data.get(&other_keys[addplot_index]).unwrap().iter().map(|&(x, y)| {
                                                (x, y)
                                            }),
                                            color
                                        )
                                    ).unwrap().label(format!("{}", &other_keys[addplot_index]))
                                        .legend(move |(x, y)| {
                                            Rectangle::new([(x - 15, y + 1), (x, y)], color)
                                        });
                                }
                            }
                        }
                    }
                }
                if para.get(key).unwrap().legend {
                    chart.configure_series_labels()
                        .position(SeriesLabelPosition::UpperRight)
                        // .margin(20)
                        // .legend_area_size(5)
                        // .border_style(BLUE)
                        // .background_style(BLUE.mix(0.1))
                        .draw().unwrap();
                    // .label_font(("Calibri", 20)).draw().unwrap();
                }
            }
        });

    if para[key].settings {
        Window::new(key.to_string()+" | Settings")
            .default_open(true)
            .show(ctx, |ui| {
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
