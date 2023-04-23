fn walk (maze: & mut Vec<Vec<char>>, x: usize, y:usize , path: &mut Vec<(usize, usize)>)  -> bool{

    // use std::{thread, time};
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // return to row 1, colum 1
    // for y in 0..maze.len(){
    //     for x in 0..maze[y].len(){
    //         print!("{}", maze[y][x]);
    //     }
    //     println!("");
    // }
    // thread::sleep(time::Duration::from_millis(500));

    if x < 0  || x >= maze[0].len(){
        return false;
    }

    if y < 0 || y >= maze.len(){
        return false;
    }

    if maze[y][x] == '*' {
        return false;
    }

    if maze[y][x] == '.' {
        return false;
    }

    if maze[y][x] == 'E' {
        path.push((x,y));
        return true;
    }

    maze[y][x] = '.';

    if walk(maze, x, y-1, path) {//up
        path.push((x,y));
        maze[y][x] = ',';
        return true;
    }

    if walk(maze, x+1, y, path) {//right
        path.push((x,y));
        maze[y][x] = ',';
        return true;
    }

    if walk(maze, x, y+1, path) {//down
        path.push((x,y));
        maze[y][x] = ',';
        return true;
    }

    if walk(maze, x-1, y, path) {//left
        path.push((x,y));
        maze[y][x] = ',';
        return true;
    }

    false
}

pub fn solve (maze: &mut Vec<Vec<char>> )-> Vec<(usize, usize)>{
    let mut start = (0,0);
    let mut end = (0,0);

    //find start
    for  y in 0..maze.len(){
        for x in 0..maze[y].len(){
            if maze[y][x] == 'S'{
                start = (x,y);
                break;
            }
        }
    }

    //find end
    for  y in 0..maze.len(){
        for x in 0..maze[y].len(){
            if maze[y][x] == 'E'{
                end = (x,y);
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

#[test]
fn maze_resolver1(){
    let maze= [
        "********E*",
        "*        *",
        "*S********"
    ];

    let mut m = Vec::new();
    for  i in 0..maze.len(){
        let mut v = Vec::new();
        for ch in maze[i].chars().collect::<Vec<char>>() {
            v.push( ch);
        }
        m.push(v);
    }

    for y in 0..m.len(){
        for x in 0..m[y].len(){
            print!("{}", m[y][x]);
        }
        println!("");
    }

    let path = solve(&mut m);
    println!("{:?}", path);
    for y in 0..m.len(){
        for x in 0..m[y].len(){
            print!("{}", m[y][x]);
        }
        println!("");
    }

}

#[test]
fn maze_resolver2(){
    let maze= [
        "********E*",
        "*        *",
        "* ********",
        "* *      *",
        "* *      *",
        "*S********"
    ];

    let mut m = Vec::new();
    for  i in 0..maze.len(){
        let mut v = Vec::new();
        for ch in maze[i].chars().collect::<Vec<char>>() {
            v.push( ch);
        }
        m.push(v);
    }

    for y in 0..m.len(){
        for x in 0..m[y].len(){
            print!("{}", m[y][x]);
        }
        println!("");
    }

    let path = solve(&mut m);
    println!("{:?}", path);
    for y in 0..m.len(){
        for x in 0..m[y].len(){
            print!("{}", m[y][x]);
        }
        println!("");
    }

}

#[test]
fn maze_resolver3(){
    let maze= [
        "**********",
        "*   ******",
        "* * ******",
        "* * ******",
        "* *      E",
        "*S********"
    ];

    let mut m = Vec::new();
    for  i in 0..maze.len(){
        let mut v = Vec::new();
        for ch in maze[i].chars().collect::<Vec<char>>() {
            v.push( ch);
        }
        m.push(v);
    }

    for y in 0..m.len(){
        for x in 0..m[y].len(){
            print!("{}", m[y][x]);
        }
        println!("");
    }

    let path = solve(&mut m);
    println!("{:?}", path);
    for y in 0..m.len(){
        for x in 0..m[y].len(){
            print!("{}", m[y][x]);
        }
        println!("");
    }

}

#[test]
fn maze_resolver4(){
    let maze= [
        "**********",
        "*        *",
        "* * *** **",
        "* * ******",
        "* *      E",
        "*S********"
    ];

    let mut m = Vec::new();
    for  i in 0..maze.len(){
        let mut v = Vec::new();
        for ch in maze[i].chars().collect::<Vec<char>>() {
            v.push( ch);
        }
        m.push(v);
    }

    for y in 0..m.len(){
        for x in 0..m[y].len(){
            print!("{}", m[y][x]);
        }
        println!("");
    }

    let path = solve(&mut m);
    println!("{:?}", path);

    for y in 0..m.len(){
        for x in 0..m[y].len(){
            print!("{}", m[y][x]);
        }
        println!("");
    }

}

#[test]
fn maze_resolver5(){
    let maze= [
        "*******************",
        "*          *      *",
        "* * ***  *** *** **",
        "* * ***      *    *",
        "* * ** *******  ***",
        "*    *            *",
        "*S********E********"
    ];

    let mut m = Vec::new();
    for  i in 0..maze.len(){
        let mut v = Vec::new();
        for ch in maze[i].chars().collect::<Vec<char>>() {
            v.push( ch);
        }
        m.push(v);
    }

    for y in 0..m.len(){
        for x in 0..m[y].len(){
            print!("{}", m[y][x]);
        }
        println!("");
    }

    let path = solve(&mut m);
    println!("{:?}", path);

    for y in 0..m.len(){
        for x in 0..m[y].len(){
            print!("{}", m[y][x]);
        }
        println!("");
    }

}
#[test]
fn maze_resolver6(){
    let maze= [
        "*******************",
        "*          *      *",
        "* * ***  *** *** **",
        "* * ***      *    *",
        "* * ** *******  ***",
        "E    *            *",
        "*S*****************"
    ];

    let mut m = Vec::new();
    for  i in 0..maze.len(){
        let mut v = Vec::new();
        for ch in maze[i].chars().collect::<Vec<char>>() {
            v.push( ch);
        }
        m.push(v);
    }

    for y in 0..m.len(){
        for x in 0..m[y].len(){
            print!("{}", m[y][x]);
        }
        println!("");
    }

    let path = solve(&mut m);
    println!("{:?}", path);

    for y in 0..m.len(){
        for x in 0..m[y].len(){
            print!("{}", m[y][x]);
        }
        println!("");
    }
}
