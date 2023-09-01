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

use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use cell_system::{CellParams, CellPosition, CellSet, CellSystem};

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.3, 0.6);
const DEFAULT_SCALE: f32 = 1.0 / 40.0;
const MAX_SCALE: f32 = 1.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(CellSystem)
        .add_systems(Startup, init_camera)
        .add_systems(Update, system_gui)
        .add_systems(Update, system_mouse_click)
        .add_systems(Update, system_draw_new_cells.before(CellSet))
        .run();
}

fn init_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = DEFAULT_SCALE;
    commands.spawn(camera);
}

fn system_gui(
    mut contexts: EguiContexts,
    mut cell_params: ResMut<CellParams>,
    mut camera: Query<&mut OrthographicProjection>,
) {
    let ctx = contexts.ctx_mut();
    ctx.set_visuals(egui::style::Visuals::light());

    let mut speed_val = cell_params.period.as_secs_f32();
    let mut camera = camera.get_single_mut().unwrap();
    let scale_slider_init = scale_to_slider(camera.scale);
    let mut scale_slider_val = scale_slider_init;
    egui::Window::new("Game of Life")
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider::new(&mut speed_val, 0.01..=5.)
                        .text("Next generation period")
                        .suffix("s")
                        .logarithmic(true),
                );
            });
            ui.horizontal(|ui| {
                let play_text = if cell_params.playing { "Pause" } else { "Play" };
                if ui.button(play_text).clicked() {
                    cell_params.playing = !cell_params.playing;
                }
                if !cell_params.playing && ui.button("Next Step").clicked() {
                    cell_params.compute_next_generation = true;
                };
            });
            ui.add(egui::Separator::default());
            ui.add(
                egui::Slider::new(&mut scale_slider_val, 1.0..=100.0)
                    .text("Scale")
                    .logarithmic(true),
            );
        });
    // This test is important to avoid triggering a resource change if not needed
    if cell_params.period.as_secs_f32() != speed_val {
        cell_params.period = Duration::from_secs_f32(speed_val);
    }
    if scale_slider_init != scale_slider_val {
        camera.scale = slider_to_scale(scale_slider_val);
    }
}

fn system_draw_new_cells(
    mut commands: Commands,
    query: Query<(Entity, &CellPosition), Added<CellPosition>>,
) {
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

fn scale_to_slider(scale: f32) -> f32 {
    (1.0 + 99.0 * (scale - DEFAULT_SCALE) / (MAX_SCALE - DEFAULT_SCALE)).clamp(1.0, 100.0)
}

fn slider_to_scale(slider: f32) -> f32 {
    ((slider - 1.0) * (MAX_SCALE - DEFAULT_SCALE) / 99.0 + DEFAULT_SCALE)
        .clamp(DEFAULT_SCALE, MAX_SCALE)
}

fn system_mouse_click(
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_cellpos: Query<(Entity, &CellPosition)>,
    buttons: Res<Input<MouseButton>>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(cursor_position) = q_windows.single().cursor_position() else {
        return;
    };
    let (camera, camera_transform) = q_camera.single();
    let Some(target_pos) = camera
        .viewport_to_world(camera_transform, cursor_position)
        .map(|ray| ray.origin.truncate().round())
    else {
        return;
    };
    debug!("Clicked on: {target_pos}");
    let new_cell = CellPosition {
        x: target_pos.x as isize,
        y: target_pos.y as isize,
    };
    for (entity, cell_pos) in q_cellpos.iter() {
        if cell_pos == &new_cell {
            commands.entity(entity).despawn();
            return;
        }
    }
    commands.spawn(new_cell);
}
