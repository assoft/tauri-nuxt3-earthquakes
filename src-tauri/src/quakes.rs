pub mod afad {
    pub fn get_last_five_events () -> Result<String, ureq::Error> {
        let response = ureq::get("https://deprem.afad.gov.tr/EventData/GetLast5Events")
            .call()?;
        Ok(response.into_string()?)
    }
}

pub mod helper {
    use chrono::{Utc, ParseError, TimeZone};
    pub fn parse_date_time(datestr: &str) -> Result<i64, ParseError> {
        let parsed = Utc.datetime_from_str(datestr, "%Y.%m.%d %H:%M:%S")?;
        Ok(parsed.timestamp())
    }
}

pub mod kandilli {
    use scraper::{Html, Selector};
    use serde::Serialize;

    use super::helper;

    #[derive(Debug, Serialize)]
    #[allow(non_snake_case)]
    pub struct Quake {
        id: i64,
        eventDate: String,
        latitude: String,
        longitude: String,
        depth: String,
        magnitude: String,
        // md: String,
        // ml: String,
        // mw: String,
        location: String,
        info: String
    }
    pub fn get_lastest_events () -> Result<Vec<Quake>, ureq::Error> {
        let response = ureq::get("http://www.koeri.boun.edu.tr/scripts/lst5.asp").call()?;
        let doc = response.into_string().unwrap();

        let query = Html::parse_fragment(&doc);

        let wrapper = Selector::parse("pre").unwrap();
        let mut quakes_data: Vec<Quake> = Vec::new();

        let table = query.select(&wrapper).last().unwrap().inner_html();

        let mut table_split = Vec::from_iter(table.lines().map(String::from));
        
        table_split.drain(0..6);

        table_split.iter().for_each(|row| {
            let row_data: Vec<String> = row.split("  ")
                .filter(|&x| !x.is_empty())
                .map(|s| {
                    String::from_utf8_lossy(s.trim().as_bytes()).to_string()
                })
                .collect();
    
            if row_data.len() == 9 {
                let magnitude = if row_data[6].eq("-.-") {row_data[5].to_string()} else {row_data[6].to_string()};
                let quake = Quake {
                    id: helper::parse_date_time(row_data[0].as_str()).unwrap(),
                    eventDate: row_data[0].to_string(),
                    latitude: row_data[1].to_string(),
                    longitude: row_data[2].to_string(),
                    depth: row_data[3].to_string(),
                    magnitude,
                    // md: row_data[4].to_string(),
                    // ml: row_data[5].to_string(),
                    // mw: row_data[6].to_string(),
                    location: row_data[7].to_string(),
                    info: row_data[8].to_string(),
                };
                quakes_data.push(quake);
            }
        });

        Ok(quakes_data)
    }
}