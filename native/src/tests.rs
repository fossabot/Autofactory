mod tests {
    use crate::blocks::*;
    struct ExampleBlockData {
        r: Rotation,
    }
    struct ExampleBlockType;
    impl BlockType<ExampleBlockData> for ExampleBlockType {
        fn get_rotation(&self, data: &ExampleBlockData) -> Rotation {
            data.r
        }
    }
    #[test]
    fn create_block_and_get_rotation() {}
}
