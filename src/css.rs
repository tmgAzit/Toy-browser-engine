#[derive(Debug)]
struct Stylesheet {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}

#[derive(Debug)]
enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug)]
struct SimpleSelector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

#[derive(Debug)]
struct Declaration {
    name: String,
    value: Value,
}

#[derive(Debug)]
struct Declaration {
    name: String,
    value: Value,
}

#[derive(Debug)]
enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug)]
enum Unit {
    Px,
}

#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

// Parse one simple selector, e.g.: `type#id.class1.class2.class3`
fn parse_simple_selector(&mut self) -> SimpleSelector {
    let mut selector = SimpleSelector {
        tag_name: None,
        id: None,
        class: Vec::new(),
    };
    while !self.eof() {
        match self.next_char() {
            '#' => {
                self.consume_char();
                selector.id = Some(self.parse_identifier());
            }
            '.' => {
                self.consume_char();
                selector.class.push(self.parse_identifier());
            }
            '*' => {
                self.consume_char();
            }
            c if valid_identifier_char(c) => {
                selector.tag_name = Some(self.parse_identifier());
            }

            _ => break,
        }
    }
    return selector;
}

fn valid_identifier_char(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_')
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        return (a, b, c);
    }
}

// Parse a rule set: `<selectors> {<declarations>}`
fn parse_rule(&mut self) -> Rule {
    Rule {
        selectors: self.parse_selectors(),
        declarations: self.parse_declarations(),
    }
}

// Parse a comma-separated list of selectors.
fn parse_selectors(&mut self) -> Vec<Selector> {
    let mut selectors = Vec::new();
    loop {
        selectors.push(Selector::Simple(self.parse_simple_selector()));
        self.consume_whitespace();
        match self.next_char() {
            ',' => {
                self.consume_char();
                self.consume_whitespace();
            }
            '{' => break, // start of declarations
            c => panic!("Unexpected character {} in selector list", c),
        }
    }
    // Return selectors with highest specificity first, for use in matching.
    selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
    return selectors;
}
