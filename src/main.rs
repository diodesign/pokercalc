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
 * based on the size of the amount to pay versus what's in the pot.
 *
 * (c) Chris Williams, 2018. Open-source software: see LICENSE
 */

mod card;
mod hand;

use card::Card;
use hand::Hand;
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

  /* create a hand object and add the player's cards to it */
  let mut hand = Hand::new();
  for card_desc in input.split_whitespace()
  {
    hand.add(match Card::new(&card_desc)
    {
      Some(c) => c,
      None => return /* bail out on error */
    });
  }

  /* calculate the strength of the hand */
  hand.rank();
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
