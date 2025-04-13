use minifb::{Key, MouseButton, MouseMode, Scale, Window, WindowOptions};
use nalgebra::{Point2};
use std::time::{Duration, Instant};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const POINT_RADIUS: i32 = 5;
const ANIMATION_STEP_DURATION: Duration = Duration::from_millis(500);
const MAX_ANIMATION_STEPS: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq)]
struct ControlPoint {
    position: Point2<f32>,
    selected: bool,
}

impl ControlPoint {
    fn new(x: f32, y: f32) -> Self {
        Self {
            position: Point2::new(x, y),
            selected: false,
        }
    }
}

struct ChaikinCurve {
    control_points: Vec<ControlPoint>,
    animation_steps: Vec<Vec<Point2<f32>>>,
    current_step: usize,
    animating: bool,
    last_step_time: Instant,
}

impl ChaikinCurve {
    fn new() -> Self {
        Self {
            control_points: Vec::new(),
            animation_steps: Vec::new(),
            current_step: 0,
            animating: false,
            last_step_time: Instant::now(),
        }
    }

    fn add_point(&mut self, x: f32, y: f32) {
        self.control_points.push(ControlPoint::new(x, y));
    }

    fn clear_points(&mut self) {
        self.control_points.clear();
        self.reset_animation();
    }

    fn reset_animation(&mut self) {
        self.animation_steps.clear();
        self.current_step = 0;
        self.animating = false;
    }

    fn start_animation(&mut self) {
        if self.control_points.len() <= 1 {
            return; // Need at least 2 points to draw a curve
        }

        self.reset_animation();
        self.generate_animation_steps();
        self.animating = true;
        self.last_step_time = Instant::now();
    }

    fn generate_animation_steps(&mut self) {
        self.animation_steps.clear();

        // Initial step is just the control points
        let initial_points: Vec<Point2<f32>> = self.control_points
            .iter()
            .map(|cp| cp.position)
            .collect();
        
        self.animation_steps.push(initial_points.clone());

        let mut current_points = initial_points;
        
        // Generate the subsequent steps
        for _ in 0..MAX_ANIMATION_STEPS {
            current_points = self.chaikin_step(&current_points);
            self.animation_steps.push(current_points.clone());
        }
    }

    fn chaikin_step(&self, points: &[Point2<f32>]) -> Vec<Point2<f32>> {
        if points.len() <= 2 {
            return points.to_vec();
        }

        let mut result = Vec::new();

        // For open curves, keep the first and last points
        result.push(points[0]);

        for i in 0..points.len() - 1 {
            let p0 = points[i];
            let p1 = points[i + 1];

            // Calculate 1/4 and 3/4 points (Chaikin's algorithm)
            // Instead of adding points directly, we interpolate between them
            let q = Point2::new(
                p0.x * 0.75 + p1.x * 0.25,
                p0.y * 0.75 + p1.y * 0.25
            );
            
            let r = Point2::new(
                p0.x * 0.25 + p1.x * 0.75,
                p0.y * 0.25 + p1.y * 0.75
            );

            result.push(q);
            result.push(r);
        }

        result.push(*points.last().unwrap());
        result
    }

    fn update_animation(&mut self) {
        if !self.animating || self.animation_steps.is_empty() {
            return;
        }

        if self.last_step_time.elapsed() >= ANIMATION_STEP_DURATION {
            self.current_step = (self.current_step + 1) % self.animation_steps.len();
            self.last_step_time = Instant::now();
        }
    }

    fn select_point_at(&mut self, x: f32, y: f32) -> bool {
        for point in &mut self.control_points {
            let dx = point.position.x - x;
            let dy = point.position.y - y;
            if dx * dx + dy * dy <= (POINT_RADIUS as f32 * POINT_RADIUS as f32) {
                point.selected = true;
                return true;
            }
        }
        false
    }

    fn deselect_all_points(&mut self) {
        for point in &mut self.control_points {
            point.selected = false;
        }
    }

    fn move_selected_point(&mut self, x: f32, y: f32) {
        for point in &mut self.control_points {
            if point.selected {
                point.position.x = x;
                point.position.y = y;
            }
        }
    }

    fn get_current_points(&self) -> Vec<Point2<f32>> {
        if self.animating && !self.animation_steps.is_empty() {
            // Return the current step of the animation
            self.animation_steps[self.current_step].clone()
        } else if self.control_points.len() >= 2 {
            // Return just the control points when not animating
            self.control_points.iter().map(|cp| cp.position).collect()
        } else if self.control_points.len() == 1 {
            // Return the single point
            vec![self.control_points[0].position]
        } else {
            // No points at all
            vec![]
        }
    }
}

fn draw_point(buffer: &mut Vec<u32>, x: i32, y: i32, color: u32) {
    if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
        buffer[y as usize * WIDTH + x as usize] = color;
    }
}

fn draw_circle(buffer: &mut Vec<u32>, center_x: i32, center_y: i32, radius: i32, color: u32) {
    for y in -radius..=radius {
        for x in -radius..=radius {
            if x * x + y * y <= radius * radius {
                draw_point(buffer, center_x + x, center_y + y, color);
            }
        }
    }
}

fn draw_line(buffer: &mut Vec<u32>, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    
    let mut err = dx + dy;
    let mut x = x0;
    let mut y = y0;
    
    loop {
        draw_point(buffer, x, y, color);
        if x == x1 && y == y1 {
            break;
        }
        
        let e2 = 2 * err;
        if e2 >= dy {
            if x == x1 {
                break;
            }
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            if y == y1 {
                break;
            }
            err += dx;
            y += sy;
        }
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    let mut window = Window::new(
        "Chaikin Curve Animation",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X1,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    // We don't want to tear now
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut chaikin = ChaikinCurve::new();
    let mut dragging = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer
        for i in buffer.iter_mut() {
            *i = 0;
        }

        // Handle mouse input
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
            let mouse_x = x as f32;
            let mouse_y = y as f32;

            if window.get_mouse_down(MouseButton::Left) {
                if !dragging {
                    // Check if we're clicking on an existing point
                    if !chaikin.select_point_at(mouse_x, mouse_y) {
                        // If not, add a new point
                        chaikin.add_point(mouse_x, mouse_y);
                        chaikin.reset_animation();
                    }
                    dragging = true;
                } else {
                    // Move the selected point
                    chaikin.move_selected_point(mouse_x, mouse_y);
                    if chaikin.animating {
                        chaikin.generate_animation_steps();
                    }
                }
            } else {
                if dragging {
                    chaikin.deselect_all_points();
                    dragging = false;
                }
            }
        }

        // Handle keyboard input
        if window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No) {
            chaikin.start_animation();
        }

        if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            chaikin.clear_points();
        }

        // Update animation
        chaikin.update_animation();

        // Draw all control points
        for point in &chaikin.control_points {
            draw_circle(
                &mut buffer, 
                point.position.x as i32, 
                point.position.y as i32, 
                POINT_RADIUS, 
                if point.selected { 0xFFFF0000 } else { 0xFFFFFFFF }
            );
        }

        // Draw the current curve
        let current_points = chaikin.get_current_points();
        if current_points.len() == 1 {
            // Draw just the point
            draw_circle(
                &mut buffer,
                current_points[0].x as i32,
                current_points[0].y as i32,
                3,
                0xFF00FF00,
            );
        } else if current_points.len() >= 2 {
            // Draw line segments
            for i in 0..current_points.len() - 1 {
                draw_line(
                    &mut buffer,
                    current_points[i].x as i32,
                    current_points[i].y as i32,
                    current_points[i + 1].x as i32,
                    current_points[i + 1].y as i32,
                    0xFF00FF00,
                );
            }
        }

        // Update the window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}