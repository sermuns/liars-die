use std::cmp::Ordering;
use rand::Rng;
use std::io;
use std::{thread, time};

#[derive(Default, Clone, Copy, PartialEq)]
struct DieGuess {
	val: u8,
	cnt: u8
}

 impl DieGuess {
	fn default() -> DieGuess {
		DieGuess {
			val: 0,
			cnt: 0
		}
	}
 }

impl PartialOrd for DieGuess {
	fn partial_cmp(&self, other: &DieGuess) -> Option<std::cmp::Ordering> {
		if self < other {
			return Some(Ordering::Less);
		}
		if self == other {
			return Some(Ordering::Equal);
		}
		if self > other {
			return Some(Ordering::Greater);
		}
		None
	}
}

#[derive(Clone, Copy)]
struct PlayerInfo {
	// will point to an array of names
    name: usize,
	is_human: bool,
    dies_left: u8,
	dies: [DieGuess; 6]
}

impl PlayerInfo {
	fn default() -> PlayerInfo {
		PlayerInfo {
			name: 0,
			is_human: false,
			dies_left: 0,
			dies: [DieGuess::default(); 6]
		}
	}
}

enum Move {
	Exact(),
	No(),
	AtLeast(DieGuess)
}

enum Agreement {
	Yes(),
	No()
}

fn read_input_number() -> usize {
	loop {
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
			Ok(..) => {
				if input.chars().count() == 3 {
					let c = input.chars().next().unwrap();
					if ('2'..='6').contains(&c) {
						return  c.to_digit(10).unwrap() as usize;
					}
				}
				println!("Please enter a number between 2 and 6");
			}
			Err(error) => println!("error: {error}"),
		}
	}
}

fn calculate_result(players: &[PlayerInfo], val: u8) -> u8 {
	let mut r = 0;
	for player in players {
		for d in 0..6 {
			if player.dies[d as usize].val == val {
				r += player.dies[d as usize].cnt;
			}
		}
	}
	r
}

fn get_last_player(players: &[PlayerInfo], current_player: usize) -> usize {
	let mut r=current_player;
	let mut found = false;
	while !found {
		if r == 0 {
			r = players.len();
		}
		r -= 1;
		if players[r].dies_left > 0 {
			found = true;
		}
	}
	r
}

fn read_agreement(message: &str) -> Agreement {
	loop {
		let mut input = String::new();
		println!("{} (y/n)",message);
		match io::stdin().read_line(&mut input) {
			Ok(..) => {
				if input.chars().count() >1 {
					let c = input.chars().next().unwrap();
					if c == 'y' || c == 'Y' {
						return Agreement::Yes();
					}
					if c == 'n' || c == 'N' {
						return Agreement::No();
					}
				}
			}
			Err(error) => println!("error: {error}"),
		}
	}
}

fn player_lost(players: &mut [PlayerInfo], looser: usize, players_left: &mut usize) -> Agreement {
	players[looser].dies_left -= 1;
	if players[looser].dies_left == 0 {
		*players_left -= 1;
	}
	if players[looser].is_human {
		println!("You lost.");
	} else {
		println!("{} lost",NPCNAMES[players[looser].name]);
	}
	if *players_left == 1 {
		return Agreement::No();
	} else if players[looser].dies_left == 0 {
		if players[looser].is_human {
			return read_agreement("You have been eliminated. Continue with current game?");
		} else {
			println!("{} has been eliminated",NPCNAMES[players[looser].name]);
		}
	}
	Agreement::Yes()
}

fn print_title() {
	println!("┌-------┐┌-------┐┌-------┐┌-------┐┌-------┐┌-------┐┌-------┐┌-------┐");
	println!("|       ||       ||       ||       ||       ||       ||       ||       |");
	println!("|  ☻    ||   ☻   ||  ☻☻☻  || ☻☻☻☻  ||  ☻☻☻  || ☻☻☻☻  ||   ☻   || ☻☻☻☻☻ |");
	println!("|  ☻    ||   ☻   || ☻   ☻ || ☻   ☻ || ☻     || ☻   ☻ ||   ☻   || ☻     |");
	println!("|  ☻    ||   ☻   || ☻☻☻☻☻ || ☻☻☻☻  ||  ☻☻☻  || ☻   ☻ ||   ☻   || ☻☻☻☻☻ |");
	println!("|  ☻    ||   ☻   || ☻   ☻ || ☻  ☻  ||     ☻ || ☻   ☻ ||   ☻   || ☻     |");
	println!("|  ☻☻☻☻ ||   ☻   || ☻   ☻ || ☻   ☻ ||  ☻☻☻  || ☻☻☻☻  ||   ☻   || ☻☻☻☻☻ |");
	println!("|       ||       ||       ||       ||       ||       ||       ||       |");
	println!("└-------┘└-------┘└-------┘└-------┘└-------┘└-------┘└-------┘└-------┘");
	println!("(c) Jens Ogniewski 2026");
	println!();
}

fn print_win() {
	println!("┌-------┐┌-------┐┌-------┐┌-------┐┌-------┐┌-------┐");
	println!("|       ||       ||       ||       ||       ||       |");
	println!("| ☻   ☻ ||  ☻☻☻  || ☻   ☻ || ☻   ☻ ||  ☻☻☻  ||  ☻☻☻  |");
	println!("|  ☻ ☻  || ☻   ☻ || ☻   ☻ || ☻ ☻ ☻ || ☻   ☻ || ☻   ☻ |");
	println!("|   ☻   || ☻   ☻ || ☻   ☻ || ☻ ☻ ☻ || ☻   ☻ || ☻   ☻ |");
	println!("|   ☻   || ☻   ☻ || ☻   ☻ || ☻ ☻ ☻ || ☻   ☻ || ☻   ☻ |");
	println!("|   ☻   ||  ☻☻☻  ||  ☻☻☻  ||  ☻ ☻  ||  ☻☻☻  || ☻   ☻ |");
	println!("|       ||       ||       ||       ||       ||       |");
	println!("└-------┘└-------┘└-------┘└-------┘└-------┘└-------┘");
}

fn print_dies(player: PlayerInfo) {
	for _i in 0..player.dies_left {
		print!("┌-------┐");
	}
	println!();
	for _i in 0..player.dies_left {
		print!("|       |");
	}
	println!();
	for d in 0..6 {
		for _i in 0..player.dies[d].cnt {
			match d {
				0 => {print!("|       |");}
				1 => {print!("| ☻     |");}
				2 => {print!("|     ☻ |");}
				3 => {print!("| ☻   ☻ |");}
				4 => {print!("| ☻   ☻ |");}
				5 => {print!("| ☻   ☻ |");}
				_ => {}
			}
		}
	}
	println!();
	for _i in 0..player.dies_left {
		print!("|       |");
	}
	println!();
	for d in 0..6 {
		for _i in 0..player.dies[d].cnt {
			match d {
				0 => {print!("|   ☻   |");}
				1 => {print!("|       |");}
				2 => {print!("|   ☻   |");}
				3 => {print!("|       |");}
				4 => {print!("|   ☻   |");}
				5 => {print!("| ☻   ☻ |");}
				_ => {}
			}
		}
	}
	println!();
	for _i in 0..player.dies_left {
		print!("|       |");
	}
	println!();
	for d in 0..6 {
		for _i in 0..player.dies[d].cnt {
			match d {
				0 => {print!("|       |");}
				1 => {print!("|     ☻ |");}
				2 => {print!("| ☻     |");}
				3 => {print!("| ☻   ☻ |");}
				4 => {print!("| ☻   ☻ |");}
				5 => {print!("| ☻   ☻ |");}
				_ => {}
			}
		}
	}
	println!();
	for _i in 0..player.dies_left {
		print!("|       |");
	}
	println!();
	for _i in 0..player.dies_left {
		print!("└-------┘");
	}
	println!();
}

fn calc_move(player: PlayerInfo, dies_left: u8, players_left: usize, current_guess: DieGuess) -> Move {
	let other_dies_left = dies_left - player.dies_left;
	let mod_dies_left   = other_dies_left % 6;
	let full_dies_left  = (other_dies_left - mod_dies_left) / 6;
	let endgame = (other_dies_left<4) || (players_left<3);
	let new_round = current_guess.cnt==0 || current_guess.val==0;
	let mut rng = rand::thread_rng();
	let bluff =  rng.gen_range(0..1);
	
	if new_round {
		let mut rg = DieGuess::default();
		if bluff>0 {
			rg.val = rng.gen_range(1..=6);
			rg.cnt = 1;
			if endgame {
				if player.dies_left>2 {
					rg.cnt += rng.gen_range(0..=1);
				}
			} else {
				if full_dies_left>1 {
					rg.cnt += rng.gen_range(1..full_dies_left);
				}
			}
		} else {
			rg = player.dies[0];
			for d in 1..6 {
				if rg<player.dies[d] {
					rg=player.dies[d];
				}
			}
			rg.cnt += full_dies_left;
			if rg.cnt>1 {
				rg.cnt -= 1;
			}
			if mod_dies_left>3 {
				rg.cnt += rng.gen_range(0..=1);
			}
		}
		return Move::AtLeast(rg);
	}
	
	if bluff>0 {
		let mut max_bluff = 1;
		if !endgame {
			let mdl  = dies_left % 6;
			let fdl = (dies_left - mdl) / 6;
			max_bluff = fdl;
			if mdl>3 {
				max_bluff += 1;
			}
		}
		if player.dies_left>3 {
			max_bluff += 1;
		}
		if current_guess.cnt < max_bluff {
			return Move::AtLeast(DieGuess {val: current_guess.val, cnt: current_guess.cnt+1});
		}
		if (current_guess.cnt == max_bluff) && current_guess.val<6 {
			let bluff_guess = rng.gen_range(current_guess.val+1..=6);
			return  Move::AtLeast(DieGuess {val: bluff_guess, cnt: current_guess.cnt});
		}
	}
	
	//playing the odds
	let mut min_guess = full_dies_left;
	if mod_dies_left>3 {
		min_guess += 1;
	}
	let start_pos = rng.gen_range(0..6);
    
	for d in 0..6 {
		let tgc = player.dies[(d+start_pos)%6].cnt + min_guess;
		if tgc>1 {
			let mut rd = DieGuess {val: (((d+start_pos)%6)+1) as u8, cnt: tgc-1};
			if rd>current_guess {
				if rd.val>current_guess.val {
					rd.cnt = current_guess.cnt;
				} else {
					rd.cnt = current_guess.cnt+1;
				}
				return  Move::AtLeast(rd);
			}
		}
	}

	for d in 0..6 {
		let tgc = player.dies[(d+start_pos)%6].cnt + min_guess;
		if tgc>0 {
			let mut rd = DieGuess {val: (((d+start_pos)%6)+1) as u8, cnt: tgc};
			if rd>current_guess {
				if rd.val>current_guess.val {
					rd.cnt = current_guess.cnt;
				} else {
					rd.cnt = current_guess.cnt+1;
				}
				return  Move::AtLeast(rd);
			}
		}
	}
	
	//need to call it
	if endgame {
		let call_it_even = rng.gen_range(0..=1);
		if (   call_it_even>0 && current_guess.cnt <= (other_dies_left>>1) + (other_dies_left%2) + player.dies[(current_guess.val-1)as usize].cnt)
		    || (player.dies[(current_guess.val-1)as usize].cnt==current_guess.cnt) {
			return Move::Exact();
		}
	} else {
		if (   player.dies[(current_guess.val-1) as usize].cnt+min_guess == current_guess.cnt)
			&& ((mod_dies_left == 0) || (mod_dies_left == 1) || (mod_dies_left == 5)) {
			return Move::Exact();
		}
	}
	
    Move::No()
}

static NPCNAMES: &[&str] = &["Alice", "Bob", "Eve", "John", "Jane", "Spock"];

fn main() {
	let mut game_on = true;
	while game_on {
		print_title();
		println!("How many players? ");
		let player_count = read_input_number();
		println!("How many starting dies? ");
		let starting_dies = read_input_number();

		//player initialization
		let mut rng = rand::thread_rng();
		let player_start_pos =  rng.gen_range(0..player_count);
		//println!("start-pos: {player_start_pos}");
		let mut players = vec![PlayerInfo::default(); player_count];
		for player in &mut players {
			player.dies_left = starting_dies as u8;
			// Yeah, I know, not the most elegant solution
			player.dies[0].val = 1;
			player.dies[1].val = 2;
			player.dies[2].val = 3;
			player.dies[3].val = 4;
			player.dies[4].val = 5;
			player.dies[5].val = 6;
		}
		players[player_start_pos].is_human = true;
		for p in 1..player_count {
			let mut name_pos =  rng.gen_range(0..6);
			let mut name_found = true;
			while name_found {
				name_found = false;
				for p2 in 1..p {
					if players[(player_start_pos+p2)%player_count].name == name_pos {
						// if name is already used => try the next one
						name_pos = (name_pos+1)%6;
						name_found = true;
						break;
					}
				}
			}
			players[(player_start_pos+p)%player_count].name = name_pos;
		}
	
		//rounds
		let mut players_left = player_count;
		let mut dies_left = starting_dies * player_count;
		let mut the_heat_is_on = true;
		let mut current_player = 0;
		while the_heat_is_on {
			let mut dg = DieGuess::default();
			for player in &mut players {
				player.dies[0].cnt = 0;
				player.dies[1].cnt = 0;
				player.dies[2].cnt = 0;
				player.dies[3].cnt = 0;
				player.dies[4].cnt = 0;
				player.dies[5].cnt = 0;
				for _ in 0..player.dies_left {
					let die_val =  rng.gen_range(0..6);
					player.dies[die_val].cnt += 1;
				}
			}

			//turns
			let mut turn_turn_turn = true;
			while turn_turn_turn {
				if players[current_player].dies_left>0 {
					let mut next_move =  Move::AtLeast(dg);
					if players[current_player].is_human {
						println!("Your turn!");
						print_dies(players[current_player]);
						println!("No of dies in game: {}", dies_left);
						'outer: loop {
							if dg.val != 0 || dg.cnt != 0 {
								'guess: loop {
									let mut input = String::new();
									println!("(G)uess, (E)xact, (N)o?");
									match io::stdin().read_line(&mut input) {
										Ok(..) => {
											let c = input.chars().next().unwrap();
											if c == 'g' || c == 'G' {
												break 'guess;
											}
											if c == 'e' || c == 'E' {
												next_move = Move::Exact();
												break 'guess;
											}
											if c == 'n' || c == 'N' {
												next_move = Move::No();
												break 'guess;
											}
										}
										Err(error) => println!("error: {error}"),
									}
								}
							}
							
							if let Move::AtLeast(ref mut ndg) = next_move {
       									'val: loop {
       										println!("Which value are you guessing? (1..6) ");
       										let mut input = String::new();
       										match io::stdin().read_line(&mut input) {
       											Ok(..) => {
       												if input.chars().count() == 3 {
       													let c = input.chars().next().unwrap();
       													if ('1'..='6').contains(&c) {
       														ndg.val = c.to_digit(10).unwrap() as u8;
       														break 'val;
       													}
       												}
       											}
       											Err(error) => println!("error: {error}"),
       										}
       									}
       									'cnt: loop {
       										let mut min_guess = dg.cnt;
       										if ndg.val <= dg.val {
       											min_guess += 1;
       										}
       										if min_guess == 0 {
       											min_guess = 1;
       										}
       										println!("How many times (>={}) ?", min_guess);
       										let mut input = String::new();
       										match io::stdin().read_line(&mut input) {
       											Ok(..) => {
       												let mut guess_cnt = 0;
       												let l = input.chars().count();
       												if l > 2 {
       													for cp in 0..l-2 {
       														let c = input.chars().nth(cp).unwrap();
       														if c.is_ascii_digit() {
       															guess_cnt = guess_cnt*10 + c.to_digit(10).unwrap() as u8;
       														}
       													}
       												}
       												if guess_cnt >= min_guess {
       													ndg.cnt = guess_cnt;
       													break 'cnt;
       												}
       											}
       											Err(error) => println!("error: {error}"),
       										}
       									}
       								}
							
							match next_move {
								Move::AtLeast(ndg) => {
									println!(": At least {} {}s", ndg.cnt, ndg.val);
								}
								Move::No() => {
									println!(": No");
								}
								Move::Exact() => {
									println!(": Exact");
								}
							}
							match read_agreement("correct?") {
								Agreement::Yes() => break 'outer,
								Agreement::No() => (),
							}
						}
					} else {
						print!("{}",NPCNAMES[players[current_player].name]);
						next_move = calc_move(players[current_player], dies_left as u8, players_left, dg);
						match next_move {
							Move::AtLeast(ndg) => {
								println!(": At least {} {}s", ndg.cnt, ndg.val);
							}
							Move::No() => {
								println!(": No");
							}
							Move::Exact() => {
								println!(": Exact");
							}
						}
					}
					match next_move {
						Move::AtLeast(ndg) => {
							dg = ndg;
							current_player = (current_player+1)%player_count;
						}
						Move::No() => {
							let guess_cnt = calculate_result(&players, dg.val);
							println!("Actually:  {} {}s", guess_cnt, dg.val);
							if guess_cnt<dg.cnt {
								let looser = get_last_player(&players, current_player);
								let game_continues = player_lost(&mut players, looser, &mut players_left);
								if let Agreement::No() = game_continues {
        										the_heat_is_on = false;
        									}
								current_player = looser;
							} else {
								let game_continues = player_lost(&mut players, current_player, &mut players_left);
								if let Agreement::No() = game_continues {
        										the_heat_is_on = false;
        									}
							}
							dies_left -= 1;
							turn_turn_turn = false;
						}
						Move::Exact() => {
							let guess_cnt = calculate_result(&players, dg.val);
							println!("Actually:  {} {}s", guess_cnt, dg.val);
							if guess_cnt==dg.cnt {
								let looser = get_last_player(&players, current_player);
								let game_continues = player_lost(&mut players, looser, &mut players_left);
								if let Agreement::No() = game_continues {
        										the_heat_is_on = false;
        									}
								current_player = looser;
							} else {
								let game_continues = player_lost(&mut players, current_player, &mut players_left);
								if let Agreement::No() = game_continues {
        										the_heat_is_on = false;
        									}
							}
							dies_left -= 1;
							turn_turn_turn = false;
						}
					}
				} else {
				  current_player = (current_player+1)%player_count;
				}
				thread::sleep(time::Duration::from_millis(500));
			}
		}
		if players_left == 1 {
			for p in 0..player_count {
				if players[p].dies_left>0 {
					if players[p].is_human {
						print_win();
						println!("Congratulations");
					} else {
						println!("{} won!",NPCNAMES[players[p].name]);
					}
				}
			}
		}
		if let Agreement::No() = read_agreement("Another game?") {
  				game_on = false;
  			}
	}
}
