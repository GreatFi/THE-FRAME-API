use rand::{TryRng, rngs::SysRng};
use base64::{engine::general_purpose::STANDARD, Engine};
use aes_gcm::{aead::Aead,Aes256Gcm, KeyInit, Nonce};
use uuid::Uuid;
use chrono::{Duration, Utc};
use crate::{entity::auth::User, error::error::AppError, repository::{auth::{retrieve_user_by_id, verify_user}, otp::{get_otp, mark_otp_used}}, service::auth::{generate_access_token, generate_refresh_token}, states::appstate::AppState};
use std::env;

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

pub fn encrypt_otp(otp: String) -> Result<String, AppError> {
    let key = env::var("OTP_ENCRYPTION_KEY")?;

    let key = STANDARD.decode(key)?;
    let cipher = Aes256Gcm::new_from_slice(&key)?;
    let mut rng = SysRng;
    let mut nonce_bytes = [0u8; 12];

    rng.try_fill_bytes(&mut nonce_bytes)?;
    let nonce = Nonce::from(nonce_bytes);
    let ciphertext = cipher.encrypt(&nonce, otp.as_bytes())?;
    let mut encrypted_data = nonce_bytes.to_vec();
    encrypted_data.extend_from_slice(&ciphertext);
    let encrypted_otp = STANDARD.encode(encrypted_data);
    Ok(encrypted_otp)
}

pub fn decrypt_otp(encrypted_otp: String) -> Result<String, AppError> {
    let key = env::var("OTP_ENCRYPTION_KEY")?;
    let key = STANDARD.decode(key)?;

    let cipher = Aes256Gcm::new_from_slice(&key)?;

    let encrypted_data = STANDARD.decode(encrypted_otp)?;

    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);

    let nonce_bytes: [u8; 12] = nonce_bytes
        .try_into()
        .map_err(|_| AppError::InvalidOtpData)?;

    let nonce = Nonce::from(nonce_bytes);

    let plaintext = cipher.decrypt(&nonce, ciphertext)?;

    let otp = String::from_utf8(plaintext)
        .map_err(|_| AppError::InvalidOtpData)?;

    Ok(otp)
}

pub async fn verify_otp(app_state: &AppState, user_id: Uuid, submitted_otp: &str) -> Result<(User, String, String), AppError>{
    let otp = get_otp(app_state, user_id).await?;
    match otp{
        Some(otp) => {
            let decrypt_otp = decrypt_otp(otp.code)?;

            if submitted_otp != decrypt_otp{
                return Err(AppError::Unauthorized);
            }else{
                let mut tx = app_state.pool.begin().await?;
                mark_otp_used(otp.id, &mut tx).await?;
                verify_user(&mut tx, user_id).await?;
                tx.commit().await?;

                let user = retrieve_user_by_id(app_state, user_id).await?;
                let ref_token = generate_refresh_token(app_state, user_id).await?;
                let access_token = generate_access_token(app_state, user_id, (Utc::now() + Duration::minutes(15)).timestamp()).await?;
                Ok((user, ref_token, access_token))
            }

        }
        None => Err(AppError::Unauthorized)
    }
}