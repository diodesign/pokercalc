/* hand.rs
 * Store a hand and calculate its strength
 *
 * (c) Chris Williams, 2018. Open-source software: see LICENSE
 */

use card::Card;
use card::Ace;

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

  /* work out the strength of the cards so far */
  pub fn rank(&mut self)
  {
    /* first sort in order, highest to lowest, treating ace as high */
    self.cards.sort_by(|a, b|
    {
      b.to_int(Ace::High).cmp(&a.to_int(Ace::High))
    });

    for c in self.cards.iter()
    {
      print!("{} ", c.describe());
      println!("");
    }
  }
}
