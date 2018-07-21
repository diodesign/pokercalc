/* hand.rs
 * Store a hand and calculate its strength
 *
 * (c) Chris Williams, 2018. Open-source software: see LICENSE
 */

use card::Card;
use card::Suit;
use card::Value;

use std::collections::HashMap;
use std::cmp::Ordering;

/* describe number of cards with the same particular value are in the hand */
struct ValueTotal
{
  value: Value,
  total: usize
}

/* describe number of cards with the same particular suit are in the hand */
struct SuitTotal
{
  total: usize
}

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
  cards: Vec<Card>,    /* all the cards in this hand (2 to 7) */
  best: Vec<Value>,    /* all the relevant cards (2-5) to calculate the score */
  score: u32,          /* final score from best 5 cards */
  strength: Strength   /* type of hand from best 5 cards */
}

impl Hand
{
  /* create a new empty hand */
  pub fn new() -> Hand
  {
    Hand
    {
      cards: Vec::<Card>::new(),
      best: Vec::<Value>::new(),
      score: 0,
      strength: Strength::Empty
    }
  }

  /* add a new card to the hand */
  pub fn add(&mut self, card: Card)
  {
    self.cards.push(card);
  }

  /* return string describing the hard */
  pub fn describe(&self) -> String
  {
    match self.strength
    {
      Strength::RoyalFlush    => format!("royal flush"),
      Strength::StraightFlush => format!("straight flush, {} high", self.best[0].to_str()),
      Strength::FourofaKind   => format!("four of a kind, {}s", self.best[0].to_str()),
      Strength::FullHouse     => format!("full house, {}s over {}s",
                                         self.best[0].to_str(),
                                         self.best[1].to_str()),
      Strength::Flush         => format!("flush, {} high", self.best[0].to_str()),
      Strength::Straight      => format!("straight, {} high", self.best[0].to_str()),
      Strength::ThreeofaKind  => format!("three of a kind, {}s", self.best[0].to_str()),
      Strength::TwoPair       => format!("two pair, {}s and {}s",
                                         self.best[0].to_str(),
                                         self.best[1].to_str()),
      Strength::Pair          => format!("pair of {}s", self.best[0].to_str()),
      Strength::HighCard      => format!("{} high", self.best[0].to_str()),
      Strength::Empty         => format!("empty")
    }
  }

  /* count up number of times each suit appears in the hand, and return a list
     of the suits and their total, sorted by suit count descending */
  fn count_suits(&mut self) -> Vec<SuitTotal>
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

    /* turn map into sorted vector of totals. we're not interested in
       the exact suit because they are all equal in hold 'em poker */
    let mut list = Vec::<SuitTotal>::new();
    for (_, count) in map.iter()
    {
      list.push(SuitTotal { total: *count });
    }
    list.sort_by(|a, b| b.total.cmp(&a.total));
    return list;
  }

  /* count up number of times each card value appears in the hand, and return a list
     of the values present and their totals, sorted by value count descending */
  fn count_values(&mut self) -> Vec<ValueTotal>
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

    /* turn map into vector of totals for each card value */
    let mut list = Vec::<ValueTotal>::new();
    for (value, count) in map.iter()
    {
      list.push(ValueTotal { value: *value, total: *count });
    }

    /* sort by per-value total, prioritizing by card value in a tie */
    list.sort_by(|a, b| match b.total.cmp(&a.total)
    {
      Ordering::Less => Ordering::Less,
      Ordering::Equal => b.value.to_u32().cmp(&a.value.to_u32()),
      Ordering::Greater => Ordering::Greater
    });
    return list;
  }

  /* work out the strength of the cards so far, and calculate a score for
     the hand */
  pub fn calc(&mut self)
  {
    /* default to high card */
    self.strength = Strength::HighCard;

    /* sort cards in order, highest to lowest, treating ace as high */
    self.cards.sort_by(|a, b| { b.value.to_u32().cmp(&a.value.to_u32()) });

    /* count up number of instances of each card value in this hand into a
       sorted list - high to low */
    let values = self.count_values();

    /* did we find four of a kind in top slot? */
    if values.first().unwrap().total == 4
    {
      self.strength = Strength::FourofaKind;
    }

    /* did we find three of a kind? */
    if values.first().unwrap().total == 3
    {
      /* is the value in the next slot a pair?
         if so, that's a full house */
      if values.len() > 1 && values[1].total == 2
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
    if values.first().unwrap().total == 2
    {
      /* did we find another pair in next slot? */
      if values.len() > 1 && values[1].total == 2
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
    if suits.first().unwrap().total >= 5
    {
      self.strength = Strength::Flush;
    }

    /* detect five-card straight, counting down ace to two (2) */
    let mut prev_value = None;
    let mut straight_count = 0;
    for card in self.cards.iter()
    {
      /* inspect the current card to compare with previous value */
      let value = card.value.to_u32();
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

        None => straight_count = straight_count + 1
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

    /* select the values of the best cards from the hand */
    match self.strength
    {
      /* take the highest card's value, there is no
         kicker or tie card for these made five-card hands */
      Strength::RoyalFlush | Strength::StraightFlush | Strength::Flush | Strength::Straight =>
      {
        let value = self.cards[0].value();
        self.best.push(value);
      },

      /* take 2 or more cards from the hand, depending on type.
         four of a kind:  XXXXY (X, Y = 2 cards)
         full house:      XXXYY (X, Y = 2 cards)
         three of a kind: XXXYZ (X, Y, Z = 3 cards)
         two pair:        XXYYZ (X, Y, Z = 3 cards)
         pair:            XXYZV (X, Y, Z, V = 4 cards)
         high card takes 5 highest cards */
      _ =>
      {
        let mut select = match self.strength
        {
          Strength::FourofaKind  => 2,
          Strength::FullHouse    => 2,
          Strength::ThreeofaKind => 3,
          Strength::TwoPair      => 3,
          Strength::Pair         => 4,
          Strength::HighCard     => 5,
          _ => unreachable!()
        };

        /* cap select if there aren't enough cards to take */
        if select > values.len()
        {
          select = values.len();
        }

        /* now select those cards for the best pile */
        for i in 0..select
        {
          self.best.push(values[i].value);
        }
      }
    }

    /* here's how we score each hand from its cards.
       each card value runs from 2 (two) to 14 (ace).
       that range fits neatly in four bits. so use the lower bits to
       compute the per-hand score, and high bits for the per-type base.

       bbbb xxxx      xxxx      xxxx      xxxx      xxxx      <-- bits 0 to 23
            1st best  2nd best  3rd best  4th best  5th best  <-- cards

       the bbbb bits (bits 20-23) select the hand type (0-9) */

    /* compute base score from type */
    let base_score = match self.strength
    {
      Strength::RoyalFlush    => 9,
      Strength::StraightFlush => 8,
      Strength::FourofaKind   => 7,
      Strength::FullHouse     => 6,
      Strength::Flush         => 5,
      Strength::Straight      => 4,
      Strength::ThreeofaKind  => 3,
      Strength::TwoPair       => 2,
      Strength::Pair          => 1,
      Strength::HighCard      => 0,
      _ => unreachable!()
    } << 20;

    let mut value_score = 0;
    let mut value_position = 5;
    for value in self.best.iter()
    {
      println!("card slot {} value {}", value_position, value.to_char());
      value_position = value_position - 1;
      value_score = value_score + (value.to_u32() << (value_position * 4));
    }

    self.score = base_score + value_score;
    println!("Score: {:x}", self.score);
  }
}
