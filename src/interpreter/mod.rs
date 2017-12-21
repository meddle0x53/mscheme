#[macro_use]
mod value;
#[macro_use]
mod macros;
pub mod runtime;

use errors::RuntimeError;
use parser::ASTNode;

use lexer::tokenize;
use parser::parse;

use interpreter::value::Value;
use interpreter::value::Value::*;
use interpreter::runtime::Runtime;
use interpreter::runtime::RuntimeNode;

pub fn run(input: &str, runtime: &RuntimeNode) -> Result<String, String> {
    let tokens = try_or_err_to_string!(tokenize(input));
    let ast = try_or_err_to_string!(parse(&tokens));
    let result = try_or_err_to_string!(eval(&ast, runtime));

    Ok(format!("{:?}", result))
}

pub fn eval(ast_nodes: &Vec<ASTNode>, runtime: &RuntimeNode) -> Result<Value, RuntimeError> {
    let mut result = empty!();

    for ast_node in ast_nodes.iter() {
        result = try!(eval_ast_node(ast_node, runtime.clone()));
    }

    Ok(result)
}

fn eval_ast_node(node: &ASTNode, runtime: RuntimeNode) -> Result<Value, RuntimeError> {
    match node {
        &ASTNode::Identifier(ref v) => {
            if let Some(val) = runtime.borrow().get_var_value(v) {
                Ok(val)
            } else {
                runtime_error!("Identifier not found! {:?}", node)
            }
        },
        &ASTNode::Integer(v) => Ok(Integer(v)),
        &ASTNode::Boolean(v) => Ok(Boolean(v)),
        &ASTNode::StringNode(ref v) => Ok(StringValue(v.clone())),
        &ASTNode::List(ref vec) => {
            if vec.len() > 0 {
                eval_list(vec, runtime.clone())
            } else {
                Ok(empty!())
            }
        }
    }
}

fn eval_list(nodes: &Vec<ASTNode>, env: RuntimeNode) -> Result<Value, RuntimeError> {
    let first = nodes.get(0).unwrap();

    match first {
        &ASTNode::Identifier(ref func) => {
            match func.as_str() {
                "define" => define(nodes, env),
                "set!"   => set(nodes, env),
                "lambda" => lambda(nodes),
                "if"     => if_construct(nodes, env),
                "and"    => and(nodes, env),
                "or"     => or(nodes, env),
                "+"      => plus(nodes, env),
                "-"      => minus(nodes, env),
                "*"      => multiplication(nodes, env),
                "/"      => division(nodes, env),
                "="     => equals(nodes, env),
                "quote"  => {
                    assert_number_of_arguments!(nodes, "quote", 2);
                    quote(node_at!(nodes, 1))
                },
                "error"  => {
                    assert_number_of_arguments!(nodes, "error", 2);

                    let e = eval_ast_node(node_at!(nodes, 1), env.clone())?;
                    runtime_error!("{}", e);
                },
                _        => func_call(func, nodes, env)
            }
        },
        _ => {
            runtime_error!("First element in an expression must be an identifier: {:?}", first);
        }
    }
}

fn quote(ast_node: &ASTNode) -> Result<Value, RuntimeError> {
    match ast_node {
        &ASTNode::Identifier(ref v) => Ok(Symbol(v.clone())),
        &ASTNode::Integer(v) => Ok(Integer(v)),
        &ASTNode::Boolean(v) => Ok(Boolean(v)),
        &ASTNode::StringNode(ref v) => Ok(StringValue(v.clone())),
        &ASTNode::List(ref values) => {
            let mut result = vec![];
            for val in values.iter() { result.push(quote(val)?) }
            Ok(List(result))
        }
    }
}

fn define(ast_nodes: &Vec<ASTNode>, runtime: RuntimeNode) -> Result<Value, RuntimeError> {
    assert_number_of_arguments!(ast_nodes, "define", 3);

    let name = match node_at!(ast_nodes, 1) {
        &ASTNode::Identifier(ref x) => x,
        _ => runtime_error!("Bad variable name in 'define': {:?}", ast_nodes)
    };

    if !is_var_defined!(runtime, name) {
        let val = eval_ast_node(node_at!(ast_nodes, 2), runtime.clone())?;
        set_var!(runtime, name, val);

        Ok(empty!())
    } else {
        runtime_error!("Variable already defined: {}", name)
    }
}

fn set(nodes: &Vec<ASTNode>, runtime: RuntimeNode) -> Result<Value, RuntimeError> {
    assert_number_of_arguments!(nodes, "set!", 3);

    let name = match node_at!(nodes, 1) {
        &ASTNode::Identifier(ref x) => x,
        _ => runtime_error!("Bad variable name in 'set!': {:?}", nodes)
    };

    if is_var_defined!(runtime, name) {
        let val = eval_ast_node(nodes.get(2).unwrap(), runtime.clone())?;
        set_var!(runtime, name, val);

        Ok(empty!())
    } else {
        runtime_error!("Can't set! an undefined variable: {}", name)
    }
}

fn lambda(nodes: &Vec<ASTNode>) -> Result<Value, RuntimeError> {
    assert_at_least_number_of_arguments!(nodes, "lambda", 3);

    let args = match node_at!(nodes, 1) {
        &ASTNode::List(ref list) => {
            list.iter().map(|arg| {
                if let &ASTNode::Identifier(ref s) = arg {
                    Ok(s.clone())
                } else {
                    runtime_error!("Bad argument in 'lambda': {:?}", arg)
                }
            }).map(|arg| arg.unwrap())
        },
        _ => runtime_error!("Bad argument list in 'lambda' definition: {:?}", nodes)
    };

    let expressions = nodes.iter().skip(2).map(|e| e.clone()).collect();
    Ok(Func(args.collect(), expressions))
}

fn if_construct(nodes: &Vec<ASTNode>, runtime: RuntimeNode) -> Result<Value, RuntimeError> {
    assert_number_of_arguments!(nodes, "if", 4);

    let condition = eval_ast_node(node_at!(nodes, 1), runtime.clone())?;
    let n = if let Boolean(false) = condition { 3 } else { 2 };

    eval_ast_node(node_at!(nodes, n), runtime.clone())
}

fn and(nodes: &Vec<ASTNode>, runtime: RuntimeNode) -> Result<Value, RuntimeError> {
    let mut result = Boolean(true);

    for n in nodes.iter().skip(1) {
        let val = eval_ast_node(n, runtime.clone())?;
        if let Boolean(false) = val {
            return Ok(Boolean(false))
        } else {
            result = val
        }
    }
    Ok(result)
}

fn or(nodes: &Vec<ASTNode>, runtime: RuntimeNode) -> Result<Value, RuntimeError> {
    for n in nodes.iter().skip(1) {
        let val = eval_ast_node(n, runtime.clone())?;
        if let Boolean(false) = val { } else { return Ok(val) }
    }
    Ok(Boolean(false))
}

fn equals(nodes: &Vec<ASTNode>, runtime: RuntimeNode) -> Result<Value, RuntimeError> {
    assert_number_of_arguments!(nodes, "=", 3);

    let left = eval_ast_node(node_at!(nodes, 1), runtime.clone())?;
    let right = eval_ast_node(node_at!(nodes, 2), runtime.clone())?;

    let left_value = if let Integer(x) = left {
        x
    } else {
        runtime_error!("Bad left value for '=' {:?}", nodes)
    };
    let right_value = if let Integer(x) = right {
        x
    } else {
        runtime_error!("Bad left value for '=' {:?}", nodes)
    };
    Ok(Boolean(left_value == right_value))
}

fn plus(nodes: &Vec<ASTNode>, runtime: RuntimeNode) -> Result<Value, RuntimeError> {
    assert_at_least_number_of_arguments!(nodes, "+", 3);

    let mut sum = 0;
    for n in nodes.iter().skip(1) {
        if let Integer(x) = eval_ast_node(n, runtime.clone())? {
            sum += x
        } else {
            runtime_error!("Unexpected node during +: {:?}", n)
        }
    };
    Ok(Integer(sum))
}

fn minus(nodes: &Vec<ASTNode>, runtime: RuntimeNode) -> Result<Value, RuntimeError> {
    assert_number_of_arguments!(nodes, "-", 3);

    let left = eval_ast_node(node_at!(nodes, 1), runtime.clone())?;
    let right = eval_ast_node(node_at!(nodes, 2), runtime.clone())?;

    let mut result = if let Integer(x) = left {
        x
    } else {
        runtime_error!("Bad left value for '-' {:?}", nodes)
    };

    result -= if let Integer(x) = right {
        x
    } else {
        runtime_error!("Bad right value for '-' -: {:?}", nodes)
    };
    Ok(Integer(result))
}

fn division(nodes: &Vec<ASTNode>, runtime: RuntimeNode) -> Result<Value, RuntimeError> {
    assert_number_of_arguments!(nodes, "/", 3);

    let left = eval_ast_node(node_at!(nodes, 1), runtime.clone())?;
    let right = eval_ast_node(node_at!(nodes, 2), runtime.clone())?;

    let mut result = if let Integer(x) = left {
        x
    } else {
        runtime_error!("Bad left value for '/' {:?}", nodes)
    };

    result /= if let Integer(x) = right {
        x
    } else {
        runtime_error!("Bad right value for '/' -: {:?}", nodes)
    };
    Ok(Integer(result))
}

fn multiplication(nodes: &Vec<ASTNode>, runtime: RuntimeNode) -> Result<Value, RuntimeError> {
    assert_at_least_number_of_arguments!(nodes, "*", 3);

    let mut product = 1;
    for n in nodes.iter().skip(1) {
        if let Integer(x) = eval_ast_node(n, runtime.clone())? {
            product *= x
        } else {
            runtime_error!("Unexpected node during *: {:?}", n)
        }
    };
    Ok(Integer(product))
}

fn func_call(func: &String, nodes: &Vec<ASTNode>, runtime: RuntimeNode) -> Result<Value, RuntimeError> {
    match get_var!(runtime, func) {
        Some(Func(args, body)) => {
            assert_number_of_arguments!(args, func, nodes.len() - 1);

            let func_runtime = scope!(runtime);
            for (arg, node) in args.iter().zip(nodes.iter().skip(1)) {
                let val = eval_ast_node(node, runtime.clone())?;
                set_var!(func_runtime, arg, val);
            }

            Ok(eval(&body, &func_runtime)?)
        },
        Some(other) => runtime_error!("Undefined function call: {:?}", other),
        None => runtime_error!("Unknown function: {}", func)
    }
}


#[test]
fn test_run_simple_expressions() {
    test_assert_run!("1", "1");
    test_assert_run!("#t", "#t");
    test_assert_run!("\"dali\"", "\"dali\"");
    test_assert_run!("(quote (1 2 3))", "'(1 2 3)");
    test_assert_run!("(lambda (x) x)", "#<procedure>");
}

#[test]
fn test_run_plus() {
    test_assert_run!("(+ 2 3)", "5");
}

#[test]
fn test_run_minus() {
    test_assert_run!("(- 9 4)", "5");
}

#[test]
fn test_run_multy() {
    test_assert_run!("(* 5 5 5)", "125");
}

#[test]
fn test_run_nested_arithmetic() {
    test_assert_run!("(+ 2 (+ 1 (- 9 3)))", "9");
}

#[test]
fn test_run_define() {
    test_assert_run!("(define x 3)\n(+ x 2)", "5");
}

#[test]
fn test_run_only_one_var_definition() {
    assert_eq!(
        run("(define x 3)\n(define x 5)", &Runtime::new()).err().unwrap(),
        "RuntimeError: Variable already defined: x"
    )
}

#[test]
fn test_run_set() {
    test_assert_run!("(define x 5) (set! x 4) (/ (+ x 1 2 3) 2)", "5");
}

#[test]
fn test_run_set_undefined() {
    assert_eq!(
        run("(set! x 5)", &Runtime::new()).err().unwrap(),
        "RuntimeError: Can't set! an undefined variable: x"
    )
}

#[test]
fn test_run_define_procedure() {
    test_assert_run!("(define inc (lambda (x) (+ x 1))) (inc 4)", "5");
}

#[test]
fn test_run_if() {
    test_assert_run!("(if #t 5 4)", "5");
    test_assert_run!("(if #f 4 5)", "5");
    test_assert_run!("(if (= 4 4) 4 5)", "4");
}

#[test]
fn test_run_and() {
    test_assert_run!("(and)", "#t");
    test_assert_run!("(and #t)", "#t");
    test_assert_run!("(and 1 2 3)", "3");
    test_assert_run!("(and 2 #f 3)", "#f");
    test_assert_run!("(and 4 #f (error 5))", "#f");
}

#[test]
fn test_run_or() {
    test_assert_run!("(or)", "#f");
    test_assert_run!("(or #f)", "#f");
    test_assert_run!("(or 1 2)", "1");
    test_assert_run!("(or 1 #f)", "1");
    test_assert_run!("(or #f 5)", "5");
    test_assert_run!("(or #f #f)", "#f");
    test_assert_run!("(or 1 (error 5))", "1");
}

#[test]
fn test_run_quote() {
    test_assert_run!("(quote #t)", "#t");
    test_assert_run!("(quote 1)", "1");
    test_assert_run!("(quote a)", "'a");
}

#[test]
fn test_run_syntax_error() {
    assert_eq!(
        run("(33^3)", &Runtime::new()).err().unwrap(),
        "SyntaxError: Unexpected symbol '^'. Expected white space or closing paren. (line: 1, column: 4)"
    )
}

#[test]
fn test_run_error() {
    assert_eq!(
        run("(error 5)", &Runtime::new()).err().unwrap(),
        "RuntimeError: 5"
    )
}
