use serde::{
    Deserialize,
    Serialize,
};
use sqlx::{
    self,
    FromRow,
};
use validator::{
    Validate,
    ValidationError,
};
use chrono;

// ------------------------------------------------------------------------------------------------ Transacoes

#[derive(Debug, Serialize, FromRow)]
pub struct TransacoesDB {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct TransacoesReqBody {
    #[validate(range(min = 1))]
    pub valor:  i32,
    #[validate(length(equal = 1), custom = "validate_type")]
    pub tipo: String,
    #[validate(length(min = 1, max = 10))]
    pub descricao: String,
}

#[derive(Debug, Serialize)]
pub struct TransacoesResp {
    pub limite: i32,
    pub saldo: i32,
}

// ------------------------------------------------------------------------------------------------ Extrato

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Saldo {
    pub total: i32,
    pub data_extrato: String,
    pub limite: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UltimasTransacoes {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
    // pub realizada_em: chrono::DateTime<chrono::Utc>>, // TIMESTAMPZ
    pub realizada_em: chrono::NaiveDateTime<>, // TIMESTAMP
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ExtratoDB {
    pub saldo: Saldo,
    pub ultimas_transacoes: Vec<UltimasTransacoes>,
}

// ------------------------------------------------------------------------------------------------ 

#[derive(Debug, FromRow)]
pub struct ValorSaldo {
	pub saldo_atual: i32
}

// ------------------------------------------------------------------------------------------------ 

fn validate_type(tipo: &str) -> Result<(), ValidationError> {
    if tipo.to_lowercase() != "c".to_string() && tipo.to_lowercase() != "d".to_string() {
        return Err(ValidationError::new("tipo invalido"));
    }
    Ok(())
}