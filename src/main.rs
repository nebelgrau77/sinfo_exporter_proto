use std::process::Command;
use regex::Regex;
use lazy_static::lazy_static;

struct Node {
    gpu_type: String,
    gpu_num: u8,
}

fn main() {
    
    let sinfo = Command::new("fakesinfo").output().expect("reading sinfo didn't work");

    let sinfo_output = String::from_utf8(sinfo.stdout).expect("retrieving stdout from sinfo output didn't work");

    for (n, line) in sinfo_output.lines().enumerate() {

        if n == 0 {
            println!("this is the data:");
        } else {

            let node = pattern_getter(line);
            println!("line: {}\tnode has {} GPUs of type {}", line, node.gpu_num, node.gpu_type);
        }


        // println!("line number {} is {}", n, line);

    }   

    //let node = pattern_getter(&sinfo_output.as_str());

    //println!("fake node has {} GPUs of type {}", node.gpu_num, node.gpu_type);


}


fn pattern_getter(text: &str) -> Node {

    lazy_static!{
        static ref RE: Regex = Regex::new(r"gpu:(\D\d*):(\d{1})\(IDX:(.*)\)").unwrap();      
    }
    
    /* this works

    let mut node = Node {
        gpu_type: "unknown".to_string(),
        gpu_num: 0,
    };

    for cap in RE.captures_iter(text) {                       
        node.gpu_type = cap[1].to_string();
        node.gpu_num = cap[2].parse::<u8>().unwrap();                
    };
    
    return node

    */

    let cap = RE.captures(text).unwrap();
    
    let node: Node = Node {
        gpu_type: cap.get(1).map_or("unknown".to_string(), |c| c.as_str().to_string()),
        gpu_num: cap.get(2).map_or(0, |c| c.as_str().parse::<u8>().unwrap()),
    };

    return node

    /*
    let node_data = RE.captures_iter(text).filter_map(|cap| {
        let groups = (cap.get(1), cap.get(2));
    });

     */

    
}