#[derive(Debug, Clone)]
pub struct InputAxis {
    negative_input: antigen::core::keyboard::Key,
    positive_input: antigen::core::keyboard::Key,
}

impl InputAxis {
    pub fn new(
        negative_input: antigen::core::keyboard::Key,
        positive_input: antigen::core::keyboard::Key,
    ) -> Self {
        InputAxis {
            negative_input,
            positive_input,
        }
    }

    pub fn get_positive_input(&self) -> antigen::core::keyboard::Key {
        self.positive_input
    }

    pub fn get_negative_input(&self) -> antigen::core::keyboard::Key {
        self.negative_input
    }
}
