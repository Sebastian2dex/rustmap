use std::ops::RangeInclusive;

/*
   User provides port in the following forms:
   1. Single port (eg. -p 80)
   2. Range (eg. -p 80-1000)
   3. No mentioned ports -> fall back to scanning first 10,000 ports
*/

pub fn parse_ports(input: Option<&str>) -> Result<RangeInclusive<u16>, String> {
    // dafault scan
    let input = match input {
        Some(e) => e,
        None => return Ok(1..=10000),
    };

    // check if ports are ranged or not
    if input.contains('-') {
        let ports_parsed: Vec<&str> = input.split('-').collect();
        if ports_parsed.len() != 2 {
            return Err(format!("Invalid Port Range"));
        } else {
            let start = ports_parsed[0].parse::<u16>();
            let end = ports_parsed[1].parse::<u16>();

            let mut parsed_start = match start {
                Ok(start) => start,
                Err(_) => return Err(format!("Invalid Port Specified")),
            };

            let mut parsed_end = match end {
                Ok(end) => end,
                Err(_) => return Err(format!("Invalid Port Specified")),
            };

            if parsed_start == 0 || parsed_end == 0 {
                return Err(String::from("Invalid Ports Specified"));
            }

            if parsed_start > parsed_end {
                std::mem::swap(&mut parsed_start, &mut parsed_end);
            }
            return Ok(parsed_start..=parsed_end);
        }
    } else {
        let port = input
            .parse::<u16>()
            .map_err(|_| String::from("Invalid Port Specified"))?;

        if port == 0 {
            return Err(String::from("Invalid Port Specified"));
        }
        return Ok(port..=port);
    }
}
