use std::error::Error;
use std::io;
use std::process;
use csv::Reader;
use crate::tools::divide;

#[test]
fn acts_division_should_work() {
    let mut rdr = csv::Reader::from_path("./acts/divide-output.csv");
    for result in rdr.unwrap().records() {
        let row = result.unwrap();
        let dividend = row.get(0).unwrap().parse::<f64>().unwrap();
        let divisor = row.get(1).unwrap().parse::<f64>().unwrap();

        let result = divide(dividend, divisor);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (dividend/divisor));
    }
}