use anyhow::{Ok, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::JwtSignOpts;

const SECRET: &str = "secret";

pub fn process_jwt_sign(claims: JwtSignOpts) -> Result<()> {
    let header = Header::new(Algorithm::HS512);
    let token = encode(&header, &claims, &EncodingKey::from_secret(SECRET.as_ref()))?;
    println!("{}", token);
    Ok(())
}
pub fn process_jwt_verify(sig: String) -> Result<String> {
    let mut validation = Validation::new(Algorithm::HS512);
    validation.validate_exp = false;
    validation.validate_aud = false;
    println!("{}", sig);
    let token = decode::<JwtSignOpts>(
        &sig,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &validation,
    )?;
    let result = format!("{:?}", token);
    println!("{:?}", result);
    Ok(result)
}
