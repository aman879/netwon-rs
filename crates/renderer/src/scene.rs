use physics::math::vec2::Vec2;

use crate::shapes::drawable::Drawable;

pub struct RenderObject {
    pub position: Vec2,
    pub shape: Box<dyn Drawable>,
}

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<RenderObject>
}

impl Scene {
    pub fn new() -> Self {
        Self { 
            objects: Vec::new(),  
        }
    }

    pub fn add_object(&mut self, object: RenderObject) {
        self.objects.push(object);
    }
}
