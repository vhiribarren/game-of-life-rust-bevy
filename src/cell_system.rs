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
use std::{collections::BTreeSet, time::Duration};

use bevy::{prelude::*, utils::HashMap};

static NEIGHBOURS_DELTA: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct CellSet;

#[derive(Clone, Component, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct CellPosition {
    pub x: isize,
    pub y: isize,
}

#[derive(Resource, Debug)]
pub struct CellParams {
    pub playing: bool,
    pub period: Duration,
}

impl Default for CellParams {
    fn default() -> Self {
        Self {
            playing: true,
            period: Duration::from_secs(1),
        }
    }
}

#[derive(Resource)]
pub struct NextGenTimer(Timer);

pub struct CellSystem;

impl Plugin for CellSystem {
    fn build(&self, app: &mut App) {
        let cell_params = CellParams::default();
        let period = cell_params.period;
        app.insert_resource(cell_params)
            .insert_resource(NextGenTimer(Timer::new(period, TimerMode::Repeating)))
            .add_systems(Update, check_cell_params_changed)
            .add_systems(Startup, init_cells.in_set(CellSet))
            .add_systems(
                Update,
                system_cells
                    .in_set(CellSet)
                    .run_if(|params: Res<CellParams>| params.playing),
            );
    }
}

fn init_cells(mut commands: Commands) {
    commands.spawn(CellPosition { x: 0, y: 0 });
    commands.spawn(CellPosition { x: -1, y: 0 });
    commands.spawn(CellPosition { x: 0, y: -1 });
    commands.spawn(CellPosition { x: 0, y: 1 });
    commands.spawn(CellPosition { x: 1, y: 1 });
}

fn check_cell_params_changed(my_res: Res<CellParams>, mut timer: ResMut<NextGenTimer>) {
    if !my_res.is_changed() {
        return;
    }
    debug!("CellParams changed to: {:?}", *my_res);
    if my_res.period != timer.0.duration() {
        timer.0.set_duration(my_res.period);
        timer.0.reset();
    }
}

fn system_cells(
    mut commands: Commands,
    query: Query<(Entity, &CellPosition)>,
    mut timer: ResMut<NextGenTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }
    let mut neighbours = HashMap::new();
    let mut spawn_candidates = BTreeSet::new();
    // Compute number of alive neighbour cells
    for (_, cell) in &query {
        for pos_delta in NEIGHBOURS_DELTA.iter() {
            let scan_pos = CellPosition {
                x: cell.x + pos_delta.0,
                y: cell.y + pos_delta.1,
            };
            let neighbours_count = match neighbours.get(&scan_pos) {
                Some(prev_val) => prev_val + 1,
                None => 1,
            };
            neighbours.insert(scan_pos.clone(), neighbours_count);
            if neighbours_count == 3 {
                // This cell has 3 neighours, might spawn a new cell
                spawn_candidates.insert(scan_pos.clone());
            } else if neighbours_count == 4 {
                // Actually, this cell has too many neighbours, removing the candidate
                spawn_candidates.remove(&scan_pos);
            }
        }
    }
    // Killing starved or overpopulated cells
    for (entity, cell) in &query {
        let neighbours_count = *neighbours.get(cell).unwrap_or(&0);
        match neighbours_count {
            0..=1 => commands.entity(entity).despawn(),
            2 => (),
            3 => {
                spawn_candidates.remove(cell);
            }
            _ => commands.entity(entity).despawn(),
        }
    }
    // Spawn new cells
    for new_cell in spawn_candidates {
        commands.spawn(new_cell);
    }
}
