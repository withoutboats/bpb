use std::time::SystemTime;

use failure::Error;

use crate::config::Config;

pub struct KeyData {
    keypair: ed25519::Keypair,
    user_id: String, timestamp: u64,
}

impl KeyData {
    pub fn create(keypair: ed25519::Keypair, user_id: String, timestamp: u64) -> KeyData {
        KeyData { keypair, user_id, timestamp }
    }

    pub fn load(config: &Config) -> Result<KeyData, Error> {
        let keypair = ed25519::Keypair {
            public: ed25519::PublicKey::from_bytes(&hex::decode(config.public())?)?,
            secret: ed25519::SecretKey::from_bytes(&hex::decode(&*config.secret()?)?)?,
        };
        Ok(KeyData::create(keypair, config.user_id().to_owned(), config.timestamp()))
    }

    pub fn sign(&self, data: &[u8]) -> Result<pbp::PgpSig, Error> {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
        Ok(pbp::PgpSig::from_dalek::<sha2::Sha256, sha2::Sha512>(
            &self.keypair,
            data,
            self.fingerprint(),
            pbp::SigType::GenericCertification,
            timestamp as u32,
        ))
    }

    pub fn keypair(&self) -> &ed25519::Keypair {
        &self.keypair
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn fingerprint(&self) -> pbp::Fingerprint {
        self.public().fingerprint()
    }

    pub fn public(&self) -> pbp::PgpKey {
        pbp::PgpKey::from_dalek::<sha2::Sha256, sha2::Sha512>(
            &self.keypair,
            pbp::KeyFlags::SIGN | pbp::KeyFlags::CERTIFY,
            self.timestamp as u32,
            &self.user_id,
        )
    }
}
