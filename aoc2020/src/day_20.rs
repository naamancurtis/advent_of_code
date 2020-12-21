use std::collections::{HashMap, HashSet};

#[aoc_generator(day20)]
pub fn generator(input: &str) -> HashMap<usize, Photo> {
    let mut map = HashMap::new();
    let mut id = 0;
    let mut photo = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            let p = Photo::new(photo);
            map.insert(id, p);
            id = 0;
            photo = Vec::new();
            continue;
        }
        if id == 0 {
            id = line[5..9].parse::<usize>().unwrap();
        } else {
            photo.push(line.chars().collect());
        }
    }
    map
}

#[aoc(day20, part1)]
pub fn puzzle_1(input: &HashMap<usize, Photo>) -> usize {
    let edge_map = generate_edge_map(input);
    edge_map
        .values()
        .fold(HashMap::new(), |mut acc, v| {
            // If only 1 of these edges exist
            if v.len() == 1 {
                for key in v {
                    let entry = acc.entry(key).or_insert(0);
                    *entry += 1;
                }
            }
            acc
        })
        .iter()
        .filter_map(|(k, v)| {
            // As we add each edge twice, if everything is double counted
            // so if there are 2 edges (count == 4) which haven't been matched then it's
            // a corner
            if *v == 4 {
                return Some(*k);
            }
            None
        })
        .product()
}

#[aoc(day20, part2)]
pub fn puzzle_2(input: &HashMap<usize, Photo>) -> usize {
    let edge_map = generate_edge_map(input);
    let dimensions = (input.len() as f64).sqrt() as usize;
    let mut photo_keys = vec![vec![0; dimensions]; dimensions];
    let mut photo = vec![vec![Photo::default(); dimensions]; dimensions];
    let mut corners: Vec<usize> = edge_map
        .values()
        .fold(HashMap::new(), |mut acc, v| {
            if v.len() == 1 {
                for key in v {
                    let entry = acc.entry(key).or_insert(0);
                    *entry += 1;
                }
            }
            acc
        })
        .iter()
        .filter_map(|(k, v)| {
            if *v == 4 {
                return Some(**k);
            }
            None
        })
        .collect();
    let starter = corners.pop().unwrap();
    let mut starter_photo = input.get(&starter).unwrap().clone();
    let mut counter = 0;
    while edge_map.get(&starter_photo.left()).unwrap().len() != 1
        && edge_map.get(&starter_photo.top()).unwrap().len() != 1
    {
        if counter == 4 {
            starter_photo.flip_horizontal();
        } else if counter == 8 {
            starter_photo.flip_horizontal();
            starter_photo.flip_vertical();
        } else if counter >= 12 {
            eprintln!("Got some error");
            break;
        }
        starter_photo.rotate_90_clockwise();
        counter += 1;
    }
    photo_keys[0][0] = starter;
    photo[0][0] = starter_photo;

    for i in 0..dimensions {
        for j in 0..dimensions {
            // Skip starter
            if i == j && i == 0 {
                continue;
            }

            let edge_to_match = photo[i][j - 1].right();
            let new_photo_key = edge_map
                .get(&edge_to_match)
                .unwrap()
                .clone()
                .into_iter()
                .find(|k| !photo_keys.iter().flatten().any(|key| key == k)) // We haven't already added the tile
                .unwrap();
            let mut new_photo = input.get(&new_photo_key).unwrap().clone();
            let mut counter = 0;
            while new_photo.left() != edge_to_match {
                if counter == 4 {
                    new_photo.flip_horizontal();
                } else if counter == 8 {
                    new_photo.flip_vertical();
                } else if counter == 12 {
                    new_photo.flip_horizontal();
                } else if counter >= 16 {
                    eprintln!("Got some error");
                    break;
                }
                new_photo.rotate_90_clockwise();
                counter += 1;
            }

            photo_keys[i][j] = new_photo_key;
            photo[i][j] = new_photo;
        }
        println!("{:?}", photo_keys[0]);
        println!("{:?}", photo[0][0].data);
        println!("{:?}", photo[0][1].data);
        println!("{:?}", photo[0][2].data);
    }
    println!("{:?}", photo);
    todo!()
}

fn generate_edge_map(input: &HashMap<usize, Photo>) -> HashMap<Vec<char>, HashSet<usize>> {
    let mut edge_map: HashMap<Vec<char>, HashSet<usize>> = HashMap::new();
    for (k, v) in input {
        for edge in v.edges() {
            let reversed_edge = edge.iter().cloned().rev().collect();
            let entry = edge_map.entry(edge).or_default();
            entry.insert(*k);
            let entry = edge_map.entry(reversed_edge).or_default();
            entry.insert(*k);
        }
    }
    edge_map
}

#[derive(Debug, Clone, Hash, PartialEq, Default)]
pub struct Photo {
    pub data: Vec<Vec<char>>,
}

impl Photo {
    pub fn new(data: Vec<Vec<char>>) -> Self {
        Self { data }
    }

    pub fn top(&self) -> Vec<char> {
        self.data[0].clone()
    }

    pub fn bottom(&self) -> Vec<char> {
        self.data[self.data.len() - 1].clone()
    }

    pub fn left(&self) -> Vec<char> {
        self.data
            .iter()
            .filter_map(|l| l.first().copied())
            .collect()
    }

    pub fn right(&self) -> Vec<char> {
        self.data.iter().filter_map(|l| l.last().copied()).collect()
    }

    pub fn edges(&self) -> Vec<Vec<char>> {
        vec![self.top(), self.bottom(), self.left(), self.right()]
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn rotate_90_clockwise(&mut self) {
        let len = self.len();
        for i in 0..len / 2 {
            for j in i..len - i - 1 {
                let temp = self.data[i][j];
                self.data[i][j] = self.data[j][len - 1 - j];
                self.data[j][len - 1 - i] = self.data[len - 1 - i][len - 1 - j];
                self.data[len - 1 - i][len - 1 - j] = self.data[len - 1 - j][i];
                self.data[len - 1 - j][i] = temp
            }
        }
    }

    pub fn flip_horizontal(&mut self) {
        let mut data = self.data.to_owned();
        data = data
            .into_iter()
            .map(|row| row.into_iter().rev().collect())
            .collect();
        self.data = data
    }

    pub fn flip_vertical(&mut self) {
        let mut data = self.data.to_owned();
        data = data.into_iter().rev().collect();
        self.data = data
    }

    pub fn strip_border(&mut self) {
        let len = self.data.len();
        let mut data = self.data.to_owned();
        // top
        data.remove(0);
        // bottom
        data.pop();
        self.data = data
            .into_iter()
            .map(|row| row.into_iter().skip(1).take(len - 2).collect())
            .collect();
    }

    pub fn consume(self) -> Vec<Vec<char>> {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_input_from_file;

    const TEST_INPUT: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...

";

    #[test]
    fn puzzle_1_test() {
        let input = generator(TEST_INPUT);
        assert_eq!(puzzle_1(&input), 20899048083289);
    }

    #[test]
    fn puzzle_1_sol() {
        let input = get_input_from_file("input/2020/day20.txt");
        let input = generator(&input);
        assert_eq!(puzzle_1(&input), 29125888761511);
    }

    // #[test]
    // fn puzzle_2_test() {
    //     let input = generator(TEST_INPUT);
    //     assert_eq!(puzzle_2(&input), 12);
    // }

    // #[test]
    // fn puzzle_2_sol() {
    //     let input = get_input_from_file("input/2020/day20.txt");
    //     let input = generator(&input);
    //     assert_eq!(puzzle_2(&input), 357);
    // }
}
