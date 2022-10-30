use proc_macro::{Group, Ident, Literal, Punct, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let body = parse_token_stream(item);
    let function_data = body.to_parse_function().unwrap();
    format!(
        "fn {}{} -> {} {}",
        function_data.function_name,
        function_data.get_args_string(),
        function_data.return_value_type,
        function_data.function_body.to_string()
    )
    .parse()
    .unwrap()
}

fn parse_token_stream(body: TokenStream) -> ParseTokenStreamData {
    let mut return_data = ParseTokenStreamData::default();
    for tt in body.into_iter() {
        match tt {
            TokenTree::Ident(n) => return_data.ident.push(n),
            TokenTree::Punct(b) => return_data.punct.push(b),
            TokenTree::Literal(l) => return_data.literal.push(l),
            TokenTree::Group(a) => return_data.group.push(a),
        }
    }
    return_data
}

#[derive(Debug, Default)]
struct ParseTokenStreamData {
    pub ident: Vec<Ident>,
    pub punct: Vec<Punct>,
    pub group: Vec<Group>,
    pub literal: Vec<Literal>,
}

#[derive(Debug)]
enum ParseToFunctionError {
    IsNotAFunction,
}

impl ParseTokenStreamData {
    pub fn is_function(&self) -> bool {
        self.ident[0].to_string() == "fn"
    }
    pub fn to_parse_function(&self) -> Result<FunctionTool, ParseToFunctionError> {
        if !self.is_function() {
            return Err(ParseToFunctionError::IsNotAFunction);
        }
        let fn_args: Vec<ArgsSaveType> = {
            let mut arg = Vec::new();
            let data = parse_token_stream(self.group[0].stream()).ident;
            for a in 0..data.len() {
                if a % 2 != 0 {
                    arg.push(ArgsSaveType {
                        name: data[a - 1].to_string(),
                        arg_type: data[a].to_string(),
                        position: arg.len(),
                    })
                }
            }
            arg
        };
        Ok(FunctionTool {
            function_name: self.ident[1].to_string(),
            return_value_type: self.ident[2].to_string(),
            args: fn_args,
            function_body: Group::new(self.group[1].delimiter(), self.group[1].stream()),
        })
    }
}

#[allow(unused)]
#[derive(Debug)]
struct FunctionTool {
    pub function_name: String,
    pub return_value_type: String,
    pub args: Vec<ArgsSaveType>,
    pub function_body: Group,
}

#[allow(unused)]
#[derive(Debug)]
struct ArgsSaveType {
    pub name: String,
    pub arg_type: String,
    pub position: usize,
}

impl FunctionTool {
    pub fn get_args_string(&self) -> String {
        let mut args = "(".to_owned();
        for a in &self.args {
            args.push_str(&format!("{}:{} ,", a.name, a.arg_type))
        }
        args.push_str(")");
        args
    }
}
