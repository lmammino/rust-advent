use crate::models::{find_loop, Map};
use nannou::prelude::*;

mod models;

const WIN_WIDTH: u32 = 1600;
const WIN_HEIGHT: u32 = 1600;

struct Model {
    _window: WindowId,
    step: usize,
    map: Map<140, 140>,
    path: Vec<(usize, usize)>,
}

pub fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let input = include_str!("../input.txt");
    let map: Map<140, 140> = input.parse().unwrap();
    let path = find_loop(&map).unwrap();

    app.set_loop_mode(LoopMode::rate_fps(60.0));

    let window = app
        .new_window()
        .resizable(false)
        .size(WIN_WIDTH, WIN_HEIGHT)
        .title("y2023day10 simulation - 00 FPS")
        .view(view) // The function that will be called for presenting graphics to a frame.
        .build()
        .unwrap();

    Model {
        step: 0,
        _window: window,
        map,
        path,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.step < model.path.len() - 1 {
        model.step += 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let w = WIN_WIDTH as f32 / 140.0;
    let h = WIN_HEIGHT as f32 / 140.0;

    let current_in_path = model.path[model.step];
    let previous_in_path = model.path.get(model.step - 1).copied();

    // draw map
    if model.step == 1 {
        frame.clear(CORNFLOWERBLUE);

        for x in 0..140 {
            for y in 0..140 {
                let draw_x = (x as f32 * w) - (WIN_WIDTH as f32 / 2.0) + (w / 2.0);
                let draw_y = -1.0 * ((y as f32 * h) - (WIN_HEIGHT as f32 / 2.0) + (h / 2.0));
                let tile = model.map.get_tile((x, y));
                let mut color = match tile {
                    models::Tile::Ground => BLACK,
                    models::Tile::Pipe(_) => SADDLEBROWN,
                };
                let is_in_path = model.path[0..model.step].contains(&(x, y));
                let is_current_cell = current_in_path == (x, y);
                if is_in_path {
                    color = YELLOW;
                }
                if is_current_cell {
                    color = YELLOWGREEN;
                }

                draw.rect().x_y(draw_x, draw_y).w_h(w, h).color(color);
                // draw text
                match tile {
                    models::Tile::Ground => {}
                    models::Tile::Pipe(pipe) => {
                        let text = format!("{}", pipe);
                        draw.text(&text)
                            .x_y(draw_x, draw_y)
                            .font_size((h - 1.0) as u32)
                            .color(BLACK);
                    }
                }
            }
        }
    }

    // draw previous part of the path
    if let Some((x, y)) = previous_in_path {
        let draw_x = (x as f32 * w) - (WIN_WIDTH as f32 / 2.0) + (w / 2.0);
        let draw_y = -1.0 * ((y as f32 * h) - (WIN_HEIGHT as f32 / 2.0) + (h / 2.0));
        draw.rect().x_y(draw_x, draw_y).w_h(w, h).color(YELLOW);

        // draw text
        let tile = model.map.get_tile((x, y));
        match tile {
            models::Tile::Ground => {}
            models::Tile::Pipe(pipe) => {
                let text = format!("{}", pipe);
                draw.text(&text)
                    .x_y(draw_x, draw_y)
                    .font_size((h - 1.0) as u32)
                    .color(BLACK);
            }
        }
    }

    // draw current part of the path
    let (x, y) = current_in_path;
    let draw_x = (x as f32 * w) - (WIN_WIDTH as f32 / 2.0) + (w / 2.0);
    let draw_y = -1.0 * ((y as f32 * h) - (WIN_HEIGHT as f32 / 2.0) + (h / 2.0));
    draw.rect().x_y(draw_x, draw_y).w_h(w, h).color(RED);

    // draw text
    let tile = model.map.get_tile((x, y));
    match tile {
        models::Tile::Ground => {}
        models::Tile::Pipe(pipe) => {
            let text = format!("{}", pipe);
            draw.text(&text)
                .x_y(draw_x, draw_y)
                .font_size((h - 1.0) as u32)
                .color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();

    app.main_window()
        .set_title(&format!("y2023day10 simulation - {} FPS", app.fps() as u32));
}
