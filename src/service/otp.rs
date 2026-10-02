use rand::{TryRng, rngs::SysRng};
use crate::error::error::AppError;
const BOUNDARY: u32 = 4_294_000_000;

pub fn generate_otp() -> Result<String, AppError>{
    let mut rng = SysRng;
    let otp_number = loop {
        let number = rng.try_next_u32()?;
        if number < BOUNDARY{
            break number % 1_000_000;

        }
    };
    let otp = format!("{:06}", otp_number);
    Ok(otp)

} 
