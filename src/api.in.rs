use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Login{
    pub email: String,
    pub password: String
}

impl Login{
    #[inline]
    pub fn new<E: ToString, P: ToString>(email: E, password: P) -> Self{
        Login{
            email: email.to_string(),
            password: password.to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token{
    #[serde(rename="token")]
    pub inner: String
}

impl Token{
    #[inline]
    pub fn new<S: ToString>(token: S) -> Self{
        Token{
            inner: token.to_string()
        }
    }
}

impl Deref for Token{
    type Target = String;

    #[inline]
    fn deref(&self) -> &Self::Target{
        &self.inner
    }
}
