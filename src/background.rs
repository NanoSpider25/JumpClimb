use macroquad::prelude::*;

pub struct Mountain {
    peaks: Vec<Point>
}
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
    scale: f32
}

impl Point {
    fn new(x: f32, y: f32) -> Point {
        Point { x, y, scale: 100.0 }
    }

    fn update_scale(&mut self, scale: f32) {
        self.scale = scale;
    }
}

impl Mountain {

    pub fn new() -> Self {
        Self {
            peaks: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        self.peaks.clear();
        let origin = Point::new(0.0, 0.0);
        self.peaks.push(origin);
        self.generate_the_tain(37);
    }

    fn generate_the_tain(&mut self, limit: i32) {
        for _ in 0..limit {
            let xseed: f32 = rand::gen_range(0.0, 1.0);
            let yseed: f32 = rand::gen_range(0.0, 1.0);
            let sseed: f32 = rand::gen_range(1.0, 200.0);
            let prev = self.peaks.last_mut().unwrap();
            let mut point = Point::new(prev.x + xseed*sseed, prev.y + yseed*sseed);
            point.update_scale(sseed);
            self.peaks.push(point);
        }
    }

    pub fn draw(&self) {
        for i in 0..self.peaks.len().saturating_sub(1) {
            let point1 = &self.peaks[i];
            let point2 = &self.peaks[i+1];
            let x1 = point1.x;
            let y1 = screen_height() - point1.y;
            let x2 = point2.x;
            let y2 = screen_height() - point2.y;
            draw_line(x1, y1, x2, y2, 5.0, BLUE);
        }
        println!("Peaks: {:?}", &self.peaks);
    }

}