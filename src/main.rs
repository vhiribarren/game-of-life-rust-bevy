/*
MIT License

Copyright (c) 2023 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

mod cell_system;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use cell_system::{CellSystem, Position};

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.3, 0.6);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(CellSystem)
        .add_systems(Startup, init_camera)
        .add_systems(Update, system_gui)
        .add_systems(Update, system_draw_new_cells)
        .run();
}

fn init_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale /= 40.;
    commands.spawn(camera);
}

fn system_gui(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();
    ctx.set_visuals(egui::style::Visuals::light());
    egui::Window::new("Game of Life")
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider::new(&mut 1., 0.1..=5.)
                        .text("Speed")
                        .step_by(0.1)
                        .logarithmic(true),
                );
            });
            ui.horizontal(|ui| {
                if ui.button("Play").clicked() {
                    println!("Play");
                }
                if ui.button("Next Step").clicked() {
                    println!("Next Step");
                };
            });
            ui.add(egui::Separator::default());
            ui.add(egui::Slider::new(&mut 100, 0..=100).integer().text("Zoom"));
        });
}

fn system_draw_new_cells(mut commands: Commands, query: Query<(Entity, &Position), Added<Position>>) {
    for (entity, pos) in query.iter() {
        commands.entity(entity).insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.5),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(pos.x as f32, pos.y as f32, 0.0),
            ..Default::default()
        });
    }
}
