use std::time::Duration;

use anyhow::anyhow;
use sqlx::PgPool;
use temporal_sdk::{ActContext, ActivityError, ActivityOptions, WfContext};
use types::auth::{ LoginWithEmailInput, Session, SignUpInput, User, UserCredentials };
use argon2::{
    password_hash::{
        rand_core::OsRng,
         PasswordHasher, SaltString
    },
    Argon2, PasswordHash, PasswordVerifier
};

use crate::temporal_helpers::execute_activity;

pub async fn sign_up_wf(ctx: WfContext, input: SignUpInput) -> Result<String, anyhow::Error> {        
    let res = execute_activity(&ctx, ActivityOptions {
        activity_type: "sign-up-activity".into(),        
        start_to_close_timeout: Some(Duration::from_secs(5)),        
        ..Default::default()},
        sign_up_activity,
        input        
    ).await?;
    
    Ok(res)
}

pub async fn login_wf(ctx: WfContext, input: LoginWithEmailInput) -> Result<String, anyhow::Error> {        
    let res = execute_activity(&ctx, ActivityOptions {
        activity_type: "login-activity".into(),        
        start_to_close_timeout: Some(Duration::from_secs(5)),        
        ..Default::default()},
        login_activity,
        input        
    ).await?;
    
    Ok(res)
}

pub async fn sign_up_activity(
    ctx: ActContext, input: SignUpInput,
) -> Result<String, ActivityError> {    
    let pool = ctx.app_data::<PgPool>().unwrap();    
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = argon2.hash_password(input.password.as_bytes(), &salt).unwrap().to_string();    
    let mut tx = pool.begin().await?;    
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (first_name, last_name)
        VALUES ($1, $2)
        RETURNING *
        "#,
        input.first_name,
        input.last_name        
    )
    .fetch_one(&mut *tx)
    .await?;
    
    let _credentials = sqlx::query_as!(
        UserCredentials,
        r#"
        INSERT INTO user_credentials (user_id, email, hashed_password)
        VALUES ($1, lower($2), $3)
        RETURNING *
        "#,
        user.id,
        input.email.to_lowercase(),
        hashed_password       
    )
    .fetch_one(&mut *tx)
    .await?;
    
    tx.commit().await?;
    
    Ok(user.id.into())

}

pub async fn login_activity(
    ctx: ActContext, input: LoginWithEmailInput,
) -> Result<String, ActivityError> {    
    let pool = ctx.app_data::<PgPool>().unwrap();     
    let credentials = sqlx::query_as!(
        UserCredentials,
        r#"
        SELECT * from user_credentials
        WHERE email = lower($1)
        "#,        
        input.email     
    )
    .fetch_one(pool)
    .await?;    
    let parsed_hash = PasswordHash::new(&credentials.hashed_password).map_err(|e| anyhow!(e))?;
    if Argon2::default().verify_password(input.password.as_bytes(), &parsed_hash).is_ok() {
        let session = sqlx::query_as!(
            Session,
            r#"
            INSERT INTO sessions (user_id, expires_at)
            VALUES ($1, current_timestamp + interval '7 days')
            RETURNING *
            "#,
            credentials.user_id,
            
        )
        .fetch_one(pool)
        .await?;    

        return Ok(session.id.into())
    }
        
    Err(ActivityError::NonRetryable(anyhow!("Invalid credentials".to_string())))

}