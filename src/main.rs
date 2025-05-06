#[derive(Debug)]
enum DiceExpression{
    Throw {times: u8, sides: u8}
}

impl DiceExpression{
    fn prase(argument: &str) -> DiceExpression{
        let mut expr = argument.chars().peekable();
        
        let mut nums = String::new();
        let mut expr_string = String::new();
        let mut last_was_numeric = false;
        
        loop{
            match expr.next(){
                Some(character) => match character {
                    character if character.is_numeric() => {
                        if !last_was_numeric {
                            expr_string.push('X');
                            last_was_numeric = true;
                        }
                        nums.push(character);
                    },
                    character => {
                        expr_string.push(character);
                        if last_was_numeric{
                            nums.push(' ');
                        }
                        last_was_numeric = false;
                        
                    },
                },
                None => break
            }
        };
        let mut nums = nums.split(' ').map(|string| string.parse::<u8>().unwrap());
        match &expr_string[0..expr_string.len()]{
            "XdX" => DiceExpression::Throw { times: nums.next().unwrap(), sides: nums.next().unwrap() },
            "dX" => DiceExpression::Throw { times: 1, sides: nums.next().unwrap() },
            _ => panic!("No such expression")
        }
        
    }
    fn eval(&self) -> i32{
        let mut result: i32 = 0;

        match self {
            DiceExpression::Throw { times, sides } => {
                for _ in 0..*times{
                    result += rand::random_range(1..((*sides as i32) + 1));
                    println!("{}", rand::random_range(1..((*sides as i32) + 1)));
                }
                result
            }
        }
    }
}

fn main() {

    let args = std::env::args().collect::<Vec<String>>();
    let arg = args.get(1).unwrap();
    let dice_expression = DiceExpression::prase(&arg[0..arg.len()]);
    println!("{:?} gives {}", dice_expression, dice_expression.eval());

}
