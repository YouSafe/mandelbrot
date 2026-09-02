use eframe::egui;
use egui::vec2;

#[derive(Debug)]
pub struct Camera {
    pub center: egui::Pos2,
    pub zoom: f32,
}

impl Camera {
    pub fn vec_screen_to_world(&self, screen: egui::Vec2) -> egui::Vec2 {
        Self::vec_flip_y(screen) / self.zoom
    }

    pub fn pos_screen_to_world(&self, screen: egui::Pos2, screen_center: egui::Pos2) -> egui::Pos2 {
        (Self::vec_flip_y(screen - screen_center) / self.zoom + self.center.to_vec2()).to_pos2()
    }

    pub fn pos_world_to_screen(&self, world: egui::Pos2, screen_center: egui::Pos2) -> egui::Pos2 {
        (Self::vec_flip_y(world - self.center) * self.zoom + screen_center.to_vec2()).to_pos2()
    }

    fn vec_flip_y(v: egui::Vec2) -> egui::Vec2 {
        vec2(v.x, -v.y)
    }
}
