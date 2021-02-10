use std::fs;

pub fn read_logs_to_string() -> String {
    fs::read_to_string("C:\\Users\\Null\\.lunarclient\\logs\\launcher\\renderer.log")
        .expect("failed reading log file")
}

pub fn read_logs_file(prev_data: &str) -> Vec<String> {
    //hardcoded path change later
    let data = read_logs_to_string().replace(prev_data, "");
    let lines = data.split_terminator("\n");
    const IMPORTANT_MESSAGES: [&str; 5] = ["has joined", "has quit!", "ONLINE: ", "Sending you to", "joined the lobby"];
    let mut data = Vec::new();
    for line in lines {
        let mut avail = false;
        for msg in &IMPORTANT_MESSAGES {
            if line.contains(msg) {
                avail = true;
                break;
            }
        }
        if !avail {
            continue;
        }
        let chat_idx = line.find("[CHAT]").unwrap() + 7;
        let new_line = &line[chat_idx..];
        data.push(new_line.to_string());
    }

    data
}

//0 - /who
//1 - joined
//2 - quit
//3 - new lobby, reset
pub fn line_type(line: &str) -> u8 {
    if line.contains("ONLINE: ") {
        return 0;
    } else if line.contains("has joined") {
        return 1;
    } else if line.contains("quit!") {
        return 2;
    } else if line.contains("Sending you to") || line.contains("joined the lobby") {
        return 3;
    }

    return 8;
}

pub fn extract_player_names(line: &str, ltype: u8) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();

    if ltype == 0 {
        let line_len = line.len() - 1;
        for n in line[8..line_len].split(", ") {
            names.push(n.to_string());
        }
    } else if ltype == 1 || ltype == 2 {
        let has_idx = line.find(" has").unwrap();
        names.push(line[0..has_idx].to_string());
    }

    names
}
