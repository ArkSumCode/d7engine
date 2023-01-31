/**
Implement Maze for an Object,
so you can use the A star pathfinding algorithm.
*/
pub trait Maze {
    /// the number of tiles in x direction
    fn width(&self) -> usize;
    /// the number of tiles in y direction
    fn height(&self) -> usize;
    /// is the tile x,y a walkable option for the pathfinding algorithm
    fn solid(&self, x: usize, y: usize) -> bool;
}

/**
An implememtation of the A star
algorithm.

Find a path from one location to another
*/
pub fn astar(maze: &dyn Maze, start: (usize, usize), end: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    // create the start and end node
    let start = Node::new(None, start, 0, 0, 0);
    let end = Node::new(None, end, 0, 0, 0);
    
    // create empty lists
    let mut open_list = vec![start];
    let mut closed_list = vec![];

    loop {
        if open_list.len() == 0 {
            // quit here no path was found
            return None;
        }

        // search for node with the lowest f value
        let current_node = {
            let mut node = Node::new(None, (0, 0), 0, 0, std::isize::MAX);
            let mut remove = 0;

            for (i, on) in open_list.iter().enumerate() {
                if on.f < node.f {
                    node = on.clone();
                    remove = i;
                }
            }

            // remove the node from the open list
            open_list.swap_remove(remove);
            node
        };

        // add the node to the closed_list
        closed_list.push(current_node.clone());

        if current_node == end {
            // the end is found
            // create the path as a vector
            let mut path = vec![];
            let mut current = current_node.clone();
            
            loop {
                path.push((current.pos.0, current.pos.1));

                match current.parent {
                    Some(parent) => current = *parent,
                    None => break,
                }
            }
            
            path.reverse();
            return Some(path);
        }

        // create the children
        // they are the adjacent tiles to 
        // the curren tile
        let mut children = vec![];
        for dir in [(-1,-1), (0,-1), (1,-1), (-1, 0), (1, 0), (-1,1), (0,1), (1,1)] {
            let x = current_node.pos.0 as isize + dir.0;
            let y = current_node.pos.1 as isize + dir.1;
            
            // check boundries
            if x < 0 || maze.width() <= x as usize {
                continue;
            }

            if y < 0 || maze.height() <= y as usize {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            // check if we can walk on the tile
            if maze.solid(x, y) {
                continue;
            } 

            // create the child
            let parent = Some(Box::new(current_node.clone()));
            let node = Node::new(parent, (x, y), 0, 0, 0);
            children.push(node);
        }
        
        'children: for mut child in children.into_iter() {
            // if we already were
            // on this tile continue
            for cn in closed_list.iter() {
                if *cn == child {
                    continue 'children;
                }
            }

            // calculate the g, h, and f values on the child
            child.g = current_node.g + 1;
            let x = (child.pos.0 as isize - end.pos.0 as isize).pow(2);
            let y = (child.pos.1 as isize - end.pos.1 as isize).pow(2);
            child.h = x + y;
            child.f = child.g + child.h;

            // if this child has a greater g than
            // its counterpart in the openlist, skipp
            for on in open_list.iter() {
                if *on == child && child.g > on.g {
                    continue 'children; 
                }
            }
            
            open_list.push(child);
        }
    }
}

#[derive(Clone)]
struct Node {
    parent: Option<Box<Node>>,
    pos: (usize, usize),
    g: isize,
    h: isize,
    f: isize,
}

impl Node {
    fn new(parent: Option<Box<Self>>, pos: (usize, usize), g: isize, h: isize, f: isize) -> Self {
        Self {
            parent, pos, 
            g, h, f
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        let x = self.pos.0 == other.pos.0;
        let y = self.pos.1 == other.pos.1;
        x && y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Solvable {}

    impl Maze for Solvable {
        fn height(&self) -> usize {
            10
        }

        fn width(&self) -> usize {
            10
        }

        fn solid(&self, x: usize, y: usize) -> bool {
            if x == 1 && y == 0 {
                return true;
            } else if x == 1 && y == 1 {
                return true;
            }

            false
        }
    }

    struct Unsolvable {}

    impl Maze for Unsolvable {
        fn height(&self) -> usize {
            10
        }

        fn width(&self) -> usize {
            10
        }

        fn solid(&self, x: usize, _: usize) -> bool {
            if x == 2 {
                return true;
            }

            false
        }
    }

    #[test]
    fn test_astar() {
        let result = astar(&Solvable {}, (0, 0), (9, 0));
        let solution = vec![(0, 0), (0, 1), (1, 2), (2, 1), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0)];
        assert_eq!(Some(solution), result);

        let result = astar(&Unsolvable {}, (0, 0), (9, 9));
        assert_eq!(None, result);
    }
}