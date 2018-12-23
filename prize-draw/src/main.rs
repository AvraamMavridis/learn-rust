use std::cmp::Ordering;

#[derive(Debug)]
struct Player {
    name: String,
    weight: i32,
}

impl Player {
    fn rank(&self) -> u32 {
        return (self.name.to_lowercase().chars().fold(0, |c, n: char| {
            return c + (n as u32) - 96;
        }) + (self.name.len() as u32)) * (self.weight as u32);
    }

    fn cmp(&self, other: &Player) -> Ordering {
        if self.rank() == other.rank() {
            return other.name.cmp(&self.name);
        }

        return self.rank().cmp(&other.rank());
    }
}

fn rank(st: &str, we: Vec<i32>, n: usize) -> String {
    if st.len() == 0 {
        return String::from("No participants");
    }

    let v: Vec<&str> = st.split(',').collect();

    if n > v.len() {
        return String::from("Not enough participants");
    }
    
    let mut players: Vec<Player> = Vec::new();

    for (index, value) in v.iter().enumerate() {
        let player = Player { name: String::from(value.clone()), weight: we[index] };
        players.push(player);
    }

    players.sort_by(|player1: &Player, player2: &Player| player1.cmp(&player2));
    players.reverse();
    
    String::from(players[n - 1].name.clone())
}



#[cfg(test)]
mod tests {
    fn testing(st: &str, we: Vec<i32>, n: usize, exp: &str) -> () {
        assert_eq!(super::rank(st, we, n), exp)
    }


    #[test]
    fn test1() {
        testing("Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin", vec![4, 2, 1, 4, 3, 1, 2], 4, "Benjamin");
    }

    #[test]
    fn test2() {
        testing("Elijah,Chloe,Elizabeth,Matthew,Natalie,Jayden", vec![1, 3, 5, 5, 3, 6], 2, "Matthew");
    }

    #[test]
    fn test3() {
        testing("Lagon,Lily", vec![1, 5], 2, "Lagon");
    }

    #[test]
    fn test4() {
        testing("Aubrey,Olivai,Abigail,Chloe,Andrew,Elizabeth", vec![3, 1, 4, 4, 3, 2], 4, "Abigail");
    }

    #[test]
    fn test5() {
        testing("", vec![3, 1, 4, 4, 3, 2], 4, "No participants");
    }

    #[test]
    fn test6() {
        testing("Some,Other", vec![3, 1, 4, 4, 3, 2], 4, "Not enough participants");
    }
}

fn main() {
     
}
