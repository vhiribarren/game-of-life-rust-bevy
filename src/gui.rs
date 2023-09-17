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

use std::time::Duration;

use crate::cell_system::{CellParams, CellPosition, CellSet};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, Color32, Ui},
    EguiContexts, EguiPlugin,
};
use egui_modal::Modal;
use rand::Rng;

type Seconds = f32;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const CELL_COLOR: Color = Color::rgb(0.0, 0.0, 0.2);
const SCALE_DEFAULT: f32 = 1.0 / 40.0;
const SCALE_MAX: f32 = 1.0;

const PERIOD_MIN: Seconds = 0.01;
const PERIOD_MAX: Seconds = 1.5;

pub struct GuiSystem;

impl Plugin for GuiSystem {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(GuiParams::default())
            .add_plugins(EguiPlugin)
            .add_systems(Startup, init_camera)
            .add_systems(Update, system_gui)
            .add_systems(Update, system_mouse_click)
            .add_systems(Update, system_keyboard_input)
            .add_systems(Update, system_draw_new_cells.before(CellSet))
            .add_systems(Update, system_draw_grid.after(system_draw_new_cells));
    }
}

#[derive(Resource, Debug)]
pub struct GuiParams {
    pub random_drag_value: u16,
}

impl Default for GuiParams {
    fn default() -> Self {
        Self {
            random_drag_value: 50_u16,
        }
    }
}

fn init_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = SCALE_DEFAULT;
    commands.spawn(camera);
}

fn system_gui(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut cell_params: ResMut<CellParams>,
    mut gui_params: ResMut<GuiParams>,
    mut q_camera: Query<(&mut OrthographicProjection, &GlobalTransform)>,
    q_cells: Query<Entity, With<CellPosition>>,
) {
    let ctx = contexts.ctx_mut();
    ctx.set_visuals(egui::style::Visuals::light());

    let (mut camera_proj, camera_transform) = q_camera.get_single_mut().unwrap();
    let speed_slider_init = period_to_slider(cell_params.period.as_secs_f32());
    let mut speed_slider_val = speed_slider_init;
    let scale_slider_init = scale_to_slider(camera_proj.scale);
    let mut scale_slider_val = scale_slider_init;

    let reset_modal = {
        let modal = Modal::new(ctx, "resel_modal");
        modal.show(|ui| {
            modal.title(ui, "Screen reset confirmation");
            modal.frame(ui, |ui| {
                modal.body(ui, "Do you confirm clearing the screen?");
            });
            modal.buttons(ui, |ui| {
                modal.button(ui, "Cancel");
                if modal.button(ui, "Clear Screen").clicked() {
                    cell_params.playing = false;
                    clear_cells(&mut commands, &q_cells);
                };
            });
        });
        modal
    };
    let random_modal = {
        let modal = Modal::new(ctx, "random_model");
        modal.show(|ui| {
            modal.title(ui, "Random reset confirmation");
            modal.frame(ui, |ui| {
                modal.body(ui, "Do you confirm filling the screen with random cells?");
            });
            modal.buttons(ui, |ui| {
                modal.button(ui, "Cancel");
                if modal.button(ui, "Random").clicked() {
                    let offset = -(gui_params.random_drag_value as isize) / 2;
                    let width = gui_params.random_drag_value as usize;
                    clear_cells(&mut commands, &q_cells);
                    random_cells(&mut commands, offset, offset, width, width);
                };
            });
        });
        modal
    };
    let mut panel_reset = |ui: &mut Ui| {
        ui.horizontal(|ui| {
            if ui.button("Clear board").clicked() {
                reset_modal.open();
            }
        });
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut gui_params.random_drag_value).suffix(" width"));
            if ui.button("Random cells").clicked() {
                random_modal.open();
            }
        });
    };
    let mut panel_run = |ui: &mut Ui| {
        ui.horizontal(|ui| {
            let play_text = if cell_params.playing { "Pause" } else { "Play" };
            if ui.button(play_text).clicked() {
                cell_params.playing = !cell_params.playing;
            }
            let next_step_btn =
                ui.add_enabled(!cell_params.playing, egui::Button::new("Next Step"));
            if !cell_params.playing && next_step_btn.clicked() {
                cell_params.compute_next_generation = true;
            };
        });
    };
    let mut panel_view = |ui: &mut Ui| {
        ui.vertical(|ui| {
            ui.add(
                egui::Slider::new(&mut speed_slider_val, 1.0..=100.0)
                    .text("Speed")
                    .show_value(false),
            );
            ui.add(
                egui::Slider::new(&mut scale_slider_val, 1.0..=100.0)
                    .text("Expand view")
                    .show_value(false)
                    .logarithmic(true),
            );
        });
    };
    let panel_info = |ui: &mut Ui| {
        ui.vertical(|ui| {
            let x = camera_transform.translation().x;
            let y = camera_transform.translation().y;
            ui.label(format!("Current position: x: {x}, y: {y}"));
            ui.add_space(5.);
            ui.label("Click to modify grid when not playing.");
            ui.label("Keyboard arrows to move around");
        });
    };
    let separator = |ui: &mut Ui| ui.add(egui::Separator::default());

    egui::Window::new("Game of Life")
        .resizable(false)
        .show(ctx, |ui| {
            panel_reset(ui);
            separator(ui);
            panel_view(ui);
            separator(ui);
            panel_run(ui);
            separator(ui);
            panel_info(ui);
        });

    // Those tests is important to avoid triggering a resource change if not needed
    if scale_slider_init != scale_slider_val {
        camera_proj.scale = slider_to_scale(scale_slider_val);
    }
    if speed_slider_init != speed_slider_val {
        cell_params.period = Duration::from_secs_f32(slider_to_period(speed_slider_val));
    }
}

fn system_draw_new_cells(
    mut commands: Commands,
    query: Query<(Entity, &CellPosition), Added<CellPosition>>,
) {
    for (entity, pos) in query.iter() {
        commands.entity(entity).insert(SpriteBundle {
            sprite: Sprite {
                color: CELL_COLOR,
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(pos.x as f32, pos.y as f32, 0.0),
            ..Default::default()
        });
    }
}

fn system_mouse_click(
    mut commands: Commands,
    cell_params: Res<CellParams>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_cellpos: Query<(Entity, &CellPosition)>,
    buttons: Res<Input<MouseButton>>,
) {
    if cell_params.playing || !buttons.just_released(MouseButton::Left) {
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

fn system_keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut q_camera_transform: Query<&mut Transform, With<Camera>>,
) {
    let (mut x, mut y) = (0, 0);
    if keys.pressed(KeyCode::Left) {
        x += -1;
    }
    if keys.pressed(KeyCode::Right) {
        x += 1;
    }
    if keys.pressed(KeyCode::Up) {
        y += 1;
    }
    if keys.pressed(KeyCode::Down) {
        y += -1;
    }
    let mut transform = q_camera_transform.single_mut();
    transform.translation += Vec3::new(x as f32, y as f32, 0.0);
}

fn system_draw_grid(
    mut contexts: EguiContexts,
    q_camera: Query<(&Camera, &OrthographicProjection, &GlobalTransform)>,
) {
    const LINE_COLOR: Color32 = Color32::BLACK;
    let (camera, camera_proj, camera_transform) = q_camera.get_single().unwrap();
    let ctx = contexts.ctx_mut();
    let transparent_frame = egui::containers::Frame {
        fill: Color32::TRANSPARENT,
        ..Default::default()
    };
    let line_width =
        (1.0 - (camera_proj.scale - SCALE_DEFAULT) / (SCALE_MAX - SCALE_DEFAULT)).powi(10);

    egui::CentralPanel::default()
        .frame(transparent_frame)
        .show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(
                bevy_egui::egui::Vec2::new(ui.available_width(), ui.available_height()),
                egui::Sense {
                    click: false,
                    drag: false,
                    focusable: false,
                },
            );
            let visible_top_left = camera
                .viewport_to_world(camera_transform, Vec2 { x: 0.0, y: 0.0 })
                .map(|ray| ray.origin.truncate())
                .unwrap();
            let (x_min, y_max) = (
                visible_top_left.x.round() as isize,
                visible_top_left.y.round() as isize,
            );
            let visible_bottom_right = camera
                .viewport_to_world(
                    camera_transform,
                    Vec2 {
                        x: response.rect.right(),
                        y: response.rect.bottom(),
                    },
                )
                .map(|ray| ray.origin.truncate())
                .unwrap();
            let (x_max, y_min) = (
                visible_bottom_right.x.round() as isize,
                visible_bottom_right.y.round() as isize,
            );
            for x in x_min..=x_max {
                let start = camera
                    .world_to_viewport(
                        camera_transform,
                        Vec3 {
                            x: x as f32 - 0.5,
                            y: y_min as f32 - 0.5,
                            z: 0.0,
                        },
                    )
                    .unwrap();
                let start = egui::Pos2::new(start.x, start.y);
                let end = camera
                    .world_to_viewport(
                        camera_transform,
                        Vec3 {
                            x: x as f32 - 0.5,
                            y: y_max as f32 + 0.5,
                            z: 0.0,
                        },
                    )
                    .unwrap();
                let end = egui::Pos2::new(end.x, end.y);
                painter.add(egui::Shape::LineSegment {
                    points: [start, end],
                    stroke: egui::Stroke {
                        width: line_width,
                        color: LINE_COLOR,
                    },
                });
            }
            for y in y_min..=y_max {
                let start = camera
                    .world_to_viewport(
                        camera_transform,
                        Vec3 {
                            x: x_min as f32 - 0.5,
                            y: y as f32 - 0.5,
                            z: 0.0,
                        },
                    )
                    .unwrap();
                let start = egui::Pos2::new(start.x, start.y);
                let end = camera
                    .world_to_viewport(
                        camera_transform,
                        Vec3 {
                            x: x_max as f32 + 0.5,
                            y: y as f32 - 0.5,
                            z: 0.0,
                        },
                    )
                    .unwrap();
                let end = egui::Pos2::new(end.x, end.y);
                painter.add(egui::Shape::LineSegment {
                    points: [start, end],
                    stroke: egui::Stroke {
                        width: line_width,
                        color: LINE_COLOR,
                    },
                });
            }
        });
}

fn clear_cells(commands: &mut Commands, q_cells: &Query<Entity, With<CellPosition>>) {
    for entity in q_cells.iter() {
        commands.entity(entity).despawn();
    }
}

fn random_cells(commands: &mut Commands, x: isize, y: isize, width: usize, height: usize) {
    let mut rng = rand::thread_rng();
    for coord_x in x..(x + width as isize) {
        for coord_y in y..(y + height as isize) {
            if rng.gen::<bool>() {
                commands.spawn(CellPosition {
                    x: coord_x,
                    y: coord_y,
                });
            }
        }
    }
}

fn period_to_slider(period: f32) -> f32 {
    (100.0 - 99.0 * (period - PERIOD_MIN) / (PERIOD_MAX - PERIOD_MIN)).clamp(1.0, 100.0)
}

fn slider_to_period(slider: f32) -> f32 {
    ((100.0 - slider) * (PERIOD_MAX - PERIOD_MIN) / 99.0 + PERIOD_MIN).clamp(PERIOD_MIN, PERIOD_MAX)
}

fn scale_to_slider(scale: f32) -> f32 {
    (1.0 + 99.0 * (scale - SCALE_DEFAULT) / (SCALE_MAX - SCALE_DEFAULT)).clamp(1.0, 100.0)
}

fn slider_to_scale(slider: f32) -> f32 {
    ((slider - 1.0) * (SCALE_MAX - SCALE_DEFAULT) / 99.0 + SCALE_DEFAULT)
        .clamp(SCALE_DEFAULT, SCALE_MAX)
}
