fn walk(maze: &mut Vec<Vec<char>>, x: i32, y: i32, path: &mut Vec<(i32, i32)>) -> bool {
    // use std::{thread, time};
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // return to row 1, colum 1
    // for y in 0..maze.len(){
    //     for x in 0..maze[y].len(){
    //         print!("{}", maze[y][x]);
    //     }
    //     println!("");
    // }
    // thread::sleep(time::Duration::from_millis(500));

    if x < 0 || x >= maze[0].len() as i32 {
        return false;
    }

    if y < 0 || y >= maze.len() as i32 {
        return false;
    }

    if maze[y as usize][x as usize] == '*' {
        return false;
    }

    if maze[y as usize][x as usize] == '.' {
        return false;
    }

    if maze[y as usize][x as usize] == 'E' {
        path.push((x, y));
        return true;
    }

    maze[y as usize][x as usize] = '.';

    if walk(maze, x, y - 1, path) {
        //up
        path.push((x, y));
        maze[y as usize][x as usize] = ',';
        return true;
    }

    if walk(maze, x + 1, y, path) {
        //right
        path.push((x, y));
        maze[y as usize][x as usize] = ',';
        return true;
    }

    if walk(maze, x, y + 1, path) {
        //down
        path.push((x, y));
        maze[y as usize][x as usize] = ',';
        return true;
    }

    if walk(maze, x - 1, y, path) {
        //left
        path.push((x, y));
        maze[y as usize][x as usize] = ',';
        return true;
    }

    false
}

pub fn solve(maze: &mut Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    //find start
    for (y, _) in maze.iter().enumerate() {
        for (x, _) in maze[y].iter().enumerate() {
            if maze[y][x] == 'S' {
                start = (x as i32, y as i32);
                break;
            }
        }
    }

    //find end
    for (y, _) in maze.iter().enumerate() {
        for (x, _) in maze[y].iter().enumerate() {
            if maze[y][x] == 'E' {
                end = (x as i32, y as i32);
                break;
            }
        }
    }

    println!("Start at {:?}", start);
    println!("End at {:?}", end);

    let mut solution = Vec::new();
    walk(maze, start.0, start.1, &mut solution);
    solution
}

#[cfg(test)]
mod test_maze{
    #[test]
    fn maze_resolver1() {
        let maze = [
            "********E*",
            "*        *",
            "*S********"];

        let mut m = Vec::new();
        for i in 0..maze.len() {
            let mut v = Vec::new();
            for ch in maze[i].chars().collect::<Vec<char>>() {
                v.push(ch);
            }
            m.push(v);
        }

        for y in 0..m.len() {
            for x in 0..m[y].len() {
                print!("{}", m[y][x]);
            }
            println!("");
        }

        let path = crate::maze::solve(&mut m);
        println!("{:?}", path);
        for y in 0..m.len() {
            for x in 0..m[y].len() {
                print!("{}", m[y][x]);
            }
            println!("");
        }
    }

    #[test]
    fn maze_resolver2() {
        let maze = [
            "********E*",
            "*        *",
            "* ********",
            "* *      *",
            "* *      *",
            "*S********",
        ];

        let mut m = Vec::new();
        for i in 0..maze.len() {
            let mut v = Vec::new();
            for ch in maze[i].chars().collect::<Vec<char>>() {
                v.push(ch);
            }
            m.push(v);
        }

        for y in 0..m.len() {
            for x in 0..m[y].len() {
                print!("{}", m[y][x]);
            }
            println!("");
        }

        let path = crate::maze::solve(&mut m);
        println!("{:?}", path);
        for y in 0..m.len() {
            for x in 0..m[y].len() {
                print!("{}", m[y][x]);
            }
            println!("");
        }
    }

    #[test]
    fn maze_resolver3() {
        let maze = [
            "**********",
            "*   ******",
            "* * ******",
            "* * ******",
            "* *      E",
            "*S********",
        ];

        let mut m = Vec::new();
        for i in 0..maze.len() {
            let mut v = Vec::new();
            for ch in maze[i].chars().collect::<Vec<char>>() {
                v.push(ch);
            }
            m.push(v);
        }

        for y in 0..m.len() {
            for x in 0..m[y].len() {
                print!("{}", m[y][x]);
            }
            println!("");
        }

        let path = crate::maze::solve(&mut m);
        println!("{:?}", path);
        for y in 0..m.len() {
            for x in 0..m[y].len() {
                print!("{}", m[y][x]);
            }
            println!("");
        }
    }

    #[test]
    fn maze_resolver4() {
        let maze = [
            "**********",
            "*        *",
            "* * *** **",
            "* * ******",
            "* *      E",
            "*S********",
        ];

        let mut m = Vec::new();
        for i in 0..maze.len() {
            let mut v = Vec::new();
            for ch in maze[i].chars().collect::<Vec<char>>() {
                v.push(ch);
            }
            m.push(v);
        }

        for y in 0..m.len() {
            for x in 0..m[y].len() {
                print!("{}", m[y][x]);
            }
            println!("");
        }

        let path = crate::maze::solve(&mut m);
        println!("{:?}", path);

        for y in 0..m.len() {
            for x in 0..m[y].len() {
                print!("{}", m[y][x]);
            }
            println!("");
        }
    }

    #[test]
    fn maze_resolver5() {
        let maze = [
            "*******************",
            "*          *      S",
            "* * ***  *** *** **",
            "* * ***      *    *",
            "* * ** *******  ***",
            "*    *            *",
            "**********E********",
        ];

        let mut m = Vec::new();
        for i in 0..maze.len() {
            let mut v = Vec::new();
            for ch in maze[i].chars().collect::<Vec<char>>() {
                v.push(ch);
            }
            m.push(v);
        }

        for y in 0..m.len() {
            for x in 0..m[y].len() {
                print!("{}", m[y][x]);
            }
            println!("");
        }

        let path = crate::maze::solve(&mut m);
        println!("{:?}", path);

        for y in 0..m.len() {
            for x in 0..m[y].len() {
                print!("{}", m[y][x]);
            }
            println!("");
        }
    }
    #[test]
    fn maze_resolver6() {
        let maze = [
            "*************S*****",
            "*          *      *",
            "* * ***  *** *** **",
            "* * ***      *    *",
            "* * ** *******  ***",
            "E    *            *",
            "*******************",
        ];

        let mut m = Vec::new();
        for i in 0..maze.len() {
            let mut v = Vec::new();
            for ch in maze[i].chars().collect::<Vec<char>>() {
                v.push(ch);
            }
            m.push(v);
        }

        for y in 0..m.len() {
            for x in 0..m[y].len() {
                print!("{}", m[y][x]);
            }
            println!("");
        }

        let path = crate::maze::solve(&mut m);
        println!("{:?}", path);

        for y in 0..m.len() {
            for x in 0..m[y].len() {
                print!("{}", m[y][x]);
            }
            println!("");
        }
    }
}
