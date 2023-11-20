#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use eframe::egui;
use egui::{Pos2, Rect, Painter, Shape, Color32, Rounding, epaint::PathShape, Stroke};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My small paint",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    )
}


struct MyApp{
    pixel_size: f32,
    min_painter: Option<Pos2>,
    max_painter: Option<Pos2>,
    rect_painter: Option<Rect>,
    path_pos: Vec<Pos2>,
    shapes: Vec<Shape>,
    pointer_pos: Pos2,
}

impl MyApp{
    fn new() -> MyApp {
        Self {
            pixel_size: 10.,
            min_painter: None,
            max_painter: None,
            rect_painter: None,
            path_pos: Vec::new(),
            shapes: Vec::new(),
            pointer_pos: Pos2::ZERO,
         }
    }
    
    pub fn add_pixel_size(&mut self, count: f32) {
        self.pixel_size += count;
    }

    pub fn lower_pixel_size(&mut self, count: f32) {
        self.pixel_size -= count;
        if self.pixel_size <= 0.{
            self.pixel_size = 1.
        }
    }

    pub fn undo(&mut self){
        let _ = self.shapes.pop();
    }
}

impl eframe::App for MyApp{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame){
        ctx.input(|input|{
            if input.pointer.button_clicked(egui::PointerButton::Primary){
                let pointer_press = input.pointer.press_origin();
                if pointer_press.is_some(){
                    match self.min_painter.is_none() {
                        true => {
                            self.min_painter = pointer_press
                        },
                        fasle => {
                            match self.max_painter.is_none() {
                                true => {
                                    let min_click: Pos2 = self.min_painter.unwrap();
                                    self.max_painter = pointer_press;
                                    let max_click: Pos2 = self.max_painter.unwrap();
                                    let cheker: [bool; 3] = [ min_click.x > max_click.x, 
                                                              min_click.y < max_click.y, 
                                                              min_click.y > max_click.y && min_click.x < max_click.x];
                                    
                                    match cheker {
                                        [true, false, false] => {
                                            self.min_painter = Some(max_click);
                                            self.max_painter = Some(min_click);
                                        },
                                        [true, true, false] => {
                                            let local_min: Pos2 = Pos2 { x: max_click.x, y: min_click.y };
                                            let local_max: Pos2 = Pos2 { x: min_click.x, y: max_click.y };
                                            self.min_painter = Some(local_min);
                                            self.max_painter = Some(local_max);
                                        },
                                        [false, false, true] => {
                                            let local_min: Pos2 = Pos2 { x: min_click.x, y: max_click.y };
                                            let local_max: Pos2 = Pos2 { x: max_click.x, y: min_click.y };
                                            self.min_painter = Some(local_min);
                                            self.max_painter = Some(local_max);
                                        },
                                        _ => {}
                                    }
                                    if false {

                                    }
                                },
                                _ => {}
                            }
                        },
                        _ => {

                        },
                    }
                }
            }

         });
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("Hello World! This is small paint");
            ui.horizontal(|ui_h: &mut egui::Ui|{
                if ui_h.button("new").clicked(){
                    self.min_painter = None;
                    self.max_painter = None;
                    self.rect_painter = None;
                    self.path_pos = Vec::new();
                    self.shapes = Vec::new();
                }
                if ui_h.button("increase 1 pix").clicked(){
                    self.add_pixel_size(1.);
                }
                if ui_h.button("increase 10 pix").clicked(){
                    self.add_pixel_size(10.);
                }
                if ui_h.button("reduce 1 pix").clicked(){
                    self.lower_pixel_size(1.);
                }
                if ui_h.button("reduce 10 pix").clicked(){
                    self.lower_pixel_size(10.);
                }
                if ui_h.button("undo").clicked(){
                    self.undo();
                }
                if ui_h.button("undo 10").clicked(){
                    for _ in 0..11{
                        self.undo();
                    }
                }
                if ui_h.button("clear").clicked(){
                    self.shapes = Vec::new();
                }

            });
         });
         egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");
            let painter = ui.painter();
            ctx.input(|input| {
                if input.pointer.is_moving(){
                    let locat_pointer_pos = input.pointer.hover_pos();
                    if locat_pointer_pos.is_some(){
                        self.pointer_pos = locat_pointer_pos.unwrap()
                    }
                    else{
                        self.pointer_pos = Pos2::ZERO;
                    }
                }
            });
            match self.rect_painter.is_none() {
                true => {
                    if self.min_painter.is_some() && self.max_painter.is_some(){
                        self.rect_painter = Some(Rect::from_min_max(self.min_painter.unwrap(), self.max_painter.unwrap()));
                        painter.rect_filled(self.rect_painter.unwrap(), Rounding::ZERO, Color32::GRAY);
                    }
                },
                false => {
                    painter.rect_filled(self.rect_painter.unwrap(), Rounding::ZERO, Color32::GRAY);
                    ctx.input(|input|{
                        let primary_button_pressed = input.pointer.primary_down();
                        let pointer_press = input.pointer.press_origin();
                        if primary_button_pressed {
                            if self.pointer_pos.x != 0. && self.pointer_pos.y != 0.{
                                let pointer_pos = self.pointer_pos;
                                let min_pos = self.min_painter.unwrap();
                                let max_pos = self.max_painter.unwrap();
                                let bool_paint_r_t: bool = pointer_pos.x > min_pos.x && pointer_pos.y > min_pos.y;
                                let bool_paint_l_b: bool = pointer_pos.x < max_pos.x && pointer_pos.y < max_pos.y;
                                if bool_paint_l_b && bool_paint_r_t{
                                    self.path_pos.push(pointer_pos);
                                    self.shapes.push(Shape::line(self.path_pos.to_vec(), 
                                                                 Stroke::new(0.1*self.pixel_size, 
                                                                            Color32::BLACK)));
                                    
                                    if self.path_pos.len() == 10{
                                        let last_pos = self.path_pos.pop().unwrap();
                                        self.path_pos = Vec::new();
                                        self.path_pos.push(last_pos);
                                    }
                                }
                                else {
                                    self.path_pos = Vec::new();
                                }
                            }
                        }
                    });
                    ctx.input(|input|{
                        if input.pointer.primary_released(){
                            self.path_pos = Vec::new();
                        }
                    });
                    
                },
                _ => {

                }
            }
            let mut local_shapes = self.shapes.clone().into_iter();
            painter.extend(local_shapes);
         });
         egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui_h: &mut egui::Ui|{
                ui_h.label(format!("Pointer position: x: {} | y:{}", self.pointer_pos.x.round(), self.pointer_pos.y.round()));
                ui_h.label(format!("Pixel size: {}", self.pixel_size.round()))
            });
         });
    }
}
