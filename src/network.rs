use std::fmt;
use ureq;

pub struct player_data {
    pub name: String,
    pub has_data: bool,
    pub final_kills: i64,
    pub final_deaths: i64,
    pub stars: i64
}

impl fmt::Debug for player_data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("player_data")
            .field("n", &self.name)
            .field("hd", &self.has_data)
            .field("fk", &self.final_kills)
            .field("fd", &self.final_deaths)
            .finish()
    }
}

impl player_data {
    pub fn new(constr_name: String) -> Self {
        Self {
            name: constr_name,
            has_data: false,
            final_kills: 0,
            final_deaths: 0,
            stars: 0
        }
    }
}

pub fn get_bedwars_stats(player_name: &str, api_key: String) -> player_data {
    let api_loc = format!(
        "https://api.hypixel.net/player?key={}&name={}",
        api_key, player_name
    );
    let res: serde_json::Value = ureq::get(&api_loc).call().unwrap().into_json().unwrap();

    if res["success"].as_bool().unwrap() == false {
        println!("failed getting stats for {}", player_name);
        return player_data::new(player_name.to_string());
    }

    if res["player"].as_object().is_none() {
        println!("failed getting stats for {}, possible nick?", player_name);
        return player_data::new(player_name.to_string());
    }

    let mut data = player_data::new(player_name.to_string());

    data.final_kills = res["player"]["stats"]["Bedwars"]["final_kills_bedwars"]
        .as_i64()
        .unwrap();
    data.final_deaths = res["player"]["stats"]["Bedwars"]["final_deaths_bedwars"]
        .as_i64()
        .unwrap();

    let mut stars = 1;
    let mut xp = res["player"]["stats"]["Bedwars"]["Experience"].as_i64().unwrap();
    //this is incorrect and a dumpsterfire anyways lol
    if xp > 1000 {
        xp -= 1000;
        stars += 1;
        if xp > 2000 {
            xp -= 2000;
            stars += 1;
            if xp > 3000 {
                xp -= 3000;
                stars += 1;
                if xp > 3500 {
                    xp -= 3500;
                    stars += 1;
                    if xp > 5000 {
                        stars += xp / 5000;
                    }
                }
            }
        }
    }

    data.stars = stars;
    data.has_data = true;

    data
}
