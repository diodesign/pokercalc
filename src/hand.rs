/* hand.rs
 * Store a hand and calculate its strength
 *
 * (c) Chris Williams, 2018. Open-source software: see LICENSE
 */

use card::Card;
use card::Suit;
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

  /* return the numbber of cards in the hand with the given suit */
  fn count_suit(&mut self, suit: Suit) -> usize
  {
    self.cards.iter().filter(|a|
    {
      a.suit() == suit
    }).count()
  }

  /* work out the strength of the cards so far */
  pub fn rank(&mut self)
  {
    /* detect flush: count up number of cards of each suit, and record highest number */
    let hearts = self.count_suit(Suit::Heart);
    let diamonds = self.count_suit(Suit::Diamond);
    let clubs = self.count_suit(Suit::Club);
    let spades = self.count_suit(Suit::Spade);

    println!("{} hearts {} diamonds {} clubs {} spades", hearts, diamonds, clubs, spades);

    /* detect straight: sort in order, highest to lowest, treating ace as high */
    self.cards.sort_by(|a, b|
    {
      b.to_int(Ace::High).cmp(&a.to_int(Ace::High))
    });
  }
}
