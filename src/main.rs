mod network;
mod parse;
use std::time;

fn main() {
    let mut prev_data = String::new();
    let mut player_datas: Vec<network::player_data> = Vec::new();
    loop {
        let mut new_update = false;
        let important_data = parse::read_logs_file(&prev_data);
        prev_data = parse::read_logs_to_string();
        for line in important_data {
            let lt = parse::line_type(&line);
            if lt == 3 {
                player_datas = Vec::new();
                continue;
            }
            let extr_names = parse::extract_player_names(&line, lt);
            //0 - /who
            //1 - joined
            //2 - quit
            //3 - new lobby, reset
            'outer: for name in extr_names.clone() {
                if lt == 2 {
                    for x in 0..player_datas.len() - 1 {
                        if player_datas.get(x).unwrap().name == name {
                            &player_datas.remove(x);
                            new_update = true;
                        }
                    }
                    continue;
                }

                for n in &player_datas {
                    if n.name == name {
                        break 'outer;
                    }
                }
                player_datas.push(network::player_data::new(name));
                new_update = true;
            }
            //println!("[{}] - {:?}", lt, extr_names);
        }

        for player in &mut player_datas {
            if player.has_data {
                continue;
            }
            let name = player.name.clone();
            let bw_stats = network::get_bedwars_stats(&name, "".to_string());
            *player = bw_stats;
        }

        if new_update && player_datas.len() > 0 {
            println!("[==========={}==========]", player_datas.len());
            for pl in &player_datas {
                if !pl.has_data {
                    continue
                }
                println!("[{}*] {}: {}fk {}fd", pl.stars, pl.name, pl.final_kills, pl.final_deaths);
            }
            println!("[=====================]");
        }
        std::thread::sleep(time::Duration::from_secs(1));
    }
}
