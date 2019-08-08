#[allow(dead_code)]
#[grammar = "typescript.pest"]
pub struct Parser;
#[allow(dead_code, non_camel_case_types)]
#[structural_match]
#[rustc_copy_clone_marker]
pub enum Rule {
    EOI,
    program,
    statement,
    empty_statement,
    eos,
    comma_or_semi,
    Semicolon,
    Comma,
    declare_statement,
    Boolean,
    True,
    False,
    Null,
    Undefined,
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::clone::Clone for Rule {
    #[inline]
    fn clone(&self) -> Rule {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::marker::Copy for Rule {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Rule::EOI,) => {
                let mut debug_trait_builder = f.debug_tuple("EOI");
                debug_trait_builder.finish()
            }
            (&Rule::program,) => {
                let mut debug_trait_builder = f.debug_tuple("program");
                debug_trait_builder.finish()
            }
            (&Rule::statement,) => {
                let mut debug_trait_builder = f.debug_tuple("statement");
                debug_trait_builder.finish()
            }
            (&Rule::empty_statement,) => {
                let mut debug_trait_builder = f.debug_tuple("empty_statement");
                debug_trait_builder.finish()
            }
            (&Rule::eos,) => {
                let mut debug_trait_builder = f.debug_tuple("eos");
                debug_trait_builder.finish()
            }
            (&Rule::comma_or_semi,) => {
                let mut debug_trait_builder = f.debug_tuple("comma_or_semi");
                debug_trait_builder.finish()
            }
            (&Rule::Semicolon,) => {
                let mut debug_trait_builder = f.debug_tuple("Semicolon");
                debug_trait_builder.finish()
            }
            (&Rule::Comma,) => {
                let mut debug_trait_builder = f.debug_tuple("Comma");
                debug_trait_builder.finish()
            }
            (&Rule::declare_statement,) => {
                let mut debug_trait_builder = f.debug_tuple("declare_statement");
                debug_trait_builder.finish()
            }
            (&Rule::Boolean,) => {
                let mut debug_trait_builder = f.debug_tuple("Boolean");
                debug_trait_builder.finish()
            }
            (&Rule::True,) => {
                let mut debug_trait_builder = f.debug_tuple("True");
                debug_trait_builder.finish()
            }
            (&Rule::False,) => {
                let mut debug_trait_builder = f.debug_tuple("False");
                debug_trait_builder.finish()
            }
            (&Rule::Null,) => {
                let mut debug_trait_builder = f.debug_tuple("Null");
                debug_trait_builder.finish()
            }
            (&Rule::Undefined,) => {
                let mut debug_trait_builder = f.debug_tuple("Undefined");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::cmp::Eq for Rule {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::hash::Hash for Rule {
    fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
        match (&*self,) {
            _ => ::std::hash::Hash::hash(
                &unsafe { ::std::intrinsics::discriminant_value(self) },
                state,
            ),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::cmp::Ord for Rule {
    #[inline]
    fn cmp(&self, other: &Rule) -> ::std::cmp::Ordering {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => ::std::cmp::Ordering::Equal,
                }
            } else {
                __self_vi.cmp(&__arg_1_vi)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::cmp::PartialEq for Rule {
    #[inline]
    fn eq(&self, other: &Rule) -> bool {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => true,
                }
            } else {
                false
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::cmp::PartialOrd for Rule {
    #[inline]
    fn partial_cmp(&self, other: &Rule) -> ::std::option::Option<::std::cmp::Ordering> {
        {
            let __self_vi = unsafe { ::std::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::std::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                }
            } else {
                __self_vi.partial_cmp(&__arg_1_vi)
            }
        }
    }
}
#[allow(clippy::all)]
impl ::pest::Parser<Rule> for Parser {
    fn parse<'i>(
        rule: Rule,
        input: &'i str,
    ) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
        mod rules {
            pub mod hidden {
                use super::super::Rule;
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn skip(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    Ok(state)
                }
            }
            pub mod visible {
                use super::super::Rule;
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn program(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.sequence(|state| {
                        self::SOI(state)
                            .and_then(|state| super::hidden::skip(state))
                            .and_then(|state| {
                                state.sequence(|state| {
                                    state.optional(|state| {
                                        self::statement(state).and_then(|state| {
                                            state.repeat(|state| {
                                                state.sequence(|state| {
                                                    super::hidden::skip(state)
                                                        .and_then(|state| self::statement(state))
                                                })
                                            })
                                        })
                                    })
                                })
                            })
                            .and_then(|state| super::hidden::skip(state))
                            .and_then(|state| self::EOI(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn statement(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::empty_statement(state).or_else(|state| {
                        state.sequence(|state| {
                            self::declare_statement(state)
                                .and_then(|state| super::hidden::skip(state))
                                .and_then(|state| state.optional(|state| self::eos(state)))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn empty_statement(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::eos(state)
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn eos(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::eos, |state| self::Semicolon(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn comma_or_semi(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Comma(state).or_else(|state| self::Semicolon(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Semicolon(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Semicolon, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(";"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Comma(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Comma, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string(","))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn declare_statement(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Undefined(state)
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Boolean(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::NonAtomic, |state| {
                        state.rule(Rule::Boolean, |state| {
                            self::True(state).or_else(|state| self::False(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn True(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::True, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.match_string("true")
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn False(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::False, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.match_string("false")
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Null(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Null, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.match_string("null")
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Undefined(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Undefined, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.match_string("undefined")
                        })
                    })
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn SOI(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.start_of_input()
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn EOI(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EOI, |state| state.end_of_input())
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::empty_statement => rules::empty_statement(state),
            Rule::eos => rules::eos(state),
            Rule::comma_or_semi => rules::comma_or_semi(state),
            Rule::Semicolon => rules::Semicolon(state),
            Rule::Comma => rules::Comma(state),
            Rule::declare_statement => rules::declare_statement(state),
            Rule::Boolean => rules::Boolean(state),
            Rule::True => rules::True(state),
            Rule::False => rules::False(state),
            Rule::Null => rules::Null(state),
            Rule::Undefined => rules::Undefined(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
