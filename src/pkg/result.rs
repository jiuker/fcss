use std::error::Error;
use core::result;

pub type CommonResult<T> = result::Result<T,Box<dyn Error>>;

