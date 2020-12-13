use std::io::{BufRead, stdin};
use ring_algorithm::chinese_remainder_theorem;

type DepartureTime = u32;
type BusId = u32;

fn main() {
    let input = read_input();

    // println!("input={0:?}", input);

    let mut input = input.iter();

    let can_depart_at = input.next()
        .map(|line| line.parse::<DepartureTime>()
            .expect(format!("Could not convert '{0}' to DepartureTime", line).as_str())
        )
        .expect("Could not fetch can_depart_at");

    let bus_ids = input.next()
        .map(|line| line.split(","))
        .expect("Could not fetch bus ID's");

    let bus_ids = bus_ids.into_iter()
        .map(|line| String::from(line))
        .collect();

    part1(can_depart_at, &bus_ids);
    part2(&bus_ids);
}

fn part1(can_depart_at: DepartureTime, bus_ids: &Vec<String>) -> BusId {
    let bus_ids: Vec<BusId> = bus_ids.into_iter()
        .filter(|id| *id != "x")
        .map(|id| id.parse::<BusId>()
            .expect(format!("Could not convert bus ID to number: {0}", id).as_str())
        )
        .collect();

    println!("part1: can_depart_at={0} bus_ids={1:?}", can_depart_at, bus_ids);

    let mut best_bus_id = BusId::max_value();
    let mut smallest_departure = DepartureTime::max_value();

    for bus_id in bus_ids {
        let closest_departure = next_departure_time_for_bus_id(can_depart_at, bus_id);

        println!("bus_id={0} closest_departure={1}", bus_id, closest_departure);

        if closest_departure < smallest_departure {
            smallest_departure = closest_departure;
            best_bus_id = bus_id;
        }
    }

    if best_bus_id == BusId::max_value() {
        panic!("Could not determine best_bus_id");
    }

    let next_departure = next_departure_time_for_bus_id(can_depart_at, best_bus_id);
    let wait_time = next_departure - can_depart_at;

    println!("part1: best_bus_id={0} next_departure={1} wait_time={2} -> result={3}",
             best_bus_id, next_departure, wait_time, best_bus_id * wait_time);

    best_bus_id
}

fn part2(bus_ids: &Vec<String>) {
    let (offsets, bus_ids): (Vec<_>, Vec<_>) = bus_ids.iter()
        .enumerate()
        .filter(|&(_offset, id)| *id != "x")
        .map(|(offset, id)| (-(offset as i128), id.parse::<i128>()
            .expect(format!("Could not convert bus ID to number: {0}", id).as_str()))
        )
        .unzip();

    println!("part2: offsets={0:?} bus_ids={1:?}", offsets, bus_ids);

    // Got actual solution (842186186521918) via https://www.dcode.fr/chinese-remainder

    // Somehow this returns 18445069871014220103 which is different than the actual solution
    let i = chinese_remainder_theorem(&offsets, &bus_ids).unwrap() as usize;

    println!("{0}", i);
}

fn next_departure_time_for_bus_id(can_depart_at: DepartureTime, bus_id: BusId) -> DepartureTime {
    (can_depart_at as f32 / bus_id as f32).ceil() as DepartureTime * bus_id
}

fn read_input() -> Vec<String> {
    stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect()
}
