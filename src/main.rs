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
        vector_of_neighbors: HashMap<&'a str, i32>
    }

    let mut vector_of_cities: Vec<City> = Vec::new();
    let mut vector_of_city_id: Vec<&str> = Vec::new();

    for i in 3..vector_of_lines.len()-1 {
        let city = vector_of_lines[i];
        let splitted_city = city.split(" ");
        let vector_of_splitted_city = splitted_city.collect::<Vec<&str>>();

        let mut hashMap: HashMap<&str, i32> = HashMap::new();

        let city_id = vector_of_splitted_city[0];
        vector_of_city_id.push(city_id);

        for t in (1..vector_of_splitted_city.len() - 1).step_by(2) {
            let neighbor_id = vector_of_splitted_city[t];
            let neighbor_distance: i32 = FromStr::from_str(vector_of_splitted_city[t+1]).unwrap();
            hashMap.insert(
                neighbor_id,
                neighbor_distance
            );
        };

        let city = City {
            id: vector_of_city_id[i-3],
            vector_of_neighbors: hashMap
        };

        vector_of_cities.push(city);
    }

    println!("{:?}", vector_of_cities[4].vector_of_neighbors);
    println!("{:?}", vector_of_cities[3].id);

    //city
    struct Vertex {
        distance: i32,
        previous: Box<Vertex>
    }

    //road
    struct Edge {

    }
}


