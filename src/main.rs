extern crate ical;

use std::io::BufReader;
use std::fs::File;

fn main() {
    let buf = BufReader::new(File::open("/mnt/c/Users/aaaaa/calendar.ics").unwrap());
    // let reader = ical::PropertyParser::from_reader(buf);
    let reader = ical::IcalParser::new(buf);
    // println!("{:?}", reader[0]);
    for line in reader {
        let entry = line.unwrap();
        let events = entry.events;
        // println!("{:?}", events);
        for event in events {
            for property in event.properties {
                if property.name == "SUMMARY" {
                    println!("{:?}", property);
                }
            }
        }
        // for property in events.properties {
        //     // if property.name ==
        //     // if property.name == "SUMMARY" {
        //     //     println!("{:?}", property);
        //     // }
        //     println!("{:?}", property);
        // }
    }
}
