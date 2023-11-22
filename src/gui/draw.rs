use eframe::{egui, epaint::{CircleShape, RectShape}};
use egui::*;

pub enum Shape{
    Circle(epaint::CircleShape),
    Rect(epaint::RectShape),
    Arrow(ArrowShape),
} 

//Arrow is not implemented as struct but it has only a function

pub struct ArrowShape{
    pub origin: Pos2,
    pub vec: Vec2,
    pub stroke: Stroke
}

#[derive(Clone, Copy)]
pub struct Rgb{
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Rgb{
    pub fn new(red: u8, green: u8, blue: u8) -> Self{
        return Rgb{red: red, green: green, blue: blue};
    }
}

pub fn create_figure(vec_shape: &mut Vec<Shape>, ctx: &egui::Context, shape: i32, color: Rgb,x:f32,y:f32,image_width:f32,image_hight:f32){
    if ctx.input(|i| i.pointer.button_clicked(PointerButton::Primary)){
        let pos_mouse = ctx.input(|i| i.pointer.hover_pos().unwrap());
       
        let x1=(x/2.0)-18.0;
        let x2=((x/2.0)+image_width)+18.0;
        let y2=image_hight+23.0;
       
        if pos_mouse.y>y && pos_mouse.y<y2 && pos_mouse.x>x1 && pos_mouse.x<x2 {
        match shape {
            0=> {    
                    let circle = epaint::CircleShape{
                        center: pos_mouse,
                        radius: 15.0,
                        fill: Color32::TRANSPARENT,
                        stroke: Stroke{width: 2.0, color: Color32::from_rgb(color.red, color.green, color.blue)}
                    };
                    vec_shape.push(Shape::Circle(circle));
                }
            1=> {
                    let rectangle = epaint::RectShape{
                        rect: Rect{min: pos_mouse, max: epaint::pos2(pos_mouse.x + 10.0, pos_mouse.y+10.0)},
                        rounding: Rounding::ZERO,
                        fill: Color32::TRANSPARENT,
                        stroke: Stroke { width: 2.0, color: Color32::from_rgb(color.red, color.green, color.blue)},
                        fill_texture_id: TextureId::User(0),
                        uv: Rect::ZERO
                    };
                    vec_shape.push(Shape::Rect(rectangle));
                }
            2=> {
                    let arrow = ArrowShape{
                        origin: pos_mouse,
                        vec: Vec2{x: 22.0, y: 55.0},
                        stroke: Stroke { width: 5.0, color: Color32::from_rgb(color.red, color.green, color.blue)},
                    };
                    vec_shape.push(Shape::Arrow(arrow));
                }
            _=>print!("Error")
        }
    }
}
}

pub fn draw(ui: &mut Ui, vec_shape: &mut Vec<Shape>){
    for shape in vec_shape.iter(){
        match shape {
            Shape::Circle(shape) =>  draw_circle(ui, shape),
            Shape::Rect(shape) => draw_rect(ui, shape),
            Shape::Arrow(shape) => draw_arrow(ui, shape),
        }
    }
}


fn draw_circle(ui: &mut Ui, shape: &CircleShape){
    let painter = ui.painter();
    painter.circle(
        shape.center,
        shape.radius,
        shape.fill, 
        shape.stroke,
    );
}

fn draw_rect(ui: &mut Ui, shape: &RectShape){
    let painter = ui.painter();
    painter.rect(
        shape.rect,
        shape.rounding,
        shape.fill,
        shape.stroke
        )
}
fn draw_arrow(ui: &mut Ui, shape: &ArrowShape){
    let painter = ui.painter();
    painter.arrow(
        shape.origin,
        shape.vec,
        shape.stroke
    );
}