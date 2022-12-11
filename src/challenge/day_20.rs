use anyhow::Context;

const SET_PIXEL: u8 = b'#';

const TILE_SIZE: usize = 10;
const FRAME_SIZE: usize = 8;

const MONSTER_LENGTH: usize = 19;
const MONSTER_PIXEL_COUNT: usize = 15;
const MONSTER_TOP: [usize; 1] = [18];
const MONSTER_MIDDLE: [usize; 8] = [0, 5, 6, 11, 12, 17, 18, 19];
const MONSTER_BOTTOM: [usize; 6] = [1, 4, 7, 10, 13, 16];

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let tiles = parse_tiles(input);
    let mut result = 1;

    for (index, tile) in tiles.iter().enumerate() {
        let count = tile
            .edges
            .iter()
            .filter(|edge| find_match(index, edge.original, &tiles).is_some())
            .count();

        if count == 2 {
            result *= tile.id;
        }
    }

    Ok(result)
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    let image = build_image(input)?;
    let monsters = count_monsters(&image);

    let pixels = image
        .iter()
        .flatten()
        .filter(|pixel| **pixel == SET_PIXEL)
        .count();

    Ok(pixels - monsters * MONSTER_PIXEL_COUNT)
}

fn count_monsters(image: &Vec<Vec<u8>>) -> usize {
    let mut count = 0;
    let image_size = image.len();

    for y in 1..(image_size - 1) {
        for x in 0..(image_size - MONSTER_LENGTH) {
            if is_vertical_monster(x, y, image) {
                count += 1;
            }
        }
    }

    for y in 0..(image_size - MONSTER_LENGTH) {
        for x in 1..(image_size - 1) {
            if is_horizontal_monster(x, y, image) {
                count += 1;
            }
        }
    }

    count
}

fn is_horizontal_monster(x: usize, y: usize, image: &[Vec<u8>]) -> bool {
    // Identifies the monster by checking for the middle of the body
    // Then it does it checks if for the bottom of the body on both sides (in case it's flipped)
    // Once it know if it's filled, it checks for the top of the body

    // forward
    (MONSTER_MIDDLE
        .iter()
        .all(|&index| image[y + index][x] == SET_PIXEL)
        // original
        && ((MONSTER_BOTTOM
            .iter()
            .all(|&index| image[y + index][x + 1] == SET_PIXEL)
            && MONSTER_TOP
                .iter()
                .all(|&index| image[y + index][x - 1] == SET_PIXEL))
            // flipped
            || (MONSTER_BOTTOM
                .iter()
                .all(|&index| image[y + index][x - 1] == SET_PIXEL)
                && MONSTER_TOP
                    .iter()
                    .all(|&index| image[y + index][x + 1] == SET_PIXEL))))
        // backward
        || (MONSTER_MIDDLE
            .iter()
            .all(|&index| image[y + MONSTER_LENGTH - index - 1][x] == SET_PIXEL)
            // original
            && ((MONSTER_BOTTOM
                .iter()
                .all(|&index| image[y + MONSTER_LENGTH - index - 1][x + 1] == SET_PIXEL)
                && MONSTER_TOP
                    .iter()
                    .all(|&index| image[y + MONSTER_LENGTH - index - 1][x - 1] == SET_PIXEL))
                // flipped
                || (MONSTER_BOTTOM
                    .iter()
                    .all(|&index| image[y + MONSTER_LENGTH - index - 1][x - 1] == SET_PIXEL)
                    && MONSTER_TOP
                        .iter()
                        .all(|&index| image[y + MONSTER_LENGTH - index - 1][x + 1] == SET_PIXEL))))
}

fn is_vertical_monster(x: usize, y: usize, image: &[Vec<u8>]) -> bool {
    // Identifies the monster by checking for the middle of the body
    // Then it does it checks if for the bottom of the body on both sides (in case it's flipped)
    // Once it know if it's filled, it checks for the top of the body

    // forward
    (MONSTER_MIDDLE
        .iter()
        .all(|&index| image[y][x + index] == SET_PIXEL)
        // original
        && ((MONSTER_BOTTOM
            .iter()
            .all(|&index| image[y + 1][x + index] == SET_PIXEL)
            && MONSTER_TOP
                .iter()
                .all(|&index| image[y - 1][x + index] == SET_PIXEL))
            // flipped
            || (MONSTER_BOTTOM
                .iter()
                .all(|&index| image[y - 1][x + index] == SET_PIXEL)
                && MONSTER_TOP
                    .iter()
                    .all(|&index| image[y + 1][x + index] == SET_PIXEL))))
        // backward
        || (MONSTER_MIDDLE
            .iter()
            .all(|&index| image[y][x + MONSTER_LENGTH - index - 1] == SET_PIXEL)
            // original
            && ((MONSTER_BOTTOM
                .iter()
                .all(|&index| image[y + 1][x + MONSTER_LENGTH - index - 1] == SET_PIXEL)
                && MONSTER_TOP
                    .iter()
                    .all(|&index| image[y - 1][x + MONSTER_LENGTH - index - 1] == SET_PIXEL))
                // flipped
                || (MONSTER_BOTTOM
                    .iter()
                    .all(|&index| image[y - 1][x + MONSTER_LENGTH - index - 1] == SET_PIXEL)
                    && MONSTER_TOP
                        .iter()
                        .all(|&index| image[y + 1][x + MONSTER_LENGTH - index - 1] == SET_PIXEL))))
}

fn build_image(input: &[&str]) -> anyhow::Result<Vec<Vec<u8>>> {
    let tiles = parse_tiles(input);

    let pixel_count = (tiles.len() as f64).sqrt() as usize * FRAME_SIZE;
    let mut buffer = vec![Vec::with_capacity(pixel_count); pixel_count];

    for (row, (mut index, mut projection)) in find_left_tiles(&tiles)?.enumerate() {
        let buffer = &mut buffer[row * FRAME_SIZE..(row + 1) * FRAME_SIZE];
        projection.draw(buffer);

        while let Some(r#match) = find_match(index, projection.right_edge(), &tiles) {
            index = r#match.index;
            let rotation = (3 - r#match.position) % 4;
            projection = Projection::new(rotation, false, !r#match.flipped, &tiles[index]);
            projection.draw(buffer);
        }
    }

    Ok(buffer)
}

fn find_left_tiles<'a>(
    tiles: &'a [Tile],
) -> anyhow::Result<impl Iterator<Item = (usize, Projection<'a>)>> {
    let index = find_top_left_tile(tiles).context("Could not find top-left tile")?;
    let first = (index, Projection::new(0, false, false, &tiles[index]));

    let iterator = std::iter::successors(Some(first), move |(index, projection)| {
        find_match(*index, projection.bottom_edge(), tiles).map(|r#match| {
            let rotation = (4 - r#match.position) % 4;
            let tile = &tiles[r#match.index];
            let projection = Projection::new(rotation, !r#match.flipped, false, tile);
            (r#match.index, projection)
        })
    });

    Ok(iterator)
}

fn find_top_left_tile(tiles: &[Tile]) -> Option<usize> {
    tiles
        .iter()
        .enumerate()
        .find_map(|(index, tile)| match find_matches(index, tile, tiles) {
            [None, Some(_), Some(_), None] => Some(index),
            _ => None,
        })
}

fn find_matches(index: usize, tile: &Tile, tiles: &[Tile]) -> [Option<Match>; 4] {
    std::array::from_fn(|i| find_match(index, tile.edges[i].original, tiles))
}

fn find_match(origin: usize, edge: EdgeLine, tiles: &[Tile]) -> Option<Match> {
    tiles
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != origin)
        .find_map(|(index, tile)| {
            tile.edges.iter().enumerate().find_map(|(i, candidate)| {
                if edge == candidate.original {
                    Some(Match::new(index, i, false))
                } else if edge == candidate.flipped {
                    Some(Match::new(index, i, true))
                } else {
                    None
                }
            })
        })
}

fn parse_tiles<'a>(input: &[&'a str]) -> Vec<Tile<'a>> {
    let mut tiles = Vec::with_capacity((input.len() + 1) / (TILE_SIZE + 2));

    for lines in input.chunks(TILE_SIZE + 2) {
        tiles.push(Tile::new(lines));
    }

    tiles
}

type EdgeLine = u16;

struct Tile<'a> {
    id: usize,
    edges: [Edge; 4],
    frame: [&'a [u8]; FRAME_SIZE],
}

impl<'a> Tile<'a> {
    fn new(input: &[&'a str]) -> Self {
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

        let frame = std::array::from_fn(|i| &input[i + 2].as_bytes()[1..FRAME_SIZE + 1]);

        Tile { id, edges, frame }
    }
}

#[derive(Copy, Clone)]
struct Edge {
    original: EdgeLine,
    flipped: EdgeLine,
}

impl Edge {
    fn new(line: impl Iterator<Item = u8>) -> Self {
        let mut original = 0;
        let mut flipped = 0;

        for (i, pixel) in line.enumerate() {
            let mask = (pixel == SET_PIXEL) as EdgeLine;
            original |= mask << i;
            flipped |= mask << (TILE_SIZE - i - 1);
        }

        Edge { original, flipped }
    }
}

struct Match {
    index: usize,
    position: usize,
    flipped: bool,
}

impl Match {
    fn new(index: usize, position: usize, flipped: bool) -> Self {
        Match {
            index,
            position,
            flipped,
        }
    }
}

struct Projection<'a> {
    rotation: usize,
    flip_x: bool,
    flip_y: bool,
    tile: &'a Tile<'a>,
}

impl<'a> Projection<'a> {
    fn new(rotation: usize, flip_x: bool, flip_y: bool, tile: &'a Tile<'a>) -> Self {
        Projection {
            rotation,
            flip_x,
            flip_y,
            tile,
        }
    }

    fn right_edge(&self) -> EdgeLine {
        self.edge(1)
    }

    fn bottom_edge(&self) -> EdgeLine {
        self.edge(2)
    }

    fn edge(&self, index: usize) -> EdgeLine {
        let horizontal = index % 2 == 1;
        let swapped = horizontal && self.flip_x || !horizontal && self.flip_y;
        let flipped = horizontal && self.flip_y || !horizontal && self.flip_x;

        let mut index = 4 + index - self.rotation;

        if swapped {
            index += 2;
        }

        let edge = self.tile.edges[index % 4];

        if flipped ^ swapped {
            edge.flipped
        } else {
            edge.original
        }
    }

    fn draw(&self, buffer: &mut [Vec<u8>]) {
        let transmute = self.rotation % 2 == 1;
        let flip_x = self.flip_x ^ (self.rotation == 1 || self.rotation == 2);
        let flip_y = self.flip_y ^ (self.rotation == 2 || self.rotation == 3);

        let rows = buffer
            .iter_mut()
            .enumerate()
            .map(|(i, buffer)| (get_frame_index(i, flip_y), buffer));

        for (y, buffer) in rows {
            for x in (0..FRAME_SIZE).map(|i| get_frame_index(i, flip_x)) {
                if transmute {
                    buffer.push(self.tile.frame[x][y]);
                } else {
                    buffer.push(self.tile.frame[y][x]);
                }
            }
        }
    }
}

fn get_frame_index(index: usize, flipped: bool) -> usize {
    if flipped {
        FRAME_SIZE - index - 1
    } else {
        index
    }
}
