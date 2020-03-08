use std::fmt;
use serde::export::Formatter;
use serde::Deserialize;
use serde::Serialize;
use crate::client::response::Response;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct BtmError{
    pub code: String,
    pub msg: String,
    pub error_detail: String,
}

impl std::error::Error for BtmError{
}

impl fmt::Display for BtmError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        let errmsg = format!(r#"{{"code":"{}","msg":"{}","error_detail":"{}"}}"#,self.code,self.msg,self.error_detail);
        write!(f,"{}",errmsg)
    }
}

impl BtmError{
    pub fn new<T> (response: Response<T>) -> Self
        where T:Serialize
    {
        Self{
            code: response.code,
            msg: response.msg,
            error_detail: response.detail
        }
    }
}