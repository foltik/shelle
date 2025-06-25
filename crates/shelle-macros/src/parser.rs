use std::iter::Peekable;

use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub enum ParseArg {
    Pipe,
    Semicolon,
    RedirectFd(i32, i32),                 // fd1, fd2
    RedirectFile(i32, TokenStream, bool), // fd1, file, append?
    ArgStr(TokenStream),
    ArgVec(TokenStream),
}

pub struct Parser<I: Iterator<Item = ParseArg>> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item = ParseArg>> Parser<I> {
    pub fn from(iter: Peekable<I>) -> Self {
        Self { iter }
    }

    pub fn parse(mut self) -> TokenStream {
        let mut ret = quote!(::shelle::Script::default());
        while self.iter.peek().is_some() {
            let cmd = self.parse_cmd();
            if !cmd.is_empty() {
                ret.extend(quote!(.cmd(#cmd)));
            }
        }
        ret
    }

    fn parse_cmd(&mut self) -> TokenStream {
        let mut cmds = quote!(::shelle::Cmdline::default());
        while self.iter.peek().is_some() {
            let cmd = self.parse_pipe();
            cmds.extend(quote!(.pipe(#cmd)));
            if !matches!(self.iter.peek(), Some(ParseArg::Pipe)) {
                self.iter.next();
                break;
            }
            self.iter.next();
        }
        cmds
    }

    fn parse_pipe(&mut self) -> TokenStream {
        let Some(ParseArg::ArgStr(program)) = self.iter.next() else {
            panic!("missing first argument");
        };

        let mut ret = quote!(::shelle::Cmd::new(#program));
        //.with_location(file!(), line!()));
        // TODO: get accurate line number once `proc_macro::Span::line()` API is stable

        while let Some(arg) = self.iter.peek() {
            match arg {
                ParseArg::RedirectFd(fd1, fd2) => {
                    if fd1 != fd2 {
                        match (fd1, fd2) {
                            (1, 2) => ret.extend(quote!(.stdout(::shelle::Stdout::ToStderr))),
                            (2, 1) => ret.extend(quote!(.stderr(::shelle::Stderr::ToStdout))),
                            _ => panic!("unsupported fd numbers: {} {}", fd1, fd2),
                        }
                    }
                }
                ParseArg::RedirectFile(fd1, file, append) => match fd1 {
                    0 => ret.extend(quote!(.stdin(::shelle::Stdin::FromFile(#file.into())))),
                    1 => ret.extend(
                        quote!(.stdin(::shelle::Stdout::ToFile { path: #file.into(), append: #append })),
                    ),
                    2 => ret.extend(
                        quote!(.stdin(::shelle::Stderr::ToFile { path: #file.into(), append: #append })),
                    ),
                    _ => panic!("unsupported fd ({}) redirect to file {}", fd1, file),
                },
                ParseArg::ArgStr(opt) => {
                    ret.extend(quote!(.arg(#opt)));
                }
                ParseArg::ArgVec(opts) => {
                    ret.extend(quote! (.args(#opts)));
                }
                ParseArg::Pipe | ParseArg::Semicolon => break,
            }
            self.iter.next();
        }
        ret
    }
}
