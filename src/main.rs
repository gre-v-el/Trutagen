use std::env;

const FULL_BOX: [[&str; 3]; 3] = [
	["\u{250F}", "\u{2533}", "\u{2513}"],
	["\u{2523}", "\u{254B}", "\u{252B}"],
	["\u{2517}", "\u{253B}", "\u{251B}"],
];
const VERTICAL_LINE: &str = "\u{2503}";
const HORIZONTAL_LINE: &str = "\u{2501}";

const ERROR_MSG: &str = "
Some helpful info:
Accepted tokens are:
 - Variable name (more on that later)
 - Or
 - Nor
 - Xor
 - And
 - Nand
 - Not
 - (
 - )

Expressions are not case sensitive - you can use lower- and uppercase letters, it will make no difference
Variable names should contain only letters and no spaces.
There should be whitespaces between tokens (parentheses are an exception).
";

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
	Or,
	Nor,
	Xor,
	And,
	Nand,
	Not,
	OpenParenthesis,
	CloseParenthesis,
}

impl Operator {
	fn precedence(&self) -> u8{
		match &self {
			Operator::Or => 1,
			Operator::Nor => 1,
			Operator::Xor => 1,
			Operator::And => 2,
			Operator::Nand => 2,
			Operator::Not => 3,
			Operator::CloseParenthesis => 0,
			Operator::OpenParenthesis => 0,
		}
	}
}

#[derive(Debug)]
pub enum Token {
	Var(usize), // an id of a variable in /variables/ vector
	Gate(Operator),
}

#[derive(Debug, Clone)]
pub struct Variable {
	pub name: String,
	pub value: bool,
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let mut expr = if args.len() == 2 {
		args[1].clone()
	} else {
		println!("Enter a logic expression");
		get_input()
	};

	prepare_input(&mut expr);
	

	let mut variables: Vec<Variable> = Vec::new();
	let mut tokens = string_to_tokens(&expr, &mut variables);
	//println!("parsed tokens:    {:?}", tokens);

	let postline = inline_to_postline(&mut tokens);

	// println!("found variables:  {:?}", variables);
	// println!("postline:         {:?}", postline);

	let all_combinations = get_all_combinations(&mut variables);
	match evaluate_all(&postline, &all_combinations) {
		Option::Some(v) => {
			println!("\n");
			let width = print_table(&all_combinations, &v);
			println!("{}{}\n\n", " ".repeat((((width as i16)-(expr.len() as i16)).max(0)/2) as usize), expr.to_uppercase());
		}
		Option::None => {
			println!("\n\"{}\" is not a valid expression!", expr.to_uppercase());
			println!("{}", ERROR_MSG);
		}
	}
}

fn print_table(combinations: &Vec<Vec<Variable>>, answers: &Vec<bool>) -> usize {

	let mut max_length = 1;
	for v in &combinations[0] {
		max_length = max_length.max(v.name.len());
	}
	max_length = max_length/2*2 + 1;
	let width: usize;
	if max_length > 1 { width = max_length + 2; }
	else { width = max_length + 4; }
	let padding: usize = (width-1)/2;


	// upper edge
	print!("{}", FULL_BOX[0][0]); 
	for _ in 1..combinations[0].len() {
		print!("{}{}", HORIZONTAL_LINE.repeat(width), FULL_BOX[0][1]);
	}
	println!("{}{}{}{}", HORIZONTAL_LINE.repeat(width), FULL_BOX[0][1], HORIZONTAL_LINE.repeat(9), FULL_BOX[0][2]);

	// the line with variable names
	print!("{}", VERTICAL_LINE);
	for i in 0..combinations[0].len() {
		print!("{0}{3}{1}{2}", 
			" ".repeat((width-combinations[0][i].name.len())/2), 
			" ".repeat(width - (width-combinations[0][i].name.len())/2 - combinations[0][i].name.len()), 
			VERTICAL_LINE, 
			combinations[0][i].name);
	}
	println!(" result  {}", VERTICAL_LINE);

	// divider
	print!("{}", FULL_BOX[1][0]); 
	for _ in 1..combinations[0].len() {
		print!("{}{}", HORIZONTAL_LINE.repeat(width), FULL_BOX[1][1]);
	}
	println!("{}{}{}{}", HORIZONTAL_LINE.repeat(width), FULL_BOX[1][1], HORIZONTAL_LINE.repeat(9), FULL_BOX[1][2]);
	// print options
	for i in 0..answers.len() {
		for j in 0..combinations[i].len() {
			print!("{1}{0}{2}{0}", " ".repeat(padding), VERTICAL_LINE, combinations[i][j].value as u8);
		}
		println!("{}    {}    {}", VERTICAL_LINE, answers[i] as u8, VERTICAL_LINE);
	}

	
	// lower edge
	print!("{}", FULL_BOX[2][0]); 
	for _ in 1..combinations[0].len() {
		print!("{}{}", HORIZONTAL_LINE.repeat(width), FULL_BOX[2][1]);
	}
	println!("{}{}{}{}", HORIZONTAL_LINE.repeat(width), FULL_BOX[2][1], HORIZONTAL_LINE.repeat(9), FULL_BOX[2][2]);

	12 + width + (combinations[0].len() - 1)*(width + 1) // table width
}

fn evaluate_all(t: &Vec<Token>, v: &Vec<Vec<Variable>>) -> Option<Vec<bool>> {
	let mut out: Vec<bool> = Vec::with_capacity(v.len());

	for i in 0..v.len() {
		match evaluate(&t, &v[i]) {
			Option::Some(b) => out.push(b),
			Option::None => return Option::None,
		}
	}

	Option::Some(out)
}

fn get_all_combinations(variables: &mut Vec<Variable>) -> Vec<Vec<Variable>> {
	let mut out: Vec<Vec<Variable>> = Vec::with_capacity((2 as usize).pow(variables.len() as u32));

	for i in 0..out.capacity() {
		out.push(Vec::with_capacity(variables.len()));
		for j in 0..variables.len() {
			let mut clone = variables[j].clone();
			clone.value = i>>(variables.len() - 1 - j) & 1 > 0;
			out[i].push(clone);
		}
	}


	out
}

fn evaluate(v: &Vec<Token>, variables: &Vec<Variable>) -> Option<bool> {
	let mut stack: Vec<bool> = Vec::new();
	
	for t in v {
		match t {
			Token::Var(index) => stack.push(variables.get(*index).unwrap().value),
			Token::Gate(gate) => {

				if let Operator::Not = gate {
					if stack.len() < 1 { return Option::None; }
					
					let one = stack.pop().unwrap();
					stack.push(!one);
				}
				else {
					if stack.len() < 2 { return Option::None; }

					let one = stack.pop().unwrap();
					let two = stack.pop().unwrap();

					match gate {
						Operator::Or => stack.push(one || two),
						Operator::Nor => stack.push(!(one || two)),
						Operator::Xor => stack.push(one ^ two),
						Operator::And => stack.push(one && two),
						Operator::Nand => stack.push(!(one && two)),
						_ => {},
					}
				}

				
			}
		}
	}

	if stack.len() == 1 { return Option::Some(stack.pop().unwrap()); }
	Option::None
}

fn inline_to_postline(v: &mut Vec<Token>) -> Vec<Token> { // shunting yard algorithm
	let mut out: Vec<Token> = Vec::new();

	let mut operator_stack: Vec<Operator> = Vec::new();
	
	while v.len() > 0 {
		match v.remove(0) {
			Token::Var(i) => {
				out.push(Token::Var(i));
			}
			Token::Gate(g) => {

				match g {
					Operator::OpenParenthesis => operator_stack.push(Operator::OpenParenthesis),
					Operator::CloseParenthesis => {
						while *operator_stack.last().unwrap() != Operator::OpenParenthesis {
							out.push(Token::Gate(operator_stack.pop().unwrap()));
						}
						operator_stack.pop();
					},
					gate => {
						while operator_stack.len() > 0 && 
							  operator_stack.last().unwrap().precedence() >= gate.precedence() 
						{
								out.push(Token::Gate(operator_stack.pop().unwrap()));
						}
						operator_stack.push(gate);
					}
				}
			}
		}
	}

	while operator_stack.len() > 0 {
		if *operator_stack.last().unwrap() == Operator::CloseParenthesis || *operator_stack.last().unwrap() == Operator::OpenParenthesis  {panic!()}
		out.push(Token::Gate(operator_stack.pop().unwrap()));
	}
	

	out
}

fn string_to_tokens(s: &String, variables: &mut Vec<Variable>) -> Vec<Token> {
	let mut v: Vec<Token> = Vec::new();

	for w in s.split_whitespace() {
		v.push(word_to_token(w, variables));
	}

	v
}

fn word_to_token(w: &str, variables: & mut Vec<Variable>) -> Token {
	match w.to_uppercase().as_str() {
		"OR" => Token::Gate(Operator::Or),
		"NOR" => Token::Gate(Operator::Nor),
		"XOR" => Token::Gate(Operator::Xor),
		"AND" => Token::Gate(Operator::And),
		"NAND" => Token::Gate(Operator::Nand),
		"NOT" => Token::Gate(Operator::Not),
		"(" => Token::Gate(Operator::OpenParenthesis),
		")" => Token::Gate(Operator::CloseParenthesis),
		b => {
			match variables.iter().position(|e| e.name == b) {
				Some(index) => {
					Token::Var(index)
				},
				None => {
					variables.push(Variable{ name: b.to_string(), value: false });
					Token::Var(variables.len()-1)
				}
			}
		},
	}
}

fn prepare_input(s: &mut String) {
	let mut i = 0;

	while i < s.len() {
		if s.chars().nth(i).unwrap() == '(' || s.chars().nth(i).unwrap() == ')' {
			if i != s.len()-1 && s.chars().nth(i+1).unwrap() != ' ' { s.insert(i+1, ' '); }
			if i != 0 && s.chars().nth(i-1).unwrap() != ' ' { 
				s.insert(i, ' '); 
				i+=1;
			}
		}
		i+=1;
	}
}

fn get_input() -> String {
	use std::io::stdin;

	let mut ret = String::new();
	stdin().read_line(&mut ret).expect("Failed to read a line");

	ret.trim().to_string()
}