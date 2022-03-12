use swf_parser::parse_swf;
use swf_types::Movie;

mod process_scml;
// steps:
// read from file
// parse all data
// create an xml object from data
// create all pngs from data
// write xml data to file

fn main() {
    read_swf();
}

// read the swf file
fn read_swf(){
    let swf_bytes: Vec<u8> = ::std::fs::read("yellowmouse.swf").expect("Failed to read movie");
    let movie: Movie = parse_swf(&swf_bytes).expect("Failed to parse SWF");  
    process_scml::process_swf(movie);
}

