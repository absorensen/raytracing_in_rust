use crate::{core::color_rgb::ColorRGB, services::image_presentation_service::save_image};
use crate::render::integrator::render_pixel;
use crate::services::service_locator::ServiceLocator;
use std::f32;
use std::time::Instant;
use confy::ConfyError;
use rayon::prelude::*;
use eframe::{egui::{self, Ui}, NativeOptions};

use crate::scene::scene_builder::SceneBuilder;
use crate::utility::render_config::RenderConfig;

mod geometry;
mod hittables;
mod materials;
mod math;
mod noise;
mod pdfs;
mod scene;
mod services;
mod textures;
mod utility;
mod core;
mod render;

fn handle_confy_error(result: Result<(), ConfyError>) -> bool {
    match result {
        Ok(_status) => {
            println!("Successfully loaded config file");
            true
        },
        Err(err) => {
            println!("Confy Error: {}", err);
            false
        },
    }
}

#[derive(Default)]
struct RenderApp {
    fullscreen_state: bool,
    render_and_show: bool,
    finished_rendering: bool,
    image_saved: bool,
    image: egui::ColorImage,
    image_texture: Option<egui::TextureHandle>,
    config: RenderConfig,
    config_save_path: String,
}

impl RenderApp {
    fn new(config: RenderConfig, config_save_path: &str) -> RenderApp {
        RenderApp { 
            fullscreen_state: false,
            render_and_show: false,
            finished_rendering: false,
            image_saved: false,
            image: egui::ColorImage::default(),
            image_texture: None,
            config,
            config_save_path: config_save_path.to_string(),
        }
    }

    fn shutdown(&self) -> bool {
        let result: Result<(), ConfyError> = confy::store_path(self.config_save_path.as_str(), &self.config);
        handle_confy_error(result)
    }

    pub fn render(&mut self) {
        self.config.update_derived_values();

        if !self.finished_rendering {

            let service_locator: ServiceLocator = SceneBuilder::build_scene(&self.config);
            
            let now: Instant = Instant::now();
            let total_pixels: usize = self.config.image_height * self.config.image_width;

            let image: Vec<ColorRGB> = 
                (0..total_pixels).into_par_iter().map(|pixel_index:usize| {
                    let mut rng = rand::thread_rng();
                    render_pixel(
                        &self.config,
                        &mut rng, 
                        &service_locator,
                        pixel_index 
                    )
            }).collect();

            let black: ColorRGB = ColorRGB::black();
            let mut flipped_image: Vec<ColorRGB> = vec![black; image.len()];
        
            for row_index in 0..(self.config.image_height / 2) {
                for column_index in 0..self.config.image_width {
                    let row_index_top = (row_index * self.config.image_width + column_index) as usize;
                    let row_index_bottom = ((self.config.image_height - row_index - 1) * self.config.image_width + column_index) as usize;
                    flipped_image[row_index_top] = image[row_index_bottom];
                    flipped_image[row_index_bottom] = image[row_index_top];
                }
            }

            let size: [usize; 2] = [self.config.image_width as usize, self.config.image_height as usize];
            let rgba: Vec<u8> = flipped_image.iter()
                .flat_map(|vector| [vector.r as u8, vector.g as u8, vector.b as u8, 255])
                .collect();

            self.image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_ref());
            println!("{} seconds elapsed", now.elapsed().as_millis() as f32 * 0.001);
            self.finished_rendering = true;

            self.image_saved = true;
            save_image(&self.config, &image);
        };
    }
}

impl eframe::App for RenderApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT
    }

    fn update(&mut self, context: &egui::Context, frame: &mut eframe::Frame) { 
        frame.set_fullscreen(self.fullscreen_state);

        custom_window_frame(context, frame, "raytracing_in_rust", |ui: &mut Ui| {
            ui.horizontal(|ui: &mut Ui| {
                ui.label("egui theme:");
                egui::widgets::global_dark_light_mode_buttons(ui);
            });

            // pub image_width: usize,
            ui.horizontal(|ui: &mut Ui| {
                ui.label("Image Width:");
                ui.add(egui::widgets::DragValue::new(&mut self.config.image_width));
            });
            
            // pub image_height: usize,
            ui.horizontal(|ui: &mut Ui| {
                ui.label("Image Height:");
                ui.add(egui::widgets::DragValue::new(&mut self.config.image_height));
            });

            // pub output_path: String,
            ui.horizontal(|ui: &mut Ui| {
                ui.label("Output Path:");
                ui.text_edit_singleline(&mut self.config.output_path);
            });

            // pub samples_per_pixel: usize,
            ui.horizontal(|ui: &mut Ui| {
                ui.label("Subpixels Per Pixel:");
                ui.add(egui::widgets::DragValue::new(&mut self.config.subpixels_per_pixel));
            });

            // pub samples_per_pixel: usize,
            ui.horizontal(|ui: &mut Ui| {
                ui.label("Samples Per Subixel:");
                ui.add(egui::widgets::DragValue::new(&mut self.config.samples_per_pixel));
            });

            // pub max_depth: usize,
            ui.horizontal(|ui: &mut Ui| {
                ui.label("Max Depth:");
                ui.add(egui::widgets::DragValue::new(&mut self.config.max_depth));
            });

            // pub scene_index: usize,
            ui.horizontal(|ui: &mut Ui| {
                ui.label("Scene Index:");
                ui.add(egui::widgets::DragValue::new(&mut self.config.scene_index));
            });
            
            // pub seed: usize,
            ui.horizontal(|ui: &mut Ui| {
                ui.label("Random Seed:");
                ui.add(egui::widgets::DragValue::new(&mut self.config.seed));
            });

            // pub use_loop_rendering: bool,
            // pub is_initialized: bool,
            ui.horizontal(|ui: &mut Ui| {
                ui.label("Use Loop Rendering:");
                ui.add(egui::widgets::Checkbox::new(&mut self.config.use_loop_rendering, ""));
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.render_and_show, "Render Image");
            });
        });

        // 
        // TODO: MAKE THIS NON-BLOCKING??
        //
        if self.render_and_show && !self.finished_rendering {
            self.render();
        }


        //
        // TODO: Make sure to resize the master window
        //
        if self.render_and_show && self.finished_rendering {
            egui::Window::new("Rendered Image")
            .open(&mut self.render_and_show)
            .resizable(true)
            .show(context, |ui| {
                let texture: &egui::TextureHandle = self.image_texture.get_or_insert_with(|| {
                    // Load the texture only once.
                    ui.ctx().load_texture(
                        "my-image",
                        self.image.clone(), //Not good performance, but this only happens once
                        egui::TextureFilter::Linear
                    )
                });

                ui.image(texture, texture.size_vec2());
            });
        }

        if !self.render_and_show && self.finished_rendering {
            self.finished_rendering = false;
            self.image_saved = false;
            self.image_texture = None;
        }
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        self.shutdown();
    }

    fn on_close_event(&mut self) -> bool {
        self.shutdown();
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.shutdown();
    }
}

fn custom_window_frame(context: &egui::Context, frame: &mut eframe::Frame, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    use egui::*;
    let text_color: Color32 = context.style().visuals.text_color();

    let height: f32 = 28.0;

    CentralPanel::default()
        .frame(Frame::none())
        .show(context, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            painter.rect(
                rect.shrink(1.0),
                10.0,
                context.style().visuals.window_fill(),
                Stroke::new(1.0, text_color),
            );

            painter.text(
                rect.center_top() + vec2(0.0, height / 2.0),
                Align2::CENTER_CENTER,
                title,
                FontId::proportional(height - 2.0),
                text_color,
            );

            painter.line_segment(
                [
                    rect.left_top() + vec2(2.0, height),
                    rect.right_top() + vec2(-2.0, height),
                ],
                Stroke::new(1.0, text_color),
            );

            let close_response: Response = ui.put ( 
                Rect::from_min_size(rect.left_top(), Vec2::splat(height)),
                Button::new(RichText::new("❌").size(height - 4.0)).frame(false),
            );

            if close_response.clicked() {
                frame.close();
            }

            let title_bar_rect = {
                let mut rect = rect;
                rect.max.y = rect.min.y + height;
                rect
            };

            let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
            if title_bar_response.is_pointer_button_down_on() {
                frame.drag_window();
            }

            let content_rect = {
                let mut rect = rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }.shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);
        });
}

pub fn run(config_path: &'static str) {
    let mut config: RenderConfig = confy::load_path(config_path).expect("Unable to load config file");
    config.update_derived_values();

    let options: NativeOptions = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::vec2(320.0 + config.image_height as f32, 100.0 + config.image_width as f32)),
        ..Default::default()
    };

    eframe::run_native(
        "raytracing_in_rust", 
        options, 
        Box::new(
            |_cc| 
            Box::new(
                RenderApp::new(
                    config, 
                    config_path
                )
            )
        )
    );
}


