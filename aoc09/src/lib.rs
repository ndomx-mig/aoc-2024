use std::{fs, io, usize};

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

pub fn defragment(disk_map: &[MemoryBlock]) -> Vec<&MemoryBlock> {
    let mut sorted: Vec<&MemoryBlock> = disk_map.iter().collect();

    let mut block_indices = find_blocks(&disk_map);
    block_indices.reverse();

    for block_space in block_indices {
        let size = block_space.len();
        let max_idx = block_space[0];
        if let Some(idx) = find_free_space(&sorted, size, max_idx) {
            let free_indices: Vec<usize> = (idx..(idx + size)).collect();
            swap_many(&mut sorted, &block_space, &free_indices);
        }
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

fn find_blocks(disk_map: &[MemoryBlock]) -> Vec<Vec<usize>> {
    let mut blocks = Vec::new();

    let mut indices: Vec<usize> = Vec::new();
    let mut state = MemoryBlock::default();
    for (idx, block) in disk_map.iter().enumerate() {
        if block.memory_type == MemoryType::FREE {
            if state.memory_type == MemoryType::BLOCK {
                state = MemoryBlock::default();
                blocks.push(indices.clone());
                indices.clear();
            }

            continue;
        }

        let b_id = block.id.unwrap();

        if let Some(id) = state.id {
            if b_id != id {
                blocks.push(indices.clone());
                indices.clear();

                state.id = Some(b_id);
            }

            indices.push(idx);
        } else {
            indices.push(idx);
            state = MemoryBlock {
                memory_type: MemoryType::BLOCK,
                id: Some(b_id),
            }
        }
    }

    if indices.len() > 0 {
        blocks.push(indices);
    }

    return blocks;
}

fn find_free_space(disk_map: &[&MemoryBlock], size: usize, max_idx: usize) -> Option<usize> {
    for (idx, blocks) in disk_map[..max_idx].windows(size).enumerate() {
        let valid_window = blocks
            .iter()
            .all(|block| block.memory_type == MemoryType::FREE);

        if valid_window {
            return Some(idx);
        }
    }

    return None;
}

fn swap_many<T>(array: &mut Vec<&T>, idx_a: &[usize], idx_b: &[usize]) {
    for (&first, &second) in idx_a.iter().zip(idx_b.iter()) {
        array.swap(first, second);
    }
}
