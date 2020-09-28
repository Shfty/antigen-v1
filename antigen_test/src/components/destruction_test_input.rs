use antigen::core::keyboard::Key;

#[derive(Debug, Copy, Clone)]
pub struct DestructionTestInput(pub Key);

impl From<Key> for DestructionTestInput {
    fn from(key: Key) -> Self {
        DestructionTestInput(key)
    }
}

impl Into<Key> for DestructionTestInput {
    fn into(self) -> Key {
        self.0
    }
}
