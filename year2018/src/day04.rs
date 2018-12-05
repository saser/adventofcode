use base::{Part, Solver};
use chrono::{NaiveDateTime, Timelike};
use regex::Regex;

use std::collections::HashMap;
use std::str::FromStr;

pub fn get_solver() -> Box<Solver> {
    Box::new(Day04)
}

struct Day04;

impl Solver for Day04 {
    fn solve(&self, part: Part, input: &str) -> Result<String, String> {
        let mut sorted_events = parse_input(input);
        sorted_events.sort();
        match part {
            Part::One => Ok(strategy_1(&sorted_events).to_string()),
            Part::Two => Ok(strategy_2(&sorted_events).to_string()),
        }
    }
}

fn parse_input(input: &str) -> Vec<Event> {
    input
        .lines()
        .map(Event::from_str)
        .map(Result::unwrap)
        .collect()
}

fn strategy_1(sorted_events: &[Event]) -> u64 {
    let guard_events = gather_guard_events(sorted_events);
    let (id, (_total_sleep, most_sleeping_minute, _most_times_asleep)) = guard_events
        .iter()
        .map(|(id, events)| (id, calculate_sleeping(events)))
        .max_by_key(|&(_id, (total_sleep, _most_sleeping_minute, _most_times_asleep))| total_sleep)
        .unwrap();
    id * most_sleeping_minute as u64
}

fn strategy_2(sorted_events: &[Event]) -> u64 {
    let guard_events = gather_guard_events(sorted_events);
    let (id, (_total_sleep, most_sleeping_minute, _most_times_asleep)) = guard_events
        .iter()
        .map(|(id, events)| (id, calculate_sleeping(events)))
        .max_by_key(
            |&(_id, (_total_sleep, _most_sleeping_minute, most_times_asleep))| most_times_asleep,
        ).unwrap();
    id * most_sleeping_minute as u64
}

fn gather_guard_events(events: &[Event]) -> HashMap<u64, Vec<Vec<(u32, EventType)>>> {
    let first_event = events[0];
    let first_event_minute = first_event.datetime.minute();
    let first_event_type = first_event.event_type;
    let mut current_guard = if let EventType::BeginsShift(id) = first_event_type {
        id
    } else {
        panic!("First event is not a begins shift event");
    };
    let mut current_events = vec![(first_event_minute, first_event_type)];
    let mut map = HashMap::new();
    for &event in &events[1..] {
        let event_minute = event.datetime.minute();
        let event_type = event.event_type;
        if let EventType::BeginsShift(id) = event_type {
            map.entry(current_guard)
                .or_insert(Vec::new())
                .push(current_events);
            current_guard = id;
            current_events = Vec::new();
        } else {
            current_events.push((event_minute, event_type));
        }
    }
    map.entry(current_guard)
        .or_insert(Vec::new())
        .push(current_events);
    map
}

fn calculate_sleeping(events: &Vec<Vec<(u32, EventType)>>) -> (u32, u32, u32) {
    let mut combined = events
        .into_iter()
        .cloned()
        .flatten()
        .filter(|(_event_minute, event_type)| {
            if let EventType::BeginsShift(_) = event_type {
                false
            } else {
                true
            }
        }).collect::<Vec<(u32, EventType)>>();
    combined.sort();
    let mut last_event_minute = 0;
    let mut total_sleep = 0;
    let mut times_asleep = 0;
    let mut most_sleeping_minute = 0;
    let mut most_times_asleep = 0;
    for &(event_minute, event_type) in &combined {
        total_sleep += (event_minute - last_event_minute) * times_asleep;
        match event_type {
            EventType::BeginsShift(_) => unreachable!(),
            EventType::FallsAsleep => times_asleep += 1,
            EventType::WakesUp => times_asleep -= 1,
        };
        if times_asleep > most_times_asleep {
            most_sleeping_minute = event_minute;
            most_times_asleep = times_asleep;
        }
        last_event_minute = event_minute;
    }
    (total_sleep, most_sleeping_minute, most_times_asleep)
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Event {
    datetime: NaiveDateTime,
    event_type: EventType,
}

impl FromStr for Event {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref EVENT_RE: Regex =
                Regex::new(r"\[(?P<datetime>\d{4}\-\d{2}\-\d{2} \d{2}:\d{2})\] (?P<event_type>.+)")
                    .unwrap();
        }
        let captures = EVENT_RE.captures(s).unwrap();
        let datetime =
            NaiveDateTime::parse_from_str(&captures["datetime"], "%Y-%m-%d %H:%M").unwrap();
        let event_type = EventType::from_str(&captures["event_type"]).unwrap();
        Ok(Event {
            datetime,
            event_type,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum EventType {
    BeginsShift(u64),
    FallsAsleep,
    WakesUp,
}

impl FromStr for EventType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BEGIN_RE: Regex = Regex::new(r"Guard #(?P<id>\d+) begins shift").unwrap();
        }
        if let Some(caps) = BEGIN_RE.captures(s) {
            let id = u64::from_str(&caps["id"]).unwrap();
            Ok(EventType::BeginsShift(id))
        } else {
            match s {
                "falls asleep" => Ok(EventType::FallsAsleep),
                "wakes up" => Ok(EventType::WakesUp),
                _ => Err(format!("invalid event: \"{}\"", s)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    mod parsing {
        use super::*;

        mod event {
            use super::*;

            #[test]
            fn begins_shift() {
                let input = "[1518-11-01 00:00] Guard #10 begins shift";
                let expected_datetime = NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0);
                let expected_event_type = EventType::BeginsShift(10);
                let expected = Event {
                    datetime: expected_datetime,
                    event_type: expected_event_type,
                };
                assert_eq!(expected, Event::from_str(input).unwrap());
            }

            #[test]
            fn falls_asleep() {
                let input = "[1518-11-01 00:42] falls asleep";
                let expected_datetime = NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 42, 0);
                let expected_event_type = EventType::FallsAsleep;
                let expected = Event {
                    datetime: expected_datetime,
                    event_type: expected_event_type,
                };
                assert_eq!(expected, Event::from_str(input).unwrap());
            }

            #[test]
            fn wakes_up() {
                let input = "[1518-11-01 00:58] wakes up";
                let expected_datetime = NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 58, 0);
                let expected_event_type = EventType::WakesUp;
                let expected = Event {
                    datetime: expected_datetime,
                    event_type: expected_event_type,
                };
                assert_eq!(expected, Event::from_str(input).unwrap());
            }
        }

        mod event_type {
            use super::*;

            #[test]
            fn begins_shift_single_digit() {
                let input = "Guard #4 begins shift";
                let expected = EventType::BeginsShift(4);
                assert_eq!(expected, EventType::from_str(input).unwrap());
            }

            #[test]
            fn begins_shift_multiple_digits() {
                let input = "Guard #1234 begins shift";
                let expected = EventType::BeginsShift(1234);
                assert_eq!(expected, EventType::from_str(input).unwrap());
            }

            #[test]
            fn begin_shift_invalid_id() {
                let input = "Guard #asd begins shift";
                assert!(EventType::from_str(input).is_err());
            }

            #[test]
            fn falls_asleep() {
                let input = "falls asleep";
                let expected = EventType::FallsAsleep;
                assert_eq!(expected, EventType::from_str(input).unwrap());
            }

            #[test]
            fn wakes_up() {
                let input = "wakes up";
                let expected = EventType::WakesUp;
                assert_eq!(expected, EventType::from_str(input).unwrap());
            }
        }
    }

    mod part1 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/04");
            let expected = "125444";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up\
            ";
            let expected = "240";
            assert_eq!(expected, solver.solve(Part::One, input).unwrap());
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn with_input() {
            let solver = get_solver();
            let input = include_str!("../../inputs/2018/04");
            let expected = "18325";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }

        #[test]
        fn example() {
            let solver = get_solver();
            let input = "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up\
            ";
            let expected = "4455";
            assert_eq!(expected, solver.solve(Part::Two, input).unwrap());
        }
    }
}
