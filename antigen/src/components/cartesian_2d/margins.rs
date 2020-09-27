use crate::entity_component_system::ComponentDebugTrait;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Margins {
    left: i64,
    right: i64,
    top: i64,
    bottom: i64,
}

impl Margins {
    pub fn new(left: i64, right: i64, top: i64, bottom: i64) -> Self {
        Margins {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn get_margins(&self) -> (i64, i64, i64, i64) {
        (self.left, self.right, self.top, self.bottom)
    }

    pub fn set_margins(&mut self, top: i64, bottom: i64, left: i64, right: i64) -> &mut Self {
        self.top = top;
        self.bottom = bottom;
        self.left = left;
        self.right = right;
        self
    }
}

impl ComponentDebugTrait for Margins {
    fn get_name() -> String {
        "Margins".into()
    }

    fn get_description() -> String {
        "UI margins".into()
    }
}
