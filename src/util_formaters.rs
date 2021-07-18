use ansi_term::Color::{Green, Red, Yellow};

pub struct Formaters {}

impl Formaters {
    pub fn seperator(margin: bool) {
        let mut divider = "-----------------------------------";
        if margin {
            divider = "\n-----------------------------------\n";
        }
        println!("{}", Green.bold().paint(divider));
    }

    pub fn title(title: &str) {
        println!(
            "\n{}",
            Red.bold().paint(format!(
                "---------------------{}---------------------",
                title
            ))
        );
    }

    pub fn subtitle(subtitle: &str) {
        println!("\n{}", Red.bold().paint(subtitle));
    }

    pub fn text(text: &str) {
        println!("\n{}", Yellow.paint(text))
    }

    pub fn appendix(titles: &[&str]) {
        Formaters::seperator(true);
        for (order, title) in titles.iter().enumerate() {
            println!(
                "{}. {}",
                Yellow.paint(order.to_string()),
                Yellow.paint(*title)
            )
        }
        Formaters::seperator(true);
    }
}
