/* hand.rs
 * Store a hand and calculate its strength
 *
 * (c) Chris Williams, 2018. Open-source software: see LICENSE
 */

use card::Card;
use card::Suit;
use card::Value;
use card::Ace;
use std::collections::HashMap;

enum Strength
{
  RoyalFlush,
  StraightFlush,
  FourofaKind,
  FullHouse,
  Flush,
  Straight,
  ThreeofaKind,
  TwoPair,
  Pair,
  HighCard,
  Empty
}

pub struct Hand
{
  cards: Vec<Card>,
  score: u64,
  strength: Strength
}

impl Hand
{
  /* create a new empty hand */
  pub fn new() -> Hand
  {
    Hand
    {
      cards: Vec::<Card>::new(),
      score: 0,
      strength: Strength::Empty
    }
  }

  /* add a new card to the hand */
  pub fn add(&mut self, card: Card)
  {
    self.cards.push(card);
  }

  /* return the numbber of cards in the hand with the given suit */
  fn count_suit(&mut self, suit: Suit) -> usize
  {
    self.cards.iter().filter(|a|
    {
      a.suit() == suit
    }).count()
  }

  /* count up number of times each card value appears in the hand, and return the totals
     as a hash table */
  fn count_values(&mut self) -> HashMap<Value, usize>
  {
    let mut map = HashMap::<Value, usize>::new();

    for card in self.cards.iter()
    {
      let value = card.value();
      let count = match map.get(&value)
      {
        Some(i) => i + 1,
        None    => 1
      };

      map.insert(value, count);
    }

    return map;
  }

  /* work out the strength of the cards so far */
  pub fn rank(&mut self)
  {
    let mut flush = false;
    let mut straight = false;
    let mut trips = false;

    /* count up number of instances of each card value */
    let mut values = self.count_values();
    for (key, val) in values.iter()
    {
      println!("key {:?} value {}", key, val);
    }

    /* detect flush: count up number of cards of each suit, and make a note if we
       hit a five-card flush */
    if self.count_suit(Suit::Heart)   == 5 ||
       self.count_suit(Suit::Diamond) == 5 ||
       self.count_suit(Suit::Club)    == 5 ||
       self.count_suit(Suit::Spade)   == 5
    {
      flush == true;
    }

    /* sort in order, highest to lowest, treating ace as high */
    self.cards.sort_by(|a, b|
    {
      b.to_int(Ace::Low).cmp(&a.to_int(Ace::Low))
    });
  }
}
