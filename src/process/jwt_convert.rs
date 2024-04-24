use anyhow::{Ok, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::SignOpts;
const SECRET: &str = "secret";

pub fn process_jwt_sign(claims: SignOpts) -> Result<()> {
    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &claims, &EncodingKey::from_secret(SECRET.as_ref()))?;
    println!("{}", token);
    Ok(())
}
pub fn process_jwt_verify(sig: String) -> Result<String> {
    println!("{}", sig);
    let mut validation = Validation::new(Algorithm::HS512);
    validation.validate_exp = false;
    validation.validate_aud = false;
    let token = decode::<SignOpts>(
        &sig,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &validation,
    )?;
    let result = format!("{:?}", token);
    Ok(result)
}
