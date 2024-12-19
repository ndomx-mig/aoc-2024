use std::{fs, io};

use memory::{memory_block::MemoryBlock, memory_type::MemoryType};

mod memory;

pub fn read_disk_map(filename: &str) -> io::Result<Vec<MemoryBlock>> {
    let contents = fs::read_to_string(filename)?;

    let mut blocks = Vec::new();
    for (idx, c) in contents.char_indices() {
        let memory_type = match idx & 0b1 {
            0 => MemoryType::BLOCK,
            _ => MemoryType::FREE,
        };

        let size = c.to_digit(10).unwrap();
        let id = (idx >> 1) as u32;

        (0..size).for_each(|_| {
            blocks.push(MemoryBlock {
                memory_type,
                id: match memory_type {
                    MemoryType::BLOCK => Some(id),
                    MemoryType::FREE => None,
                },
            });
        });
    }

    return Ok(blocks);
}

pub fn sort_disk_map(disk_map: &[MemoryBlock]) -> Vec<&MemoryBlock> {
    let mut sorted: Vec<&MemoryBlock> = disk_map.iter().collect();

    let mut free_indices: Vec<usize> = Vec::new();
    let mut block_indices: Vec<usize> = Vec::new();

    for (idx, block) in disk_map.iter().enumerate() {
        match block.memory_type {
            MemoryType::BLOCK => block_indices.push(idx),
            MemoryType::FREE => free_indices.push(idx),
        }
    }

    block_indices.reverse();

    for idx in 0..disk_map.len() {
        let free_idx = free_indices[idx];
        let block_idx = block_indices[idx];
        if free_idx > block_idx {
            break;
        }

        sorted.swap(free_idx, block_idx);
    }

    return sorted;
}

pub fn calculate_checksum(disk_map: &[&MemoryBlock]) -> u64 {
    return disk_map
        .iter()
        .enumerate()
        .filter_map(|(idx, block)| match block.id {
            Some(id) => Some(((idx as u32) * id) as u64),
            _ => None,
        })
        .sum();
}
