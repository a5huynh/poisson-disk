mod utils;

use js_sys::Math;
use wasm_bindgen::prelude::*;

const PI: f64 = 3.141529;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Point(pub usize, pub usize);

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    // No object at this location
    EMPTY = 0,
    // No longer on the active list
    DEAD = 1,
    // In the active list (used to generate additional points)
    ACTIVE = 2,
}

#[wasm_bindgen]
pub struct PoissonDisk {
    width: u32,
    height: u32,
    cell_size: f64,
    cell_width: f64,
    cell_height: f64,
    cells: Vec<Cell>,
    radius: u32,
    num_samples: u32,
    // Grid used to determine point sampling.
    grid: Vec<Option<(usize, usize)>>,
    // List of points we want to generate more points around.
    active: Vec<(usize, usize)>,
    samples: Vec<Point>,
}

#[wasm_bindgen]
impl PoissonDisk {
    pub fn new(width: u32, height: u32, radius: u32, num_samples: u32) -> Self {
        let cells = vec![Cell::EMPTY; (width * height) as usize];

        // Step 0
        // Initialize an n-dimensional background grid for storing samples

        // We choose cell size to be radius / (dimensions) so that we
        // are guaranteed to have at most one point in any given cell.
        let cell_size = radius as f64 / (2.0 as f64).sqrt();
        let cell_width = (width as f64 / cell_size).ceil() + 1.0;
        let cell_height = (height as f64 / cell_size).ceil() + 1.0;
        let grid = vec![None; (cell_width * cell_height) as usize];

        let mut disk = PoissonDisk {
            width,
            height,
            cells,
            cell_size,
            cell_width,
            cell_height,
            grid,
            radius,
            num_samples,
            active: Vec::new(),
            samples: Vec::new(),
        };

        // Step 1
        // Select the initial sample to be randomly chosen uniformly in the domain.
        let point = (
            (Math::random() * width as f64) as usize,
            (Math::random() * height as f64) as usize,
        );

        disk.insert_point(point);
        disk.active.push(point);

        disk
    }

    fn distance(&self, pa: (usize, usize), pb: (usize, usize)) -> f64 {
        let dx = pa.0 as f64 - pb.0 as f64;
        let dy = pa.1 as f64 - pb.1 as f64;

        (dx * dx + dy * dy).sqrt()
    }

    fn is_valid(&self, point: (usize, usize)) -> bool {
        let xidx = (point.0 as f64 / self.cell_size).floor();
        let yidx = (point.1 as f64 / self.cell_size).floor();

        let start_x = (xidx - 2.0).max(0.0) as usize;
        let end_x = (xidx + 2.0).min(self.cell_width - 1.0) as usize;
        let start_y = (yidx - 2.0).max(0.0) as usize;
        let end_y = (yidx + 2.0).min(self.cell_height - 1.0) as usize;

        for x in start_x..end_x {
            for y in start_y..end_y {
                let cell_idx = y * self.cell_width as usize + x;
                if let Some(grid_point) = self.grid[cell_idx] {
                    if self.distance(point, grid_point) <= self.radius.into() {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn insert_point(&mut self, point: (usize, usize)) {
        let cell_x = (point.0 as f64 / self.cell_size).floor();
        let cell_y = (point.1 as f64 / self.cell_size).floor();

        let idx = point.1 * self.width as usize + point.0;
        self.cells[idx] = Cell::ACTIVE;

        let cell_idx = (cell_y * self.cell_width + cell_x) as usize;
        self.grid[cell_idx] = Some(point);
    }

    fn new_point(&mut self, point: (usize, usize)) -> (usize, usize) {
        let theta = 2.0 * PI * Math::random();
        // Pick a random radius between `r` and `2r`
        let new_radius = self.radius as f64 * (Math::random() + 1.0);
        // Find new coordinates relative to point p.
        let new_x = point.0 as f64 + new_radius * theta.cos();
        let new_y = point.1 as f64 + new_radius * theta.sin();

        (
            new_x.max(0.0).min(self.width as f64 - 1.0) as usize,
            new_y.max(0.0).min(self.height as f64 - 1.0) as usize,
        )
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn num_points(&self) -> usize {
        self.samples.len()
    }

    pub fn point_at_idx(&self, idx: usize) -> Point {
        let point = &self.samples[idx];
        Point(point.0, point.1)
    }

    pub fn reset(&mut self) {
        self.active.clear();
        self.grid.clear();
        self.samples.clear();

        let point = (
            (Math::random() * self.width as f64) as usize,
            (Math::random() * self.height as f64) as usize,
        );

        self.insert_point(point);
        self.active.push(point);
    }

    pub fn tick(&mut self) -> bool {
        // While the active list is not empty, choose a random index.
        if self.active.is_empty() {
            return false;
        }

        // Choose a point randomly from active list
        let idx = (Math::random() * (self.active.len() - 1) as f64) as usize;
        let point = self.active[idx];

        // Generate up to `k` points chosen uniformly from the spherical
        // annulus between radius `r` and `2r` around `x_{i}`.
        let mut found = false;
        for _ in 0..self.num_samples {
            let new_point = self.new_point(point);
            // Add the new point to the grid, active list, and to the
            // final grid.
            if self.is_valid(new_point) {
                self.insert_point(new_point);
                self.active.push(new_point);
                self.samples.push(Point(new_point.0, new_point.1));
                found = true;
            }
        }

        if !found {
            self.active.remove(idx);
            let cidx = point.1 * self.width as usize + point.0;
            self.cells[cidx] = Cell::DEAD;
        }

        true
    }
}
