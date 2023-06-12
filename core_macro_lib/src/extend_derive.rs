use std::cell::RefCell;
use std::iter::Peekable;
use std::vec;

use proc_macro2::Delimiter;
use proc_macro2::Group;
use proc_macro2::Punct;
use proc_macro2::Spacing;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenTree;

use crate::ast::attribute::AstAttribute;
use crate::ast::method::AstMethod;
use crate::ast::property::AstProperty;


// this is the struct that will be used to extend the derive macro,
// but does not use the derive macro itself. This is so that we can
// do some pre-processing before or after the derive macro is called.
pub struct ExtendDerive<'a> {
    pub struct_attrs_tokens: Vec<TokenTree>,
    struct_attrs: Vec<AstAttribute>,

    pub struct_vis: Option<Ident>,
    pub struct_type: Option<Ident>,
    pub struct_name: Option<Ident>,
    pub struct_generics: Vec<TokenTree>,
    pub struct_where_clause: Vec<TokenTree>,
    pub struct_inner: Option<Group>,
    pub struct_semi: Option<Punct>,

    tokens_to_append: RefCell<Vec<TokenTree>>,

    prepend_processors: RefCell<Vec<&'a dyn Fn(&ExtendDerive) -> Vec<TokenTree>>>,
    append_processors: RefCell<Vec<&'a dyn Fn(&ExtendDerive) -> Vec<TokenTree>>>,
    inner_processors: RefCell<Vec<&'a dyn Fn(&ExtendDerive) -> Vec<TokenTree>>>,
}

impl<'a> ExtendDerive<'a> {
    pub(crate) fn new(
        struct_attrs_tokens: Vec<TokenTree>,
        struct_vis: Option<Ident>,
        struct_type: Option<Ident>,
        struct_name: Option<Ident>,
        struct_generics: Vec<TokenTree>,
        struct_where_clause: Vec<TokenTree>,
        struct_inner: Option<Group>,
        struct_semi: Option<Punct>,
        struct_attrs: Vec<AstAttribute>,
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
            struct_attrs,
            tokens_to_append: RefCell::new(vec![]),
            prepend_processors: std::cell::RefCell::new(vec![]),
            append_processors: std::cell::RefCell::new(vec![]),
            inner_processors: std::cell::RefCell::new(vec![]),
        }
    }

    pub fn parse(
        _attr: TokenStream, item: TokenStream
    ) -> Result<Self, std::io::Error> {
        let mut it = TokenStream::from(item).into_iter().peekable();
        let mut struct_attrs_tokens = vec![];
        let mut struct_type: Option<Ident> = None;
        let mut struct_name: Option<Ident> = None;
        let mut struct_generics = vec![];
        let mut struct_where_clause = vec![];
        let mut struct_inner: Option<Group> = None;
        let mut struct_semi: Option<Punct> = None;

        let mut struct_attrs = vec![];
    
        // check for attributes before struct
        loop {
            let attr_start_punct = if let Some(token) = it.peek() {
                // println!("peek attributes: {:?}", token);
                match token {
                    TokenTree::Punct(punct) => {
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
                    TokenTree::Group(group) => {
                        struct_attrs_tokens.push(token.clone());
                        let mut it = group.stream().into_iter().peekable();
                        let name = if let Some(name_token) = it.next() {
                            match name_token.clone() {
                                TokenTree::Ident(name_ident) => name_ident,
                                _ => {
                                    panic!("Expected attribute name, not {:?}.", name_token);
                                }
                            }
                        } else { panic!("Expected attribute name."); };
                        
                        let contents_group = if let Some(contents_token) = it.peek() {
                            match contents_token {
                                TokenTree::Group(group) => Some(group),
                                _ => {
                                    panic!("Expected attribute contents group, not {:?}.", contents_token);
                                }
                            }
                        } else {
                            None
                        };
                
                        struct_attrs.push(AstAttribute::new(attr_start_punct, name.clone(), contents_group.cloned()));
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
        let struct_vis = Self::get_property_visibility(&mut it);
    
        // expecting either struct or function
        if let Some(token) = it.next() {
            // println!("checking for struct, function, or impl: {:?}", token);
            match token {
                TokenTree::Ident(ident) => {
                    let ident_str = ident.to_string();
                    struct_type = Some(ident.clone());

                    match ident_str.as_str() {
                        "struct" | "fn" => {
                            // get name
                            let token = it.next().unwrap();
                            match token {
                                TokenTree::Ident(ident) => {
                                    struct_name = Some(ident.clone());
                                },
                                _ => {
                                    panic!("Expected struct or function name, not {:?}.", token.clone());
                                }
                            }

                            // check for generics
                            if let Some(token) = it.peek() {
                                match token {
                                    TokenTree::Punct(punct) => {
                                        let c = punct.as_char();
                                        if c == '<' {
                                            struct_generics.push(it.next().unwrap());
                                            let mut punct_stack = vec![];
                                            loop {
                                                if let Some(token) = it.next() {
                                                    struct_generics.push(token.clone());
                                                    match &token {
                                                        TokenTree::Punct(punct) => {
                                                            let c = punct.as_char();
                                                            match c {
                                                                '<' => {
                                                                    punct_stack.push(c);
                                                                },
                                                                '>' => {
                                                                    if punct_stack.len() == 0 {
                                                                        break;
                                                                    } else {
                                                                        if punct_stack.pop().unwrap() != '<' {
                                                                            panic!("Expected <, not {:?}.", token);
                                                                        }
                                                                    }
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

                            // check for where clause
                            if let Some(token) = it.peek() {
                                match token {
                                    TokenTree::Ident(ident) => {
                                        let ident_str = ident.to_string();
                                        if ident_str == "where" {
                                            struct_where_clause.push(it.next().unwrap());
                                            loop {
                                                if let Some(token) = it.peek() {
                                                    struct_where_clause.push(token.clone());
                                                    match &token {
                                                        TokenTree::Group(group) => {
                                                            if group.delimiter() == Delimiter::Brace {
                                                                break;
                                                            } else {
                                                                struct_where_clause.push(token.clone());
                                                            }
                                                        },
                                                        _ => {
                                                            struct_where_clause.push(it.next().unwrap());
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

                            // get inner
                            let token = it.next().unwrap();
                            match token {
                                TokenTree::Group(group) => {
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
                                        TokenTree::Punct(punct) => {
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
                                    // struct_semi = Some(Punct::new(';', Spacing::Alone).clone());
                                    // println!("Expected semi colon but reached end of stream.");
                                }
                            }
                        },
                        "impl" => {
                            // get generics

                            // get name
                            let token = it.next().unwrap();
                            match token {
                                TokenTree::Ident(ident) => {
                                    struct_name = Some(ident.clone());
                                },
                                _ => {
                                    panic!("Expected struct or function name, not {:?}.", token.clone());
                                }
                            }

                            // get inner
                            let token = it.next().unwrap();
                            match token {
                                TokenTree::Group(group) => {
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
        let struct_vis = &self.struct_vis;
        let struct_type = &self.struct_type;
        let struct_name = &self.struct_name;
        let struct_semi = &self.struct_semi;

        TokenStream::from_iter(
            self.generate_prepend_code().into_iter()
                .chain(self.struct_attrs_tokens.clone().into_iter())
                .chain(vec![struct_vis.clone(), struct_type.clone(), struct_name.clone()].into_iter().filter(|x| x.is_some()).map(|x|TokenTree::Ident(x.unwrap())))
                .chain(self.struct_generics.clone().into_iter())
                .chain(self.struct_where_clause.clone().into_iter())
                .chain(vec![self.generate_struct_inner()].into_iter().map(|x|TokenTree::Group(x))
                .chain(vec![struct_semi.clone()].into_iter().filter(|x| x.is_some()).map(|x|TokenTree::Punct(x.unwrap()))))
                .chain(self.generate_append_code().into_iter())
                .chain(self.tokens_to_append.borrow().clone().into_iter())
        )
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

    #[allow(dead_code)]
    pub fn append(&self, quote: TokenStream) {
        self.tokens_to_append.borrow_mut().extend_from_slice(&quote.into_iter().collect::<Vec<TokenTree>>());
    }

    #[allow(dead_code)]
    pub fn add_prepend_processor(&self, processor: &'a dyn Fn(&ExtendDerive) -> Vec<TokenTree>) {
        self.prepend_processors.borrow_mut().push(processor);
    }

    #[allow(dead_code)]
    pub fn add_append_processor(&self, processor: &'a dyn Fn(&ExtendDerive) -> Vec<TokenTree>) {
        self.append_processors.borrow_mut().push(processor);
    }

    #[allow(dead_code)]
    pub fn add_inner_processor(&self, processor: &'a dyn Fn(&ExtendDerive) -> Vec<TokenTree>) {
        self.inner_processors.borrow_mut().push(processor);
    }

    // closure function for checking property visibility
    pub fn get_property_visibility<T>(it: &mut Peekable<T>) -> Option<Ident> where T: Iterator<Item=TokenTree> {
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

    pub(crate) fn get_struct_properties(&self) -> Vec<AstProperty> {
        let mut properties = vec![];
        let mut property_attributes: Vec<AstAttribute> = vec![];
        let mut property_name: Option<Ident> = None;
        let mut property_colon: Option<Punct> = None;
        let mut property_type: Vec<TokenTree> = vec![];
        let mut it = self.struct_inner.as_ref().unwrap().stream().into_iter().peekable();
        let mut punct_stack = vec![];

        let mut property_visibility = Self::get_property_visibility(&mut it);
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
                                    properties.push(AstProperty::new(property_attributes, property_visibility, property_name.unwrap().clone(), property_colon, property_type.clone()));
                                    property_name = None;
                                    property_colon = None;
                                    property_type = vec![];
                                    property_visibility = Self::get_property_visibility(&mut it);
                                    property_attributes = vec![];
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
                            '#' => {
                                if property_name.is_none() && property_visibility.is_none() {
                                    // get attribute
                                    let attrib_token = match it.next().unwrap() {
                                        TokenTree::Group(group) => {
                                            group
                                        },
                                        _ => {
                                            panic!("Expected attribute group, not {:?}.", token)
                                        }
                                    };
                                    
                                    let mut it = attrib_token.stream().into_iter().peekable();
                                    
                                    let attrib_name = match it.next().unwrap() {
                                        TokenTree::Ident(ident) => {
                                            ident
                                        },
                                        _ => {
                                            panic!("Expected attribute name, not {:?}.", token)
                                        }
                                    };

                                    let attrib_contents = if let Some(token) = it.next() {
                                        match token {
                                            TokenTree::Group(group) => {
                                                Some(group)
                                            },
                                            _ => {
                                                None
                                            }
                                        }
                                    } else {
                                        None
                                    };

                                    property_attributes.push(AstAttribute::new(punct.clone(), attrib_name, attrib_contents));
                                } else {
                                    panic!("Expected something other than {:?}.", token)
                                }
                            },
                            _ => {
                                panic!("Expected semicolon or comma for property, not {:?}.", token);
                            }
                        }
                    },
                    TokenTree::Group(_) => {
                        if property_name.is_some() {
                            property_type.push(token.clone());
                        } else {
                            panic!("Expected punct or ident for property type, not {:?}.", token)
                        }
                    },
                    TokenTree::Ident(ident) => {
                        if let Some(_) = &property_name {
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

        if property_name.is_some() {
            properties.push(AstProperty::new(property_attributes, property_visibility, property_name.unwrap().clone(), property_colon, property_type.clone()));
        }

        properties
    }

    pub(crate) fn get_struct_method(
        &self, 
        it: &mut Peekable<impl Iterator<Item=TokenTree>>,
    ) -> AstMethod {
        let mut method_generics: Vec<TokenTree> = vec![];
        let mut method_return_type: Vec<TokenTree> = vec![];

        let mut method_args = vec![];
        let mut method_arg_name: Option<Ident> = None;
        let mut method_arg_colon: Option<Punct> = None;
        let mut method_arg_type: Vec<TokenTree> = vec![];

        let mut punct_stack = vec![];

        let mut method_attributes: Vec<AstAttribute> = vec![];
        loop {
            let start_punct = if let Some(token) = it.peek() {
                match token {
                    TokenTree::Punct(punct) => {
                        let c = punct.as_char();
                        if c == '#' {
                            punct.clone()
                        } else {
                            break;
                        }
                    },
                    _ => break,
                }
            } else {
                break;
            };
            it.next();

            // get attribute
            let attrib_token = it.next().unwrap();
            let attrib_group = match attrib_token {
                TokenTree::Group(group) => {
                    group
                },
                _ => {
                    panic!("Expected attribute group, not {:?}.", attrib_token)
                }
            };
            
            let mut it = attrib_group.stream().into_iter().peekable();
            
            let name_token = it.next().unwrap();
            let attrib_name = match name_token {
                TokenTree::Ident(ident) => {
                    ident
                },
                _ => {
                    panic!("Expected attribute name, not {:?}.", name_token)
                }
            };

            let attrib_contents = if let Some(token) = it.next() {
                match token {
                    TokenTree::Group(group) => {
                        Some(group)
                    },
                    _ => {
                        None
                    }
                }
            } else {
                None
            };

            method_attributes.push(AstAttribute::new(start_punct, attrib_name, attrib_contents));
        }

        let method_visibility = Self::get_property_visibility(it);
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
                                                method_args.push(AstProperty::new(vec![], None, method_arg_name.unwrap(), method_arg_colon, method_arg_type.clone()));
                                                method_arg_name = None;
                                                method_arg_colon = None;
                                                method_arg_type = vec![];
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
                                            if method_arg_name.is_some() {
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
                                TokenTree::Group(_) => {
                                    if method_arg_name.is_some() {
                                        method_arg_type.push(token.clone());
                                    } else {
                                        panic!("Expected punct or ident for method argument type, not {:?}.", token)
                                    }
                                },
                                TokenTree::Ident(ident) => {
                                    if let Some(_) = &method_arg_name {
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
            method_args.push(AstProperty::new(vec![], None, method_arg_name.unwrap().clone(), method_arg_colon, method_arg_type.clone()));
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
                                                TokenTree::Punct(_) => {
                                                    method_return_type.push(token.clone());
                                                },
                                                TokenTree::Group(group) => {
                                                    match group.delimiter() {
                                                        Delimiter::Parenthesis |
                                                        Delimiter::Bracket |
                                                        Delimiter::None => {
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
                                            panic!("Expected ';', not end of stream.");
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
            method_attributes,
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

    pub fn finalize_attrs(&self) -> Vec<TokenTree> {
        self.struct_attrs
            .iter()
            .flat_map(|x| {
                let name = &x.name;
                let name_str = name.to_string();
                let value = if let Some(a) = &x.content {
                    a.to_string()
                } else {
                    String::new()
                };

                quote::quote! {
                    Rc::new(ReflectedAttribute::new(
                        #name_str.to_string(),
                        #value.to_string(),
                        None,
                    )),
                }.into_iter()
            })
            .collect::<Vec<TokenTree>>()
    }
}