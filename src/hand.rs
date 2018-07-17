/* hand.rs
 * Store a hand and calculate its strength
 *
 * (c) Chris Williams, 2018. Open-source software: see LICENSE
 */

use card::Card;
use card::Suit;
use card::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Strength
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

  /* return hand strength */
  pub fn strength(self) -> Strength
  {
    self.strength
  }

  /* count up number of times each suit appears in the hand, and return a list
     of the suits and their counts, sorted by count descending */
  fn count_suits(&mut self) -> Vec<(Suit, usize)>
  {
    let mut map = HashMap::<Suit, usize>::new();

    for card in self.cards.iter()
    {
      let suit = card.suit();
      let count = match map.get(&suit)
      {
        Some(i) => i + 1,
        None    => 1
      };

      map.insert(suit, count);
    }

    /* turn map into sorted vector of totals */
    let mut list = Vec::<(Suit, usize)>::new();
    for (suit, count) in map.iter()
    {
      list.push((*suit, *count));
    }

    list.sort_by(|a, b| b.1.cmp(&a.1));
    return list;
  }

  /* count up number of times each card value appears in the hand, and return a list
     of the values present and their counts, sorted by count descending */
  fn count_values(&mut self) -> Vec<(Value, usize)>
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

    /* turn map into sorted vector of totals */
    let mut list = Vec::<(Value, usize)>::new();
    for (value, count) in map.iter()
    {
      list.push((*value, *count));
    }

    list.sort_by(|a, b| b.1.cmp(&a.1));
    return list;
  }

  /* work out the strength of the cards so far */
  pub fn calc(&mut self)
  {
    /* default to high card */
    self.strength = Strength::HighCard;

    /* sort cards in order, highest to lowest, treating ace as high */
    self.cards.sort_by(|a, b| { b.to_int().cmp(&a.to_int()) });

    /* count up number of instances of each card value in this hand into a
       sorted list - high to low */
    let values = self.count_values();

    /* did we find four of a kind in top slot? */
    if values.first().unwrap().1 == 4
    {
      self.strength = Strength::FourofaKind;
    }

    /* did we find three of a kind? */
    if values.first().unwrap().1 == 3
    {
      /* is the value in the next slot a pair?
         if so, that's a full house */
      if values.len() > 1 && values[1].1 == 2
      {
        self.strength = Strength::FullHouse;
      }
      /* if not, then we have a normal 3 of a kind */
      else
      {
        self.strength = Strength::ThreeofaKind;
      }
    }

    /* did we find a pair? */
    if values.first().unwrap().1 == 2
    {
      /* did we find another pair in next slot? */
      if values.len() > 1 && values[1].1 == 2
      {
        self.strength = Strength::TwoPair;
      }
      /* nope, just one pair */
      else
      {
        self.strength = Strength::Pair;
      }
    }

    /* detect five-card flush */
    let suits = self.count_suits();
    if suits.first().unwrap().1 == 5
    {
      self.strength = Strength::Flush;
    }

    /* detect five-card straight, counting down ace to two (2) */
    let mut prev_value = None;
    let mut straight_count = 0;
    for card in self.cards.iter()
    {
      /* inspect the current card to compare with previous value */
      let value = card.to_int();
      match prev_value
      {
        Some(v) =>
        {
          /* if it's a consecutively lower number then increase the count */
          if value == (v - 1)
          {
            straight_count = straight_count + 1;
          }
          else
          {
            /* ..or start over, with this being the new first card in the straight */
            straight_count = 1;
          }

          /* if the fourth card in a straight is a two (2) and the highest card
             is an ace, then this is a wheel (low ace to five) so deal with this edge case */
          if straight_count == 4 && card.value() == Value::Two &&
             self.cards[0].value() == Value::Ace
          {
            straight_count = straight_count + 1;
          }

          /* stop right now if we've found five straight cards */
          if straight_count == 5
          {
            break;
          }
        },

        None =>
        {
          straight_count = straight_count + 1;
        }
      }

      prev_value = Some(value);
    }

    /* handle straight, straight flushes, and royals */
    if straight_count >= 5
    {
      if self.strength == Strength::Flush
      {
        if self.cards.first().unwrap().value() == Value::Ace
        {
          self.strength = Strength::RoyalFlush;
        }
        else
        {
          self.strength = Strength::StraightFlush;
        }
      }
      else
      {
        self.strength = Strength::Straight;
      }
    }
  }
}
