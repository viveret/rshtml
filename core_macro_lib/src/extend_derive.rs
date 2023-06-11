use std::cell::RefCell;
use std::iter::Peekable;
use std::vec;

use proc_macro2::Group;
use proc_macro2::Punct;
use proc_macro2::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenTree;
use quote::quote;

use crate::ast::method::AstMethod;


pub struct ExtendDerive<'a> {
    pub struct_attrs_tokens: Vec<TokenTree>,
    pub struct_attrs_simple: Vec<(String, String)>,
    pub struct_attrs: Vec<(Punct, Ident, Option<Group>)>,

    pub struct_vis: Option<Ident>,
    pub struct_type: Option<Ident>,
    pub struct_name: Option<Ident>,
    pub struct_generics: TokenStream,
    pub struct_where_clause: TokenStream,
    pub struct_inner: Option<Group>,
    pub struct_semi: Option<Punct>,

    tokens_to_append: RefCell<Vec<TokenTree>>,

    prepend_processors: RefCell<Vec<&'a dyn Fn(&ExtendDerive) -> Vec<TokenTree>>>,
    append_processors: RefCell<Vec<&'a dyn Fn(&ExtendDerive) -> Vec<TokenTree>>>,
    inner_processors: RefCell<Vec<&'a dyn Fn(&ExtendDerive) -> Vec<TokenTree>>>,
}

impl<'a> ExtendDerive<'a> {
    pub fn new(
        struct_attrs_tokens: Vec<TokenTree>,
        struct_vis: Option<Ident>,
        struct_type: Option<Ident>,
        struct_name: Option<Ident>,
        struct_generics: TokenStream,
        struct_where_clause: TokenStream,
        struct_inner: Option<Group>,
        struct_semi: Option<Punct>,
        struct_attrs: Vec<(Punct, Ident, Option<Group>)>,
    ) -> Self {
        Self {
            struct_attrs_tokens,
            struct_vis,
            struct_type,
            struct_name,
            struct_generics,
            struct_where_clause,
            struct_inner,
            struct_semi,
            struct_attrs_simple: struct_attrs.iter().map(|x| {
                (
                    x.1.to_string(),
                    if let Some(g) = &x.2 { g.to_string() } else { "".to_string() }
                )
            }).collect::<Vec<(String, String)>>(),
            struct_attrs,
            tokens_to_append: RefCell::new(vec![]),
            prepend_processors: std::cell::RefCell::new(vec![]),
            append_processors: std::cell::RefCell::new(vec![]),
            inner_processors: std::cell::RefCell::new(vec![]),
        }
    }

    pub fn parse(
        attr: TokenStream, item: TokenStream
    ) -> Result<Self, std::io::Error> {
        let mut it = TokenStream::from(item).into_iter().peekable();
        let mut struct_attrs_tokens = vec![];
        let mut struct_vis: Option<Ident> = None;
        let mut struct_type: Option<Ident> = None;
        let mut struct_name: Option<Ident> = None;
        let mut struct_generics = quote!{};
        let mut struct_where_clause = quote!{};
        let mut struct_inner: Option<Group> = None;
        let mut struct_semi: Option<Punct> = None;

        let mut struct_attrs = vec![];
    
        // check for attributes before struct
        loop {
            let attr_start_punct = if let Some(token) = it.peek() {
                // println!("peek attributes: {:?}", token);
                match token {
                    proc_macro2::TokenTree::Punct(punct) => {
                        if punct.as_char() == '#' {
                            struct_attrs_tokens.push(token.clone());
                            punct.clone()
                        } else {
                            break;
                        }
                    }
                    _ => {
                        break;
                    },
                }
            } else {
                break;
            };
            it.next(); // since we parsed the punct, move to next token

            // enter into attribute group
            if let Some(token) = it.next() {
                match &token {
                    proc_macro2::TokenTree::Group(group) => {
                        struct_attrs_tokens.push(token.clone());
                        let mut it = group.stream().into_iter().peekable();
                        let name = if let Some(name_token) = it.next() {
                            match name_token.clone() {
                                proc_macro2::TokenTree::Ident(name_ident) => {
                                    name_ident
                                },
                                _ => {
                                    panic!("Expected attribute name, not {:?}.", name_token);
                                }
                            }
                        } else { panic!("Expected attribute name."); };
                        
                        let contents_group = if let Some(contents_token) = it.peek() {
                            match contents_token {
                                proc_macro2::TokenTree::Group(group) => {
                                    Some(group)
                                },
                                _ => {
                                    panic!("Expected attribute contents group, not {:?}.", contents_token);
                                }
                            }
                        } else {
                            None
                        };
                
                        struct_attrs.push((attr_start_punct, name.clone(), contents_group.cloned()));
                    },
                    _ => {
                        panic!("Expected attribute group, not {:?}.", token)
                    }
                }
            } else {
                break;
            }
        }
    
        // check for visibility
        if let Some(token) = it.peek() {
            // println!("peek visibility: {:?}", token);
            match token {
                proc_macro2::TokenTree::Ident(ident) => {
                    let ident_str = ident.to_string();
                    match ident_str.as_str() {
                        "pub" | "protected" => {
                            struct_vis = Some(ident.clone());
                            it.next();
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        }
    
        // expecting either struct or function
        if let Some(token) = it.next() {
            println!("checking for struct, function, or impl: {:?}", token);
            match token {
                proc_macro2::TokenTree::Ident(ident) => {
                    let ident_str = ident.to_string();
                    struct_type = Some(ident.clone());

                    match ident_str.as_str() {
                        "struct" | "fn" => {
                            // get name
                            let token = it.next().unwrap();
                            match token {
                                proc_macro2::TokenTree::Ident(ident) => {
                                    struct_name = Some(ident.clone());
                                },
                                _ => {
                                    panic!("Expected struct or function name, not {:?}.", token.clone());
                                }
                            }

                            // get inner
                            let token = it.next().unwrap();
                            match token {
                                proc_macro2::TokenTree::Group(group) => {
                                    // println!("struct_inner: {}", group.to_string());
                                    struct_inner = Some(group);
                                },
                                _ => {
                                    panic!("Expected struct or function inner, not {:?}.", token.clone());
                                }
                            }

                            if ident_str == "struct" {
                                // check for semi colon
                                if let Some(token) = it.next() {
                                    match &token {
                                        proc_macro2::TokenTree::Punct(punct) => {
                                            if punct.as_char() == ';' {
                                                struct_semi = Some(punct.clone());
                                            } else {
                                                panic!("Expected semi colon, not {:?}.", token);
                                            }
                                        },
                                        _ => {
                                            panic!("Expected semi colon, not {:?}.", token);
                                        }
                                    }
                                } else {
                                    // struct_semi = Some(Punct::new(';', proc_macro2::Spacing::Alone).clone());
                                    // println!("Expected semi colon but reached end of stream.");
                                }
                            }
                        },
                        "impl" => {
                            // get generics

                            // get name
                            let token = it.next().unwrap();
                            match token {
                                proc_macro2::TokenTree::Ident(ident) => {
                                    struct_name = Some(ident.clone());
                                },
                                _ => {
                                    panic!("Expected struct or function name, not {:?}.", token.clone());
                                }
                            }

                            // get inner
                            let token = it.next().unwrap();
                            match token {
                                proc_macro2::TokenTree::Group(group) => {
                                    // println!("struct_inner: {}", group.to_string());
                                    struct_inner = Some(group);
                                },
                                _ => {
                                    panic!("Expected struct or function inner, not {:?}.", token.clone());
                                }
                            }
                        },
                        _ => {
                            panic!("Expected struct or function, not ident {}.", ident_str);
                        }
                    }
                },
                _ => {
                    panic!("Expected struct or function, not token {:?}.", token);
                }
            }
        }

        Ok(Self::new(struct_attrs_tokens, struct_vis, struct_type, struct_name, struct_generics, struct_where_clause, struct_inner, struct_semi, struct_attrs))
    }

    pub fn finalize(&self) -> TokenStream {
        let struct_attrs = proc_macro2::TokenStream::from_iter(self.struct_attrs_tokens.clone());

        let struct_vis = &self.struct_vis;
        let struct_type = &self.struct_type;
        let struct_name = &self.struct_name;
        let struct_generics = &self.struct_generics;
        let struct_where_clause = &self.struct_where_clause;
        let struct_inner = self.generate_struct_inner();
        let struct_semi = &self.struct_semi;

        let tokens_to_prepend = self.generate_prepend_code();
        let tokens_to_append = self.generate_append_code();

        let prepend = TokenStream::from_iter(tokens_to_prepend.iter().cloned());
        let append_to_end = TokenStream::from_iter(tokens_to_append.iter().cloned().chain(self.tokens_to_append.borrow().clone().into_iter()));

        println!("prepend: {}", tokens_to_prepend.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
        // println!("struct_attrs: {}", self.struct_attrs_tokens.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
        // println!("struct_vis: {:?}", struct_vis);
        // println!("struct_type: {:?}", struct_type);
        // println!("struct_name: {:?}", struct_name);
        // println!("struct_generics: {}", struct_generics.to_string());
        // println!("struct_where_clause: {}", struct_where_clause.to_string());
        // println!("struct_inner: {}", struct_inner.to_string());
        // println!("struct_semi: {:?}", struct_semi);
        println!("append: {}", tokens_to_append.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));

        quote::quote! {
            #prepend
            #struct_attrs
            #struct_vis #struct_type #struct_name #struct_generics #struct_where_clause #struct_inner #struct_semi
            #append_to_end
        }
    }

    pub fn generate_append_code(&self) -> Vec<TokenTree> {
        self.append_processors.borrow().iter().flat_map(|x| x(self)).collect::<Vec<TokenTree>>()
    }

    pub fn generate_prepend_code(&self) -> Vec<TokenTree> {
        self.prepend_processors.borrow().iter().flat_map(|x| x(self)).collect::<Vec<TokenTree>>()
    }

    fn generate_struct_inner(&self) -> Group {
        let original = self.struct_inner.as_ref().unwrap().clone();

        if self.inner_processors.borrow().len() > 0 {
            let mut inner_tokens = vec![];
            for token in original.stream().into_iter() {
                match token {
                    TokenTree::Group(group) => {
                        let mut inner_group_tokens = vec![];
                        for inner_token in group.stream().into_iter() {
                            inner_group_tokens.push(inner_token);
                        }
                        for processor in self.inner_processors.borrow().iter() {
                            inner_group_tokens.extend_from_slice(&processor(self));
                        }
                        inner_tokens.push(TokenTree::Group(Group::new(group.delimiter(), TokenStream::from_iter(inner_group_tokens))));
                    },
                    _ => {
                        inner_tokens.push(token);
                    }
                }
            }
            Group::new(original.delimiter(), TokenStream::from_iter(inner_tokens))
        } else {
            original
        }
    }

    pub fn append(&self, quote: TokenStream) {
        self.tokens_to_append.borrow_mut().extend_from_slice(&quote.into_iter().collect::<Vec<TokenTree>>());
    }

    pub fn add_prepend_processor(&self, processor: &'a dyn Fn(&ExtendDerive) -> Vec<TokenTree>) {
        self.prepend_processors.borrow_mut().push(processor);
    }

    pub fn add_append_processor(&self, processor: &'a dyn Fn(&ExtendDerive) -> Vec<TokenTree>) {
        self.append_processors.borrow_mut().push(processor);
    }

    pub fn add_inner_processor(&self, processor: &'a dyn Fn(&ExtendDerive) -> Vec<TokenTree>) {
        self.inner_processors.borrow_mut().push(processor);
    }

    // closure function for checking property visibility
    pub fn get_property_visibility<T>(&self, it: &mut Peekable<T>) -> Option<Ident> where T: Iterator<Item=TokenTree> {
        let vis = if let Some(vis_token) = it.peek() {
            match vis_token {
                TokenTree::Ident(ident) => {
                    let ident_str = ident.to_string();
                    match ident_str.as_str() {
                        "pub" | "protected" => {
                            Some(ident.clone())
                        },
                        _ => None,
                    }
                },
                _ => None,
            }
        } else {
            None
        };

        if vis.is_some() {
            it.next();
        }

        vis
    }

    pub(crate) fn get_struct_properties(&self) -> Vec<(Option<Ident>, Ident, Vec<TokenTree>)> {
        let mut properties = vec![];
        let mut property_visibility: Option<Ident> = None;
        let mut property_name: Option<Ident> = None;
        let mut property_colon: Option<Punct> = None;
        let mut property_type: Vec<TokenTree> = vec![];
        let mut it = self.struct_inner.as_ref().unwrap().stream().into_iter().peekable();
        let mut punct_stack = vec![];

        property_visibility = self.get_property_visibility(&mut it);
        loop {
            if let Some(token) = it.next() {
                match &token {
                    TokenTree::Punct(punct) => {
                        let c = punct.as_char();
                        match c {
                            ';' => {
                                break;
                            },
                            ',' => {
                                if punct_stack.len() > 0 {
                                    property_type.push(token.clone());
                                } else {
                                    properties.push((property_visibility, property_name.unwrap().clone(), property_type.clone()));
                                    property_name = None;
                                    property_colon = None;
                                    property_type = vec![];
                                    property_visibility = self.get_property_visibility(&mut it);
                                }
                            },
                            ':' => {
                                property_colon = Some(punct.clone());
                            },
                            '<' => {
                                if property_name.is_some() {
                                    punct_stack.push(c);
                                    property_type.push(token.clone());
                                } else {
                                    panic!("Expected punct or ident for property type, not {:?}.", token)
                                }
                            },
                            '>' => {
                                if property_name.is_some() {
                                    if punct_stack.pop().unwrap() == '<' {
                                        property_type.push(token.clone());
                                    } else {
                                        panic!("Expected punct or ident for property type, not {:?}.", token)
                                    }
                                } else {
                                    panic!("Expected punct or ident for property name, not {:?}.", token)
                                }
                            },
                            '\'' => {
                                if property_name.is_some() {
                                    property_type.push(token.clone());
                                } else {
                                    panic!("Expected punct or ident for property name, not {:?}.", token)
                                }
                            },
                            _ => {
                                panic!("Expected semicolon or comma after property, not {:?}.", token);
                            }
                        }
                    },
                    TokenTree::Group(group) => {
                        if property_name.is_some() {
                            property_type.push(token.clone());
                        } else {
                            panic!("Expected punct or ident for property type, not {:?}.", token)
                        }
                    },
                    TokenTree::Ident(ident) => {
                        if let Some(pname) = &property_name {
                            property_type.push(token.clone());
                        } else {
                            property_name = Some(ident.clone());
                        }

                    },
                    _ => {
                        panic!("Expected punct, ident, or group, not {:?}.", token);
                    }
                }
            } else {
                break;
            }
        }

        properties
    }

    pub(crate) fn get_struct_method(
        &self, 
        it: &mut Peekable<impl Iterator<Item=TokenTree>>,
    ) -> AstMethod {
        let mut method_visibility: Option<Ident> = None;
        let mut method_generics: Vec<TokenTree> = vec![];
        let mut method_return_type: Vec<TokenTree> = vec![];

        let mut method_args = vec![];
        let mut method_arg_name: Option<Ident> = None;
        let mut method_arg_colon: Option<Punct> = None;
        let mut method_arg_type: Vec<TokenTree> = vec![];

        let mut punct_stack = vec![];

        method_visibility = self.get_property_visibility(it);
        // should be fn
        if let Some(token) = it.next() {
            match &token {
                TokenTree::Ident(ident) => {
                    if ident.to_string() != "fn" {
                        panic!("Expected fn ident, not {:?}.", token);
                    }
                },
                _ => {
                    panic!("Expected fn, not {:?}.", token);
                }
            }
        } else {
            panic!("Expected fn, not end of stream.");
        }
        
        // should be method name
        let method_name = if let Some(token) = it.next() {
            match &token {
                TokenTree::Ident(ident) => {
                    ident.clone()
                },
                _ => {
                    panic!("Expected method name, not {:?}.", token);
                }
            }
        } else {
            panic!("Expected method name, not end of stream.");
        };
        
        // check for generics
        if let Some(token) = it.peek() {
            match token {
                TokenTree::Punct(punct) => {
                    let c = punct.as_char();
                    if c == '<' {
                        method_generics.push(it.next().unwrap().clone());
                        loop {
                            if let Some(token) = it.next() {
                                method_generics.push(token.clone());
                                match &token {
                                    TokenTree::Punct(punct) => {
                                        let c = punct.as_char();
                                        match c {
                                            '>' => {
                                                break;
                                            },
                                            _ => {
                                            }
                                        }
                                    },
                                    _ => {
                                    }
                                }
                            } else {
                                panic!("Expected >, not end of stream.");
                            }
                        }
                    }
                },
                _ => {},
            }
        }

        // get arguments
        if let Some(token) = it.next() {
            match &token {
                TokenTree::Group(group) => {
                    let mut it = group.stream().into_iter().peekable();
                    loop {
                        if let Some(token) = it.next() {
                            match &token {
                                TokenTree::Punct(punct) => {
                                    let c = punct.as_char();
                                    match c {
                                        ',' => {
                                            if punct_stack.len() > 0 {
                                                method_arg_type.push(token.clone());
                                            } else {
                                                method_args.push((method_arg_name.unwrap().clone(), method_arg_type.clone()));
                                                method_arg_name = None;
                                                method_arg_colon = None;
                                                method_arg_type = vec![];
                                                method_visibility = self.get_property_visibility(&mut it);
                                            }
                                        },
                                        '<' => {
                                            if method_arg_name.is_some() {
                                                punct_stack.push(c);
                                                method_arg_type.push(token.clone());
                                            } else {
                                                panic!("Expected punct or ident for method argument name, not {:?}.", token)
                                            }
                                        },
                                        '>' => {
                                            if method_arg_name.is_some() {
                                                if punct_stack.pop().unwrap() == '<' {
                                                    method_arg_type.push(token.clone());
                                                } else {
                                                    panic!("Expected punct or ident for method argument type, not {:?}.", token)
                                                }
                                            } else {
                                                panic!("Expected punct or ident for method argument name, not {:?}.", token)
                                            }
                                        },
                                        ':' => {
                                            if method_arg_colon.is_some() {
                                                method_arg_type.push(token.clone());
                                            } else {
                                                method_arg_colon = Some(punct.clone());
                                            }
                                        },
                                        '\'' => {
                                            if method_arg_colon.is_some() {
                                                method_arg_type.push(token.clone());
                                            } else {
                                                panic!("Expected punct or ident for property name, not {:?}.", token)
                                            }
                                        },
                                        '&' => {
                                            if method_arg_name.is_some() {
                                                method_arg_type.push(token.clone());
                                            } else {
                                                // check if next token is self
                                                if let Some(token) = it.next() {
                                                    match &token {
                                                        TokenTree::Ident(ident) => {
                                                            if ident.to_string() == "self" {
                                                                method_arg_name = Some(ident.clone());
                                                                method_arg_colon = Some(Punct::new(':', proc_macro2::Spacing::Alone));
                                                                method_arg_type.push(Ident::new("Self", proc_macro2::Span::call_site()).into());
                                                            } else {
                                                                panic!("Expected self, not {:?}.", token);
                                                            }
                                                        },
                                                        _ => {
                                                            panic!("Expected self, not {:?}.", token);
                                                        }
                                                    }
                                                } else {
                                                    panic!("Expected self, not end of stream.");
                                                }
                                            }
                                        },
                                        _ => {
                                            panic!("Expected semicolon or comma after method argument, not {:?}.", token);
                                        }
                                    }
                                },
                                TokenTree::Group(group) => {
                                    if method_arg_name.is_some() {
                                        method_arg_type.push(token.clone());
                                    } else {
                                        panic!("Expected punct or ident for method argument type, not {:?}.", token)
                                    }
                                },
                                TokenTree::Ident(ident) => {
                                    if let Some(pname) = &method_arg_name {
                                        method_arg_type.push(token.clone());
                                    } else {
                                        method_arg_name = Some(ident.clone());
                                    }

                                },
                                _ => {
                                    panic!("Expected punct, ident, or group for method argument, not {:?}.", token);
                                }
                            }
                        } else {
                            break;
                        }
                    }
                },
                _ => {
                    panic!("Expected arguments group, not {:?}.", token);
                }
            }
        } else {
            panic!("Expected arguments, not end of stream.");
        }

        if method_arg_name.is_some() {
            method_args.push((method_arg_name.unwrap().clone(), method_arg_type.clone()));
            method_arg_name = None;
            method_arg_colon = None;
            method_arg_type = vec![];
        }

        // check for return type
        if let Some(token) = it.peek() {
            match token {
                TokenTree::Punct(punct) => {
                    let c = punct.as_char();
                    if c == '-' {
                        it.next();
                        // next char should be '>'
                        let token = it.next().unwrap();
                        match &token {
                            TokenTree::Punct(punct) => {
                                let c = punct.as_char();
                                if c == '>' {
                                    loop {
                                        if let Some(token) = it.next() {
                                            match &token {
                                                TokenTree::Punct(punct) => {
                                                    method_return_type.push(token.clone());
                                                },
                                                TokenTree::Group(group) => {
                                                    match group.delimiter() {
                                                        proc_macro2::Delimiter::Parenthesis |
                                                        proc_macro2::Delimiter::Bracket |
                                                        proc_macro2::Delimiter::None => {
                                                            method_return_type.push(token.clone());
                                                        },
                                                        _ => {
                                                            break;
                                                        }
                                                    }
                                                },
                                                _ => {
                                                    method_return_type.push(token.clone());
                                                }
                                            }
                                        } else {
                                            panic!("Expected ;, not end of stream.");
                                        }
                                    }
                                } else {
                                    panic!("Expected >, not {:?}.", token);
                                }
                            },
                            _ => {
                                panic!("Expected >, not {:?}.", token);
                            }
                        }
                    }
                },
                _ => {},
            }
        }

        AstMethod::new(
            method_visibility, method_name, method_generics, method_args, method_return_type
        )
    }

    pub(crate) fn get_struct_methods(&self) -> Vec<AstMethod> {
        let mut methods = vec![];
        let mut it = self.struct_inner.as_ref().unwrap().stream().into_iter().peekable();
        loop {
            if let Some(_) = it.peek() {
                methods.push(self.get_struct_method(&mut it));
            } else {
                break;
            }
        }
        methods
    }
}