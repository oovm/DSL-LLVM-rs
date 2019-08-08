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
    EmptyStatement,
    eos,
    comma_or_semi,
    Semicolon,
    Comma,
    DeclareStatement,
    data,
    literal,
    Boolean,
    True,
    False,
    Null,
    Undefined,
    SYMBOL,
    NameCharacter,
    NameStartCharacter,
    Underline,
    Dollar,
    Byte,
    Byte_BIN,
    Byte_OCT,
    Byte_HEX,
    Zero,
    B,
    O,
    X,
    Number,
    Decimal,
    DecimalBad,
    Exponent,
    Integer,
    Dot,
    E,
    String,
    DoubleEscape,
    SingleEscape,
    S1,
    S2,
    WHITESPACE,
    SPACE_SEPARATOR,
    NEWLINE,
    CR,
    LF,
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
            (&Rule::EmptyStatement,) => {
                let mut debug_trait_builder = f.debug_tuple("EmptyStatement");
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
            (&Rule::DeclareStatement,) => {
                let mut debug_trait_builder = f.debug_tuple("DeclareStatement");
                debug_trait_builder.finish()
            }
            (&Rule::data,) => {
                let mut debug_trait_builder = f.debug_tuple("data");
                debug_trait_builder.finish()
            }
            (&Rule::literal,) => {
                let mut debug_trait_builder = f.debug_tuple("literal");
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
            (&Rule::SYMBOL,) => {
                let mut debug_trait_builder = f.debug_tuple("SYMBOL");
                debug_trait_builder.finish()
            }
            (&Rule::NameCharacter,) => {
                let mut debug_trait_builder = f.debug_tuple("NameCharacter");
                debug_trait_builder.finish()
            }
            (&Rule::NameStartCharacter,) => {
                let mut debug_trait_builder = f.debug_tuple("NameStartCharacter");
                debug_trait_builder.finish()
            }
            (&Rule::Underline,) => {
                let mut debug_trait_builder = f.debug_tuple("Underline");
                debug_trait_builder.finish()
            }
            (&Rule::Dollar,) => {
                let mut debug_trait_builder = f.debug_tuple("Dollar");
                debug_trait_builder.finish()
            }
            (&Rule::Byte,) => {
                let mut debug_trait_builder = f.debug_tuple("Byte");
                debug_trait_builder.finish()
            }
            (&Rule::Byte_BIN,) => {
                let mut debug_trait_builder = f.debug_tuple("Byte_BIN");
                debug_trait_builder.finish()
            }
            (&Rule::Byte_OCT,) => {
                let mut debug_trait_builder = f.debug_tuple("Byte_OCT");
                debug_trait_builder.finish()
            }
            (&Rule::Byte_HEX,) => {
                let mut debug_trait_builder = f.debug_tuple("Byte_HEX");
                debug_trait_builder.finish()
            }
            (&Rule::Zero,) => {
                let mut debug_trait_builder = f.debug_tuple("Zero");
                debug_trait_builder.finish()
            }
            (&Rule::B,) => {
                let mut debug_trait_builder = f.debug_tuple("B");
                debug_trait_builder.finish()
            }
            (&Rule::O,) => {
                let mut debug_trait_builder = f.debug_tuple("O");
                debug_trait_builder.finish()
            }
            (&Rule::X,) => {
                let mut debug_trait_builder = f.debug_tuple("X");
                debug_trait_builder.finish()
            }
            (&Rule::Number,) => {
                let mut debug_trait_builder = f.debug_tuple("Number");
                debug_trait_builder.finish()
            }
            (&Rule::Decimal,) => {
                let mut debug_trait_builder = f.debug_tuple("Decimal");
                debug_trait_builder.finish()
            }
            (&Rule::DecimalBad,) => {
                let mut debug_trait_builder = f.debug_tuple("DecimalBad");
                debug_trait_builder.finish()
            }
            (&Rule::Exponent,) => {
                let mut debug_trait_builder = f.debug_tuple("Exponent");
                debug_trait_builder.finish()
            }
            (&Rule::Integer,) => {
                let mut debug_trait_builder = f.debug_tuple("Integer");
                debug_trait_builder.finish()
            }
            (&Rule::Dot,) => {
                let mut debug_trait_builder = f.debug_tuple("Dot");
                debug_trait_builder.finish()
            }
            (&Rule::E,) => {
                let mut debug_trait_builder = f.debug_tuple("E");
                debug_trait_builder.finish()
            }
            (&Rule::String,) => {
                let mut debug_trait_builder = f.debug_tuple("String");
                debug_trait_builder.finish()
            }
            (&Rule::DoubleEscape,) => {
                let mut debug_trait_builder = f.debug_tuple("DoubleEscape");
                debug_trait_builder.finish()
            }
            (&Rule::SingleEscape,) => {
                let mut debug_trait_builder = f.debug_tuple("SingleEscape");
                debug_trait_builder.finish()
            }
            (&Rule::S1,) => {
                let mut debug_trait_builder = f.debug_tuple("S1");
                debug_trait_builder.finish()
            }
            (&Rule::S2,) => {
                let mut debug_trait_builder = f.debug_tuple("S2");
                debug_trait_builder.finish()
            }
            (&Rule::WHITESPACE,) => {
                let mut debug_trait_builder = f.debug_tuple("WHITESPACE");
                debug_trait_builder.finish()
            }
            (&Rule::SPACE_SEPARATOR,) => {
                let mut debug_trait_builder = f.debug_tuple("SPACE_SEPARATOR");
                debug_trait_builder.finish()
            }
            (&Rule::NEWLINE,) => {
                let mut debug_trait_builder = f.debug_tuple("NEWLINE");
                debug_trait_builder.finish()
            }
            (&Rule::CR,) => {
                let mut debug_trait_builder = f.debug_tuple("CR");
                debug_trait_builder.finish()
            }
            (&Rule::LF,) => {
                let mut debug_trait_builder = f.debug_tuple("LF");
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
                    if state.atomicity() == ::pest::Atomicity::NonAtomic {
                        state.repeat(|state| super::visible::WHITESPACE(state))
                    } else {
                        Ok(state)
                    }
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
                    self::EmptyStatement(state)
                        .or_else(|state| {
                            state.sequence(|state| {
                                self::DeclareStatement(state)
                                    .and_then(|state| super::hidden::skip(state))
                                    .and_then(|state| state.optional(|state| self::eos(state)))
                            })
                        })
                        .or_else(|state| {
                            state.sequence(|state| {
                                self::literal(state)
                                    .and_then(|state| super::hidden::skip(state))
                                    .and_then(|state| state.optional(|state| self::eos(state)))
                            })
                        })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn EmptyStatement(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::EmptyStatement, |state| self::eos(state))
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
                pub fn DeclareStatement(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Undefined(state)
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn data(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::data, |state| self::literal(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn literal(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Boolean(state)
                        .or_else(|state| self::Null(state))
                        .or_else(|state| self::Undefined(state))
                        .or_else(|state| self::Byte(state))
                        .or_else(|state| self::Number(state))
                        .or_else(|state| self::SYMBOL(state))
                        .or_else(|state| self::String(state))
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
                #[allow(non_snake_case, unused_variables)]
                pub fn SYMBOL(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SYMBOL, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                self::NameStartCharacter(state).and_then(|state| {
                                    state.repeat(|state| self::NameCharacter(state))
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NameCharacter(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::ASCII_DIGIT(state).or_else(|state| self::NameStartCharacter(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NameStartCharacter(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Dollar(state)
                        .or_else(|state| self::Underline(state))
                        .or_else(|state| self::ASCII_ALPHA(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Underline(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Underline, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("_"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dollar(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dollar, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("$"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Byte_BIN(state)
                        .or_else(|state| self::Byte_OCT(state))
                        .or_else(|state| self::Byte_HEX(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte_BIN(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::Byte_BIN, |state| {
                            state.sequence(|state| {
                                self::Zero(state)
                                    .and_then(|state| self::B(state))
                                    .and_then(|state| {
                                        state.sequence(|state| {
                                            state
                                                .optional(|state| self::Underline(state))
                                                .and_then(|state| self::ASCII_BIN_DIGIT(state))
                                        })
                                    })
                                    .and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                state
                                                    .optional(|state| self::Underline(state))
                                                    .and_then(|state| self::ASCII_BIN_DIGIT(state))
                                            })
                                        })
                                    })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte_OCT(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::Byte_OCT, |state| {
                            state.sequence(|state| {
                                self::Zero(state)
                                    .and_then(|state| self::O(state))
                                    .and_then(|state| {
                                        state.sequence(|state| {
                                            state
                                                .optional(|state| self::Underline(state))
                                                .and_then(|state| self::ASCII_OCT_DIGIT(state))
                                        })
                                    })
                                    .and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                state
                                                    .optional(|state| self::Underline(state))
                                                    .and_then(|state| self::ASCII_OCT_DIGIT(state))
                                            })
                                        })
                                    })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Byte_HEX(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::Byte_HEX, |state| {
                            state.sequence(|state| {
                                self::Zero(state)
                                    .and_then(|state| self::X(state))
                                    .and_then(|state| {
                                        state.sequence(|state| {
                                            state
                                                .optional(|state| self::Underline(state))
                                                .and_then(|state| self::ASCII_HEX_DIGIT(state))
                                        })
                                    })
                                    .and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                state
                                                    .optional(|state| self::Underline(state))
                                                    .and_then(|state| self::ASCII_HEX_DIGIT(state))
                                            })
                                        })
                                    })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Zero(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("0")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn B(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state
                        .match_string("B")
                        .or_else(|state| state.match_string("b"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn O(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state
                        .match_string("O")
                        .or_else(|state| state.match_string("o"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn X(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state
                        .match_string("X")
                        .or_else(|state| state.match_string("x"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Number(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::Exponent(state)
                        .or_else(|state| self::Decimal(state))
                        .or_else(|state| self::DecimalBad(state))
                        .or_else(|state| self::Integer(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Decimal(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::Decimal, |state| {
                            state.sequence(|state| {
                                self::Integer(state)
                                    .and_then(|state| self::Dot(state))
                                    .and_then(|state| self::ASCII_DIGIT(state))
                                    .and_then(|state| {
                                        state.repeat(|state| self::ASCII_DIGIT(state))
                                    })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DecimalBad(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::DecimalBad, |state| {
                            state
                                .sequence(|state| {
                                    self::Integer(state).and_then(|state| self::Dot(state))
                                })
                                .or_else(|state| {
                                    state.sequence(|state| {
                                        self::Dot(state)
                                            .and_then(|state| self::ASCII_DIGIT(state))
                                            .and_then(|state| {
                                                state.repeat(|state| self::ASCII_DIGIT(state))
                                            })
                                    })
                                })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Exponent(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::Exponent, |state| {
                            state.sequence(|state| {
                                self::Decimal(state)
                                    .or_else(|state| self::Integer(state))
                                    .and_then(|state| self::E(state))
                                    .and_then(|state| {
                                        self::Decimal(state).or_else(|state| self::Integer(state))
                                    })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Integer(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Integer, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state.sequence(|state| {
                                self::ASCII_DIGIT(state).and_then(|state| {
                                    state.repeat(|state| {
                                        state.sequence(|state| {
                                            state
                                                .optional(|state| self::Underline(state))
                                                .and_then(|state| self::ASCII_DIGIT(state))
                                        })
                                    })
                                })
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn Dot(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::Dot, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("."))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn E(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state
                        .match_string("E")
                        .or_else(|state| state.match_string("e"))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn String(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    self::SingleEscape(state).or_else(|state| self::DoubleEscape(state))
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn DoubleEscape(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::DoubleEscape, |state| {
                            state.sequence(|state| {
                                self::S2(state)
                                    .and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                state
                                                    .lookahead(false, |state| self::S2(state))
                                                    .and_then(|state| self::ANY(state))
                                            })
                                        })
                                    })
                                    .and_then(|state| self::S2(state))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SingleEscape(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::CompoundAtomic, |state| {
                        state.rule(Rule::SingleEscape, |state| {
                            state.sequence(|state| {
                                self::S1(state)
                                    .and_then(|state| {
                                        state.repeat(|state| {
                                            state.sequence(|state| {
                                                state
                                                    .lookahead(false, |state| self::S1(state))
                                                    .and_then(|state| self::ANY(state))
                                            })
                                        })
                                    })
                                    .and_then(|state| self::S1(state))
                            })
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn S1(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\'")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn S2(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_string("\"")
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn WHITESPACE(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.atomic(::pest::Atomicity::Atomic, |state| {
                        self::NEWLINE(state).or_else(|state| self::SPACE_SEPARATOR(state))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn SPACE_SEPARATOR(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::SPACE_SEPARATOR, |state| {
                        state
                            .match_string(" ")
                            .or_else(|state| state.match_string("\t"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn NEWLINE(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::NEWLINE, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| {
                            state
                                .sequence(|state| self::CR(state).and_then(|state| self::LF(state)))
                                .or_else(|state| self::CR(state))
                                .or_else(|state| self::LF(state))
                        })
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn CR(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::CR, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\r"))
                    })
                }
                #[inline]
                #[allow(non_snake_case, unused_variables)]
                pub fn LF(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.rule(Rule::LF, |state| {
                        state.atomic(::pest::Atomicity::Atomic, |state| state.match_string("\n"))
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
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_HEX_DIGIT(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state
                        .match_range('0'..'9')
                        .or_else(|state| state.match_range('a'..'f'))
                        .or_else(|state| state.match_range('A'..'F'))
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ANY(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.skip(1)
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_BIN_DIGIT(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'1')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_OCT_DIGIT(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'7')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_DIGIT(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state.match_range('0'..'9')
                }
                #[inline]
                #[allow(dead_code, non_snake_case, unused_variables)]
                pub fn ASCII_ALPHA(
                    state: Box<::pest::ParserState<Rule>>,
                ) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> {
                    state
                        .match_range('a'..'z')
                        .or_else(|state| state.match_range('A'..'Z'))
                }
            }
            pub use self::visible::*;
        }
        ::pest::state(input, |state| match rule {
            Rule::program => rules::program(state),
            Rule::statement => rules::statement(state),
            Rule::EmptyStatement => rules::EmptyStatement(state),
            Rule::eos => rules::eos(state),
            Rule::comma_or_semi => rules::comma_or_semi(state),
            Rule::Semicolon => rules::Semicolon(state),
            Rule::Comma => rules::Comma(state),
            Rule::DeclareStatement => rules::DeclareStatement(state),
            Rule::data => rules::data(state),
            Rule::literal => rules::literal(state),
            Rule::Boolean => rules::Boolean(state),
            Rule::True => rules::True(state),
            Rule::False => rules::False(state),
            Rule::Null => rules::Null(state),
            Rule::Undefined => rules::Undefined(state),
            Rule::SYMBOL => rules::SYMBOL(state),
            Rule::NameCharacter => rules::NameCharacter(state),
            Rule::NameStartCharacter => rules::NameStartCharacter(state),
            Rule::Underline => rules::Underline(state),
            Rule::Dollar => rules::Dollar(state),
            Rule::Byte => rules::Byte(state),
            Rule::Byte_BIN => rules::Byte_BIN(state),
            Rule::Byte_OCT => rules::Byte_OCT(state),
            Rule::Byte_HEX => rules::Byte_HEX(state),
            Rule::Zero => rules::Zero(state),
            Rule::B => rules::B(state),
            Rule::O => rules::O(state),
            Rule::X => rules::X(state),
            Rule::Number => rules::Number(state),
            Rule::Decimal => rules::Decimal(state),
            Rule::DecimalBad => rules::DecimalBad(state),
            Rule::Exponent => rules::Exponent(state),
            Rule::Integer => rules::Integer(state),
            Rule::Dot => rules::Dot(state),
            Rule::E => rules::E(state),
            Rule::String => rules::String(state),
            Rule::DoubleEscape => rules::DoubleEscape(state),
            Rule::SingleEscape => rules::SingleEscape(state),
            Rule::S1 => rules::S1(state),
            Rule::S2 => rules::S2(state),
            Rule::WHITESPACE => rules::WHITESPACE(state),
            Rule::SPACE_SEPARATOR => rules::SPACE_SEPARATOR(state),
            Rule::NEWLINE => rules::NEWLINE(state),
            Rule::CR => rules::CR(state),
            Rule::LF => rules::LF(state),
            Rule::EOI => rules::EOI(state),
        })
    }
}
