use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use validator::Validate;
use crate::errors::TaskError;
use crate::models::{CreateTaskDto, NewTask, Pool, Status, Task, UpdateTaskDto};
use crate::schema::task::dsl::*;
use log::{debug, error, warn, info, Level};

pub async fn create_task(
    pool: web::Data<Pool>,
    task_data: web::Json<CreateTaskDto>,
) -> Result<HttpResponse, TaskError> {
    task_data.validate().map_err(TaskError::ValidationError)?;
    
    info!("Creating new task with title: {}", task_data.title);
    
    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        TaskError::InternalError("Database connection error".into())
    })?;

    let new_task = NewTask {
        title: task_data.title.clone(),
        description: task_data.description.clone(),
        status: Status::Pending,
    };

    let created_task = diesel::insert_into(task)
        .values(&new_task)
        .get_result::<Task>(&mut conn)
        .map_err(|e| {
            error!("Failed to create task: {}", e);
            TaskError::DatabaseError(e.to_string())
        })?;

    info!("Successfully created task with id: {}", created_task.id);
    Ok(HttpResponse::Created().json(created_task))
}
pub async fn get_all_tasks(pool: web::Data<Pool>) -> impl Responder {

    let mut conn: PooledConnection<ConnectionManager<PgConnection>>  = pool.get().expect("Failed to get DB connection");

    
    match task.load::<Task>(&mut conn) {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_task(
    pool: web::Data<Pool>,
    task_id: web::Path<i32>
) -> Result<HttpResponse, TaskError> {
    info!("Fetching task with id: {}", task_id);
    
    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        TaskError::InternalError("Database connection error".into())
    })?;

    let task_id_inner = task_id.into_inner();
    let result = task.find(task_id_inner)
        .get_result::<Task>(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                warn!("Task not found with id: {}", task_id_inner);
                TaskError::NotFound(format!("Task with id {} not found", task_id_inner))
            }
            e => {
                error!("Database error while fetching task: {}", e);
                TaskError::DatabaseError(e.to_string())
            }
        })?;

    info!("Successfully fetched task with id: {}", task_id_inner);
    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_task(
    pool: web::Data<Pool>,
    task_id: web::Path<i32>,
    task_data: web::Json<UpdateTaskDto>,
) -> Result<HttpResponse, TaskError> {
    task_data.validate().map_err(TaskError::ValidationError)?;

    info!("Updating task with id: {}", task_id);

    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        TaskError::InternalError("Database connection error".into())
    })?;

    let task_id_inner = task_id.into_inner();
    let updated_task = diesel::update(task.find(task_id_inner))
        .set(&task_data.into_inner())
        .get_result::<Task>(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                warn!("Task not found with id: {}", task_id_inner);
                TaskError::NotFound(format!("Task with id {} not found", task_id_inner))
            }
            e => {
                error!("Failed to update task: {}", e);
                TaskError::DatabaseError(e.to_string())
            }
        })?;

    info!("Successfully updated task with id: {}", task_id_inner);
    Ok(HttpResponse::Ok().json(updated_task))
}

pub async fn delete_task(pool: web::Data<Pool>, task_id: web::Path<i32>) -> Result<HttpResponse, TaskError> {
    use crate::schema::task::dsl::*;
    use diesel::prelude::*;

    let mut conn = pool.get().map_err(|e| {
        error!("Failed to get DB connection: {}", e);
        TaskError::InternalError("Database connection error".into())
    })?;

    let task_id_inner = task_id.into_inner();

    match task.find(task_id_inner).first::<Task>(&mut conn) {
        Ok(_) => {
            match diesel::delete(task.find(task_id_inner)).execute(&mut conn) {
                Ok(_) => {
                    info!("Successfully deleted task with id: {}", task_id_inner);
                    Ok(HttpResponse::NoContent().finish())
                },
                Err(e) => {
                    error!("Failed to delete task: {}", e);
                    Err(TaskError::DatabaseError(e.to_string()))
                },
            }
        },
        Err(diesel::result::Error::NotFound) => {
            warn!("Task not found with id: {}", task_id_inner);
            Err(TaskError::NotFound(format!("Task with id {} not found", task_id_inner)))
        },
        Err(e) => {
            error!("Database error while fetching task: {}", e);
            Err(TaskError::DatabaseError(e.to_string()))
        },
    }
}
