use std::env;
use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let splitted_content = contents.split("\n");
    let vector_of_lines = splitted_content.collect::<Vec<&str>>();

    let limit_of_father: i32 = vector_of_lines[0].parse().unwrap();
    let city_amount: i32 = vector_of_lines[1].parse().unwrap();

    let splitted_cities_of_lm = vector_of_lines[2].split(" ");
    let cities_of_lm = splitted_cities_of_lm.collect::<Vec<&str>>();
    let city_of_mecnun = cities_of_lm[0];
    let city_of_leyla = cities_of_lm[1];  

    struct City<'a> {
        id: &'a str,
        hashmap_of_neighbors: HashMap<&'a str, i32>,
        neg_id_vec: Vec<&'a str>,
        neg_dist_vec: Vec<i32>
    }

    let mut vector_of_cities: Vec<City> = Vec::new();
    let mut vector_of_city_id: Vec<&str> = Vec::new();

    for i in 3..vector_of_lines.len()-1 {
        let city = vector_of_lines[i];
        let splitted_city = city.split(" ");
        let vector_of_splitted_city = splitted_city.collect::<Vec<&str>>();

        let mut hashMap: HashMap<&str, i32> = HashMap::new();

        let mut neighbor_id_vec: Vec<&str> = Vec::new();
        let mut neighbor_dist_vec: Vec<i32> = Vec::new();

        let city_id = vector_of_splitted_city[0];
        vector_of_city_id.push(city_id);

        for t in (1..vector_of_splitted_city.len() - 1).step_by(2) {
            let neighbor_id = vector_of_splitted_city[t];
            let neighbor_distance: i32 = FromStr::from_str(vector_of_splitted_city[t+1]).unwrap();

            neighbor_id_vec.push(neighbor_id);
            neighbor_dist_vec.push(neighbor_distance);

            hashMap.insert(
                neighbor_id,
                neighbor_distance
            );
        };

        let city = City {
            id: vector_of_city_id[i-3],
            hashmap_of_neighbors: hashMap,
            neg_id_vec: neighbor_id_vec,
            neg_dist_vec: neighbor_dist_vec
        };

        vector_of_cities.push(city);
    }

    //city
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: usize,
        position: usize,
        previous: usize
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    //road
    struct Edge {
        node: usize,
        cost: usize,
    }    

    fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
        let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
        let mut prev: Vec<_> = (0..adj_list.len()).map(|_| usize::MIN).collect();
        //let mut prevler = Vec::<_>::new();

        let mut heap = BinaryHeap::new();

        dist[start] = 0;
        heap.push(State {cost: 0, position: start, previous: 1});

        while let Some(State { cost, position, previous }) = heap.pop() {
            if position == goal { 
                println!("{:?}", prev);
                return Some(cost); 
            }

            if cost > dist[position] {continue;}

            for edge in &adj_list[position] {
                let next = State { cost: cost + edge.cost, position: edge.node, previous: position + 1 };

                if next.cost < dist[next.position] {
                    heap.push(next);
                    dist[next.position] = next.cost;
                    prev[position] = next.previous;
                }
            }
        }
        None
    }

    let mut graph = Vec::<Vec<Edge>>::new();
    let mut graph_honeymoon = Vec::<Vec<Edge>>::new();

    let lenght_to_c = 7;

    for i in 0..vector_of_cities.len(){
        let city = &vector_of_cities[i];
        let mut edges = Vec::<Edge>::new();
        let mut edges_honeymoon = Vec::<Edge>::new();


        for t in 0..city.neg_id_vec.len() {
            let nodest = city.neg_id_vec[t];
            if nodest.chars().nth(0).unwrap() != 'c' {
                let nodesto = nodest.replace("d", "");
                let node_u32: u32 = nodesto.parse().unwrap();
                let node: usize = node_u32 as usize - 1;  //to usize
                let costest = city.neg_dist_vec[t];
                let cost: usize = costest as usize;

                let edge = Edge{
                    node: node,
                    cost: cost
                };
                edges_honeymoon.push(edge);
            } else {
                let nodesto = nodest.replace("c", "");
                let node_u32: u32 = nodesto.parse().unwrap();
                let node: usize = node_u32 as usize - 1;  //to usize
                let costest = city.neg_dist_vec[t];
                let cost: usize = costest as usize;
                //to usize
    
                let edge = Edge{
                    node: node,
                    cost: cost
                };
                edges.push(edge);
            }
        }

        graph.push(edges);
        graph_honeymoon.push(edges_honeymoon);
    }

    let mut start_node_s = city_of_mecnun.replace("c", "");
    let start_node_u: u32 = start_node_s.trim().parse().unwrap();
    let start_node: usize = start_node_u as usize - 1;

    let mut target_node_s = city_of_leyla.replace("c", "");
    let target_node_u: u32 = target_node_s.trim().parse().unwrap();
    let target_node: usize = target_node_u as usize - 1;

    let mut int1 = 0;

    if shortest_path(&graph, start_node, target_node).unwrap() > limit_of_father.try_into().unwrap() {
        int1 = -1;
    }else {
        int1 = shortest_path(&graph, start_node, target_node).unwrap() as i32;
    }

    shortest_path(&graph, start_node, target_node);

    println!("Result: {:?}" , shortest_path(&graph, start_node, target_node)); 
    println!("Result: {:?}" , int1); 
}


