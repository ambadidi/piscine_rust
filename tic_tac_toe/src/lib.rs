pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if diagonals("O", &table) || horizontal("O", &table) || vertical("O", &table) { return "player O won".to_string();}
    else if diagonals("X", &table) || horizontal("X", &table) || vertical("X", &table) { return "player X won".to_string();}
    "tie".to_string()
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let symbol = player.to_owned()+player+player;
    if table[0][0].to_owned()+table[1][1]+table[2][2] == symbol ||
    table[0][2].to_owned()+table[1][1]+table[2][0] == symbol {
        return true;
    }
    false
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let symbol = player.to_owned()+player+player;
    if table[0][0].to_owned()+table[0][1]+table[0][2] == symbol || 
    table[1][0].to_owned()+table[1][1]+table[1][2] == symbol || 
    table[2][0].to_owned()+table[2][1]+table[2][2] == symbol {
        return true;
    }
    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    let symbol = player.to_owned()+player+player;
    if table[0][0].to_owned()+table[1][0]+table[2][0] == symbol ||
    table[0][1].to_owned()+table[1][1]+table[2][1] == symbol ||
    table[0][2].to_owned()+table[1][2]+table[2][2] == symbol {
        return true;
    }
    false
}

