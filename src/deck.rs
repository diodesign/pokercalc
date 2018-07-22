/* deck.rs
 * Define a Texas Hold 'em poker deck of cards as an object
 *
 * (c) Chris Williams, 2018. Open-source software: see LICENSE
 */

 use card::Card;
 use card::Suit;
 use card::Value;

pub struct Deck
{
  cards: Vec<Card>
}

impl Deck
{
  /* create a full deck of cards. Rust won't easily iterate over
     an enum so we have to do it this way... */
  pub fn new() -> Deck
  {
    /* start off with an empty deck */
    let mut deck = Deck { cards: Vec::<Card>::new() };

    /* run through the suits */
    for suit_loop in 0..4
    {
      let suit = match suit_loop
      {
        0 => Suit::Heart,
        1 => Suit::Diamond,
        2 => Suit::Club,
        3 => Suit::Spade,
        _ => unreachable!()
      };

      /* for each suit, go through each card value */
      for value_loop in 2..15
      {
        let value = match value_loop
        {
          2 => Value::Two,
          3 => Value::Three,
          4 => Value::Four,
          5 => Value::Five,
          6 => Value::Six,
          7 => Value::Seven,
          8 => Value::Eight,
          9 => Value::Nine,
          10 => Value::Ten,
          11 => Value::Jack,
          12 => Value::Queen,
          13 => Value::King,
          14 => Value::Ace,
          _ => unreachable!()
        };

        deck.cards.push(Card { value: value, suit: suit });
      }
    }

    return deck;
  }

  pub fn remove(&mut self, victim: &Card)
  {
    let mut i = 0;
    let mut found = false;

    for card in self.cards.iter()
    {
      if card.suit == victim.suit && card.value == victim.value
      {
        found = true;
        break;
      }

      i = i + 1;
    }

    if found == true
    {
      self.cards.remove(i);
    }
  }

  pub fn cards(&mut self) -> &mut Vec<Card>
  {
    &mut self.cards
  }
}
