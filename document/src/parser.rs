use super::{Document,Root,RootChild,Element};

struct Parser;

struct ParsedAttribute<'a> {
    name: &'a str,
    value: &'a str,
}

impl Parser {
    fn new() -> Parser {
        Parser
    }

    fn parse_preamble<'a>(&self, xml: &'a str) -> &'a str {
        // Parse the preamble
        let idx = xml.find_str("?>").expect("No preamble end");
        let end_of_preamble = idx + "?>".len();
        xml.slice_from(end_of_preamble)
    }

    fn optional_space<'a>(&self, xml: &'a str) -> &'a str {
        match xml.slice_space() {
            Some((_, next_xml)) => next_xml,
            None => xml,
        }
    }

    fn parse_attribute_value_quote<'a>(&self, xml: &'a str, quote: &str) -> Option<(&'a str, &'a str)> {
        let (_, xml) = match xml.slice_literal(quote) {
            None => return None,
            Some(x) => x,
        };
        // TODO: don't consume & or <
        let (value, xml) = xml.slice_until(quote).expect("No value");
        let (_, xml) = xml.slice_literal(quote).expect("No quote");

        Some((value, xml))
    }

    fn parse_attribute<'a>(&self, xml: &'a str) -> Option<(ParsedAttribute<'a>, &'a str)> {
        let (name, xml) = match xml.slice_name() {
            Some(x) => x,
            None => return None,
        };

        let xml = self.optional_space(xml);
        let (_, xml) = xml.slice_literal("=").expect("No equal sign");
        let xml = self.optional_space(xml);

        // Pattern: alternate
        let (value, xml) = match self.parse_attribute_value_quote(xml, "'") {
            Some(x) => x,
            None => match self.parse_attribute_value_quote(xml, "\"") {
                Some(x) => x,
                None => fail!("No attribute value"),
            },
        };

        Some((ParsedAttribute{name: name, value: value}, xml))
    }

    fn parse_element<'a>(&self, doc: Document, xml: &'a str) -> (Element, &'a str) {
        let (_, after_start_brace) = xml.slice_literal("<").expect("no start brace");

        let (name, after_name) = after_start_brace.slice_name().expect("failed to parse a name!");

        let (_, after_space) = after_name.slice_space().expect("no space after name");

        let mut attrs = Vec::new();
        let after_attr = match self.parse_attribute(after_space) {
            None => after_space,
            Some((attr, after_attr)) => {
                attrs.push(attr);
                after_attr
            },
        };

        let after_attr = self.optional_space(after_attr);

        let (_, after_end_brace) = after_attr.slice_literal("/>").expect("no end brace");

        let e = doc.new_element(name.to_string());
        for attr in attrs.iter() {
            e.set_attribute(attr.name.to_string(), attr.value.to_string());
        }

        (e, after_end_brace)
    }

    fn parse(&self, xml: &str) -> Document {
        let doc = Document::new();

        let after_preamble = self.parse_preamble(xml);

        let (element, _tail) = self.parse_element(doc.clone(), after_preamble);
        doc.root().append_child(element);

        doc
    }
}

trait XmlStr<'a> {
    fn slice_at(&self, position: uint) -> (&'a str, &'a str);
    fn slice_until(&self, s: &str) -> Option<(&'a str, &'a str)>;
    fn slice_literal(&self, expected: &str) -> Option<(&'a str, &'a str)>;
    fn slice_start_rest(&self, is_first: |char| -> bool, is_rest: |char| -> bool) -> Option<(&'a str, &'a str)>;
    fn slice_name(&self) -> Option<(&'a str, &'a str)>;
    fn slice_space(&self) -> Option<(&'a str, &'a str)>;
}

impl<'a> XmlStr<'a> for &'a str {
    fn slice_at(&self, position: uint) -> (&'a str, &'a str) {
        (self.slice_to(position), self.slice_from(position))
    }

    fn slice_until(&self, s: &str) -> Option<(&'a str, &'a str)> {
        match self.find_str(s) {
            Some(position) => Some(self.slice_at(position)),
            None => None
        }
    }

    fn slice_literal(&self, expected: &str) -> Option<(&'a str, &'a str)> {
        if self.starts_with(expected) {
            Some(self.slice_at(expected.len()))
        } else {
            None
        }
    }

    fn slice_start_rest(&self,
                        is_first: |char| -> bool,
                        is_rest: |char| -> bool)
                        -> Option<(&'a str, &'a str)>
    {
        let mut positions = self.char_indices();

        match positions.next() {
            Some((_, c)) if is_first(c) => (),
            Some((_, _)) => return None,
            None => return None,
        };

        let mut positions = positions.skip_while(|&(_, c)| is_rest(c));
        match positions.next() {
            Some((offset, _)) => Some(self.slice_at(offset)),
            None => Some((self.clone(), "")),
        }
    }

    fn slice_name(&self) -> Option<(&'a str, &'a str)> {
        self.slice_start_rest(|c| c.is_name_start_char(), |c| c.is_name_char())
    }

    fn slice_space(&self) -> Option<(&'a str, &'a str)> {
        self.slice_start_rest(|c| c.is_space_char(), |c| c.is_space_char())
    }
}

trait XmlChar {
    fn is_name_start_char(&self) -> bool;
    fn is_name_char(&self) -> bool;
    fn is_space_char(&self) -> bool;
}

impl XmlChar for char {
    fn is_name_start_char(&self) -> bool {
        match *self {
            ':'                        |
            'A'..'Z'                   |
            '_'                        |
            'a'..'z'                   |
            '\U000000C0'..'\U000000D6' |
            '\U000000D8'..'\U000000F6' |
            '\U000000F8'..'\U000002FF' |
            '\U00000370'..'\U0000037D' |
            '\U0000037F'..'\U00001FFF' |
            '\U0000200C'..'\U0000200D' |
            '\U00002070'..'\U0000218F' |
            '\U00002C00'..'\U00002FEF' |
            '\U00003001'..'\U0000D7FF' |
            '\U0000F900'..'\U0000FDCF' |
            '\U0000FDF0'..'\U0000FFFD' |
            '\U00010000'..'\U000EFFFF' => true,
            _ => false,
        }
    }

    fn is_name_char(&self) -> bool {
        if self.is_name_start_char() { return true; }
        match *self {
            '-'                |
            '.'                |
            '0'..'9'           |
            '\u00B7'           |
            '\u0300'..'\u036F' |
            '\u203F'..'\u2040' => true,
            _ => false
        }
    }

    fn is_space_char(&self) -> bool {
        match *self {
            '\x20' |
            '\x09' |
            '\x0D' |
            '\x0A' => true,
            _ => false,
        }
    }
}

trait Hax {
    fn first_child(&self) -> Option<RootChild>;
}

impl Hax for Root {
    fn first_child(&self) -> Option<RootChild> {
        self.children().remove(0)
    }
}

#[test]
fn parses_a_document_with_a_single_element() {
    let parser = Parser::new();
    let doc = parser.parse("<?xml version='1.0' ?><hello />");
    let top = doc.root().first_child().unwrap().element().unwrap();

    assert_eq!(top.name().as_slice(), "hello");
}

#[test]
fn parses_an_element_with_an_attribute() {
    let parser = Parser::new();
    let doc = parser.parse("<?xml version='1.0' ?><hello scope='world'/>");
    let top = doc.root().first_child().unwrap().element().unwrap();

    assert_eq!(top.get_attribute("scope").unwrap().as_slice(), "world");
}

#[test]
fn parses_an_element_with_an_attribute_using_double_quotes() {
    let parser = Parser::new();
    let doc = parser.parse("<?xml version='1.0' ?><hello scope=\"world\"/>");
    let top = doc.root().first_child().unwrap().element().unwrap();

    assert_eq!(top.get_attribute("scope").unwrap().as_slice(), "world");
}
