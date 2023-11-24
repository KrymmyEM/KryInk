//! This example shows that you can use egui in parallel from multiple threads.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use eframe::egui;
use egui::{Pos2, Rect, Painter, Shape, Color32, Rounding, epaint::PathShape, Stroke, ViewportInfo, Vec2};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My small paint",
        options,
        Box::new(|_cc| Box::new(BaseWindow::new())),
    )
}


struct BaseWindow{
    pixel_size: f32,
    height: Option<i32>,
    width: Option<i32>,
    path_pos: Vec<Pos2>,
    canvas: Vec<Shape>,
    shapes: Vec<Shape>,
    pointer_pos: Pos2,
    origin_pos: Option<Vec2>
}

impl BaseWindow{
    fn new() -> BaseWindow {
        Self {
            pixel_size: 10.,
            height: None,
            width: None,
            path_pos: Vec::new(),
            canvas: Vec::new(),
            shapes: Vec::new(),
            pointer_pos: Pos2::ZERO,
            origin_pos: None,
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

    fn draw_canvas(&mut self, height: i32, width: i32) -> Vec<Shape>{
        let pixels: Vec<Shape> = Vec::new();
        
        return pixels;
    }

}

impl eframe::App for BaseWindow{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.origin_pos.is_none(){
                let monitor_data = ui.available_size();
                println!("{:?}", monitor_data);
                let mut origin_pos_local = monitor_data;
                origin_pos_local.x = ((monitor_data.x.floor()/2.0) as i32) as f32;
                origin_pos_local.y = ((monitor_data.y.floor()/2.0) as i32) as f32;
                println!("{:?}",origin_pos_local);
                self.origin_pos = Some(origin_pos_local);
            }
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
            if self.height.is_some() && self.width.is_some(){
                        
                painter.extend(self.draw_canvas(self.height.unwrap(), self.width.unwrap()).clone().into_iter());
            }
            
            ctx.input(|input|{
                let primary_button_pressed = input.pointer.primary_down();
                let pointer_press = input.pointer.press_origin();
                if primary_button_pressed {
                    
                }
            });
            ctx.input(|input|{
                if input.pointer.primary_released(){
                    self.path_pos = Vec::new();
                }
            });
                    
            let mut local_shapes = self.shapes.clone().into_iter();
            painter.extend(local_shapes);
            });
    
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("Hello World! This is small paint");
            ui.horizontal(|ui_h: &mut egui::Ui|{
                if ui_h.button("new").clicked(){
                    self.height = None;
                    self.width = None;
                    self.path_pos = Vec::new();
                    self.canvas = Vec::new();
                    self.shapes = Vec::new();
                }
                if ui_h.button("increase 1 pix").clicked(){
                    self.add_pixel_size(1.);
                }
                if ui_h.button("increase 10 pix").clicked(){
                    self.add_pixel_size(10.);
                }
                ui_h.add(egui::Slider::new(&mut self.pixel_size, 0.0..=5000.0).prefix("Pixel size:"));
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
    
            egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui_h: &mut egui::Ui|{
                ui_h.label(format!("Pointer position: x: {} | y:{}", self.pointer_pos.x.floor(), self.pointer_pos.y.floor()));
                if self.height.is_some() && self.width.is_some(){
                    ui_h.label(format!("Canvas size: height: {} | width:{}", self.height.unwrap(), self.width.unwrap()));
                }
            });
            });
    }
}