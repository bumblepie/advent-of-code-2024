
use std::ops::Range;
use itertools::Itertools;

pub fn part_1(input_file: &str) {
    let filesystem = parse_file(input_file);
    let compacted_filesystem = filesystem.compact();
    println!("Checksum: {}", compacted_filesystem.checksum());
}

pub fn part_2(input_file: &str) {
    let filesystem = parse_file(input_file);
    let compacted_filesystem = filesystem.compact_v2();
    println!("Checksum: {}", compacted_filesystem.checksum());
}

fn parse_file(input_file: &str) -> FileSystem {
    let line = std::fs::read_to_string(input_file)
        .expect("Unable to read file");
    let mut next_file_id = 0;
    let mut pointer = 0;
    let mut next_block_is_empty = false;
    let mut filesystem = FileSystem {
        blocks: Vec::new(),
    };
    for c in line.chars() {
        let length = c.to_digit(10).expect("Unexpected character") as usize;
        if length == 0 {
            if !next_block_is_empty {
                next_file_id += 1;
            }
            next_block_is_empty = !next_block_is_empty;
            continue;
        }
        let file_id = if next_block_is_empty {
            None
        } else {
            let id = Some(next_file_id);
            next_file_id += 1;
            id
        };
        filesystem.blocks.push(ContiguousFileBlocks {
            file_id,
            range: pointer..pointer + length,
        });

        next_block_is_empty = !next_block_is_empty;
        pointer += length;
    }
    filesystem
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct FileSystem {
    blocks: Vec<ContiguousFileBlocks>
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct ContiguousFileBlocks {
    file_id: Option<u64>,
    range: Range<usize>,
}

impl FileSystem {
    fn compact(&self) -> FileSystem {
        let mut compacted = self.clone();

        let mut pointer = 0;

        while pointer < compacted.blocks.len() {
            let next_block = &compacted.blocks[pointer];

            if next_block.file_id.is_some() {
                pointer += 1;
                continue;
            }

            let empty_block = compacted.blocks.remove(pointer);

            let mut replacement_block = compacted.blocks.pop().unwrap();
            while replacement_block.file_id.is_none() {
                replacement_block = compacted.blocks.pop().unwrap();
            }

            if pointer > compacted.blocks.len() {
                // The replacement block is the only thing after the empty block we're replacing
                // So just add it back directly, otherwise we run into index out of range errors
                compacted.blocks.push(replacement_block);
                continue;
            }

            let empty_length = empty_block.range.len();
            let replacement_length = replacement_block.range.len();

            if empty_length > replacement_length {
                // Insert remaining free space first, so it comes after the replacement blocks
                compacted.blocks.insert(pointer, ContiguousFileBlocks {
                    file_id: None,
                    range: (empty_block.range.start + replacement_length)..empty_block.range.end,
                });
                compacted.blocks.insert(pointer, ContiguousFileBlocks {
                    file_id: replacement_block.file_id,
                    range: empty_block.range.start..(empty_block.range.start + replacement_length),
                });
            } else {
                compacted.blocks.insert(pointer, ContiguousFileBlocks {
                    file_id: replacement_block.file_id,
                    range: empty_block.range,
                });
                if replacement_length > empty_length {
                    compacted.blocks.push(ContiguousFileBlocks {
                        file_id: replacement_block.file_id,
                        range: replacement_block.range.start..(replacement_block.range.end - empty_length),
                    })
                }
            }
            pointer += 1
        }
        compacted
    }

    fn checksum(&self) -> u64 {
        self.blocks.iter()
            .map(|block| {
                let positions_sum = (block.range.start + block.range.end - 1) as u64 * block.range.len() as u64 / 2;
                positions_sum * block.file_id.unwrap_or(0)
            })
            .sum()
    }

    fn compact_v2(&self) -> FileSystem {
        let mut compacted = self.clone();

        let max_file_id = compacted.blocks.iter()
            .filter_map(|block| block.file_id)
            .last()
            .expect("There should be a file");

        for file_id in (0..(max_file_id + 1)).rev() {
            let (initial_position, block) = compacted.blocks.iter()
                .find_position(|block| block.file_id == Some(file_id))
                .expect("Couldn't find file");
            let block_length = block.range.len();
            let first_empty_space = compacted.blocks.iter()
                .take(initial_position)
                .find_position(|space| {
                    space.file_id.is_none()
                        && space.range.len() >= block_length
                });

            if let Some((insert_position, _)) = first_empty_space {
                // Grab block being moved and replace with empty block
                // Technically we should merge this with surrounding empty blocks, but I don't think it
                // matters in our case, as we're only trying to move files to the left
                let block = compacted.blocks.remove(initial_position);
                compacted.blocks.insert(initial_position, ContiguousFileBlocks {
                    file_id: None,
                    range: block.range,
                });

                // Replace empty block with the block being moved, plus padding for any extra unused space
                let empty_block = compacted.blocks.remove(insert_position);
                if empty_block.range.len() > block_length {
                    // Padding is needed - either merge with the next block if it's empty, or insert a new empty block
                    if compacted.blocks[insert_position].file_id.is_none() {
                        compacted.blocks[insert_position].range.start = empty_block.range.start + block_length;
                    } else {
                        compacted.blocks.insert(insert_position, ContiguousFileBlocks {
                            file_id: None,
                            range: (empty_block.range.start + block_length)..empty_block.range.end,
                        });
                    }
                }
                compacted.blocks.insert(insert_position, ContiguousFileBlocks {
                    file_id: block.file_id,
                    range: empty_block.range.start..(empty_block.range.start + block_length),
                });
            }
        }
        compacted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let input_file = "inputs/day-9-example.txt";
/*
00...111...2...333.44.5555.6666.777.888899
*/
        let expected_filesystem = FileSystem {
            blocks: vec![
                ContiguousFileBlocks { file_id: Some(0), range: 0..2 },
                ContiguousFileBlocks { file_id: None, range: 2..5 },
                ContiguousFileBlocks { file_id: Some(1), range: 5..8 },
                ContiguousFileBlocks { file_id: None, range: 8..11 },
                ContiguousFileBlocks { file_id: Some(2), range: 11..12 },
                ContiguousFileBlocks { file_id: None, range: 12..15 },
                ContiguousFileBlocks { file_id: Some(3), range: 15..18 },
                ContiguousFileBlocks { file_id: None, range: 18..19 },
                ContiguousFileBlocks { file_id: Some(4), range: 19..21 },
                ContiguousFileBlocks { file_id: None, range: 21..22 },
                ContiguousFileBlocks { file_id: Some(5), range: 22..26 },
                ContiguousFileBlocks { file_id: None, range: 26..27 },
                ContiguousFileBlocks { file_id: Some(6), range: 27..31 },
                ContiguousFileBlocks { file_id: None, range: 31..32 },
                ContiguousFileBlocks { file_id: Some(7), range: 32..35 },
                ContiguousFileBlocks { file_id: None, range: 35..36 },
                ContiguousFileBlocks { file_id: Some(8), range: 36..40 },
                ContiguousFileBlocks { file_id: Some(9), range: 40..42 },
            ]
        };
        assert_eq!(parse_file(input_file), expected_filesystem);
    }

    #[test]
    fn test_compact() {
        let input_file = "inputs/day-9-example.txt";
        let filesystem = parse_file(input_file);
        let expected_filesystem = FileSystem {
            blocks: vec![
                ContiguousFileBlocks { file_id: Some(0), range: 0..2 },
                ContiguousFileBlocks { file_id: Some(9), range: 2..4 },
                ContiguousFileBlocks { file_id: Some(8), range: 4..5 },
                ContiguousFileBlocks { file_id: Some(1), range: 5..8 },
                ContiguousFileBlocks { file_id: Some(8), range: 8..11 },
                ContiguousFileBlocks { file_id: Some(2), range: 11..12 },
                ContiguousFileBlocks { file_id: Some(7), range: 12..15 },
                ContiguousFileBlocks { file_id: Some(3), range: 15..18 },
                ContiguousFileBlocks { file_id: Some(6), range: 18..19 },
                ContiguousFileBlocks { file_id: Some(4), range: 19..21 },
                ContiguousFileBlocks { file_id: Some(6), range: 21..22 },
                ContiguousFileBlocks { file_id: Some(5), range: 22..26 },
                ContiguousFileBlocks { file_id: Some(6), range: 26..27 },
                ContiguousFileBlocks { file_id: Some(6), range: 27..28 },
            ]
        };
        assert_eq!(filesystem.compact(), expected_filesystem);
    }

    #[test]
    fn test_checksum() {
        let input_file = "inputs/day-9-example.txt";
        let filesystem = parse_file(input_file);
        let compacted_filesystem = filesystem.compact();
        assert_eq!(compacted_filesystem.checksum(), 1928);
    }

    #[test]
    fn test_compact_v2() {
        let input_file = "inputs/day-9-example.txt";
        let filesystem = parse_file(input_file);
        let expected_filesystem = FileSystem {
            blocks: vec![
                ContiguousFileBlocks { file_id: Some(0), range: 0..2 },
                ContiguousFileBlocks { file_id: Some(9), range: 2..4 },
                ContiguousFileBlocks { file_id: Some(2), range: 4..5 },
                ContiguousFileBlocks { file_id: Some(1), range: 5..8 },
                ContiguousFileBlocks { file_id: Some(7), range: 8..11 },
                ContiguousFileBlocks { file_id: None, range: 11..12 },
                ContiguousFileBlocks { file_id: Some(4), range: 12..14 },
                ContiguousFileBlocks { file_id: None, range: 14..15 },
                ContiguousFileBlocks { file_id: Some(3), range: 15..18 },
                ContiguousFileBlocks { file_id: None, range: 18..19 },
                ContiguousFileBlocks { file_id: None, range: 19..21 },
                ContiguousFileBlocks { file_id: None, range: 21..22 },
                ContiguousFileBlocks { file_id: Some(5), range: 22..26 },
                ContiguousFileBlocks { file_id: None, range: 26..27 },
                ContiguousFileBlocks { file_id: Some(6), range: 27..31 },
                ContiguousFileBlocks { file_id: None, range: 31..32 },
                ContiguousFileBlocks { file_id: None, range: 32..35 },
                ContiguousFileBlocks { file_id: None, range: 35..36 },
                ContiguousFileBlocks { file_id: Some(8), range: 36..40 },
                ContiguousFileBlocks { file_id: None, range: 40..42 },
            ]
        };
        assert_eq!(filesystem.compact_v2(), expected_filesystem);
    }

    #[test]
    fn test_checksum_v2() {
        let input_file = "inputs/day-9-example.txt";
        let filesystem = parse_file(input_file);
        let compacted_filesystem = filesystem.compact_v2();
        assert_eq!(compacted_filesystem.checksum(), 2858);
    }
}