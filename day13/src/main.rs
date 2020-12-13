use std::io::{BufRead, stdin};

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

    let bus_ids: Vec<BusId> = bus_ids.into_iter()
        .filter(|id| *id != "x")
        .map(|id| id.parse::<BusId>()
            .expect(format!("Could not convert bus ID to number: {0}", id).as_str())
        )
        .collect();

    let best_bus_id = part1(can_depart_at, &bus_ids)
        .expect("Could not determine best_bus_id");

    let next_departure = next_departure_time_for_bus_id(can_depart_at, best_bus_id);
    let wait_time = next_departure - can_depart_at;
    println!("part1: best_bus_id={0} next_departure={1}, wait_time={2} -> result={3}",
             best_bus_id, next_departure, wait_time, best_bus_id * wait_time);
}

fn part1(can_depart_at: DepartureTime, bus_ids: &Vec<BusId>) -> Option<BusId> {
    println!("part1: can_depart_at={0} bus_ids={1:?}", can_depart_at, bus_ids);

    let mut earliest_bus = None;
    let mut smallest_departure = DepartureTime::max_value();

    for &bus_id in bus_ids {
        let mut closest_departure = next_departure_time_for_bus_id(can_depart_at, bus_id);

        println!("bus_id={0} closest_departure={1}", bus_id, closest_departure);

        if closest_departure < smallest_departure {
            smallest_departure = closest_departure;
            earliest_bus = Some(bus_id);
        }
    }

    earliest_bus
}

fn next_departure_time_for_bus_id(can_depart_at: DepartureTime, bus_id: BusId) -> DepartureTime {
    (can_depart_at as f32/ bus_id as f32).ceil() as DepartureTime * bus_id
}

fn read_input() -> Vec<String> {
    stdin().lock().lines()
        .map(|maybe_line| maybe_line.expect("Error while reading line"))
        .collect()
}
