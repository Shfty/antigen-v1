use antigen::core::keyboard::Key;

#[derive(Debug, Copy, Clone)]
pub struct DestructionTestInputData(pub Key);

impl From<Key> for DestructionTestInputData {
    fn from(key: Key) -> Self {
        DestructionTestInputData(key)
    }
}

impl Into<Key> for DestructionTestInputData {
    fn into(self) -> Key {
        self.0
    }
}
