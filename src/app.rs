use egui::{CentralPanel, Color32, DragValue, Frame, Panel, Pos2, Sense, Slider, Style, Window};

use crate::{
    camera::Camera,
    painter::{FractalPainter, PainterUniform},
};

pub struct MandelbrotSettings {
    max_iterations: u32,
}

impl Default for MandelbrotSettings {
    fn default() -> Self {
        Self {
            max_iterations: 256,
        }
    }
}

pub struct JuliaSetBrotSettings {
    max_iterations: u32,
    point_c: Pos2,
    is_floating: bool,
}

impl Default for JuliaSetBrotSettings {
    fn default() -> Self {
        Self {
            max_iterations: 50,
            point_c: Pos2::ZERO,
            is_floating: true,
        }
    }
}

pub struct MandelApp {
    juliaset_painter: FractalPainter,
    juliaset_settings: JuliaSetBrotSettings,
    juliaset_camera: Camera,

    mandelbrot_painter: FractalPainter,
    mandelbrot_settings: MandelbrotSettings,
    mandelbrot_camera: Camera,
    frames: u32,
}

impl MandelApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let render_state = cc
            .wgpu_render_state
            .as_ref()
            .expect("app only built with wgpu");
        Self {
            juliaset_painter: FractalPainter::new(render_state),
            juliaset_settings: JuliaSetBrotSettings::default(),
            juliaset_camera: Camera {
                center: Pos2::new(0.0, 0.0),
                zoom: 125.0,
            },
            mandelbrot_painter: FractalPainter::new(render_state),
            mandelbrot_settings: MandelbrotSettings::default(),
            mandelbrot_camera: Camera {
                center: Pos2::new(-0.5, 0.0),
                zoom: 500.0,
            },
            frames: 0,
        }
    }
}

impl eframe::App for MandelApp {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        Panel::left("left_panel").show_inside(ui, |ui| {
            ui.heading("Mandelbrot Settings");
            ui.label("Max iterations");
            ui.add(Slider::new(
                &mut self.mandelbrot_settings.max_iterations,
                1..=2000,
            ));

            ui.separator();

            ui.heading("Julia Settings");
            ui.label("Max iterations");
            ui.add(Slider::new(
                &mut self.juliaset_settings.max_iterations,
                1..=300,
            ));

            ui.label("Point c");

            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut self.juliaset_settings.point_c.x).min_decimals(9));
                ui.label("Real");
            });

            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut self.juliaset_settings.point_c.y).min_decimals(9));
                ui.label("Imaginary");
            });

            if ui.button("Copy camera center").clicked() {
                self.juliaset_settings.point_c = self.mandelbrot_camera.center;
            }

            ui.checkbox(&mut self.juliaset_settings.is_floating, "Floating window");

            ui.separator();

            ui.heading("Camera");

            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut self.mandelbrot_camera.center.x).min_decimals(9));
                ui.label("Real");
            });

            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut self.mandelbrot_camera.center.y).min_decimals(9));
                ui.label("Imaginary");
            });

            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut self.mandelbrot_camera.zoom));
                ui.label("Zoom");
            });

            if ui.button("Reset").clicked() {
                self.mandelbrot_camera = Camera {
                    center: Pos2::new(-0.5, 0.0),
                    zoom: 500.0,
                }
            }

            ui.separator();

            ui.label(format!("Frames: {}", self.frames));

            ui.take_available_width();
        });

        if self.juliaset_settings.is_floating {
            Window::new("Julia Set")
                .collapsible(false)
                .resizable(true)
                .show(ui.ctx(), |ui| {
                    self.ui_juliaset(ui);
                });
        } else {
            Panel::right("right_panel")
                .default_size(ui.available_size().x * 0.5)
                .max_size(ui.available_size().x * 0.8)
                .frame(Frame::side_top_panel(&Style::default()).inner_margin(0.0))
                .show_inside(ui, |ui| {
                    self.ui_juliaset(ui);
                });
        }

        CentralPanel::no_frame().show_inside(ui, |ui| {
            self.ui_mandelbrot(ui);
        });

        self.frames += 1;
    }
}

impl MandelApp {
    fn ui_mandelbrot(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), Sense::drag());

        let ctx = ui.ctx();

        if response.dragged_by(egui::PointerButton::Secondary) {
            let mouse_pos = ctx
                .input(|i| i.pointer.interact_pos())
                .expect("must be defined inside rect");

            self.juliaset_settings.point_c = self
                .mandelbrot_camera
                .pos_screen_to_world(mouse_pos, rect.center());
        }

        if response.hovered() {
            let zoom_delta = ctx.input(|i| i.zoom_delta());
            if zoom_delta != 1.0
                && let Some(mouse_pos) = ctx.input(|i| i.pointer.hover_pos())
            {
                // Convert mouse to world before zoom
                let world_before = self
                    .mandelbrot_camera
                    .pos_screen_to_world(mouse_pos, rect.center());

                self.mandelbrot_camera.zoom *= zoom_delta;
                self.mandelbrot_camera.zoom = self.mandelbrot_camera.zoom.clamp(0.1, 100000000.0);

                // Convert again after zoom
                let world_after = self
                    .mandelbrot_camera
                    .pos_screen_to_world(mouse_pos, rect.center());

                // Adjust offset so zoom centers on cursor
                self.mandelbrot_camera.center += world_before - world_after;
            }
        }

        if response.dragged_by(egui::PointerButton::Primary) {
            let delta = ctx.input(|i| i.pointer.delta());
            self.mandelbrot_camera.center -= self.mandelbrot_camera.vec_screen_to_world(delta);
        }

        self.mandelbrot_painter.paint(
            ui,
            rect,
            PainterUniform {
                c: [0.0; 2], // ignored
                scale: self.mandelbrot_camera.zoom,
                translation: self.mandelbrot_camera.center.into(),
                size: rect.size().into(),
                max_iters: self.mandelbrot_settings.max_iterations,
                is_julia: 0,
                _pad: Default::default(),
            },
        );

        ui.painter().circle_filled(
            self.mandelbrot_camera
                .pos_world_to_screen(self.juliaset_settings.point_c, rect.center()),
            3.0,
            Color32::RED,
        );
    }

    fn ui_juliaset(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), Sense::drag());

        let ctx = ui.ctx();

        if response.hovered() {
            let zoom_delta = ctx.input(|i| i.zoom_delta());
            if zoom_delta != 1.0
                && let Some(mouse_pos) = ctx.input(|i| i.pointer.hover_pos())
            {
                // Convert mouse to world before zoom
                let world_before = self
                    .juliaset_camera
                    .pos_screen_to_world(mouse_pos, rect.center());

                self.juliaset_camera.zoom *= zoom_delta;
                self.juliaset_camera.zoom = self.juliaset_camera.zoom.clamp(0.1, 100000000.0);

                // Convert again after zoom
                let world_after = self
                    .juliaset_camera
                    .pos_screen_to_world(mouse_pos, rect.center());

                // Adjust offset so zoom centers on cursor
                self.juliaset_camera.center += world_before - world_after;
            }
        }

        if response.dragged_by(egui::PointerButton::Primary) {
            let delta = ctx.input(|i| i.pointer.delta());
            self.juliaset_camera.center -= self.juliaset_camera.vec_screen_to_world(delta);
        }

        self.juliaset_painter.paint(
            ui,
            rect,
            PainterUniform {
                c: self.juliaset_settings.point_c.into(),
                scale: self.juliaset_camera.zoom,
                translation: self.juliaset_camera.center.into(),
                size: rect.size().into(),
                max_iters: self.juliaset_settings.max_iterations,
                _pad: Default::default(),
                is_julia: 1,
            },
        );
    }
}
