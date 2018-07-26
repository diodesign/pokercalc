/* pokercalc
 * Calculate the cards required to beat you in a given hand of Texas Hold 'em
 *
 * Usage: Run and enter your two hole cards followed by three to five community cards,
 * all space separated. Each card is of the case insensitive format (2-9tjqka)(hdcs).
 * For example, the following two cards are the ace of diamonds and 9 of hearts: Ad 9h
 *
 * A full community board and your two hole cards could be entered as follows:
 * Ad 9h Js Jc 3h Ks 7c
 *
 * Ad 9h = your hole cards, Js Jc 3h = the flop, Ks = the turn, 7c = the river
 *
 * With the given cards, pokercalc works out all the two hole cards your opponent
 * needs to beat you, if possible, and lists those options, and percentage chance
 * of having those cards. Repeat until you hit Control-C or Control-D (or send an EOF)
 *
 * Hint: Use these percentages to decide whether it is worth calling a bet or raise,
 * based on the size of the amount to pay versus what's in the pot, and your opponent's range
 *
 * (c) Chris Williams, 2018. Open-source software: see LICENSE
 */

mod card;
mod hand;
mod deck;

use card::Card;
use hand::Hand;
use deck::Deck;

use std::io;

/* process
   Break up an input string of card descriptions into cards and then calculate required
   cards to beat the player. Only accept 2, 5, 6 or 7 cards (two holes + flop,
   two holes + flop + river, or two holes + flop + river + turn) */
fn process(input: String)
{
  let card_count = input.split_whitespace().count();
  if card_count < 2 || card_count == 3 || card_count == 4 || card_count > 7
  {
    println!("Invalid number of cards ({})", card_count);
    return;
  }

  /* create two hand objects: one with the player's hole and community cards,
     and the other with just the community cards - which will be used by the opponent */
  let mut card_count = 0;
  let mut hand = Hand::new();
  let mut community = Hand::new();
  for card_desc in input.split_whitespace()
  {
    match card_count
    {
      /* first two hards go to the player */
      0 | 1 => hand.add(Card::new(&card_desc).unwrap()),

      /* all remaining cards are the player's and community cards */
      _ =>
      {
        hand.add(Card::new(&card_desc).unwrap());
        community.add(Card::new(&card_desc).unwrap());
      }
    }

    card_count = card_count + 1;
  }

  /* calculate the strength of the hand, and tell the player */
  hand.calc();
  println!("Your hand: {}", hand.describe());

  /* now create a deck excluding the cards we can see */
  let mut deck = Deck::new();
  for card in hand.cards().iter()
  {
    deck.remove(card);
  }

  print!("Opponent needs: ");
  let unknown_cards = deck.cards().len();
  let mut running_odds = 0.0;

  /* iterate over hand combinations */
  loop
  {
    /* grab this card, run it through the others, and remove it.
       we're taking every two hole cards from the remaining deck and
       running them against the community cards to see which hole cards
       beat the player's */
    match deck.cards().pop()
    {
      Some(hole1) =>
      {
        for hole2 in deck.cards().iter()
        {
          let mut opponent = community.clone();
          opponent.add(hole1);
          opponent.add(*hole2);
          opponent.calc();
          if opponent.score() > hand.score()
          {
            print!("( {} {} ) ", hole1.describe(), hole2.describe());

            /* keep a running total of the odds for drawing these two hole cards given
               from the deck of unseen cards. we multiply by two to take into account
               the fact AK is the same as KA */
            running_odds = running_odds +
                        2.0 * ((1.0 / unknown_cards as f32) * (1.0 / (unknown_cards - 1) as f32));
          }
        }
      },

      None => break
    }
  }

  println!("\n{:.1}% chance opponent has better cards", running_odds * 100.0);
}

/* handle frontend IO */
fn main()
{
  /* read a line in from STDIN until EOF, at which point, give up */
  loop
  {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer)
    {
      Ok(bytes) =>
      {
        /* zero bytes read indicates EOF. otherwise, treat as a line of input,
           stripping the newline and making it all lowercase */
        if bytes > 0
        {
          process(buffer.replace('\n', "").to_lowercase());
        }
        else
        {
          break; /* escape to program exit */
        }
      },
      Err(_e) => break /* escape to program exit */
    }
  }
}
