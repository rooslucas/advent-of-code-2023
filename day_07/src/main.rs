use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Debug, Eq)]
struct Hand {
    cards: String,
    bid: usize,
    division: Vec<usize>,
    label: Option<Win>,
}
#[derive(Debug, Eq, PartialEq, PartialOrd)]
enum CardType {
    J,
    N(u32),
    T,
    Q,
    K,
    A,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Win {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.label > other.label {
            return Ordering::Greater;
        } else if self.label < other.label {
            return Ordering::Less;
        } else if self.label == other.label {
            for l in 0..self.cards.len() {
                let own_cards: Vec<_> = self.cards.chars().collect();
                let own = match own_cards[l] {
                    'A' => CardType::A,
                    'K' => CardType::K,
                    'Q' => CardType::Q,
                    'T' => CardType::T,
                    '9' => CardType::N(9),
                    '8' => CardType::N(8),
                    '7' => CardType::N(7),
                    '6' => CardType::N(6),
                    '5' => CardType::N(5),
                    '4' => CardType::N(4),
                    '3' => CardType::N(3),
                    '2' => CardType::N(2),
                    'J' => CardType::J,
                    _ => panic!("AAAAAAA own"),
                };

                let other_cards: Vec<_> = other.cards.chars().collect();
                let other = match other_cards[l] {
                    'A' => CardType::A,
                    'K' => CardType::K,
                    'Q' => CardType::Q,
                    'T' => CardType::T,
                    '9' => CardType::N(9),
                    '8' => CardType::N(8),
                    '7' => CardType::N(7),
                    '6' => CardType::N(6),
                    '5' => CardType::N(5),
                    '4' => CardType::N(4),
                    '3' => CardType::N(3),
                    '2' => CardType::N(2),
                    'J' => CardType::J,
                    _ => panic!("AAAAAAA other"),
                };

                if own > other {
                    return Ordering::Greater;
                } else if own < other {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        } else {
            Ordering::Equal
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
impl Hand {
    fn check_winning(&mut self) {
        // let types = vec!["five", "four", "full", "three", "two", "one"];
        let card_types = vec![
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];
        let splits: Vec<_> = self.cards.chars().collect();
        // println!("{:?}", splits);

        let numbs: Vec<_> = card_types
            .iter()
            .map(|x| splits.iter().filter(|&n| *n == *x).count())
            .collect();

        self.division = numbs.clone();

        //println!("{:?}", numbs);
        if self.cards.contains('J') {
            let j_numbs = self.division[3];
            println!("{j_numbs}");
            if j_numbs != 5 {
                self.division.remove(3);
                println!("{:?}", self.division);

                let max = &(self.division).iter().max().unwrap();
                let index = &(self.division)
                    .iter()
                    .position(|element| &element == max)
                    .unwrap();

                self.division[*index] += j_numbs;
            } else if j_numbs == 5 {
                println!("{:#?}", self);
            }
        }

        if self.division.contains(&5) {
            self.label = Some(Win::Five);
            println!("{:#?}", self);
        } else if self.division.contains(&4) {
            self.label = Some(Win::Four);
        } else if self.division.contains(&3) && self.division.contains(&2) {
            self.label = Some(Win::Full);
        } else if self.division.contains(&3) {
            self.label = Some(Win::Three);
        } else if self.division.iter().filter(|&n| *n == 2).count() == 2 {
            self.label = Some(Win::Two);
        } else if self.division.contains(&2) {
            self.label = Some(Win::One);
        } else if self.division.contains(&1) {
            self.label = Some(Win::High);
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

fn main() {
    let file_path = "./input/day_7.txt";
    println!("In file {}", file_path);

    let binding = read_to_string(file_path).unwrap();
    let liness: Vec<_> = binding.lines().collect();

    let hands = parse(liness);
    part_1(hands);
}

fn parse(input: Vec<&str>) -> Vec<Hand> {
    let hands: Vec<_> = input
        .iter()
        .map(|h| {
            let splith: Vec<_> = h.split(" ").collect();
            Hand {
                cards: splith[0].to_string(),
                bid: splith[1].parse::<usize>().expect("error error "),
                division: Vec::new(),
                label: None,
            }
        })
        .collect();

    // println!("{:#?}", hands);

    hands
}

fn part_1(mut input: Vec<Hand>) {
    for hand in 0..input.len() {
        input[hand].check_winning();
        // println!("{:#?}", input[hand]);
    }
    input.sort();

    println!("{:#?}", input);

    let total_winnings: usize = (0..input.len())
        .map(|i| {
            let hand = &input[i];
            hand.bid * (i + 1)
        })
        .sum();
    println!("{:#?}", total_winnings);
}
