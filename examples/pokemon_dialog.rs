use prediput::select::{Select, SelectOpt};

use colored::*;

const BG_TEXT_COLOR: (u8, u8, u8) = (120, 120, 120);

fn main() {
    #[cfg(target_os = "linux")]
    println!("Detected linux machine");
    
    #[cfg(target_os = "windows")] {
        println!("Detected winodows machine; toggled virtual terminal on");
        colored::control::set_virtual_terminal(true).unwrap();
    }

    let (br, bg, bb) = BG_TEXT_COLOR;

    let prefix = "➤  ".green().bold().to_string();
    let a = "Quick Attack".truecolor(br, bg, bb).to_string();
    let b = "Flamethrower".truecolor(br, bg, bb).to_string();
    let c = "Blizzard".truecolor(br, bg, bb).to_string();
    let d = "Trick Room".truecolor(br, bg, bb).to_string();
    let a_selected = format!(
        "{}{}",
        "Quick Attack".truecolor(255, 255, 255),
        ": Fear me Rattata".truecolor(br, bg, bb)
    );
    let b_selected = format!(
        "{}{}",
        "Flamethrower".truecolor(240, 47, 33),
        ": That's a lot of damage".truecolor(br, bg, bb)
    );
    let c_selected = format!(
        "{}{}",
        "Blizzard".truecolor(33, 225, 239),
        format!(
            ": 10% chance to inflict {}, which basically is a free Pokemon",
            "Freeze".truecolor(33, 225, 239)
        )
        .truecolor(br, bg, bb)
    );
    let d_selected = format!(
        "{}{}",
        "Trick Room".truecolor(240, 33, 232),
        ": Ooga baluga let's go Tortuga".truecolor(br, bg, bb)
    );

    let sel = Select::new(
        &prefix,
        vec![
            SelectOpt::new(&a, Some(&a_selected), "Quick Attack"),
            SelectOpt::new(&b, Some(&b_selected), "Flamethrower"),
            SelectOpt::new(&c, Some(&c_selected), "Blizzard"),
            SelectOpt::new(&d, Some(&d_selected), "Trick Room"),
        ],
    )
    .clear_after()
    .aligned();

    let s = sel.prompt(&"Make a decision:".to_owned()).unwrap();
    println!("You selected {}", s);
}
