use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::f64;

fn main() -> Result<(), Box<dyn Error>> {
    // Relative path (project root). Put input.csv next to Cargo.toml
    let input_path = Path::new("resources/slyc.csv");
    let output_path = Path::new("output/output.gpx");

    // Build a reader with no headers
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(input_path)?;

    // Create a writer for the output CSV
   // let mut wtr = csv::Writer::from_path(output_path)?;

    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    // Iterate records (StringRecord). Each record is a row of fields.
    for result in rdr.records() {
        let record = result?; // Propagate CSV parsing errors
        // Print fields to console
        let fields: Vec<&str> = record.iter().collect();
        println!("{:?}", fields);

        let mut longitude_decimal = 0.0;
        let mut latitude_decimal =0.0;
        if let Some((a, b, c)) = parse_values(fields[1]) {
            println!("First: {}, Second: {}, Third: {}", a, b, c);
            let latitude_degrees = a;
            let latitude_minutes = b;
            let latitude_seconds = c as f64;
            let latitude_direction = 'N';

            latitude_decimal = dms_to_decimal(
                latitude_degrees,
                latitude_minutes,
                latitude_seconds,
                latitude_direction,
            );



        } else {
            println!("Invalid input format on lat :{} " ,  fields[1] );
        }

        if let Some((a, b, c)) = parse_values(fields[2]) {
            println!("First: {}, Second: {}, Third: {}", a, b, c);
            let longitude_degrees = a;
            let longitude_minutes = b;
            let longitude_seconds = c as f64;
            let longitude_direction = 'W';

             longitude_decimal = dms_to_decimal(
                longitude_degrees,
                longitude_minutes,
                longitude_seconds,
                longitude_direction,
            );




        } else {
            println!("Invalid input format on long :{} " ,  fields[2] );
        }

        println!("Latitude: {}", latitude_decimal);
        println!("Longitude: {}", longitude_decimal);

        let output = format!("Name: {}, Latitude: {}, Longitude: {}", fields[0], latitude_decimal, longitude_decimal);


        // Write same record to output.gpx
        //wtr.write_record(record.iter())?;
        writeln!(writer, "{}", output)?;
    }

    // Flush writer to ensure everything is written
    writer.flush()?;
    println!("Finished. Wrote {}", output_path.display());

    Ok(())
}





fn dms_to_decimal(degrees: i32, minutes: i32, seconds: f64, direction: char) -> f64 {
    let mut decimal = degrees as f64;
    decimal += (minutes as f64) / 60.0;
    decimal += seconds / 3600.0;

    match direction {
        'S' | 'W' => -decimal,
        _ => decimal,
    }
}

fn parse_values(input: &str) -> Option<(i32, i32, i32)> {
    // Split the string by whitespace
    let mut parts = input.split_whitespace();

    // First number is just an integer
    let first = parts.next()?.parse::<i32>().ok()?;

    // Second part is a float-like string
    let second_part = parts.next()?;

    // Split by decimal point
    let mut decimal_parts = second_part.split('.');
    let second = decimal_parts.next()?.parse::<i32>().ok()?;
    let third = decimal_parts.next()?.parse::<i32>().ok()?;

    Some((first, second, third))
}

