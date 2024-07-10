pub fn decode_provider_id(resp: &str) -> String {
    let chars = resp.chars().collect::<Vec<char>>();
    let mut output = String::new();
    let mut counter = 0;
    let length = chars.len();
    let (mut x, mut y, mut r, mut n, mut a, mut ox, mut oy);
    while counter < length-1 {
        ox = chars[counter];
        oy = chars[counter+1]; 
        x = hexchar_to_i32(ox); 
        y = hexchar_to_i32(oy); 
        r = (8-y).rem_euclid(16);
        n = 16*x + (r+15).rem_euclid(16) + 1; 
        a = (64-n).rem_euclid(128);
        output.push(char:: from (a as u8)); 
        counter += 2;
    }
    output.replace("clock", "clock.json") 
}

fn hexchar_to_i32(ch: char) -> i32 {
    match ch {
        '0'..='9' => (ch as u8 - b'0') as i32,
        'a'..='f' => (ch as u8 - b'a' + 10) as i32,
        'A'..='F' => (ch as u8 - b'A' + 10) as i32,
        _ => panic!("Invalid hexadecimal character"),
    }
}
