#[derive(Debug, Clone)]
pub struct InputAxisData {
    negative_input: antigen::core::keyboard::Key,
    positive_input: antigen::core::keyboard::Key,
}

impl InputAxisData {
    pub fn new(
        negative_input: antigen::core::keyboard::Key,
        positive_input: antigen::core::keyboard::Key,
    ) -> Self {
        InputAxisData {
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
