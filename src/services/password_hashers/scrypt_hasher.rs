use scrypt::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Scrypt,
};
use shaku::Component;

#[derive(Component)]
#[shaku(interface = super::hasher::PasswordHasher)]
pub struct ScryptHasher {}

#[tonic::async_trait]
impl super::hasher::PasswordHasher for ScryptHasher {
    async fn hash_password(&self, password: String) -> String {
        let password_hashing_task = tokio::task::spawn_blocking(move || {
            Scrypt
                .hash_password_customized(
                    password.as_bytes(),
                    None,
                    None,
                    scrypt::Params::new(14, 8, 1).unwrap(),
                    SaltString::generate(&mut OsRng).as_salt(),
                )
                .expect("Error while calculating password's hash")
                .to_string()
        });

        password_hashing_task
            .await
            .expect("Error while awaiting the password hashing task")
    }

    async fn verify_password(&self, password: String, hash: String) -> bool {
        let password_verification_task = tokio::task::spawn_blocking(move || {
            let password_hash = PasswordHash::new(hash.as_str())
                .expect("Error while converting string hash to PasswordHash");

            match Scrypt.verify_password(password.as_bytes(), &password_hash) {
                Ok(_) => true,
                Err(_) => false,
            }
        });

        password_verification_task
            .await
            .expect("Error while verifying the password")
    }
}
