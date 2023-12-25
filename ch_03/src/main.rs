use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "arithmetic.pest"]
struct ArithmeticParser;

fn main() {
    let input = "123 + 456"; // Input data to parse

    // Parse the input using the defined grammar
    let pairs =
        ArithmeticParser::parse(Rule::arithmetic, input).unwrap_or_else(|e| panic!("{}", e));

    // Process parsed data or traverse the parse tree
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::number => println!("Number:  {}", inner_pair.as_str()),
                Rule::optional_space => println!("Optional Space:  {}", inner_pair.as_str()),
                Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
                Rule::operator => println!("Operator:   {}", inner_pair.as_str()),
                Rule::arithmetic => unreachable!(),
            };
        }
    }
}
