use ratatui::{style::Stylize, widgets::Paragraph};

pub const SEPARATOR: &str = concat!("   \n", "▐▓▌\n", "   \n", "▐▓▌\n",);
pub const ZERO: &str = concat!("▐▓▓▓▓▌\n", "▐▓▌▐▓▌\n", "▐▓▌▐▓▌\n", "▐▓▌▐▓▌\n", "▐▓▓▓▓▌\n");
pub const ONE: &str = concat!("▐▓▓▌ \n", " ▐▓▌ \n", " ▐▓▌ \n", " ▐▓▌ \n", "▐▓▓▓▌\n");
pub const TWO: &str = concat!("▐▓▓▓▓▌\n", "   ▐▓▌\n", "▐▓▓▓▓▌\n", "▐▓▌   \n", "▐▓▓▓▓▌\n");
pub const THREE: &str = concat!("▐▓▓▓▓▌\n", "   ▐▓▌\n", "▐▓▓▓▓▌\n", "   ▐▓▌\n", "▐▓▓▓▓▌\n");
pub const FOUR: &str = concat!("▐▓▌▐▓▌\n", "▐▓▌▐▓▌\n", "▐▓▓▓▓▌\n", "   ▐▓▌\n", "   ▐▓▌\n",);
pub const FIVE: &str = concat!("▐▓▓▓▓▌\n", "▐▓▌   \n", "▐▓▓▓▓▌\n", "   ▐▓▌\n", "▐▓▓▓▓▌\n",);
pub const SIX: &str = concat!("▐▓▓▓▓▌\n", "▐▓▌   \n", "▐▓▓▓▓▌\n", "▐▓▌▐▓▌\n", "▐▓▓▓▓▌\n");
pub const SEVEN: &str = concat!("▐▓▓▓▓▌\n", "▐▓▌▐▓▌\n", "   ▐▓▌\n", "   ▐▓▌\n", "   ▐▓▌\n",);
pub const EIGHT: &str = concat!("▐▓▓▓▓▌\n", "▐▓▌▐▓▌\n", "▐▓▓▓▓▌\n", "▐▓▌▐▓▌\n", "▐▓▓▓▓▌\n",);
pub const NINE: &str = concat!("▐▓▓▓▓▌\n", "▐▓▌▐▓▌\n", "▐▓▓▓▓▌\n", "   ▐▓▌\n", "   ▐▓▌\n",);

pub fn convert_to_assci(char: &char) -> Paragraph<'_> {
    let content = match char {
        '0' => ZERO,
        '1' => ONE,
        '2' => TWO,
        '3' => THREE,
        '4' => FOUR,
        '5' => FIVE,
        '6' => SIX,
        '7' => SEVEN,
        '8' => EIGHT,
        '9' => NINE,
        _ => "",
    };
    Paragraph::new(content).yellow().centered()
}
