//! Helper functions will be moved here

pub mod pattern_re {

    use regex::Regex;
    use lazy_static::lazy_static;

    pub struct Node {
        /* holds various info about a single node */
        //node_name: String,
        pub partition_name: String,
        pub partition_type: String,
        pub instance_type: String,
        pub gpu_type: String,
        pub gpu_num: u8,
    }

    /// Gets the data from output string using RegEx
    pub fn pattern_getter(text: &str) -> Node {       

        lazy_static!{
            static ref RE: Regex = Regex::new(r"(\S*)-(\S*)-(\S*)-(\d{1})\s*gpu:(\D\d*):(\d{1})\(IDX:(.*)\)").unwrap();      
        }
      
    
        let cap = RE.captures(text).unwrap();
        
        let node: Node = Node {
            //node_name: cap.get(1).map_or("unknown".to_string(), |c| c.as_str().to_string()),
            partition_name: cap.get(1).map_or("unknown".to_string(), |c| c.as_str().to_string()),
            partition_type: cap.get(2).map_or("unknown".to_string(), |c| c.as_str().to_string()),
            instance_type: cap.get(3).map_or("unknown".to_string(), |c| c.as_str().to_string()),
            gpu_type: cap.get(5).map_or("unknown".to_string(), |c| c.as_str().to_string()),
            gpu_num: cap.get(6).map_or(0, |c| c.as_str().parse::<u8>().unwrap()),
        };
    
        return node
        
    }
}