//!

use std::process::Command;
use regex::Regex;
use lazy_static::lazy_static;
use prometheus_exporter::prometheus::register_gauge;
use std::net::SocketAddr;

struct Node {
    /* holds various info about a single node */
    gpu_type: String,
    gpu_num: u8,
}

fn main() {       
    
    // parse the address used to bind exporter to
    let addr_raw = "127.0.0.1:9199";
    let addr: SocketAddr = addr_raw.parse().expect("couldn't parse listening address");

    // start exporter and update metrics every second
    let exporter = prometheus_exporter::start(addr).expect("couldn't start the exporter");
    let duration = std::time::Duration::from_millis(1000);

    // create metric ()
 
    let metric_a100s_used = register_gauge!("node_used_a100s", "number of a100 GPUs used").expect("couldn't create gauge");
    let metric_a100s_total = register_gauge!("node_total_a100s", "number of a100 GPUs available").expect("couldn't create gauge");
    let metric_t4s_used = register_gauge!("node_used_t4s", "number of t4 GPUs used").expect("couldn't create gauge");
    let metric_t4s_total = register_gauge!("node_total_t4s", "number of t4 GPUs available").expect("couldn't create gauge");
    

    loop {
        {
            // will block until duration is elapsed
            let _guard = exporter.wait_duration(duration);

            println!("updating metrics");

            // update metrics with new values
            
            // get info from the external program (fakesinfo in this case)
            let sinfo = Command::new("fakesinfo").output().expect("reading sinfo didn't work");

            let sinfo_output = String::from_utf8(sinfo.stdout).expect("retrieving stdout from sinfo output didn't work");

            // want to collect numbers of used and total GPUs from all the nodes
            let mut a100s_used: u8 = 0;
            let mut a100s_total: u8 = 0;
            let mut t4s_used: u8 = 0;
            let mut t4s_total: u8 = 0;

            // count the GPUs in the nodes
            for line in sinfo_output.lines().skip(1) {

                let node = pattern_getter(line);

                match node.gpu_type.as_str() {
                    "a100" => {
                        a100s_used += node.gpu_num;
                        a100s_total += 8; // a100 nodes have 8 gpus each
                    }
                    "t4" => {
                        t4s_used += node.gpu_num;
                        t4s_total += 4; // t4 nodes have 4 gpus each
                    }
                    _ => ()
                }

                // just for debugging
                println!("original line:\t{}", line); 

            }

            println!("a100s: {}/{}, t4s: {}/{}", a100s_used, a100s_total, t4s_used, t4s_total);

            metric_a100s_used.set(a100s_used as f64);
            metric_a100s_total.set(a100s_total as f64);
            metric_t4s_used.set(t4s_used as f64);
            metric_t4s_total.set(t4s_total as f64);


        }
    }


}


fn pattern_getter(text: &str) -> Node {

    lazy_static!{
        static ref RE: Regex = Regex::new(r"gpu:(\D\d*):(\d{1})\(IDX:(.*)\)").unwrap();      
    }
  

    let cap = RE.captures(text).unwrap();
    
    let node: Node = Node {
        gpu_type: cap.get(1).map_or("unknown".to_string(), |c| c.as_str().to_string()),
        gpu_num: cap.get(2).map_or(0, |c| c.as_str().parse::<u8>().unwrap()),
    };

    return node
    
}