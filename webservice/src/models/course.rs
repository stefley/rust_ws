use crate::errors::MyError;
use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub teacher_id: Option<i32>,
    pub id: i32,
    pub name: Option<String>,
    pub time: Option<NaiveDateTime>,

    pub description: Option<String>, // 课程描述
    pub format: Option<String>,      // 课程格式
    pub structure: Option<String>,   // 课程结构
    pub duration: Option<String>,    // 课程持续时间
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: Option<String>,
    pub description: Option<String>, // 课程描述
    pub format: Option<String>,      // 课程格式
    pub structure: Option<String>,   // 课程结构
    pub duration: Option<String>,    // 课程持续时间
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

// impl From<web::Json<CreateCourse>> for CreateCourse {
//     fn from(course: web::Json<CreateCourse>) -> Self {
//         CreateCourse {
//             teacher_id: course.teacher_id,
//             name: course.name.clone(),
//             description: course.format.clone(),
//             format: course.format.clone(),
//             structure: course.structure.clone(),
//             duration: course.duration.clone(),
//             price: course.price,
//             language: course.language.clone(),
//             level: course.level.clone(),
//         }
//     }
// }

impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = MyError;

    fn try_from(course: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>, // 课程描述
    pub format: Option<String>,      // 课程格式
    pub structure: Option<String>,   // 课程结构
    pub duration: Option<String>,    // 课程持续时间
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(course: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: course.name.clone(),
            description: course.description.clone(),
            format: course.format.clone(),
            structure: course.structure.clone(),
            duration: course.duration.clone(),
            price: course.price,
            language: course.language.clone(),
            level: course.level.clone(),
        }
    }
}
