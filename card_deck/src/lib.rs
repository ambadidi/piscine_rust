use rand::Rng;
#[derive(Debug, PartialEq, Eq)]
pub enum Suit {
	Heart,
	Diamond,
	Spade,
	Club,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Rank {
	Ace,
	King,
	Queen,
	Jack,
	Number(u8),
}

impl Suit {
	pub fn random() -> Suit {
		let number = rand::thread_rng().gen_range(1,5);
		Suit::translate(number)
	}

	pub fn translate(value: u8) -> Suit {
		match value {
			1 => Suit::Heart,
			2 => Suit::Diamond,
			3 => Suit::Spade,
			4 => Suit::Club,
            _ => Suit::Spade,
		}
	}
}

impl Rank {
	pub fn random() -> Rank {
		let number = rand::thread_rng().gen_range(1,14);
		Rank::translate(number)
	}

	pub fn translate(value: u8) -> Rank {
		match value {
			1 => Rank::Ace,
			2 => Rank::Number(2),
			3 => Rank::Number(3),
			4 => Rank::Number(4),
			5 => Rank::Number(5),
			6 => Rank::Number(6),
			7 => Rank::Number(7),
			8 => Rank::Number(8),
			9 => Rank::Number(9),
			10 => Rank::Number(10),
			11 => Rank::Jack,
			12 => Rank::Queen,
			13 => Rank::King,
            _ => Rank::Ace,
		}
	}
}
#[derive(Debug, PartialEq, Eq)]
pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
	if card.suit == Suit::Spade && card.rank == Rank::Ace { true} 
	else { false}
}