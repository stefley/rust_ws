 use super::models::*;
 use chrono::NaiveDateTime;
 use sqlx::mysql::MySqlPool;
 use super::errors::MyError;

 pub async fn get_courses_for_teacher_db
 (pool: &MySqlPool, teacher_id: i32) -> Result<Vec<Course>, MyError> {
    let rows = sqlx::query!(
        r#"SELECT id, teacher_id, name, time
        FROM course
        WHERE teacher_id = ?"#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    let courses: Vec<Course> = rows.iter()
        .map(|r| Course {
            id: r.id,
            teacher_id: r.teacher_id.unwrap(),
            name: r.name.clone().unwrap(),
            time: Some(NaiveDateTime::from(r.time.unwrap()))
        })
        .collect();

    // courses长度为0则未找到
    match courses.len() {
        0 => Err(MyError::NotFound("Courses not found for teacher".into())),
        _ => Ok(courses),
    }
 }

 pub async fn get_course_details_db(pool: &MySqlPool, teacher_id: i32, course_id: i32) -> Result<Course, MyError> {
     let row = sqlx::query!(
        r#"SELECT id, teacher_id, name, time
        FROM course
        WHERE teacher_id = ? and id = ?"#,
        teacher_id,
        course_id
     )
     .fetch_one(pool)
     .await;

    if let Ok(row) = row {
        Ok(Course {
            id: row.id,
            teacher_id: row.teacher_id.unwrap(),
            name: row.name.clone().unwrap(),
            time:Some(NaiveDateTime::from(row.time.unwrap())) 
        })
    } else {
        Err(MyError::NotFound("Course Id not found".into()))
    }

 }

 pub async fn post_new_course_db(pool: &MySqlPool, new_course: Course) -> Result<Course, MyError> {
    let row = sqlx::query!(
        r#"INSERT INTO course (id, teacher_id, name)
        VALUES (?,?,?)"#,
        new_course.id,
        new_course.teacher_id,
        new_course.name
    )
    .execute(pool)
    .await?;

    println!("result: {:?}", row);

    let row = sqlx::query!(
        r#"SELECT * FROM course WHERE id = ?"#,
        row.last_insert_id()
    )
    .fetch_one(pool)
    .await?;
    
    Ok(Course {
        id: row.id,
        teacher_id: row.teacher_id.unwrap(),
        name: row.name.clone().unwrap(),
        time: Some(NaiveDateTime::from(row.time.unwrap()))
    })
 }