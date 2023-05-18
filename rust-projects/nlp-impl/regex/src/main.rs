use regex::Regex;

fn main() {
    let text = "10.50.8.124 - - [19/Feb/2022:21:00:21 -0300] \"GET /my-server/logout HTTP/1.1\" 200 1592";

    let re = Regex::new(r#"(\S*).*\[(.*)\]\s"(\S*)\s(\S*)\s([^"]*)"\s(\S*)\s(\S*)"#).unwrap();
    
    for cap in re.captures_iter(text) {
        println!("ip: {}", &cap[1]);
        println!("date: {}", &cap[2]);
        println!("method: {}", &cap[3]);
        println!("route: {}", &cap[4], );
        println!("protocol: {}",  &cap[5], );
        println!("response: {}",  &cap[6], );
        println!("response_time: {}",&cap[7]);
    }
}
