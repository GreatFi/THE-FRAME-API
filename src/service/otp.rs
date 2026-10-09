use rand::{TryRng, rngs::SysRng};
use base64::{engine::general_purpose::STANDARD, Engine};
use aes_gcm::{aead::Aead,Aes256Gcm, KeyInit, Nonce};
use uuid::Uuid;
use chrono::{Duration, Utc};
use crate::repository::otp::{create_otp, increment_otp_attempts};
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

pub fn encrypt_otp(otp: &String) -> Result<String, AppError> {
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
            increment_otp_attempts(app_state, otp.id).await?;
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

pub async fn send_otp(app_state: &AppState, user_id: Uuid) -> Result<(), AppError>{

    let user = retrieve_user_by_id(app_state, user_id).await?;

    if user.is_verified {
        return Err(AppError::Conflict);
    }
    
    if app_state.send_limiter.check_key(&user_id).is_err() {
        return Err(AppError::TooManyRequests);
    }
    let existing_otp = get_otp(app_state, user_id).await?;
    let otp = match existing_otp{
        Some(otp) =>{
            let plaintext_otp = decrypt_otp(otp.code)?;
            plaintext_otp
        }
        None => {
            let otp = generate_otp()?;
            let encrypted_otp = encrypt_otp(&otp)?;
            let otp_expiry = Utc::now() + Duration::minutes(5);
            create_otp(app_state, user_id, encrypted_otp, otp_expiry).await?;
            otp
        }
    };



    app_state.mailer.send_otp_to_email(&user.email, &otp).await?;
    Ok(())

    // send the OTP to the user via email or SMS
}