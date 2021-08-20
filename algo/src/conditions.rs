pub fn verify() {
    let rank: u8 = 5;
    let shinobi: bool = false;
    let style: String = String::from("Fire");

    if rank < 5  && shinobi{
        println!("Top Level Shinobi! {} Style Jutsu User", style);
    }
    else if rank <= 5  && shinobi {
        println!("Low level Shinobi. Style: {} Style Jutsu User", style);
    }
    else {
        println!("Becoming A Ninja Requires Conviction! Go Get Some...");
    }

    let eligible_for_missions = if shinobi {true} else {false};
    println!("Is Eligible for Missions: {}", eligible_for_missions);
}