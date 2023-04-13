use mysql_async::{prelude::Queryable,params};
use axum::{
    http::StatusCode,
    extract::{Query,Json}
};
use serde::Serialize;
use std::collections::HashMap;
use serde_json::{Value, json};
use encoding_rs::GBK;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
struct Student {
    pub term: String,
    pub grade: String,
    pub sno: String,
    pub stu_name: String,
    pub sex: String,
    pub fname: String,
    pub sname: String,
    pub tcno: String,
    pub lcno: String,
    pub level: String,
    pub enable: String
}

pub mod database;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
struct Response {
    pub success: bool,
    pub data: Option<Student>
}

async fn verify_userpw(user_id:&str,user_pw:&str)->Option<Student>{
    let mut conn = database::get_connect().await;

    let (user_id,_,_) = GBK.encode(user_id);
    let (user_pw,_,_) = GBK.encode(user_pw);

    let query_result = conn.exec_first("call proc_hwapp_forum_userauth(:un,MD5(:up))", params!{
        "un"=>user_id.to_vec(),
        "up"=>user_pw.to_vec()}).await.map( |row|{
            row.map(|(term,grade,sno,stu_name,sex,fname,sname,tcno,lcno,level,enable)| {
                
                let term:Option<Vec<u8>> = term;
                let term = term.unwrap_or([].to_vec());
                let (term,_,_) = GBK.decode(&term);
                let term = term.to_string();
                
                let grade:Option<Vec<u8>> = grade;
                let grade = grade.unwrap_or([].to_vec());
                let (grade, _, _) = GBK.decode(&grade);
                let grade = grade.to_string();

                let sno:Option<Vec<u8>> = sno;
                let sno = sno.unwrap_or([].to_vec());
                let (sno, _, _) = GBK.decode(&sno);
                let sno = sno.to_string();

                let stu_name:Option<Vec<u8>> = stu_name;
                let stu_name = stu_name.unwrap_or([].to_vec());
                let (stu_name, _, _) = GBK.decode(&stu_name);
                let stu_name = stu_name.to_string();

                let sex:Option<Vec<u8>> = sex;
                let sex = sex.unwrap_or([].to_vec());
                let (sex, _, _) = GBK.decode(&sex);
                let sex = sex.to_string();

                let fname:Option<Vec<u8>> = fname;
                let fname = fname.unwrap_or([].to_vec());
                let (fname, _, _) = GBK.decode(&fname);
                let fname = fname.to_string();

                let sname:Option<Vec<u8>> = sname;
                let sname = sname.unwrap_or([].to_vec());
                let (sname, _, _) = GBK.decode(&sname);
                let sname = sname.to_string();

                let tcno:Option<Vec<u8>> = tcno;
                let tcno = tcno.unwrap_or([].to_vec());
                let (tcno, _, _) = GBK.decode(&tcno);
                let tcno = tcno.to_string();

                let lcno:Option<Vec<u8>> = lcno;
                let lcno = lcno.unwrap_or([].to_vec());
                let (lcno, _, _) = GBK.decode(&lcno);
                let lcno = lcno.to_string();

                let level:Option<Vec<u8>> = level;
                let level = level.unwrap_or([].to_vec());
                let (level, _, _) = GBK.decode(&level);
                let level = level.to_string();

                let enable:Option<Vec<u8>> = enable;
                let enable = enable.unwrap_or([].to_vec());
                let (enable, _, _) = GBK.decode(&enable);
                let enable = enable.to_string();
                
                Student{term,grade,sno,stu_name,sex,fname,sname,tcno,lcno,level,enable}
            })
        });
    match query_result {
        Ok(result)=>{
            return result
        }
        Err(_)=>{
            return None
        }
    }
}

pub async fn login_auth(Query(_params): Query<HashMap<String,String>>) -> (StatusCode, Json<Value>){
    let stu_no = _params.get("stuno");
    let stu_pw = _params.get("stupw");

    if stu_no.is_none() || stu_pw.is_none(){
        return (StatusCode::BAD_REQUEST, Json(json!("Not enough query parameter")));
    }

    let resp = verify_userpw(stu_no.unwrap(), stu_pw.unwrap()).await;

    if resp.is_none(){
        return (StatusCode::OK, Json(json!(Response{success:false,data:None})));
    }

    return (StatusCode::OK, Json(json!(Response{success:true,data:resp})));
}