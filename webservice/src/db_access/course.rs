 use crate::models::course::{Course, UpdateCourse, CreateCourse};
 use crate::errors::MyError;
 use sqlx::mysql::MySqlPool;

 pub async fn get_courses_for_teacher_db
 (pool: &MySqlPool, teacher_id: i32) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"SELECT *
        FROM course
        WHERE teacher_id = ?"#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    // let courses: Vec<Course> = rows.iter()
    //     .map(|r| Course {
    //         id: r.id,
    //         teacher_id: r.teacher_id.unwrap(),
    //         name: r.name.clone().unwrap(),
    //         time: Some(NaiveDateTime::from(r.time.unwrap()))
    //     })
    //     .collect();

    // // courses长度为0则未找到
    // match courses.len() {
    //     0 => Err(MyError::NotFound("Courses not found for teacher".into())),
    //     _ => Ok(courses),
    // }

    Ok(rows)
 }

 pub async fn get_course_details_db(pool: &MySqlPool, teacher_id: i32, course_id: i32) -> Result<Course, MyError> {
     let row = sqlx::query_as!(
        Course,
        r#"SELECT *
        FROM course
        WHERE teacher_id = ? and id = ?"#,
        teacher_id,
        course_id
     )
     .fetch_optional(pool)
     .await?;

    // if let Ok(row) = row {
    //     Ok(Course {
    //         id: row.id,
    //         teacher_id: row.teacher_id.unwrap(),
    //         name: row.name.clone().unwrap(),
    //         time:Some(NaiveDateTime::from(row.time.unwrap())) 
    //     })
    // } else {
    //     Err(MyError::NotFound("Course Id not found".into()))
    // }
    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course Id not found".into()))
    }

 }

 pub async fn post_new_course_db(pool: &MySqlPool, new_course: CreateCourse) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level)
        VALUES (?,?,?,?,?,?,?,?,?)"#,
        new_course.teacher_id,
        new_course.name,
        new_course.description,
        new_course.format,
        new_course.structure,
        new_course.duration,
        new_course.price,
        new_course.language,
        new_course.level,
    )
    .execute(pool)
    .await?;


    let row = sqlx::query!(
        r#"SELECT * FROM course WHERE id = ?"#,
        row.last_insert_id()
    )
    .fetch_one(pool)
    .await?;
    
    Ok(Course {
        id: row.id,
        teacher_id: row.teacher_id,
        name: row.name.clone(),
        time: row.time.clone(),
        description: row.description.clone(),
        format: row.format.clone(),
        structure: row.structure.clone(),
        duration: row.duration.clone(),
        price: row.price,
        language: row.language.clone(),
        level: row.level.clone(),
    })
 }

 pub async fn delete_course_db(pool: &MySqlPool, teacher_id: i32, id: i32) -> Result<String, MyError > {
    let course_row = sqlx::query!(
        "DELETE FROM course WHERE teacher_id = ? and id =?",
        teacher_id,
        id
    )
    .execute(pool)
    .await?;
    Ok(format!("Deleted {:?} record", course_row))
 }

 pub async fn update_course_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse
 ) -> Result<Course, MyError> {
    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM course WHERE teacher_id = ? and id = ?",
        teacher_id,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Course Id not found".into()))?;

    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name.unwrap_or_default()
    };
    let description: String = if let Some(desc) = update_course.description {
        desc
    } else {
        current_course_row.description.unwrap_or_default()
    };
    let format: String = if let Some(format) = update_course.format {
        format
    } else {
        current_course_row.format.unwrap_or_default()
    };
    let structure: String = if let Some(structure) = update_course.structure {
        structure
    } else {
        current_course_row.structure.unwrap_or_default()
    };
    let duration: String = if let Some(duration) = update_course.duration {
        duration
    } else {
        current_course_row.duration.unwrap_or_default()
    };
    let price: i32 = if let Some(price) = update_course.price {
        price
    } else {
        current_course_row.price.unwrap_or_default()
    };
    let language: String = if let Some(language) = update_course.language {
        language
    } else {
        current_course_row.language.unwrap_or_default()
    };
    let level: String = if let Some(level) = update_course.level {
        level
    } else {
        current_course_row.level.unwrap_or_default()
    }; 

    let course_row = sqlx::query_as!(
        Course,
        "UPDATE course SET name = ?, description = ?, format =?, structure = ?, duration = ?, price = ?, language = ?, level = ?
            WHERE teacher_id = ? and id = ?",
            name,
            description,
            format,
            structure,
            duration,
            price,
            language,
            level,
            teacher_id,
            id
    )
    .execute(pool)
    .await?;

    let course_row = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE id = ?"#,
        course_row.last_insert_id()
    )
    .fetch_one(pool)
    .await?;

    Ok(course_row)
 }