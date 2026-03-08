use crate::high::inter_token_kind::InterTokenKind;

pub fn glue(lhs: &InterTokenKind, rhs: &InterTokenKind) -> Option<InterTokenKind> {
    use InterTokenKind as Kind;

    match (lhs, rhs) {
        (
            Kind::LitInteger {
                content,
                suffix: None,
            },
            Kind::Id(id),
        ) => Some(Kind::LitInteger {
            content: content.to_owned(),
            suffix: Some(id.to_owned()),
        }),
        (
            Kind::LitFloat {
                content,
                suffix: None,
            },
            Kind::Id(id),
        ) => Some(Kind::LitFloat {
            content: content.to_owned(),
            suffix: Some(id.to_owned()),
        }),
        (Kind::Sub, Kind::Gt) => Some(Kind::Arrow), // `->`
        (Kind::Colon, Kind::Colon) => Some(Kind::PathSep), // `::`
        (Kind::Add, Kind::Assign) => Some(Kind::AddAssign), // `+=`
        (Kind::Sub, Kind::Assign) => Some(Kind::SubAssign), // `-=`
        (Kind::Mul, Kind::Assign) => Some(Kind::MulAssign), // `*=`
        (Kind::Div, Kind::Assign) => Some(Kind::DivAssign), // `/=`
        (Kind::Mod, Kind::Assign) => Some(Kind::ModAssign), // `%=`
        (Kind::Pow, Kind::Assign) => Some(Kind::PowAssign), // `**=`
        (Kind::BitwiseNot, Kind::Assign) => Some(Kind::BitwiseNotAssign), // `~=`
        (Kind::BitwiseXor, Kind::Assign) => Some(Kind::BitwiseXorAssign), // `^=`
        (Kind::BitwiseAnd, Kind::Assign) => Some(Kind::BitwiseAndAssign), // `&=`
        (Kind::BitwiseOr, Kind::Assign) => Some(Kind::BitwiseOrAssign), // `|=`
        (Kind::BitwiseShiftLeft, Kind::Assign) => Some(Kind::BitwiseShiftLeftAssign), // `<<=`
        (Kind::BitwiseShiftRight, Kind::Assign) => Some(Kind::BitwiseShiftRightAssign), // `>>=`
        (Kind::Mul, Kind::Mul) => Some(Kind::Pow),  // `**`
        (Kind::Lt, Kind::Lt) => Some(Kind::BitwiseShiftLeft), // `<<`
        (Kind::Gt, Kind::Gt) => Some(Kind::BitwiseShiftRight), // `>>`
        (Kind::Assign, Kind::Assign) => Some(Kind::Eq), // `==`
        (Kind::Bang, Kind::Assign) => Some(Kind::Neq), // `!=`
        (Kind::Lt, Kind::Assign) => Some(Kind::Le), // `<=`
        (Kind::Gt, Kind::Assign) => Some(Kind::Ge), // `>=`
        _ => None,
    }
}
