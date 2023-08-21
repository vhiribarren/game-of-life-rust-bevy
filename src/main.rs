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
use std::collections::BTreeSet;

use bevy::{prelude::*, utils::HashMap};

fn main() {
    App::new()
        .add_systems(Startup, init_cells)
        .add_systems(Update, cell_system)
        .run();
}

fn init_cells(mut commands: Commands) {
    commands.spawn(Position { x: 0, y: 0 });
    commands.spawn(Position { x: 1, y: 0 });
    commands.spawn(Position { x: -1, y: 0 });
    commands.spawn(Position { x: 0, y: 1 });
}

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

fn cell_system(mut commands: Commands, query: Query<(Entity, &Position)>) {
    let mut neighbours = HashMap::new();
    let mut spawn_candidates = BTreeSet::new();
    // Compute number of alive neighbour cells
    for (_, cell) in &query {
        for pos_delta in NEIGHBOURS_DELTA.iter() {
            let scan_pos = Position {
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
        let neighbours_count = *neighbours
            .get(cell)
            .expect("Shoud have been inserted in previous loop");
        println!("cell {:?} has {:?} neighbours", cell, neighbours.get(cell));
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
        println!("spawn new cell {:?}", new_cell);
        commands.spawn(new_cell);
    }
}

#[derive(Clone, Component, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
struct Position {
    x: isize,
    y: isize,
}
