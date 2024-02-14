use actix_web::{
    get, 
    post, 
    web, 
    Responder, 
    HttpResponse,
    HttpRequest,
};
use super::models;
use crate::{
    AppState,
    HASHMAP,
};
use sqlx::postgres::PgRow;
use sqlx::Row;
use chrono::Local;
use validator::Validate;

// ------------------------------------------------------------------------------------------------ health_check

#[get("/")]
async fn health_check() -> impl Responder {
    return HttpResponse::Ok();
}

// ------------------------------------------------------------------------------------------------ transacoes

#[post("/clientes/{id}/transacoes")]
async fn transacoes(req: HttpRequest, req_body: web::Json<models::TransacoesReqBody>, app_state: web::Data<AppState>) -> impl Responder {

    match req_body.validate() {
        Ok(_) => (),
        Err(_) => return HttpResponse::UnprocessableEntity().finish(),
    };

    let client_id = match req.match_info().get("id").unwrap().parse::<i32>() {
        Ok(r) => {
            if r > 5 || r < 0 { 
                return HttpResponse::NotFound().finish(); 
            }
            r
        },
        Err(_) => return HttpResponse::UnprocessableEntity().finish(),
    };

    let tipo_lower = req_body.tipo.to_lowercase();
    let limite = HASHMAP.get(&client_id).unwrap_or(&0);

    let mut valor_transacao = req_body.valor;
    if tipo_lower == "d" {
        valor_transacao = req_body.valor * -1;
    }
    match sqlx::query("SELECT * FROM \"realizar_transacao\"($1, $2, $3, $4, $5, $6)")
        .bind(valor_transacao)
        .bind(client_id)
        .bind(req_body.valor)
        .bind(tipo_lower)
        .bind(req_body.descricao.to_owned())
        .bind(limite)
        .map(|row: PgRow| models::RetornoRealizarTransacao {
            saldo: row.get("sa"),
        })
        .fetch_one(&app_state.db_pool)
        .await
        {
            Ok(r) => {
                let resp = models::TransacoesResp {
                    limite: *limite,
                    saldo: r.saldo,
                };
                return HttpResponse::Ok().json(resp)
            },
            Err(_) => return HttpResponse::UnprocessableEntity().finish(),
        }
}

// ------------------------------------------------------------------------------------------------ extrato

#[get("/clientes/{id}/extrato")]
async fn extrato(req: HttpRequest, app_state: web::Data<AppState>) -> impl Responder {

    let client_id = match req.match_info().get("id").unwrap().parse::<i32>() {
        Ok(r) => {
            if r > 5 || r < 0 { 
                return HttpResponse::NotFound().finish(); 
            }
            r
        },
        Err(_) => return HttpResponse::UnprocessableEntity().finish(),
    };

    let limite = HASHMAP.get(&client_id).unwrap();
    let hora_consulta = Local::now().format("%Y-%m-%dT%H:%M:%S.%fZ").to_string();

    // ------------------------------------------------------------------------------------------------ V1

    // match sqlx::query("SELECT valor FROM saldos WHERE cliente_id=$1 FOR UPDATE;")
    //     .bind(client_id)
    //     .map(|row: PgRow| models::Saldo {
    //         total: row.get("valor"),
    //         data_extrato: hora_consulta.to_owned(),
    //         limite: limite.to_owned(),
    //     })
    //     .fetch_one(&app_state.db_pool)
    //     .await
    //     {
    //         Ok(r1) => {
    //             match sqlx::query("SELECT valor, tipo, descricao, realizada_em FROM transacoes WHERE cliente_id=$1 ORDER BY realizada_em DESC LIMIT 10;")
    //                 .bind(client_id)
    //                 .map(|row: PgRow| models::UltimasTransacoes {
    //                     valor: row.get("valor"),
    //                     tipo: row.get("tipo"),
    //                     descricao: row.get("descricao"),
    //                     realizada_em: row.get("realizada_em"),
    //                 })
    //                 .fetch_all(&app_state.db_pool)
    //                 .await
    //                 {
    //                     Ok(r2) => {
    //                         let resp = models::ExtratoDB {
    //                             saldo: r1,
    //                             ultimas_transacoes: r2,
    //                         };
    //                         return HttpResponse::Ok().json(resp)
    //                     },
    //                     Err(e) => return HttpResponse::ImATeapot().body(e.to_string()),
    //                 }
    //         },
    //         Err(e) => return HttpResponse::ImATeapot().body(e.to_string()),
    //     }

    // ------------------------------------------------------------------------------------------------ V2

    let saldo = match sqlx::query("SELECT valor FROM saldos WHERE cliente_id=$1")
        .bind(client_id)
        .map(|row: PgRow| models::Saldo {
            total: row.get("valor"),
            data_extrato: hora_consulta.to_owned(),
            limite: limite.to_owned(),
        })
        .fetch_one(&app_state.db_pool)
        .await
        {
            Ok(r) => r,
            Err(e) => return HttpResponse::ImATeapot().body(e.to_string()),
        };

    let ultimas_transacoes = match sqlx::query("SELECT valor, tipo, descricao, realizada_em FROM transacoes WHERE cliente_id=$1 ORDER BY realizada_em DESC LIMIT 10;")
        .bind(client_id)
        .map(|row: PgRow| models::UltimasTransacoes {
            valor: row.get("valor"),
            tipo: row.get("tipo"),
            descricao: row.get("descricao"),
            realizada_em: row.get("realizada_em"),
        })
        .fetch_all(&app_state.db_pool)
        .await
        {
            Ok(r) => r,
            Err(e) => return HttpResponse::ImATeapot().body(e.to_string()),
        };

    let resp = models::ExtratoDB {
        saldo: saldo,
        ultimas_transacoes: ultimas_transacoes,
    };
    return HttpResponse::Ok().json(resp)
}

// ------------------------------------------------------------------------------------------------ extrato

#[get("/reset_db")]
async fn reset_db(app_state: web::Data<AppState>) -> impl Responder {

    match sqlx::query("CALL reset_db()")
        .execute(&app_state.db_pool)
        .await
        {
            Ok(_) => return HttpResponse::Ok().body("db reset!"),
            Err(e) => return HttpResponse::ImATeapot().body(e.to_string()),
        }
}

// ------------------------------------------------------------------------------------------------ config

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(health_check)
        .service(transacoes)
        .service(extrato)
        .service(reset_db);
}