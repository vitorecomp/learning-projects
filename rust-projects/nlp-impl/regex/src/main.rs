use regex::Regex;

fn main() {
    let text = "10.50.8.124 - - [19/Feb/2022:21:00:21 -0300] \"GET /my-server/logout HTTP/1.1\" 200 1592";

    let re = Regex::new(r#"(\S*).*\[(.*)\]\s"(\S*)\s(\S*)\s([^"]*)"\s(\S*)\s(\S*)"#).unwrap();
    
    for cap in re.captures_iter(text) {
        println!("ip: {} date: {} method: {}", &cap[1], &cap[2], &cap[3]);
        println!("route: {} protocol: {} response: {} response_time: {}", &cap[4], &cap[5], &cap[6], &cap[7]);
    }
}
