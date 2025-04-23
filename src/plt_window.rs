use std::collections::HashMap;
use eframe::egui;
use eframe::egui::Window;
use egui_plotter::EguiBackend;
use plotters::prelude::*;
use plotters::style::full_palette::GREY;

pub(crate) fn new(ctx: &egui::Context, key: &String,
    data: HashMap<String,Vec<(f64,f64)>>) {

    let (x_min, x_max) = data[key].iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &(x, _)|
        (min.min(x), max.max(x)));
    let (y_min, y_max) = data[key].iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &(_, y)|
        (min.min(y), max.max(y)));
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
                .build_cartesian_2d(x_min..x_max, y_min..y_max)
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
