//! This example shows that you can use egui in parallel from multiple threads.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use eframe::glow::HasContext;
use eframe::{egui, glow};
use egui::epaint::{PathShape, image::ColorImage};
use egui::{Pos2, Rect, Painter, Shape, Color32, Rounding, Stroke, ViewportInfo, Vec2, Style};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default(),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "My small paint",
        options,
        Box::new(|_cc| Box::new(BaseWindow::new())),
    )
}


enum Tools {
    mouse,
    hand,
    pen
}

struct BaseWindow{
    pixel_size: f32,
    height: Option<f32>,
    width: Option<f32>,
    start_pos: Pos2,
    end_pos: Pos2,
    monitor_data: Vec2,
    path_pos: Vec<Pos2>,
    canvas: Vec<Shape>,
    canvas_ready: bool,
    shapes: Vec<Shape>,
    start_pointer_pos: Pos2,
    pointer_pos: Pos2,
    zoom: f32,
    zoom_pos: Pos2,
    origin_pos: Option<Vec2>,
    tool: Tools
}

impl BaseWindow{
    fn new() -> BaseWindow {
        Self {
            pixel_size: 10.,
            height: None,
            width: None,
            start_pos: Pos2::ZERO,
            end_pos: Pos2::ZERO,
            monitor_data: Vec2::ZERO,
            path_pos: Vec::new(),
            canvas: Vec::new(),
            canvas_ready: false,
            shapes: Vec::new(),
            start_pointer_pos: Pos2::ZERO,
            pointer_pos: Pos2::ZERO,
            zoom: 1.,
            zoom_pos: Pos2::ZERO,
            origin_pos: None,
            tool: Tools::mouse
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

    fn draw_canvas(&mut self, height: f32, width: f32){
        let half_height = height/2.0;
        let half_width = width/2.0;
        let mut start_pos = Pos2::ZERO;
        let mut end_pos = Pos2::ZERO;
        if self.origin_pos.is_some(){
            self.start_pos = self.origin_pos.unwrap().to_pos2();
            self.end_pos = self.origin_pos.unwrap().to_pos2();
        }
        
        self.start_pos.x -= half_width * self.zoom;
        self.start_pos.y -= half_height * self.zoom;
        self.end_pos.x += half_width * self.zoom;
        self.end_pos.y += half_height * self.zoom;
        
        let shape = Shape::rect_filled(Rect { min:Pos2 { x: self.start_pos.x, y: self.start_pos.y}, 
            max:Pos2 { x: self.end_pos.x, y: self.end_pos.y}, }, 
            Rounding::default(),
                Color32::WHITE);
        self.canvas.push(shape);

    }
    
    fn paint (&mut self){
        if self.pointer_pos.x != 0. && self.pointer_pos.y != 0.{
            let pointer_pos = self.pointer_pos;
            let min_pos = self.start_pos;
            let max_pos = self.end_pos;
            let bool_paint_r_t: bool = pointer_pos.x > min_pos.x && pointer_pos.y > min_pos.y;
            let bool_paint_l_b: bool = pointer_pos.x < max_pos.x && pointer_pos.y < max_pos.y;
            if bool_paint_l_b && bool_paint_r_t{
                self.path_pos.push(pointer_pos);
                self.shapes.push(Shape::line(self.path_pos.to_vec(), 
                                                Stroke::new(0.1*self.pixel_size, 
                                                        Color32::BLACK)));
                self.shapes.push(Shape::circle_filled(self.pointer_pos, 
                                (0.1*self.pixel_size)/2.0, 
                                Color32::BLACK));
                if self.path_pos.len() == 2{
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

    fn move_shapes(&mut self) {
        let difference_pos: Vec2 = Vec2::new(self.start_pointer_pos.x - self.pointer_pos.x, self.start_pointer_pos.y - self.pointer_pos.y);
        
    }
    

}

impl eframe::App for BaseWindow{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.origin_pos.is_none(){
                self.monitor_data = ui.available_size();
                println!("{:?}", self.monitor_data);
                let mut origin_pos_local = self.monitor_data;
                origin_pos_local.x = ((self.monitor_data.x.floor()/2.0) as i32) as f32;
                origin_pos_local.y = ((self.monitor_data.y.floor()/2.0) as i32) as f32;
                println!("{:?}",origin_pos_local);
                self.origin_pos = Some(origin_pos_local);
                self.height = Some(origin_pos_local.y as f32);
                self.width = Some(origin_pos_local.x as f32);
            }
            let painter = ui.painter();
            ctx.input(|input| {
                if input.pointer.is_moving(){
                    let local_pointer_pos = input.pointer.hover_pos();
                    if local_pointer_pos.is_some(){
                        self.pointer_pos = local_pointer_pos.unwrap();
                        let primary_button_pressed = input.pointer.primary_down();
                        if primary_button_pressed {
                            match self.tool {
                                Tools::pen => {
                                    self.paint()
                                },
                                Tools::hand => {
                                    if self.start_pointer_pos.x != 0. && self.start_pointer_pos.y != 0.{
                                        self.start_pointer_pos = self.pointer_pos;
                                    }
                                    else {
                                        self.move_shapes();
                                    }
                                },
                                Tools::mouse => {},
                                _ => {}
                            }
                        }
                        else{
                            match self.tool{
                                Tools::pen => {
                                    self.path_pos = Vec::new();
                                },
                                Tools::hand => {
                                    self.start_pointer_pos = Pos2::ZERO
                                },
                                Tools::mouse => {},
                                _ => {}
                            }
                            
                        }
                    }
                    else{
                        self.pointer_pos = Pos2::ZERO;
                    }
                }
            });
            if self.height.is_some() && self.width.is_some() && !self.canvas_ready {
                self.draw_canvas(self.height.unwrap(), self.width.unwrap());
                self.canvas_ready = !self.canvas_ready;
            }

            painter.extend(self.canvas.clone().into_iter());
            painter.extend(self.shapes.clone().into_iter());

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
                if ui_h.button("check").clicked(){
                    let glow_opinion = frame.gl();
                    if glow_opinion.is_some(){
                        let glow = glow_opinion.unwrap().as_ref();
                        if self.height.is_some() && self.width.is_some(){
                            println!("{:?}", unsafe { glow.read_pixels(self.start_pos.x as i32,self.start_pos.y as i32,
                                self.width.unwrap() as i32,self.height.unwrap() as i32,4,4,glow::PixelPackData::BufferOffset(4)) } );
                        }
                       
                    } 

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
        egui::SidePanel::left("left_side panel").show(ctx, |ui|{
            ui.vertical(|ui|{
                if ui.button("Mouse").clicked(){
                    self.tool = Tools::mouse
                }
                if ui.button("Hand").clicked(){
                    self.tool = Tools::hand
                }
                if ui.button("Pen").clicked(){
                    self.tool = Tools::pen
                }

            });
        });

    }
}