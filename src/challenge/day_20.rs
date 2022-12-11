const TILE_SIZE: usize = 10;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let tiles = parse_tiles(input);
    let mut result = 1;

    for (i, tile) in tiles.iter().enumerate() {
        let count = tile
            .edges
            .iter()
            .filter(|edge| {
                tiles
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| i != *j)
                    .flat_map(|(_, tile)| &tile.edges)
                    .any(|candidate| {
                        edge.original == candidate.original || edge.flipped == candidate.original
                    })
            })
            .count();

        if count == 2 {
            result *= tile.id;
        }
    }

    Ok(result)
}

fn parse_tiles(input: &[&str]) -> Vec<Tile> {
    let mut tiles = Vec::<Tile>::with_capacity((input.len() + 1) / (TILE_SIZE + 2));

    for lines in input.chunks(TILE_SIZE + 2) {
        tiles.push(Tile::new(lines));
    }

    tiles
}

struct Tile {
    id: usize,
    edges: [Edge; 4],
}

impl Tile {
    fn new(input: &[&str]) -> Self {
        let id = input[0][5..9].parse().unwrap();
        let lines = &input[1..=TILE_SIZE];

        let top = lines[0].bytes();
        let bottom = input[TILE_SIZE].bytes().rev();
        let left = lines.iter().rev().map(|line| line.as_bytes()[0]);
        let right = lines.iter().map(|line| *line.as_bytes().last().unwrap());

        let edges = [
            Edge::new(top),
            Edge::new(right),
            Edge::new(bottom),
            Edge::new(left),
        ];

        Tile { id, edges }
    }
}

struct Edge {
    original: u16,
    flipped: u16,
}

impl Edge {
    fn new(line: impl Iterator<Item = u8>) -> Self {
        let mut original = 0;
        let mut flipped = 0;

        for (i, pixel) in line.enumerate() {
            let mask = (pixel == b'#') as u16;

            original |= mask << i;
            flipped |= mask << (TILE_SIZE - i - 1);
        }

        Edge { original, flipped }
    }
}
