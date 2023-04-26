use crate::db_access::teacher;
use crate::errors::MyError;
use crate::models::teacher::{ CreateTeacher, Teacher, UpdateTeacher };
use sqlx::mysql::MySqlPool;

pub async fn get_all_teachers_db(pool: &MySqlPool) -> Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query!("SELECT id, name, picture_url, profile FROM teacher")
        .fetch_all(pool)
        .await?;

    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone().unwrap_or("".into()),
            picture_url: r.picture_url.clone().unwrap_or("".into()),
            profile: r.profile.clone().unwrap_or("".into())
        })
        .collect();

    match teachers.len() { 
        0 => Err(MyError::NotFound("No teachers found".into())),
        _ => Ok(teachers)
    }
}

pub async fn get_teacher_details_db(pool: &MySqlPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        "SELECT id, name, picture_url, profile FROM teacher WHERE id = ?",
        teacher_id
    )
    .fetch_one(pool)
    .await
    .map(|r| Teacher {
        id: r.id,
        name: r.name.unwrap(),
        picture_url: r.picture_url.unwrap(),
        profile: r.profile.unwrap()
    })
    .map_err(|_err| MyError::NotFound("Teacher Id not found".into()))?;

    Ok(row)
}

// pub async fn post_new_teacher_db(
//     pool: &MySqlPool,
//     new_teacher: CreateTeacher
// ) -> Result<Teacher, MyError> {
//     let row = sqlx::query!(
//         "INSERT INTO teacher (name, pictuer_url, profile)
//         VALUES(?,?,?) RETURNING id, name, picture_url, profile",
//         new_teacher.name,
//         new_teacher.pictuer_url,
//         new_teacher.profile
//     )
//     .fetch_one(pool)
//     .await?;

//     Ok(Teacher { id: row.id, name: row.name, picture_url: row.picture, profile: row.profile })
// }