use std::collections::HashMap;
use eframe::egui;
use eframe::egui::Window;
use egui_plotter::EguiBackend;
use plotters::prelude::*;

pub(crate) struct Plotpara {
    pub(crate) x_min:f64, pub(crate) x_max:f64,
    pub(crate) x_rescale: bool,
    pub(crate) y_min:f64, pub(crate) y_max:f64,
    pub(crate) y_rescale: bool,
}

pub(crate) fn new(ctx: &egui::Context, key: &String,
                  data: &HashMap<String,Vec<(f64,f64)>>,
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

            let root = EguiBackend::new(ui).into_drawing_area();
            let mut chart = ChartBuilder::on(&root)
                .margin(5)
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
}
