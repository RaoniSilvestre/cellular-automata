use macroquad::prelude::*;

use crate::automaton::{Automaton, Point};

const HEADER: f32 = 40.0;

#[derive(Debug, Clone, Copy)]
pub struct Layout {
    origin_x: f32,
    origin_y: f32,
    cell_w: f32,
    cell_h: f32,
    width: usize,
    height: usize,
}

impl Layout {
    pub fn compute<A: Automaton>(automaton: &A) -> Self {
        let board = automaton.board();
        let width = board.width();
        let height = board.height();

        let sw = screen_width();
        let sh = screen_height() - HEADER;
        let cell_w = sw / width as f32;
        let cell_h = sh / height as f32;

        Self {
            origin_x: 0.0,
            origin_y: HEADER,
            cell_w,
            cell_h,
            width,
            height,
        }
    }

    pub fn screen_to_cell(&self, (mx, my): (f32, f32)) -> Option<Point> {
        if mx < self.origin_x || my < self.origin_y {
            return None;
        }

        let col = ((mx - self.origin_x) / self.cell_w) as usize;
        let row = ((my - self.origin_y) / self.cell_h) as usize;

        if col >= self.width || row >= self.height {
            return None;
        }

        Some((row as i32, col as i32))
    }
}

pub fn draw<A: Automaton>(automaton: &A, layout: &Layout) {
    clear_background(BLACK);

    let board = automaton.board();

    for (idx, cell) in board.cells().iter().enumerate() {
        let row = idx / layout.width;
        let col = idx % layout.width;
        let x = layout.origin_x + col as f32 * layout.cell_w;
        let y = layout.origin_y + row as f32 * layout.cell_h;

        draw_rectangle(
            x,
            y,
            layout.cell_w,
            layout.cell_h,
            automaton.paint_cell(cell),
        );
    }

    let title = format!("{} — Iterations: {}", A::name(), automaton.iterations());
    draw_text(&title, 10.0, 24.0, 24.0, WHITE);
    draw_legend(automaton);
}

fn draw_legend<A: Automaton>(automaton: &A) {
    let legend = automaton.legend();
    if legend.is_empty() {
        return;
    }

    const SWATCH: f32 = 12.0;
    const FONT: f32 = 18.0;
    const PAD: f32 = 6.0;
    const ROW_H: f32 = 20.0;

    let label_width = legend
        .iter()
        .map(|(label, _)| measure_text(label, None, FONT as u16, 1.0).width)
        .fold(0.0, f32::max);

    let panel_w = PAD * 3.0 + SWATCH + label_width;
    let panel_h = PAD * 2.0 + legend.len() as f32 * ROW_H;
    let panel_x = screen_width() - panel_w - 10.0;
    let panel_y = HEADER;

    draw_rectangle(
        panel_x,
        panel_y,
        panel_w,
        panel_h,
        Color::from_rgba(0, 0, 0, 200),
    );

    for (i, (label, color)) in legend.iter().enumerate() {
        let y = panel_y + PAD + i as f32 * ROW_H;
        draw_rectangle(panel_x + PAD, y, SWATCH, SWATCH, *color);
        draw_text(
            label,
            panel_x + PAD + SWATCH + PAD,
            y + SWATCH - 2.0,
            FONT,
            WHITE,
        );
    }
}
