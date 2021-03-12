#![feature(box_syntax)]
#![feature(box_patterns)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod instructions;

use std::{
    collections::{hash_map, HashMap, HashSet},
    fmt::write,
};

use pest::{iterators::Pair, Parser};

const ANY_TYPE: &'static str = "Bytes";
const VOID_TYPE: &'static str = "Empty";
const BYTE_TYPE: &'static str = "Byte";
const CHAR_TYPE: &'static str = "Char";
const STRING_TYPE: &'static str = "String";

#[derive(Parser, Debug)]
#[grammar = "cvm gramar.pest"]
struct CVMParser;

enum File {
    Use(String),
    Allow(String, String),
    Function(Function),
}

#[derive(Clone, Debug)]
enum Expression {
    Function(String, Vec<Expression>),
    MethodCall(Box<Expression>, String, Vec<Expression>),
    VariantAccess(Box<Expression>, String),
    // Index(Box<Expression>, Box<Expression>), -> This becomes index(Byte) -> Byte
    Variable(Variable),
    Value(Vec<u8>),
    Type(String),
    Cast(Box<Expression>, String),
}

impl Expression {
    fn compile(&self, ctx: &mut CVMCompCtx, vars: &HashMap<String, usize>) -> usize {
        match self {
            Expression::Function(e, args) => {
                let args_pointer: Vec<usize> = args
                    .iter()
                    .map(|x| {
                        let var = ctx.new_var();
                        let data = x.compile(ctx, vars);
                        ctx.instructions
                            .push(instructions::Instruction::Mov(var, data));
                        var
                    })
                    .collect();
                match e.as_str() {
                    "add" => {
                        let nv = ctx.new_var();
                        ctx.instructions.push(instructions::Instruction::Op(
                            instructions::OperationType::Add,
                            nv,
                            args_pointer[0],
                            args_pointer[1],
                        ));
                        nv
                    }
                    "sub" => {
                        let nv = ctx.new_var();
                        ctx.instructions.push(instructions::Instruction::Op(
                            instructions::OperationType::Sub,
                            nv,
                            args_pointer[0],
                            args_pointer[1],
                        ));
                        nv
                    }
                    "mul" => {
                        let nv = ctx.new_var();
                        ctx.instructions.push(instructions::Instruction::Op(
                            instructions::OperationType::Mul,
                            nv,
                            args_pointer[0],
                            args_pointer[1],
                        ));
                        nv
                    }
                    "merge" => {
                        let nv = ctx.new_var();
                        ctx.instructions.push(instructions::Instruction::Op(
                            instructions::OperationType::Merge,
                            nv,
                            args_pointer[0],
                            args_pointer[1],
                        ));
                        nv
                    }
                    "mod" => {
                        let nv = ctx.new_var();
                        ctx.instructions.push(instructions::Instruction::Op(
                            instructions::OperationType::Mod,
                            nv,
                            args_pointer[0],
                            args_pointer[1],
                        ));
                        nv
                    }
                    "div" => {
                        let nv = ctx.new_var();
                        ctx.instructions.push(instructions::Instruction::Op(
                            instructions::OperationType::Div,
                            nv,
                            args_pointer[0],
                            args_pointer[1],
                        ));
                        nv
                    }
                    "len" => {
                        let nv = ctx.new_var();
                        ctx.instructions
                            .push(instructions::Instruction::Len(nv, args_pointer[0]));
                        nv
                    }
                    "print" => {
                        let nv = ctx.new_var();
                        ctx.instructions
                            .push(instructions::Instruction::Cst(nv, vec![]));
                        ctx.instructions
                            .push(instructions::Instruction::Prt(args_pointer[0]));
                        nv
                    }
                    "input" => {
                        let nv = ctx.new_var();
                        ctx.instructions.push(instructions::Instruction::Inp(nv));
                        nv
                    }
                    "read" => {
                        let nv = ctx.new_var();
                        ctx.instructions.push(instructions::Instruction::Read(
                            nv,
                            args_pointer[0],
                            args_pointer[1],
                            args_pointer[2],
                        ));
                        nv
                    }
                    _ => {
                        let func = (*ctx
                            .ctx
                            .functions
                            .get(e)
                            .unwrap()
                            .get_for_input(
                                &args
                                    .iter()
                                    .map(|x| x.get_type(&ctx.ctx))
                                    .collect::<Vec<_>>(),
                                &ctx.ctx,
                            )
                            .expect("Can't get function"))
                        .clone();
                        func.compile(ctx, &args_pointer, None)
                    }
                }
            }
            Expression::MethodCall(a, b, c) => {
                if let box Expression::Type(a) = a {
                    let args: Vec<usize> = c
                        .iter()
                        .map(|x| {
                            let var = ctx.new_var();
                            let tmp = x.compile(ctx, vars);
                            ctx.instructions
                                .push(instructions::Instruction::Mov(var, tmp));
                            var
                        })
                        .collect();
                    let func = (*ctx
                        .ctx
                        .types
                        .get(a)
                        .expect("Can't get type in static method call")
                        .get_function(
                            b,
                            &c.iter().map(|x| x.get_type(&ctx.ctx).to_owned()).collect(),
                            true,
                            &ctx.ctx,
                        )
                        .expect("Can't get static method"))
                    .clone();
                    func.compile(ctx, &args, None)
                } else {
                    let args: Vec<usize> = c
                        .iter()
                        .map(|x| {
                            let var = ctx.new_var();
                            let data = x.compile(ctx, vars);
                            ctx.instructions
                                .push(instructions::Instruction::Mov(var, data));
                            var
                        })
                        .collect();
                    let compiled = a.compile(ctx, vars);
                    let self_var = ctx.new_var();
                    let func = (*ctx
                        .ctx
                        .types
                        .get(a.get_type(&ctx.ctx))
                        .expect("Can't get type in static method call")
                        .get_function(
                            b,
                            &c.iter().map(|x| x.get_type(&ctx.ctx).to_owned()).collect(),
                            false,
                            &ctx.ctx,
                        )
                        .expect("Can't get method"))
                    .clone();
                    ctx.instructions
                        .push(instructions::Instruction::Mov(self_var, compiled));
                    func.compile(ctx, &args, Some(self_var))
                }
            }
            Expression::Variable(e) => *vars.get(&e.name).unwrap(),
            Expression::Value(e) => {
                let var = ctx.new_var();
                ctx.instructions
                    .push(instructions::Instruction::Cst(var, e.clone()));
                var
            }
            Expression::Type(e) => {
                println!("TYPE compile shouldn't be called");
                0
            }

            Expression::Cast(e, _) => e.compile(ctx, vars),
            Expression::VariantAccess(e, b) => {
                let var = ctx.new_var();
                let i = if let box Expression::Type(e) = e {
                    e
                } else {
                    panic!("Variants can only be used on types not expressions {:?}", e)
                };
                ctx.instructions.push(instructions::Instruction::Cst(
                    var,
                    ctx.ctx
                        .types
                        .get(i)
                        .unwrap()
                        .variants
                        .get(b)
                        .unwrap()
                        .clone(),
                ));
                var
            }
        }
    }
}

#[derive(Default, Debug)]
struct CompilationContext {
    types: HashMap<String, Type>,
    functions: HashMap<String, Functions>,
}

impl CompilationContext {
    fn add_function(&mut self, function: Function) {
        if let Some(e) = self.functions.get_mut(&function.name) {
            e.functions.push(function);
        } else {
            self.functions.insert(
                function.name.clone(),
                Functions {
                    name: function.name.clone(),
                    functions: vec![function],
                },
            );
        }
    }
}

impl Expression {
    pub fn get_type<'a>(&'a self, context: &'a CompilationContext) -> &'a str {
        match self {
            Expression::Function(name, arguments) => {
                if let Some(e) = context.functions.get(name) {
                    let args: Vec<&'a str> =
                        arguments.iter().map(|x| x.get_type(context)).collect();
                    &e.get_for_input(&args, context)
                        .expect("Can't find function matching type arguments")
                        .return_type
                } else {
                    panic!("Function {} not found", name);
                }
            }
            Expression::MethodCall(expr, method_name, arguments) => {
                let args: Vec<String> = arguments
                    .iter()
                    .map(|x| x.get_type(context).to_owned())
                    .collect();
                let expr_type = expr.get_type(context);
                if let Some(e) = context
                    .types
                    .get(expr_type)
                    .expect("Can't get type for method call")
                    .get_function(method_name, &args, false, context)
                {
                    &e.return_type
                } else {
                    panic!("Function {} not found for type {}", method_name, expr_type);
                }
            }
            Expression::Variable(e) => &e.var_type,
            Expression::Value(e) => {
                if e.len() == 1 {
                    if is_ascii(e) {
                        CHAR_TYPE
                    } else {
                        BYTE_TYPE
                    }
                } else {
                    if is_ascii(e) {
                        STRING_TYPE
                    } else {
                        ANY_TYPE
                    }
                }
            }
            Expression::Type(e) => e,
            Expression::Cast(_, e) => e,
            Expression::VariantAccess(e, _) => e.get_type(context),
        }
    }
}

fn is_ascii(array: &[u8]) -> bool {
    !array.iter().any(|x| (*x < 32 && *x != 10) || *x > 126)
}

#[derive(Debug, Clone)]
enum Instruction {
    If(
        Expression,
        bool,
        Expression,
        Vec<Instruction>,
        Option<Vec<Instruction>>,
    ),
    Expression(Expression),
    ForEach(Variable, Expression, Vec<Instruction>),
    ForRange(Variable, Expression, Expression, Vec<Instruction>),
    Loop(Vec<Instruction>),
    Break,
    Continue,
    Assign(Variable, Expression),
    Return(Expression),
}

struct CVMCompCtx {
    ctx: CompilationContext,
    current_var: usize,
    function: usize,
    loops: Vec<usize>,
    loop_id: usize,
    if_n: usize,
    instructions: Vec<instructions::Instruction>,
}

impl CVMCompCtx {
    fn new_function(&mut self) -> usize {
        self.function += 1;
        self.function
    }

    fn new_loop(&mut self) -> usize {
        self.loop_id += 1;
        self.loop_id
    }

    fn new_var(&mut self) -> usize {
        self.current_var += 1;
        self.current_var
    }

    fn new_if(&mut self) -> usize {
        self.if_n += 1;
        self.if_n
    }
}

impl Instruction {
    fn compile(
        &self,
        ctx: &mut CVMCompCtx,
        function_data: (usize, usize), /* Function id, Function return var location */
        vars: &mut HashMap<String, usize>,
    ) {
        match self {
            Instruction::If(a, b, c, d, e) => {
                let if_id = ctx.new_if();
                let expr1 = ctx.new_var();
                let expr1_r = a.compile(ctx, vars);
                ctx.instructions
                    .push(instructions::Instruction::Mov(expr1, expr1_r));
                let expr2 = ctx.new_var();
                let expr2_r = c.compile(ctx, vars);
                ctx.instructions
                    .push(instructions::Instruction::Mov(expr2, expr2_r));
                ctx.instructions
                    .push(instructions::Instruction::If(!*b, expr1, expr2));
                ctx.instructions
                    .push(instructions::Instruction::GtLabel(format!(
                        "if_else_{}",
                        if_id
                    )));
                d.iter().for_each(|x| x.compile(ctx, function_data, vars));
                ctx.instructions
                    .push(instructions::Instruction::GtLabel(format!(
                        "if_end_{}",
                        if_id
                    )));
                ctx.instructions
                    .push(instructions::Instruction::Label(format!(
                        "if_else_{}",
                        if_id
                    )));
                if let Some(e) = e {
                    e.iter().for_each(|x| x.compile(ctx, function_data, vars));
                }
                ctx.instructions
                    .push(instructions::Instruction::Label(format!(
                        "if_end_{}",
                        if_id
                    )));
            }
            Instruction::Expression(e) => {
                e.compile(ctx, vars);
            }
            Instruction::Assign(e, a) => {
                let expr1 = vars.get(&e.name).cloned().unwrap_or_else(|| ctx.new_var());
                let p = a.get_type(&ctx.ctx);
                if !ctx
                    .ctx
                    .types
                    .get(p)
                    .unwrap()
                    .is_child_of(&e.var_type, &ctx.ctx)
                {
                    panic!("Can't assign a {} with {}", e.var_type, p);
                }
                let expr1_r = a.compile(ctx, vars);
                ctx.instructions
                    .push(instructions::Instruction::Mov(expr1, expr1_r));
                vars.insert(e.name.to_owned(), expr1);
            }
            Instruction::Return(e) => {
                let expr1_r = e.compile(ctx, vars);
                ctx.instructions
                    .push(instructions::Instruction::Mov(function_data.1, expr1_r));
                ctx.instructions
                    .push(instructions::Instruction::GtLabel(format!(
                        "fn_end_{}",
                        function_data.0
                    )));
            }
            Instruction::Loop(e) => {
                let loop_id = ctx.new_loop();
                ctx.loops.push(loop_id);
                ctx.instructions
                    .push(instructions::Instruction::Label(format!(
                        "loop_start_{}",
                        loop_id
                    )));
                e.iter().for_each(|x| x.compile(ctx, function_data, vars));
                ctx.instructions
                    .push(instructions::Instruction::GtLabel(format!(
                        "loop_start_{}",
                        loop_id
                    )));
                ctx.instructions
                    .push(instructions::Instruction::Label(format!(
                        "loop_end_{}",
                        loop_id
                    )));
                ctx.loops.pop();
            }
            Instruction::Break => {
                ctx.instructions
                    .push(instructions::Instruction::GtLabel(format!(
                        "loop_end_{}",
                        ctx.loops.last().unwrap()
                    )));
            }
            Instruction::Continue => {
                ctx.instructions
                    .push(instructions::Instruction::GtLabel(format!(
                        "loop_start_{}",
                        ctx.loops.last().unwrap()
                    )));
            }
            Instruction::ForEach(a, b, c) => {
                let for_id = ctx.new_if();
                let b_type = b.get_type(&ctx.ctx);
                let object = Variable {
                    name: format!("~foreach_object_{}", for_id),
                    var_type: b_type.to_owned(),
                };
                let objectlen = Variable {
                    name: format!("~foreach_objectlen_{}", for_id),
                    var_type: BYTE_TYPE.to_owned(),
                };
                let index = Variable {
                    name: format!("~foreach_index_{}", for_id),
                    var_type: BYTE_TYPE.to_owned(),
                };
                Instruction::Assign(object.clone(), b.clone()).compile(ctx, function_data, vars);
                Instruction::Assign(
                    objectlen.clone(),
                    Expression::MethodCall(
                        box Expression::Variable(object.clone()),
                        "len".to_owned(),
                        vec![],
                    ),
                )
                .compile(ctx, function_data, vars);
                Instruction::Assign(index.clone(), Expression::Value(vec![0])).compile(
                    ctx,
                    function_data,
                    vars,
                );
                Instruction::Loop({
                    let mut ol = Vec::new();
                    ol.push(Instruction::If(
                        Expression::Variable(objectlen.clone()),
                        true,
                        Expression::Variable(index.clone()),
                        vec![Instruction::Break],
                        None,
                    ));
                    ol.push(Instruction::Assign(
                        a.clone(),
                        Expression::MethodCall(
                            box Expression::Variable(object.clone()),
                            "index".to_owned(),
                            vec![Expression::Variable(index.clone())],
                        ),
                    ));
                    ol.push(Instruction::Assign(
                        index.clone(),
                        Expression::MethodCall(
                            box Expression::Variable(index.clone()),
                            "add".to_owned(),
                            vec![Expression::Value(vec![1])],
                        ),
                    ));
                    ol.append(&mut c.clone());
                    ol
                })
                .compile(ctx, function_data, vars);
            }
            Instruction::ForRange(typed_var, expr1, expr2, code) => {
                let for_id = ctx.new_if();
                let expr1_var = Variable {
                    name: format!("~for_range1_{}", for_id),
                    var_type: expr1.get_type(&ctx.ctx).to_owned(),
                };
                let expr2_var = Variable {
                    name: format!("~for_range2_{}", for_id),
                    var_type: expr2.get_type(&ctx.ctx).to_owned(),
                };
                Instruction::Assign(expr1_var.clone(), expr1.clone()).compile(
                    ctx,
                    function_data,
                    vars,
                );
                Instruction::Assign(expr2_var.clone(), expr2.clone()).compile(
                    ctx,
                    function_data,
                    vars,
                );
                Instruction::Loop({
                    let mut lo = Vec::new();
                    lo.push(Instruction::If(
                        Expression::Variable(expr1_var.clone()),
                        true,
                        Expression::Variable(expr2_var.clone()),
                        vec![Instruction::Break],
                        None,
                    ));
                    lo.push(Instruction::Assign(
                        typed_var.clone(),
                        Expression::Variable(expr1_var.clone()),
                    ));
                    lo.push(Instruction::Assign(
                        expr1_var.clone(),
                        Expression::MethodCall(
                            box Expression::Variable(expr1_var.clone()),
                            "add".to_owned(),
                            vec![Expression::Value(vec![1])],
                        ),
                    ));
                    lo.append(&mut code.clone());
                    lo
                })
                .compile(ctx, function_data, vars);
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Variable {
    name: String,
    var_type: String,
}

impl Variable {
    fn parse(pair: Pair<Rule>) -> Self {
        let mut i = pair.into_inner();
        Self {
            var_type: i.next().unwrap().as_str().trim().to_owned(),
            name: i.next().unwrap().as_str().trim().to_owned(),
        }
    }

    fn is_child_of(&self, other_type: &str, ctx: &CompilationContext) -> bool {
        ctx.types
            .get(&self.var_type)
            .unwrap()
            .is_child_of(other_type, ctx)
    }
}

#[derive(Debug, Clone)]
struct Function {
    name: String,
    arguments: Vec<Variable>,
    return_type: String,
    code: Vec<Instruction>,
}

impl Function {
    fn compile(&self, ctx: &mut CVMCompCtx, pos: &[usize], self_pos: Option<usize>) -> usize {
        let id = ctx.new_function();
        let return_var = ctx.new_var();
        let mut vars = HashMap::new();
        vars.extend(
            self.arguments
                .iter()
                .zip(pos.iter())
                .map(|(x, y)| (x.name.to_owned(), *y)),
        );
        if let Some(e) = self_pos {
            vars.insert("self".to_owned(), e);
        }
        for i in &self.code {
            i.compile(ctx, (id, return_var), &mut vars);
        }
        ctx.instructions
            .push(instructions::Instruction::Label(format!("fn_end_{}", id)));
        return_var
    }
}

#[derive(Debug)]
struct Functions {
    name: String,
    functions: Vec<Function>,
}

impl Functions {
    fn get_for_input<'a>(
        &'a self,
        arguments: &[&str],
        context: &CompilationContext,
    ) -> Option<&'a Function> {
        self.functions.iter().find(|x| {
            if x.arguments.len() != arguments.len() {
                return false;
            }
            !x.arguments.iter().zip(arguments.iter()).any(|(x, y)| {
                !(x.var_type == ANY_TYPE
                    || &x.var_type == y
                    || context.types.get(*y).unwrap().is_child_of(&x.name, context))
            })
        })
    }
}

#[derive(Debug)]
struct Type {
    name: String,
    size: Option<u8>,
    variants: HashMap<String, Vec<u8>>,
    functions: HashMap<String, Functions>,
    parent: String,
    static_functions: HashMap<String, Functions>,
}

impl Type {
    fn is_child_of(&self, other_type: &str, ctx: &CompilationContext) -> bool {
        if other_type == self.name {
            return true;
        }
        if self.name == ANY_TYPE {
            return false;
        }
        ctx.types
            .get(&self.parent)
            .unwrap()
            .is_child_of(other_type, ctx)
    }

    fn add_static_function(&mut self, function: Function) {
        if let Some(e) = self.static_functions.get_mut(&function.name) {
            e.functions.push(function);
        } else {
            self.static_functions.insert(
                function.name.clone(),
                Functions {
                    name: function.name.clone(),
                    functions: vec![function],
                },
            );
        }
    }

    fn get_function<'a>(
        &'a self,
        a: &str,
        types: &Vec<String>,
        is_static: bool,
        ctx: &'a CompilationContext,
    ) -> Option<&'a Function> {
        if let Some(e) = if is_static {
            &self.static_functions
        } else {
            &self.functions
        }
        .get(a)
        .map(|x| {
            x.get_for_input(
                &types.iter().map(|x| x.as_str()).collect::<Vec<&str>>(),
                ctx,
            )
        })
        .flatten()
        {
            return Some(e);
        }
        if &self.parent == &self.name {
            println!("Exiting function search due to parent");
            return None;
        }
        println!("Searching in parent {} for {}", self.parent, a);
        ctx.types
            .get(&self.parent)
            .unwrap()
            .get_function(a, types, is_static, ctx)
    }

    fn add_function(&mut self, function: Function) {
        if let Some(e) = self.functions.get_mut(&function.name) {
            e.functions.push(function);
        } else {
            self.functions.insert(
                function.name.clone(),
                Functions {
                    name: function.name.clone(),
                    functions: vec![function],
                },
            );
        }
    }
}

#[derive(Debug)]
struct ParseExpressionContext {
    variables: HashMap<String, Variable>,
    types: HashSet<String>,
    current_for: usize,
}

impl ParseExpressionContext {
    fn new_for(&mut self) -> usize {
        self.current_for += 1;
        self.current_for
    }
}

fn parse_type_insides(inside: Pair<Rule>, context: &mut CompilationContext, name: &str) {
    let inside = inside.into_inner().next().unwrap();
    if inside.as_rule() == Rule::type_function {
        parse_type_function(inside, context, name);
    } else if inside.as_rule() == Rule::type_ref {
        let mut inside = inside.into_inner();
        let ref_name = inside.next().unwrap().as_str().trim();
        let ref_value = inside.skip(1).next().unwrap();
        let ref_value = match ref_value.as_rule() {
            Rule::string => ref_value
                .into_inner()
                .next()
                .unwrap()
                .as_str()
                .as_bytes()
                .to_vec(),
            Rule::number_array => ref_value
                .into_inner()
                .map(|x| x.as_str().trim().parse().expect("Not a u8"))
                .collect(),
            _ => unreachable!(),
        };
        context
            .types
            .get_mut(name)
            .unwrap()
            .variants
            .insert(ref_name.to_owned(), ref_value);
        println!("REFERING1");
    } else {
        panic!("Invalid type_inside token {:?}", inside)
    }
}

fn file_parser(cvm: Pair<Rule>, context: &mut CompilationContext) {
    match cvm.as_rule() {
        Rule::file_element => file_parser(cvm.into_inner().next().unwrap(), context),
        Rule::file => cvm.into_inner().for_each(|x| file_parser(x, context)),
        Rule::line => file_parser(cvm.into_inner().next().unwrap(), context),
        Rule::use_statement => {
            println!("Uses aren't implemented yet");
        }
        Rule::function => {
            context.add_function(parse_function(cvm, None, context));
        }
        Rule::type_statement => {
            let mut cvm = cvm.into_inner();
            let name = cvm.next().unwrap().as_str().trim();
            context.types.insert(
                name.to_owned(),
                Type {
                    variants: HashMap::new(),
                    name: name.to_owned(),
                    parent: ANY_TYPE.to_owned(),
                    size: None,
                    functions: HashMap::new(),
                    static_functions: HashMap::new(),
                },
            );
            let p = cvm.next().unwrap();
            let p = if Rule::literal == p.as_rule() {
                context.types.get_mut(name).unwrap().parent = p.as_str().trim().to_owned();
                cvm.next().unwrap()
            } else {
                p
            };
            let p = if Rule::equal == p.as_rule() {
                context.types.get_mut(name).unwrap().size =
                    Some(cvm.next().unwrap().as_str().trim().parse().unwrap());
                cvm.next().unwrap()
            } else {
                p
            };
            parse_type_insides(p, context, name);
            cvm.for_each(|x| parse_type_insides(x, context, name));
        }
        e => {
            panic!("Unexpected {:?} token in file parse", e)
        }
    }
}

fn parse_type_function(cvm: Pair<Rule>, context: &mut CompilationContext, type_name: &str) {
    let mut cvm = cvm.into_inner();
    let a = cvm.next().unwrap();
    let is_static = a.as_rule() == Rule::keyword_static;
    if is_static {
        let func = parse_function(cvm.next().unwrap(), None, context);
        context
            .types
            .get_mut(type_name)
            .expect("Can't get type")
            .add_static_function(func);
    } else {
        let func = parse_function(a, Some(type_name.to_owned()), context);
        context
            .types
            .get_mut(type_name)
            .expect("Can't get type")
            .add_function(func);
    };
}

fn parse_function(
    cvm: Pair<Rule>,
    in_type: Option<String>,
    context: &CompilationContext,
) -> Function {
    let mut cvm = cvm.into_inner();
    let name = cvm.next().unwrap().as_str().trim().to_owned();
    let arguments = cvm
        .next()
        .unwrap()
        .into_inner()
        .map(|x| {
            let mut cvms = x.into_inner();
            Variable {
                var_type: cvms.next().unwrap().as_str().trim().to_owned(),
                name: cvms.next().unwrap().as_str().trim().to_owned(),
            }
        })
        .collect::<Vec<Variable>>();
    let c = cvm.next().unwrap();
    let (return_type, c) = if c.as_rule() == Rule::literal {
        (c.as_str().trim().to_owned(), cvm.next().unwrap())
    } else {
        (VOID_TYPE.to_owned(), c)
    };
    let mut context = ParseExpressionContext {
        types: context.types.iter().map(|(x, _)| x.to_owned()).collect(),
        variables: {
            let mut map = HashMap::new();
            if let Some(e) = in_type {
                map.insert(
                    "self".to_owned(),
                    Variable {
                        name: "self".to_owned(),
                        var_type: e,
                    },
                );
            }
            map.extend(arguments.iter().map(|x| (x.name.to_owned(), x.clone())));
            map
        },
        current_for: 0,
    };
    Function {
        name,
        arguments,
        return_type,
        code: c
            .into_inner()
            .flat_map(|x| parse_instructions(x, &mut context))
            .collect(),
    }
}

fn parse_instructions(cvm: Pair<Rule>, context: &mut ParseExpressionContext) -> Vec<Instruction> {
    vec![match cvm.as_rule() {
        Rule::instruction => return parse_instructions(cvm.into_inner().next().unwrap(), context),
        Rule::if_statement => {
            let mut inner = cvm.into_inner();
            let mut boolean_test = inner.next().unwrap().into_inner();
            let a = boolean_test.next().unwrap();
            let (inverted, a) = if a.as_rule() == Rule::not {
                (true, boolean_test.next().unwrap())
            } else {
                (false, a)
            };
            let a = parse_expression(a, context);
            let b = boolean_test
                .next()
                .map(|x| {
                    if inverted {
                        x.as_rule() != Rule::double_equal
                    } else {
                        x.as_rule() == Rule::double_equal
                    }
                })
                .unwrap_or(!inverted);
            let c = boolean_test
                .next()
                .map(|x| parse_expression(x, context))
                .unwrap_or_else(|| {
                    Expression::VariantAccess(
                        box Expression::Type("Boolean".to_owned()),
                        "true".to_owned(),
                    )
                });
            Instruction::If(
                a,
                b,
                c,
                inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .flat_map(|x| parse_instructions(x, context))
                    .collect(),
                inner.next().map(|y| {
                    y.into_inner()
                        .flat_map(|x| parse_instructions(x, context))
                        .collect()
                }),
            )
        }
        Rule::expr => Instruction::Expression(parse_expression(cvm, context)),
        Rule::return_statement => {
            Instruction::Return(parse_expression(cvm.into_inner().next().unwrap(), context))
        }
        Rule::var_declaration => {
            let mut inner = cvm.into_inner();
            let mut def = inner.next().unwrap().into_inner();
            let var_type = def.next().unwrap().as_str().trim().to_owned();
            let var_name = def.next().unwrap().as_str().trim().to_owned();
            let var = Variable {
                name: var_name.clone(),
                var_type: var_type,
            };
            context.variables.insert(var_name, var.clone());
            Instruction::Assign(
                var,
                parse_expression(inner.skip(1).next().unwrap(), context),
            )
        }
        Rule::var_assignement => {
            let mut inner = cvm.into_inner();
            let a = inner.next().unwrap();
            let b = inner.next().unwrap();
            let c = inner.next().unwrap();
            let d = inner.next();
            if let Some(d) = d {
                if a.as_rule() == Rule::literal {
                    let a = context
                        .variables
                        .get(a.as_str().trim())
                        .expect("Can't find variable")
                        .clone();
                    Instruction::Assign(
                        a.clone(),
                        Expression::MethodCall(
                            box Expression::Variable(a),
                            match b.as_rule() {
                                Rule::add => "add",
                                Rule::subtract => "sub",
                                Rule::multiply => "mul",
                                Rule::divide => "div",
                                Rule::xor => "xor",
                                Rule::merge => "merge",
                                e => panic!("Invalid operator {:?}", e),
                            }
                            .to_owned(),
                            vec![parse_expression(d, context)],
                        ),
                    )
                } else {
                    let mut a = a.into_inner();
                    let i = context
                        .variables
                        .get(a.next().unwrap().as_str().trim())
                        .expect("Can't find variable")
                        .clone();
                    let inside = parse_expression(a.next().unwrap(), context);
                    Instruction::Assign(
                        i.clone(),
                        Expression::MethodCall(
                            box Expression::Variable(i.clone()),
                            "replace".to_owned(),
                            vec![Expression::MethodCall(
                                box Expression::MethodCall(
                                    box Expression::Variable(i),
                                    "index".to_owned(),
                                    vec![inside.clone()],
                                ),
                                match b.as_rule() {
                                    Rule::add => "add",
                                    Rule::subtract => "sub",
                                    Rule::multiply => "mul",
                                    Rule::divide => "div",
                                    Rule::xor => "xor",
                                    Rule::merge => "merge",
                                    e => panic!("Invalid operator {:?}", e),
                                }
                                .to_owned(),
                                vec![parse_expression(d, context)],
                            )],
                        ),
                    )
                }
            } else {
                if a.as_rule() == Rule::literal {
                    Instruction::Assign(
                        context
                            .variables
                            .get(a.as_str().trim())
                            .expect("Can't find variable")
                            .clone(),
                        parse_expression(c, context),
                    )
                } else {
                    let mut a = a.into_inner();
                    let b = context
                        .variables
                        .get(a.next().unwrap().as_str().trim())
                        .expect("Can't find variable")
                        .clone();
                    Instruction::Assign(
                        b.clone(),
                        Expression::MethodCall(
                            box Expression::Variable(b),
                            "replace".to_owned(),
                            vec![parse_expression(a.next().unwrap(), context)],
                        ),
                    )
                }
            }
        }
        Rule::loop_statement => Instruction::Loop(
            cvm.into_inner()
                .next()
                .unwrap()
                .into_inner()
                .flat_map(|x| parse_instructions(x, context))
                .collect(),
        ),
        Rule::break_instruction => Instruction::Break,
        Rule::continue_instruction => Instruction::Continue,
        Rule::comment => return vec![],
        Rule::for_statement => {
            let mut inner = cvm.into_inner();
            let typed_var = Variable::parse(inner.next().unwrap());
            let expr = inner.next().unwrap();
            let block = inner.next().unwrap();
            let for_id = context.new_for();
            match expr.as_rule() {
                Rule::range => {
                    let mut expr = expr.into_inner();
                    let expr1 = parse_expression(expr.next().unwrap(), context);
                    let expr2 = parse_expression(expr.next().unwrap(), context);

                    context
                        .variables
                        .insert(typed_var.name.to_owned(), typed_var.clone());
                    return vec![Instruction::ForRange(
                        typed_var,
                        expr1,
                        expr2,
                        block
                            .into_inner()
                            .next()
                            .unwrap()
                            .into_inner()
                            .flat_map(|x| parse_instructions(x, context))
                            .collect(),
                    )];
                }
                Rule::expr => {
                    return vec![Instruction::ForEach(
                        typed_var,
                        parse_expression(expr, context),
                        block
                            .into_inner()
                            .next()
                            .unwrap()
                            .into_inner()
                            .flat_map(|x| parse_instructions(x, context))
                            .collect(),
                    )]
                }
                _ => (),
            }
            return vec![];
        }
        e => {
            panic!("Unexpected token in instruction {:?}", e)
        }
    }]
}

// Rule::expr
fn parse_expression(cvm: Pair<Rule>, context: &ParseExpressionContext) -> Expression {
    match cvm.as_rule() {
        Rule::expr => {
            let mut expr = cvm.into_inner();
            let mut ex = parse_expression(expr.next().unwrap(), context);
            let mut last_operand = None;

            let mut next_lit = false;

            for i in expr {
                if i.as_rule() == Rule::cdot {
                    next_lit = true;
                    continue;
                }
                if next_lit {
                    next_lit = false;
                    ex = Expression::VariantAccess(box ex, i.as_str().trim().to_owned());
                    continue;
                }
                if i.as_rule() == Rule::indexing {
                    let inner = i.into_inner().next().unwrap();
                    if inner.as_rule() == Rule::expr {
                        ex = Expression::MethodCall(
                            Box::new(ex),
                            "index".to_owned(),
                            vec![parse_expression(inner, context)],
                        );
                    } else {
                        let mut inner = inner.into_inner();
                        ex = Expression::MethodCall(
                            Box::new(ex),
                            "index_range".to_owned(),
                            vec![
                                parse_expression(inner.next().unwrap(), context),
                                parse_expression(inner.next().unwrap(), context),
                            ],
                        );
                    }
                    continue;
                }
                if i.as_rule() == Rule::method_call {
                    let mut method = i.into_inner();
                    ex = Expression::MethodCall(
                        Box::new(ex),
                        method.next().unwrap().as_str().trim().to_owned(),
                        method
                            .next()
                            .unwrap()
                            .into_inner()
                            .map(|x| parse_expression(x, context))
                            .collect(),
                    );
                } else {
                    if let Some(e) = last_operand {
                        ex = Expression::MethodCall(
                            Box::new(ex),
                            match e {
                                Rule::add => "add",
                                Rule::subtract => "sub",
                                Rule::multiply => "mul",
                                Rule::divide => "div",
                                Rule::xor => "xor",
                                Rule::merge => "merge",
                                _ => unreachable!(),
                            }
                            .to_owned(),
                            vec![parse_expression(i, context)],
                        );
                        last_operand = None;
                    } else {
                        if matches!(
                            i.as_rule(),
                            Rule::add
                                | Rule::subtract
                                | Rule::multiply
                                | Rule::divide
                                | Rule::xor
                                | Rule::merge
                        ) {
                            last_operand = Some(i.as_rule())
                        } else if i.as_rule() == Rule::literal {
                            ex = Expression::Cast(box ex, i.as_str().trim().to_owned());
                        } else {
                            panic!("Invalid operator {:?}", i)
                        }
                    }
                }
            }
            ex
        }
        Rule::literal => {
            let st = cvm.as_str().trim();
            if let Some(e) = context.variables.get(st) {
                Expression::Variable(e.clone())
            } else if context.types.contains(st) {
                Expression::Type(st.to_owned())
            } else {
                panic!("Variable or type `{}` not found\n{}", st, cvm);
            }
        }
        Rule::number_array => Expression::Value(
            cvm.into_inner()
                .map(|x| x.as_str().trim().parse().expect("Not a u8"))
                .collect(),
        ),
        Rule::string => Expression::Value(
            cvm.into_inner()
                .next()
                .unwrap()
                .as_str()
                .as_bytes()
                .to_vec(),
        ),
        Rule::indexing => {
            let mut v = cvm.into_inner();
            Expression::MethodCall(
                Box::new(parse_expression(v.next().unwrap(), context)),
                "index".to_owned(),
                vec![parse_expression(v.next().unwrap(), context)],
            )
        }
        Rule::function_call => {
            let mut v = cvm.into_inner();
            Expression::Function(
                v.next().unwrap().as_str().to_owned(),
                v.next()
                    .unwrap()
                    .into_inner()
                    .map(|x| parse_expression(x, context))
                    .collect(),
            )
        }
        e => {
            panic!("This token wasn't expected in expression {:?}", e)
        }
    }
}

fn main() {
    let file = std::fs::read_to_string("cvm.cvm").unwrap();
    let json = match CVMParser::parse(Rule::line, &file) {
        Ok(e) => e,
        Err(e) => {
            println!("{}", e);
            panic!();
        }
    }
    .next()
    .unwrap();
    let mut context = CompilationContext::default();
    // context.types.insert(ANY_TYPE.to_owned(), Type {
    //     functions: HashMap::new(),
    //     allowed_from: Vec::new(),
    //     static_functions: HashMap::new(),
    // });
    file_parser(json, &mut context);

    let func = context
        .functions
        .get("main")
        .unwrap()
        .get_for_input(&Vec::new(), &context)
        .unwrap()
        .clone();
    let mut cctx = CVMCompCtx {
        ctx: context,
        current_var: 0,
        function: 0,
        loops: Vec::new(),
        loop_id: 0,
        if_n: 0,
        instructions: Vec::new(),
    };
    func.compile(&mut cctx, &Vec::new(), None);
    let out = instructions::Instruction::clean(cctx.instructions)
        .iter()
        .chain(vec![instructions::Instruction::End].iter())
        .enumerate()
        .map(|(i, x)| format!("l{} {}", i, x.to_raw()))
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write("new.cbm", &out);
    println!("{}", out);
}
