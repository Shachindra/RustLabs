pub fn lists() {
    let asuma = Style::Wind;
    let bee = Style::Lightning;
    let naruto = Style::Light;
    let sasuke = Style::Shadow;

    attack(asuma);
    attack(bee);
    attack(naruto);
    attack(sasuke);
}

enum Style {
    // Styles
    Fire,
    Lightning,
    Wind,
    Earth,
    Water,
    Light,
    Shadow
}

fn attack(a: Style) {
    // Match style 
    match a {
        Style::Fire => println!("Fireball"),
        Style::Lightning => println!("CloudLightning"),
        Style::Wind => println!("Tornado"),
        Style::Earth => println!("Earth Style"),
        Style::Water => println!("Water Style"),
        Style::Light => println!("BlazingBomb"),
        Style::Shadow => println!("Inferno"),
    }
}