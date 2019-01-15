use crate::utils;
use regex::Regex;
use std::collections::HashMap;

pub fn day4(lines: &mut Vec<String>) {
    println!("Running Day 4 - a");

    lines.sort(); // ... seriously? wow, that was easy.

    let timeRex = Regex::new(r"\d+:(?P<m>\d+)").unwrap();
    let beginRex = Regex::new(r"Guard #(?P<g>\d+)").unwrap();

    let mut guards: HashMap<i32, Guard> = HashMap::new();

    let mut guardOnDuty: Option<i32> = Option::None;
    let mut asleep: Option<i32> = Option::None;

    for line in lines.iter() {
        if line.contains("falls asleep") {
            let minute = utils::parse(&timeRex.captures(line).unwrap()["m"]);
            asleep = Option::Some(minute);
        } else if line.contains("wakes up") {
            let minute = utils::parse(&timeRex.captures(line).unwrap()["m"]);
            let guard = guards.get_mut(&guardOnDuty.unwrap()).unwrap();
            guard.sleep(asleep.unwrap(), minute);
        } else {
            let guardId = utils::parse(&beginRex.captures(line).unwrap()["g"]);
            let mut guard = guards.entry(guardId).or_insert(Guard::new());
            guard.id = guardId;
            guardOnDuty = Option::Some(guard.id);
        }
    }

    guards.values_mut().for_each(Guard::finalize);

    let sleepiestGuard = guards.values().max_by_key(|g| g.minsAsleep).unwrap();

    println!(
        "Guard # {} :: Mins Asleep = {}, Min Most Asleep = {}, Times Asleep in Min = {}, Checksum = {}",
        sleepiestGuard.id,
        sleepiestGuard.minsAsleep,
        sleepiestGuard.minMost,
        sleepiestGuard.mostInMin,
        sleepiestGuard.id * sleepiestGuard.minMost
    );

    println!("Running Day 4 - b");

    let predictableGuard = guards.values().max_by_key(|g| g.mostInMin).unwrap();

    println!(
        "Guard # {} :: Mins Asleep = {}, Min Most Asleep = {}, Times Asleep in Min = {}, Checksum = {}",
        predictableGuard.id,
        predictableGuard.minsAsleep,
        predictableGuard.minMost,
        predictableGuard.mostInMin,
        predictableGuard.id * predictableGuard.minMost
    );
}

struct Guard {
    id: i32,
    mins: [i32; 60],
    minsAsleep: i32,
    minMost: i32,
    mostInMin: i32,
}

impl Guard {
    fn new() -> Guard {
        Guard {
            id: 0,
            mins: [0; 60],
            minsAsleep: 0,
            minMost: -1,
            mostInMin: -1,
        }
    }

    fn sleep(&mut self, from: i32, to: i32) {
        for i in from..to {
            self.mins[i as usize] += 1;
        }
    }

    fn finalize(&mut self) {
        self.minsAsleep = self.mins.iter().sum();
        for (i, val) in self.mins.iter().enumerate() {
            if *val > self.mostInMin {
                self.mostInMin = *val;
                self.minMost = i as i32;
            }
        }
    }
}
