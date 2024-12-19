enum MovingDirection {
    Up,
    Right,
    Down,
    Left,
}

pub fn problem_06_v1() {
    let mut map = load_map(PROBLEM6_V1_INPUT);

    iterate_map(&mut map);

    let count = count_mapped(&map);
    println!("{count}"); // 4722
}

pub fn print_map(map: &Vec<Vec<char>>) {
    let field = map
        .into_iter()
        .map(|a| a.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");

    println!("{field}");
    println!("\n");
}

pub fn iterate_map(map: &mut Vec<Vec<char>>) {
    let mut dir = MovingDirection::Up;
    let max_x = map[0].len() as i32;
    let max_y = map.len() as i32;
    let (mut x, mut y) = staring_position(&map);

    loop {
        // print_map(&map);
        let mut next_x = x;
        let mut next_y = y;

        match dir {
            MovingDirection::Up => next_y -= 1,
            MovingDirection::Right => next_x += 1,
            MovingDirection::Down => next_y += 1,
            MovingDirection::Left => next_x -= 1,
        }

        if next_x < 0 || next_y < 0 || next_x >= max_x || next_y >= max_y {
            map[y as usize][x as usize] = 'X';
            return;
        }

        let next_char = map[next_y as usize][next_x as usize];

        let rotate = match next_char {
            '#' => {
                dir = match dir {
                    MovingDirection::Up => MovingDirection::Right,
                    MovingDirection::Right => MovingDirection::Down,
                    MovingDirection::Down => MovingDirection::Left,
                    MovingDirection::Left => MovingDirection::Up,
                };
                true
            }
            _ => false,
        };

        if rotate {
            continue;
        }

        map[y as usize][x as usize] = 'X';
        x = next_x;
        y = next_y;
    }
}

pub fn load_map(str: &str) -> Vec<Vec<char>> {
    let mut vec = Vec::new();
    for line in str.split("\n") {
        let mut chars = Vec::new();
        for item in line.chars() {
            chars.push(item);
        }
        vec.push(chars);
    }

    vec
}

pub fn staring_position(map: &Vec<Vec<char>>) -> (i32, i32) {
    let mut y = 0;
    for row in map {
        let mut x = 0;
        for el in row {
            if *el == '^' {
                return (x, y);
            }
            x += 1;
        }
        y += 1;
    }

    panic!("start not found!")
}

pub fn count_mapped(map: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in map {
        for j in i {
            match j {
                'X' => count += 1,
                '^' => count += 1,
                _ => {}
            }
        }
    }

    count
}

const PROBLEM6_V1_INPUT: &str = "..#.....................#............#....#...........#...............................#.....#.....................................
.................#............................#.............##..#...............................#....#..#.........#...............
.............................................##.........................................#....................#...............#...#
...##....................#...............#.....#....................#......##..............#............................#....#....
.................................#..#..............#...........#.............#..#........................#............#...........
.................#.........................#...........................#........................#........................#........
...#....................................#....#.........#................#...............#.....#................#.....#............
.............#.#..........................................................#...#..................#....#.......#.....#...#.........
.....#........................#....##...#............................................................#............................
..................#...#.........................................#..................#.............................#.#........#.....
......#...............................#.#..........##..#...........#.........................#.......#....................#.......
.................................#..............#...............#........#..........#.........#............#.........#.......#...#
..#..#............#...#.....................#...............#............#.#........................#........#...............#....
..............#..#.#........................#.........#...........##.......#..........#.....................#..............#...#..
..........................#.................##...................................#....#.........#.................................
..................#..#........#....#...............#..............#.......#.......#......................................#..#...#.
..........#.#.................#.........#...................#.......................##......#....................#.......#........
..........#.........#...#......................................................................#.....#............#........#......
...##.............#...........................................................#..#....................##.#...#.#..#.........#..#..
.....#......#.............................#.......#...#...#...............#.....#............#........#..#.#......................
........#................#.........#.......................................................................#..........#...........
.......#........................................#..........#...............................#..#...................................
..................##.......#.......#.....#............#.......#...#.........#....##...............#..#.............#..........#...
.......#...........#...........#...........................#.............#.........#..#........................#.##...............
.....................................................................#........#....#...........................................#..
....#...........#......#.........#....................................#.................................#......#..................
...#........................................#..#...............................#.....................#.#......#...................
..#.....#.................................#.................#....#............................................#......#............
...................................................##...........#.......#.............#.................#.......#.................
..........................................................##..............#....#....#.............#...............................
........................................................#.....................................#................................#..
..............##....#..#..........#...........#......#..#....#.......#................#............................#..#...#.......
..........................#....................#.................................................#................................
#................................................#.....................#..#...............##........#...#...#.........#...........
...........#.................................#..#....................#.#.......#..........................#.......#.......#.......
..............................#......#......#.................#.................#.................#.#.............#..#.#..........
.......................#....................#....................................#.....#.#.......#............................#...
...............................................................................................#.#................................
#..................#.....#...............................#..............#...#.....................................................
..........................................................#.................................................#.....................
..........#.........................#.....................................................#.........#..........#................#.
........#..............#..#......................................#.........#............#.................................#.......
.......#.........................................................#.........#................#...............................#.....
...........#............................................#...#.............................#.....^................................#
................#.............#...#..............................#..................................#.............................
.........................................................................................................#........................
.................................................#........#.......................#.#.....#......................#................
.........#.........#...#....................................................#.....................................................
##.##..#...................................#......................................................................................
.................#...........................................................................................#........#...........
.........................................................................#........................#....#......................#...
..............#......................#........#..........................#.........#..............................................
.......................#............................................................................##............................
.....#.................#........#...#...............#.............................#......................#........................
....#...................................................................................#............##...........................
................#............................#............................................#.......................................
.......................................................................................................#......#...................
......................................................................................#...#.......................................
...............................................#........#..........#............................#..............#.......#..........
..............#.....................#........#..#............................#...#..#.#...................................#.......
..........#...............................#............................#...........................................#..............
....#............#.#....#......................................................................#..................................
.......................#.......................#................................#..............#.....................#.........#..
...........#.........................#.............#.#........#........#..........................................#...............
.......#......................#....................#...............................#......................#...........#...........
............#.....................................................................................................................
....#.......#............................................................#.........#.............#..........#....#..........#.....
.......#.........................................................................#..................#..#....#.......#.............
............................#....#.......#......#.......#.....................#.................#.................................
.......................................................#.............#..........................#.................................
....................................#.....................................##......................#............................#..
..............#...............#....................#.......................#................#......................#..............
....................#...........................#...................................................................#.............
...#..........................#...........................#............................................................#..........
..............#.................#.............................#..#.......#.......................#.##.............................
....................#.....#...#..............................................#......#...........#.................#..#....#.......
#......................................#.....................#..#..#.#...........#.............#.................................#
.....................#...........................................................#..........#............##..........#............
.......#..................................#...............#..........#....#...................................................#...
.............#.................#...........................#..............#.................................................#.....
...............................#......................................##...................................#.#......#..........#..
.................##....................#.................................................#......................#................#
........#....................................#..........#.........................#..................#......................#.....
...#....#...........................#..............................#.#..........................#.................................
.#...................................#..#......#.................................................................#................
.................................#...........#................#.#........................#....................................##..
#..#.....##...............#................................................................#....#......#.........................#
.#....#......................................................#.........................#.......................................#..
.#..............................................#.................................................................#...............
...............#..........................................#...#...............................#.#..#.#........#...................
..............#................................#.......#..................................#.........................#........#....
.......................##..................................................#......................................................
..............................#...............................##..................................................................
#....................#...........................................................#.............................#..................
#..................................................................#...#..........................................................
#..#........#...................................#..#..........##..............#...................................................
.......................#.....................#....................................................................................
.....................#...........#.............................................................#........................#....#....
...............#............................................................................##...#..........................#.....
...#......................................#.......................................................................................
....#...........................#.........#.................................................#..........#...#........#....#........
......#....#.#............................#..........................................#........................................#...
...........#...#.............................................#......................#..................#.....................#....
...#..........#........#..........#.......#..............#.............................#.......................#................#.
...........#...............#...........................................................#..........................................
..#..........##.................................................#...#...............#.......................#...............##....
.......................#..............#.............#......................................................#..................#...
..............#..#........##..........#...................................................#.............##....................#...
...................#..................................................#................#......#.....#.........#.#...#.............
..................#.................................................#.................................................#.........#.
......................................#.#........#.....#...#..........#..............#.........................#...#..............
.................................#..............#..#.....................#................#.............#..#..#...................
.........#.......#.#..............#..#...............#.................#..................................#..#....................
.............................#....#.......##..................................................................................##..
.....................................................................................#........#.......#......#....................
........#...........#..........#..........#............................#...............#........#.#...........................#...
.#.................#...#..............#...#...................#........#...............#..........................................
.........#............................................##........#......#.....................#.......................#............
.......................................#..........#..#......#........#..........#..........................#....................#.
....#......#..........................#....................#.......................................#.....###.......#..............
.......#..................#....#.......................................................................#..........................
....#........................#.............................................................#.......#..........................#...
......................#.................................#..........#................#..#.........................#................
...........#....................................................#..............#...........................#......................
.....#................#........#...#....#........#............#...............#.......................#..#.........#..............
#..................................................#...........................#....#...........#.................................
....#.#.....................................#.#...............#................#....................................#.............
............#....................#.#.#...............................................#...................##.......................
...##..............#........................#.............#.......#...........#.#.##..............................#....##.........
......#...........#....................................................#.........................##...............................";
