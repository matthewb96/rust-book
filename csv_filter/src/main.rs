use std::path::Path;
use std::fs::File;
use serde::Deserialize;

fn main() {
    println!("CSV Filter");

    // test_read_csv();
    test_read_csv2();
}

#[derive(Debug, Deserialize)]
struct Segment {
    p: u8,
    m: u8,
    g: u8,
    soc: u8,
    ns: u8,
    ca: u8,
    tp: u8,
    val: f64,
}

fn test_read_csv() {
    let file_path = Path::new(r#"C:\Users\MattBuckley\Documents\Projects\NorMITs Demand\Outputs\NoTEM\iter9.6\SC01_JAM\hb_productions\reports\hb_notem_segmented_2018_segment_totals.csv"#);

    
    let file = File::open(file_path).expect("error reading file");
    let mut reader = csv::Reader::from_reader(file);
    println!("{:?}", reader.headers());
    for result in reader.deserialize() {
        let record: Segment = result.expect("error with result");
        if record.p == 1 && record.m == 3 {
            println!("{:?}", record);
        }
    }
}


fn test_read_csv2() {
    let file_path = Path::new(r#"C:\Users\MattBuckley\Documents\Projects\Freight Lot3\Delta Process Tests\model_forecast_demand_standard.csv"#);

    let file = File::open(file_path).expect("error reading file");
    let mut reader = csv::Reader::from_reader(file);
    println!("{:?}", reader.headers());
    let mut count = 0;
    for result in reader.records() {
        let record = result.expect("error with result");
        if record[1] == "0".to_owned() {
            count += 1;
        }
    }

    println!("{}", count);
}