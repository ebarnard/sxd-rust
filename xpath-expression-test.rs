extern crate xpath;

use std::collections::HashMap;

use xpath::XPathValue;
use xpath::{Boolean, Number, String, Nodes};
use xpath::{Node, Nodeset};
use xpath::expression::XPathExpression;
use xpath::expression::{ExpressionAnd,
                        ExpressionEqual,
                        ExpressionNotEqual,
                        ExpressionFunction,
                        ExpressionLiteral,
                        ExpressionMath,
                        ExpressionPredicate,
                        ExpressionRelational};
use xpath::XPathFunction;
use xpath::XPathEvaluationContext;

struct FailExpression;

impl<'n> XPathExpression<'n> for FailExpression {
    fn evaluate(&self, _: &XPathEvaluationContext<'n>) -> XPathValue<'n> {
        fail!("Should never be called");
    }
}

#[test]
fn expression_and_returns_logical_and() {
    let left  = box ExpressionLiteral{value: Boolean(true)};
    let right = box ExpressionLiteral{value: Boolean(true)};

    let node = Node::new();
    let funs = HashMap::new();
    let context = XPathEvaluationContext::new(&node, &funs);
    let expr = ExpressionAnd{left: left, right: right};

    let res = expr.evaluate(&context);

    assert_eq!(res, Boolean(true));
}

#[test]
fn expression_and_short_circuits_when_left_argument_is_false() {
    let left  = box ExpressionLiteral{value: Boolean(false)};
    let right = box FailExpression;

    let node = Node::new();
    let funs = HashMap::new();
    let context = XPathEvaluationContext::new(&node, &funs);
    let expr = ExpressionAnd{left: left, right: right};

    expr.evaluate(&context);
    // assert_not_fail
}

#[test]
fn expression_equal_compares_as_boolean_if_one_argument_is_a_boolean() {
    let actual_bool = box ExpressionLiteral{value: Boolean(false)};
    let truthy_str = box ExpressionLiteral{value: String("hello".to_string())};

    let node = Node::new();
    let funs = HashMap::new();
    let context = XPathEvaluationContext::new(&node, &funs);
    let expr = ExpressionEqual{left: actual_bool, right: truthy_str};

    let res = expr.evaluate(&context);

    assert_eq!(res, Boolean(false));
}

#[test]
fn expression_equal_compares_as_number_if_one_argument_is_a_number() {
    let actual_number = box ExpressionLiteral{value: Number(-42.0)};
    let number_str = box ExpressionLiteral{value: String("-42.0".to_string())};

    let node = Node::new();
    let funs = HashMap::new();
    let context = XPathEvaluationContext::new(&node, &funs);
    let expr = ExpressionEqual{left: number_str, right: actual_number};

    let res = expr.evaluate(&context);
    assert_eq!(res, Boolean(true));
}

#[test]
fn expression_equal_compares_as_string_otherwise() {
    let a_str = box ExpressionLiteral{value: String("hello".to_string())};
    let b_str = box ExpressionLiteral{value: String("World".to_string())};

    let node = Node::new();
    let funs = HashMap::new();
    let context = XPathEvaluationContext::new(&node, &funs);
    let expr = ExpressionEqual{left: a_str, right: b_str};

    let res = expr.evaluate(&context);
    assert_eq!(res, Boolean(false));
}

#[test]
fn expression_not_equal_negates_equality() {
    let a_str = box ExpressionLiteral{value: Boolean(true)};
    let b_str = box ExpressionLiteral{value: Boolean(false)};

    let node = Node::new();
    let funs = HashMap::new();
    let context = XPathEvaluationContext::new(&node, &funs);
    let expr = ExpressionNotEqual::new(a_str, b_str);

    let res = expr.evaluate(&context);
    assert_eq!(res, Boolean(true));
}

struct StubFunction<'n> {
    value: XPathValue<'n>,
}

impl<'n> XPathFunction<'n> for StubFunction<'n> {
    fn evaluate(&self,
                _: &XPathEvaluationContext,
                _: Vec<XPathValue>) -> XPathValue<'n>
    {
        self.value.clone()
    }
}

#[test]
fn expression_function_evaluates_input_arguments() {
    let arg_expr: Box<XPathExpression> = box ExpressionLiteral{value: Boolean(true)};
    let fun = box StubFunction{value: String("the function ran".to_string())};

    let node = Node::new();
    let mut funs: HashMap<String, Box<XPathFunction>> = HashMap::new();
    funs.insert("test-fn".to_string(), fun);
    let context = XPathEvaluationContext::new(&node, &funs);
    let expr = ExpressionFunction{name: "test-fn".to_string(), arguments: vec!(arg_expr)};

    let res = expr.evaluate(&context);
    assert_eq!(res, String("the function ran".to_string()));
}

#[test]
fn expression_function_unknown_function_is_reported_as_an_error() {
    let node = Node::new();
    let funs = HashMap::new();
    let context = XPathEvaluationContext::new(&node, &funs);
    let expr = ExpressionFunction{name: "unknown-fn".to_string(), arguments: vec!()};

    expr.evaluate(&context);
    // TODO: report errors better
}

#[test]
fn expression_math_does_basic_math() {
    let left  = box ExpressionLiteral{value: Number(10.0)};
    let right = box ExpressionLiteral{value: Number(5.0)};

    let node = Node::new();
    let funs = HashMap::new();
    let context = XPathEvaluationContext::new(&node, &funs);
    let expr = ExpressionMath::multiplication(left, right);

    let res = expr.evaluate(&context);
    assert_eq!(res, Number(50.0));
}

#[test]
fn expression_step_numeric_predicate_selects_that_node() {
    let input_node_1 = Node::new_with_parent(1);
    let input_node_2 = Node::new_with_parent(2);
    let mut input_nodeset = Nodeset::new();
    input_nodeset.add(&input_node_1);
    input_nodeset.add(&input_node_2);

    let selected_nodes = box ExpressionLiteral{value: Nodes(input_nodeset)};
    let predicate = box ExpressionLiteral{value: Number(1.0)};

    let node = Node::new();
    let funs = HashMap::new();
    let context = XPathEvaluationContext::new(&node, &funs);
    let expr = ExpressionPredicate{node_selector: selected_nodes, predicate: predicate};

    let res = expr.evaluate(&context);

    match res {
        Nodes(ns) => {
            let mut e = Nodeset::new();
            e.add(&input_node_1);

            assert_eq!(ns, e);
        },
        _ => fail!("Not a nodeset"),
    }
}

#[test]
fn expression_step_false_predicate_selects_no_nodes() {
    let input_node_1 = Node::new_with_parent(1);
    let input_node_2 = Node::new_with_parent(2);
    let mut input_nodeset = Nodeset::new();
    input_nodeset.add(&input_node_1);
    input_nodeset.add(&input_node_2);

    let selected_nodes = box ExpressionLiteral{value: Nodes(input_nodeset)};
    let predicate = box ExpressionLiteral{value: Boolean(false)};

    let node = Node::new();
    let funs = HashMap::new();
    let context = XPathEvaluationContext::new(&node, &funs);
    let expr = ExpressionPredicate{node_selector: selected_nodes, predicate: predicate};

    let res = expr.evaluate(&context);

    match res {
        Nodes(ns) => {
            let e = Nodeset::new();

            assert_eq!(ns, e);
        },
        _ => fail!("Not a nodeset"),
    }
}

#[test]
fn expression_relational_does_basic_comparisons() {
    let left  = box ExpressionLiteral{value: Number(10.0)};
    let right = box ExpressionLiteral{value: Number(5.0)};

    let node = Node::new();
    let funs = HashMap::new();
    let context = XPathEvaluationContext::new(&node, &funs);
    let expr = ExpressionRelational::less_than(left, right);

    let res = expr.evaluate(&context);
    assert_eq!(res, Boolean(false));
}
