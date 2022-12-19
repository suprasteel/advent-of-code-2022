pub trait ToChar {
    fn to_char(&self) -> char;
}

pub trait FromChar {
    fn from_char(char: char) -> Self;
}

pub trait TryFromChar: ToChar {
    type Err;
    fn try_from_char(char: char) -> Result<Self, Self::Err>
    where
        Self: Sized;
}

impl ToChar for char {
    fn to_char(&self) -> char {
        *self
    }
}

impl FromChar for char {
    fn from_char(char: char) -> Self {
        char
    }
}

impl ToChar for usize {
    fn to_char(&self) -> char {
        match *self {
            v @ 0..=9 => (('0' as u32 + v as u32) as u8) as char,
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _ => '+',
        }
    }
}

impl TryFromChar for usize {
    type Err = String;
    fn try_from_char(char: char) -> Result<Self, Self::Err> {
        match char {
            v @ '0'..='9' => Ok(((v as u32 - '0' as u32) as u8) as usize),
            'a' => Ok(10),
            'b' => Ok(11),
            'c' => Ok(12),
            'd' => Ok(13),
            'e' => Ok(14),
            'f' => Ok(15),
            e @ _ => Err(format!("Impossible to convert {} to usize", e)),
        }
    }
}

// automatic TryFromChar impl for all struct implementing FromChar
impl<T> TryFromChar for T
where
    T: FromChar + ToChar,
{
    type Err = ();

    fn try_from_char(char: char) -> Result<Self, Self::Err>
    where
        Self: Sized,
    {
        Ok(T::from_char(char))
    }
}
