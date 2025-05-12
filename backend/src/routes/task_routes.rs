use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTask {
    pub name: String,
    pub notes: Option<String>,
    pub project_id: Option<Uuid>,
}

#[derive(Serialize)]
struct TaskResponse {
    id: Uuid,
    name: String,
    notes: Option<String>,
}

pub async fn create_task(
    db: web::Data<PgPool>,
    payload: web::Json<CreateTask>,
) -> impl Responder {
    let task_id = Uuid::new_v4();

    let result = sqlx::query!(
        r#"
        INSERT INTO tasks (id, name, notes, project_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, notes
        "#,
        task_id,
        payload.name,
        payload.notes,
        payload.project_id
    )
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(TaskResponse {
            id: record.id,
            name: record.name,
            notes: record.notes,
        }),
        Err(e) => {
            eprintln!("Failed to insert task: {e}");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn task_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/tasks")
            .route(web::post().to(create_task))
    );
}
