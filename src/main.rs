use actix_web::{delete, post, get, web, HttpServer, App, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use dotenv::dotenv;
use std::env;

#[derive(Deserialize, Serialize, Clone, sqlx::FromRow)]
struct Actividad {
    id: i32,
    encargado: String,
    credito: String,
    nombre: String,
    descripcion: String,
}

#[post("/insertar-actividad")]
async fn insertar_actividad(
    actividad: web::Form<Actividad>,
    pool: web::Data<MySqlPool>
) -> HttpResponse {
    let res = sqlx::query(
        "INSERT INTO actividades (encargado, credito, nombre, descripcion) VALUES (?, ?, ?, ?)"
    )
    .bind(&actividad.encargado)
    .bind(&actividad.credito)
    .bind(&actividad.nombre)
    .bind(&actividad.descripcion)
    .execute(pool.get_ref())
    .await;

    match res {
        Ok(_) => HttpResponse::Ok().json("Actividad insertada correctamente"),
        Err(_) => HttpResponse::InternalServerError().json("Error al insertar actividad"),
    }
}

#[get("/actividad")]
async fn actividades_extra(pool: web::Data<MySqlPool>) -> HttpResponse {
    let actividades = sqlx::query_as::<_, Actividad>("SELECT * FROM actividades")
    .fetch_all(pool.get_ref())
    .await;

    match actividades {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().json("Error al obtener actividades"),
    }
}

#[get("/actividad-id/{id}")]
async fn actividad_id(
    parametros: web::Path<u8>,
    pool: web::Data<MySqlPool>
) -> HttpResponse {
    let actividades = sqlx::query_as::<_, Actividad>("SELECT * FROM actividades WHERE id = ?")
    .bind(&parametros.into_inner())
    .fetch_all(pool.get_ref())
    .await;

    match actividades {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().json("Error al obtener actividades"),
    }
}

#[delete("/borrar-actividad/{id}")]
async fn borrar_actividad(
    parametros: web::Path<u8>, 
    pool: web::Data<MySqlPool>
) -> HttpResponse {
    let res = sqlx::query(
        "DELETE FROM actividades WHERE id = ?"
    )
    .bind(&parametros.into_inner())
    .execute(pool.get_ref())
    .await;

    match res {
        Ok(_) => HttpResponse::Ok().json("Actividad eliminada correctamente"),
        Err(_) => HttpResponse::InternalServerError().json("Error al insertar actividad"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no está configurada");
    println!("DATABASE_URL: {}", database_url);
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error al conectar a la base de datos");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(insertar_actividad)
            .service(actividades_extra)
            .service(actividad_id)
            .service(borrar_actividad)
    })
    .bind(("0.0.0.0", 5050))?
    .run()
    .await
}