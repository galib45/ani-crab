fn main() {
    let resp = "175948514e4c4f57175b54575b5307515c050f5c0a0c0f0b0f0c0e590a0c0b5b0a0c0a010e5a0e0b0e0a0e5e0e0f0b090a010f080e5e0e0a0e0b0e010f0d0a010d0f0f090e5e0b0a0e5d0c5a0b0d0f0f0f5b0e0d0f090f080b5e0d0c0e0c0e0f0e0f0a010f0d0f0b0e0c0a010b0f0a0c0a590a0c0f0d0f0a0f0c0e0b0e0f0e5a0e0b0f0c0c5e0e0a0a0c0b5b0a0c0c0a0f0c0e010f0e0e0c0e010f5d0a0c0a590a0c0e0a0e0f0f0a0e0b0a0c0b5b0a0c0b0c0b0e0b0c0b0a0a5a0b0e0b090a5a0b0e0b5e0d0a0b0e0b090b5b0b0b0b0f0b5b0b0e0b0e0a000b0e0b0e0b0e0d5b0a0c0a590a0c0f0a0f0c0e0f0e000f0d0e590e0f0f0a0e5e0e010e000d0a0f5e0f0e0e0b0a0c0b5b0a0c0f0d0f0b0e0c0a0c0a590a0c0e5c0e0b0f5e0a0c0b5b0a0c0e0b0f0e0a5a0a010e5a0e0b0e0a0e5e0e0f0b090a010f080e5e0e0a0e0b0e010f0d0a010d0f0f090e5e0b0a0e5d0c5a0b0d0f0f0f5b0e0d0f090f080b5e0d0c0e0c0e0f0e0f0a010f0d0f0b0e0c0a010b0f0a0c0f5a".to_string();
    let chars = resp.chars().collect::<Vec<char>>();
    let chunks: Vec<_> = chars.chunks(2)
        .map(|c| c.iter().collect::<String>())
        .collect();
    let mut output = String::new();
    for chunk in chunks {
        let ch = match chunk.as_str() {
            "01" => '9',
            "08" => '0',
            "05" => '=',
            "0a" => '2',
            "0b" => '3',
            "0c" => '4',
            "07" => '?',
            "00" => '8',
            "5c" => 'd',
            "0f" => '7',
            "5e" => 'f',
            "17" => '/',
            "54" => 'l',
            "09" => '1',
            "48" => 'p',
            "4f" => 'w',
            "0e" => '6',
            "5b" => 'c',
            "5d" => 'e',
            "0d" => '5',
            "53" => 'k',
            "1e" => '&',
            "5a" => 'b',
            "59" => 'a',
            "4a" => 'r',
            "4c" => 't',
            "4e" => 'v',
            "57" => 'o',
            "51" => 'i',
            &_ => todo!(),
        };
        output.push(ch);
    }
    output = output.replace("clock", "clock.json");
    println!("{output}")
}
