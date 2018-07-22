/* card.rs
 * Define a Texas Hold 'em poker card as an object
 *
 * (c) Chris Williams, 2018. Open-source software: see LICENSE
 */

/* abstracted representation of the value of a card */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Value
{
  Two, Three, Four, Five, Six, Seven, Eight, Nine,
  Ten, Jack, Queen, King, Ace
}

impl Value
{
  /* convert internal card value into a string
     <= string holding card value */
  pub fn to_str(self) -> &'static str
  {
    match self
    {
      Value::Two   => "two",
      Value::Three => "three",
      Value::Four  => "four",
      Value::Five  => "five",
      Value::Six   => "six",
      Value::Seven => "seven",
      Value::Eight => "eight",
      Value::Nine  => "nine",
      Value::Ten   => "ten",
      Value::Jack  => "jack",
      Value::Queen => "queen",
      Value::King  => "king",
      Value::Ace   => "ace"
    }
  }

  /* convert internal card value into a text character
     <= text character representing card value */
  pub fn to_char(self) -> char
  {
    match self
    {
      Value::Two   => '2',
      Value::Three => '3',
      Value::Four  => '4',
      Value::Five  => '5',
      Value::Six   => '6',
      Value::Seven => '7',
      Value::Eight => '8',
      Value::Nine  => '9',
      Value::Ten   => 'T',
      Value::Jack  => 'J',
      Value::Queen => 'Q',
      Value::King  => 'K',
      Value::Ace   => 'A'
    }
  }

  /* convert internal card value into an unsigned integer
     <= integer between 2 (two) and 14 (ace) */
  pub fn to_u32(self) -> u32
  {
    match self
    {
      Value::Two   => 2,
      Value::Three => 3,
      Value::Four  => 4,
      Value::Five  => 5,
      Value::Six   => 6,
      Value::Seven => 7,
      Value::Eight => 8,
      Value::Nine  => 9,
      Value::Ten   => 10,
      Value::Jack  => 11,
      Value::Queen => 12,
      Value::King  => 13,
      Value::Ace   => 14
    }
  }
}

/* define the suit of the card */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit
{
  Heart,
  Diamond,
  Club,
  Spade
}

impl Suit
{
  /* convert card suit into a text character
     <= text character representing card suit */
  pub fn to_char(self) -> char
  {
     match self
     {
       Suit::Heart   => '\u{2665}',
       Suit::Diamond => '\u{2666}',
       Suit::Club    => '\u{2663}',
       Suit::Spade   => '\u{2660}',
     }
   }
}

/* the card object */
#[derive(Debug, Clone, Copy)]
pub struct Card
{
  pub value: Value, /* abstracted representation of the value */
  pub suit: Suit    /* suit of the card */
}

impl Card
{
  /* new
     create new card from card description
     => desc = lowercase syntax string defining this card (2-9tjqka)(hdcs)
     <= Card object or None if error
  */
  pub fn new(desc: &str) -> Option<Card>
  {
    let value = match desc.chars().nth(0)
    {
      Some(c) => match c
      {
        '2' => Value::Two,
        '3' => Value::Three,
        '4' => Value::Four,
        '5' => Value::Five,
        '6' => Value::Six,
        '7' => Value::Seven,
        '8' => Value::Eight,
        '9' => Value::Nine,
        't' => Value::Ten,
        'j' => Value::Jack,
        'q' => Value::Queen,
        'k' => Value::King,
        'a' => Value::Ace,
        _ =>
        {
          println!("Invalid value '{}' in card '{}'", c, desc);
          return None; /* bail out due to invalid value */
        }
      }
      None => return None /* bail out due to empty string */
    };

    let suit = match desc.chars().nth(1)
    {
      Some(c) => match c
      {
        'h' => Suit::Heart,
        'd' => Suit::Diamond,
        'c' => Suit::Club,
        's' => Suit::Spade,
        _ =>
        {
          println!("Invalid suit '{}' in card '{}'", c, desc);
          return None; /* bail out due to invalid suit */
        }
      }
      None =>
      {
        println!("Invalid card '{}'", desc);
        return None /* bail out due to invalid string */
      }
    };

    let card = Card
    {
      value: value,
      suit: suit
    };

    return Some(card);
  }

  /* describe
     Return a string describing the card */
  pub fn describe(&self) -> String
  {
    let mut desc = String::new();
    desc.push(self.value.to_char());
    desc.push(self.suit.to_char());
    return desc;
  }

  /* return the suit of the card */
  pub fn suit(&self) -> Suit
  {
    self.suit
  }

  /* return the value of the card */
  pub fn value(&self) -> Value
  {
    self.value
  }
}
