use crate::blocks::{block_type::BlockType, block_type_info::BlockTypeInfo};

#[derive(Copy)]
pub struct BlockInfo {
    block_type: BlockType,
}

impl BlockInfo {
    pub fn new(block_type: BlockType) -> BlockInfo {
        BlockInfo {
            block_type: block_type,
        }
    }

    pub fn get_block_type_info(&self) -> &'static BlockTypeInfo {
        self.block_type.get_block_type_info().unwrap()
    }

    pub fn get_block_type(&self) -> &BlockType {
        &self.block_type
    }
}

impl Clone for BlockInfo {
    fn clone(&self) -> BlockInfo {
        BlockInfo {
            block_type: self.block_type,
        }
    }
}
