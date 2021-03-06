#[derive(Debug, PartialEq)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, PartialEq)]
pub enum ArgsErr {
    TooFew,
    TooMany,
    InvalidInteger(String),
    IntegerTooSmall(u32),
}

// FIXME: Can we use a more general type here?
// NOTE: This was a newtype around std::env::Args but needed to be more general for testing.
// NOTE: It's quite frustrating to get right. Still right but working.
struct ParseArgs(std::vec::IntoIter<String>);

impl ParseArgs {
    fn new(args: Vec<String>) -> Self {
        ParseArgs(args.into_iter())
    }

    fn require_arg(&mut self) -> Result<String, ArgsErr> {
        self.0.next().ok_or(ArgsErr::TooFew)
    }
}

#[allow(clippy::or_fun_call)]
pub fn parse_args(args: Vec<String>) -> Result<Frame, ArgsErr> {
    use self::ArgsErr::*;

    let mut args = ParseArgs::new(args);
    //let mut args = ParseArgs(args.into_iter());

    let width_s = args.require_arg()?;
    let height_s = args.require_arg()?;

    // Require end of arguments here.
    let mut require_no_more_args = || match args.0.next() {
        None => Ok(()),
        Some(_) => Err(TooMany),
    };

    require_no_more_args()?;

    let parse_dimension = |s: String| {
        let dim = s.parse().or(Err(InvalidInteger(s)));
        match dim {
            Ok(dim) => {
                if dim < 2 {
                    Err(IntegerTooSmall(dim))
                } else {
                    Ok(dim)
                }
            }
            Err(e) => Err(e),
        }
    };

    let width = parse_dimension(width_s)?;
    let height = parse_dimension(height_s)?;

    Ok(Frame { width, height })
}

#[cfg(test)]
mod tests {
    use self::ArgsErr::*;
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Err(TooFew),
            parse_args(std::env::args().skip(1).collect::<Vec<_>>())
        );
        assert_eq!(Err(TooFew), parse_args(vec![]));
        assert_eq!(Err(TooFew), parse_args(vec!["one".to_string()]));
        assert_eq!(
            Err(InvalidInteger("one".to_string())),
            parse_args(vec!["one".to_string(), "two".to_string()])
        );
        assert_eq!(
            Err(InvalidInteger("two".to_string())),
            parse_args(vec!["1".to_string(), "two".to_string()])
        );
        assert_eq!(
            Err(InvalidInteger(" 1".to_string())),
            parse_args(vec![" 1".to_string(), "two".to_string()])
        );
        assert_eq!(
            Err(InvalidInteger("1 ".to_string())),
            parse_args(vec!["1 ".to_string(), "two".to_string()])
        );
        assert_eq!(
            Ok(Frame {
                width: 40,
                height: 20
            }),
            parse_args(vec!["40".to_string(), "20".to_string()])
        );
        assert_eq!(
            Err(TooMany),
            parse_args(vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string()
            ])
        );
    }
}
