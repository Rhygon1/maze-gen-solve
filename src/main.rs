use image::{ImageReader, GenericImageView, GenericImage, Rgba};
use std::ops::Add;
use std::time::{Duration, Instant};
use std::collections::{HashMap};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::hash::{Hasher, BuildHasher};

#[derive(Default)]
struct MyHasher {
    state: u64,
}

impl Hasher for MyHasher {
    fn finish(&self) -> u64 {
        self.state 
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut hash: u64 = 0;
        for b in 0..3 {
            let byte = bytes[b];
            hash ^= byte as u64;
            hash = hash.wrapping_mul(16777619);
        }
        self.state = hash;

    }
}

#[derive(Default)]
struct MyBuildHasher;

impl BuildHasher for MyBuildHasher {
    type Hasher = MyHasher;
    
    fn build_hasher(&self) -> MyHasher {
        MyHasher::default()
    }
}

// type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<AHasher>>;

type FastHashMap<K, V> = HashMap<K, V, MyBuildHasher>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut img = ImageReader::open("output_go.png")?.decode()?;
    let size: usize = img.dimensions().0.try_into().unwrap();

    let mut maze: Vec<Vec<u8>> = vec![];
    let mut visited: Vec<Vec<bool>> = vec![];
    for (_x, y, p) in img.pixels(){
        if maze.len() <= y.try_into().unwrap(){
            maze.push(vec![]);
            visited.push(vec![]);
        }
        maze[y as usize].push(p[0] >> 7);
        visited[y as usize].push(false);
    }
    visited[1][1] = true;
    
    let first: usize = 12;
    let second = (i32::pow(2, first.try_into().unwrap()) - 1) as usize;

    let start = Instant::now();
    let total_path =  a_star(1 << first | 1, (size-2) << first | (size-2), size, &maze, &mut visited, first, second);
    println!("{:?}", start.elapsed());
    let fill = Rgba([255 as u8, 0 as u8, 0 as u8, 0 as u8]);

    for i in 0..total_path.len(){
        let p = total_path[i];
        let point = [p >> first, p & second];
        img.put_pixel(point[1] as u32, point[0] as u32, fill);
        if i != 0{
            let prev_p = total_path[i-1];
            let prev_point = [prev_p >> first, prev_p & second];
            let mid: [usize; 2];
            if prev_point[1] == point[1]{
                if prev_point[0] > point[0]{
                    mid = [point[0]+1, point[1]];
                } else {
                    mid = [point[0]-1, point[1]];
                }
            } else {
                if prev_point[1] > point[1]{
                    mid = [point[0], point[1]+1];
                } else {
                    mid = [point[0], point[1]-1];
                }
            }
            img.put_pixel(mid[1] as u32, mid[0] as u32, fill);
        }
    }
    img.put_pixel(0, 1, fill);
    img.put_pixel((size-2) as u32, (size-1) as u32, fill);

    let _ = img.save("result.png");

    Ok(())
}

fn a_star(start: usize, goal: usize, size: usize, maze: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>, first: usize, second: usize) -> Vec<usize>{
    let mut open_set = PriorityQueue::new();
    // let mut came_from: HashMap<usize, usize> = HashMap::new();
    // let mut g_score = HashMap::from([(start, 0 as usize)]);
    let f_start = h(start, goal, first, second);
    // let mut f_score = HashMap::from([(start, f_start)]);

    let mut came_from: FastHashMap<usize, usize> = FastHashMap::default();

    let mut all_time = Duration::from_nanos(0);
    
    open_set.push(start, Reverse(f_start));
    while open_set.len() > 0{
        let curr = match open_set.pop(){
            Some(e) => e.0,
            None => {
                panic!("WHAT");
            }
        };
        if curr == goal{
            println!("{:?}", all_time);
            return reconstruct_path(&came_from, goal);
        }
        let neighbors = get_neighbors(curr, size, first, second, maze, visited);
        for idx in 0..neighbors.len(){
            let neighbor = neighbors[idx];
            // let came_from = Arc::clone
            // let tentative_g_score = match all_hash_maps.get(&curr){
                //     Some(e) => e[1]+2,
                //     None => (1 << 30) as usize
                // };
                // let neigh_g_score = match all_hash_maps.get(&neighbor){
            //     Some(e) => e[1],
            //     None => (1 << 30) as usize
            // };
            
            // if !visited[neighbor >> first][neighbor & second]{
            // if tentative_g_score < neigh_g_score{
            let new_f_score = h(neighbor, goal, first, second);
            // let start_time = Instant::now();
            came_from.insert(neighbor, curr);
            // all_time = all_time.add(start_time.elapsed());
            // came_from.insert(neighbor, curr);
            // g_score.insert(neighbor, tentative_g_score);
            // f_score.insert(neighbor, new_f_score);
            open_set.push(neighbor, Reverse(new_f_score));
            visited[neighbor >> first][neighbor & second] = true;
            // }
            // }
        }
        
    }

    return vec![];
}

fn get_neighbors(curr: usize, size: usize, first: usize, second: usize, maze: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>) -> Vec<usize>{
    let mut neighbors: Vec<usize> = vec![];
    let curr_cell_0 = curr >> first;
    let curr_cell_1 = curr & second;

    let double_row = 1 << (first + 1);
    if  maze[curr_cell_0-1][curr_cell_1] == 1 && curr_cell_0 > 1 && !visited[curr_cell_0-2][curr_cell_1]{
        neighbors.push(curr-double_row);
	}
    if maze[curr_cell_0][curr_cell_1-1] == 1 && curr_cell_1 > 1 && !visited[curr_cell_0][curr_cell_1-2]{
        neighbors.push(curr-2)
	}
    if maze[curr_cell_0][curr_cell_1+1] == 1 && curr_cell_1 < size-2 && !visited[curr_cell_0][curr_cell_1+2]{
        neighbors.push(curr+2)
	}
    if maze[curr_cell_0+1][curr_cell_1] == 1 && curr_cell_0 < size-2 && !visited[curr_cell_0+2][curr_cell_1]{
        neighbors.push(curr+double_row)
	}

    return neighbors;
}

fn reconstruct_path(came_from: &FastHashMap<usize, usize>, start: usize) -> Vec<usize>{
    let mut current = start;
    let mut total_path = Vec::from([current]);
    while came_from.contains_key(&current){
        current = came_from[&current];
        total_path.push(current);
    }
    return total_path;
}

fn h(curr: usize, end: usize, first: usize, second: usize) -> usize{
    let [x1, y1, x2, y2] = [curr >> first, curr & second, end >> first, end & second];
	return ((y2-y1)*(y2-y1))+((x2-x1)*(x2-x1));
}
