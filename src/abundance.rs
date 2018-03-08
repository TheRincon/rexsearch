// use rsearch;
use std::string::String;


pub fn get_abundance(header: &str) -> i64 {
        let size_pos = header.find("size=").unwrap_or(1 as usize); // look for "size=" in the header, return index of start
        let semi_or_end = header[size_pos..].find(";").unwrap_or(header[size_pos..].len()); //find ";" or end of str
        return header[(size_pos+5)..(size_pos+semi_or_end)].parse::<i64>().unwrap_or(1 as i64); // return everything after "size=" as an i64 i.e. the "abundance"
}