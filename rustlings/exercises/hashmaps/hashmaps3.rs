// hashmaps3.rs

// A list of scores (one per line) of a soccer match is given. Each line
// is of the form :
// <team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
// Example: England,France,4,2 (England scored 4 goals, France 2).

// You have to build a scores table containing the name of the team, goals
// the team scored, and goals the team conceded. One approach to build
// the scores table is to use a Hashmap. The solution is partially
// written to use a Hashmap, complete it to pass the test.

// Make me pass the tests!

// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a hint.

use std::collections::HashMap;

// A structure to store team name and its goal details.
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn	ft_fill_team(team_name:&String, team_score:u8, adverse_score:u8, scores:&mut HashMap<String, Team>) {
	if scores.contains_key(team_name) {
		let old_team = scores.get(team_name).unwrap();
		let mut score = old_team.goals_scored;
		let mut conceded = old_team.goals_conceded;
		score += team_score;
		conceded += adverse_score;
		let temp = Team {
			name: team_name.clone(),
			goals_conceded: conceded,
			goals_scored: score,
		};
		scores.insert((*team_name).clone(), temp);
	}
	else if !scores.contains_key(team_name) {
		let temp = Team {
			name: team_name.clone(),
			goals_conceded: adverse_score,
			goals_scored: team_score,
		};
		scores.insert((*team_name).clone(), temp);
	}
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

	println!{"[{}]", results};
    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();

        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
		ft_fill_team(& team_1_name, team_1_score, team_2_score, &mut scores);
		ft_fill_team(&team_2_name, team_2_score, team_1_score, &mut scores);
	}

    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
