//!

use std::process::Command;
use std::net::SocketAddr;
use prometheus_exporter::prometheus::{register_int_gauge};
use clap::{Arg, arg, command, value_parser, builder::Str};

use exporter_proto::pattern_re;


fn main() {       
    
        //App::new("GPU sinfo Prometheus exporter")
    let matches = command!()    
        .version("0.1.0")
        .author("Michal")
        .about("Prometheus exporter for GPU number from SLURM sinfo")   
        .arg(Arg::new("PORT")
                    .short('p')
                    .long("port")              
                    .value_parser(value_parser!(u16))      
                    .default_value("9199")
                    .help("Which port Prometheus should listen to"))
        .arg(Arg::new("INTERVAL")
                    .short('i')
                    .long("interval")              
                    .value_parser(value_parser!(u16))      
                    .default_value("5000")
                    .help("How often the metrics should be updated, in miliseconds"))
        .get_matches();

    let port = matches.get_one::<u16>("PORT").unwrap();
    
    let addr_raw = format!("127.0.0.1:{}", port);
    let addr: SocketAddr = addr_raw.parse().expect("couldn't parse listening address");

    // start exporter and update metrics every second
    let exporter = prometheus_exporter::start(addr).expect("couldn't start the exporter");

    let interval = matches.get_one::<u16>("INTERVAL").unwrap();

    let duration = std::time::Duration::from_millis(*interval as u64);

    // create metrics 
    let metric_a100s_used = register_int_gauge!("node_used_a100s", "number of a100 GPUs used").expect("couldn't create gauge");
    let metric_a100s_total = register_int_gauge!("node_total_a100s", "number of a100 GPUs available").expect("couldn't create gauge");
    
    let metric_t4s_used = register_int_gauge!("node_used_t4s", "number of t4 GPUs used").expect("couldn't create counter");
    let metric_t4s_total = register_int_gauge!("node_total_t4s", "number of t4 GPUs available").expect("couldn't create counter");
        
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
            for line in sinfo_output.lines() {

                let node = pattern_re::pattern_getter(line);

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
                println!("original line:\t{}, partition name: {}, partition type: {}, instance_type: {}", line, node.partition_name, node.partition_type, node.instance_type); 

            }

            println!("a100s: {}/{}, t4s: {}/{}", a100s_used, a100s_total, t4s_used, t4s_total);

            metric_a100s_used.set(a100s_used as i64);
            metric_a100s_total.set(a100s_total as i64);            
            metric_t4s_used.set(t4s_used as i64);
            metric_t4s_total.set(t4s_total as i64);



        }
    }


}


