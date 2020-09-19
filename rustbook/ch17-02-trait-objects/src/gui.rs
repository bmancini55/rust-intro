/// Defines a trait for the common behavior of drawing a GUI element.
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    /// Vector of draw trait objects. The trait object points to both
    /// an instance of the type implementing the trait and a table that
    /// is used to look up the trait methodvs on that type at runtime.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    /// Implemented using trait objects allows us to abstract across
    /// common behavior. We could have also implemented this using
    /// generic type parameters, but that would only allow a single
    /// concrete class to be used at runtime.
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // todo
    }
}
