/* pokercalc
 * Calculate the cards required to beat you in a given hand of Texas Hold 'em
 *
 * (c) Chris Williams, 2018. Open-source software: see LICENSE
 */

mod card;
use card::Card;
use std::io;

fn process(buffer: String)
{
  let card = match Card::new(&buffer)
  {
    Some(c) => c,
    None => return /* bail out on error */
  };

  println!("{}", card.describe());
}

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
          break;
        }
      },
      Err(_e) => break
    }
  }
}
