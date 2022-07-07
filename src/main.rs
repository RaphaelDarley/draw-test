use nannou::prelude::*;
use nannou::noise::*;

const THING_NUM: usize = 1000;
const WIDTH:u32 = 768;
const HEIGHT:u32 = 768;

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing {
    positions: Vec<Vector2>,
}
impl Thing {
    pub fn new(p: Vector2) -> Self {
        Thing { positions: vec![p] }
    }
    pub fn rand() -> Self {
        Thing { positions: vec![vec2(
            (random::<f32>()-0.5)*WIDTH as f32,
            (random::<f32>()-0.5)*HEIGHT as f32
        )] }
    }
}

struct Model {
    _window: window::Id,
    things: Vec<Thing>,
    noise: Perlin,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(WIDTH, HEIGHT).view(view).build().unwrap();
    let mut things = Vec::new();
        
    for _i in 0..THING_NUM{
        let thing = Thing::rand();
        things.push(thing);
    }
    let noise = Perlin::new();

    Model { _window, things, noise }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let sn = 0.01;

    for thing in model.things.iter_mut(){
        thing.positions.clear();
        *thing = Thing::rand();

        for _ in 0..50 {
            let last = thing.positions.last().unwrap().clone();
            let next = last + vec2(
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 0.0]) as f32,
                model.noise.get([sn*last.x as f64, sn*last.y as f64, 1.0]) as f32);
            thing.positions.push(next);
        }

    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    if app.elapsed_frames() == 1 {draw.background().color(BLACK);}
    draw.rect().w_h(WIDTH as f32, HEIGHT as f32).color(srgba(0.0,0.0,0.0,0.1));

    for thing in model.things.iter(){
        // draw.ellipse().xy(t.position).color(WHITE).radius(3.0);
        draw.polyline().points(thing.positions.iter().cloned()).color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

