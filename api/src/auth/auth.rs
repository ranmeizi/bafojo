use axum::{
    extract::{Json, Path, Query as ReqQuery, State},
    http::{header::SET_COOKIE, StatusCode},
    response::{AppendHeaders, IntoResponse},
};
use axum_extra::extract::WithRejection;
use bfj_common::{dto::auth::LoginSuccDto,error::{AuthErr, CustErr}, res::Res,utils::jwt,AppState};
use bfj_core::{auth, crypto::into_md5_psw, system::user, PageParams};
use serde::Deserialize;

/**
 * admin 登陆接口
 */
pub async fn login(
    state: State<AppState>,
    WithRejection(params, _): WithRejection<Json<auth::LoginParams>, Res>,
) -> impl IntoResponse {
    // 获取 密码 和 盐
    let res = auth::Query::get_psw_by_uname(&state.db, &params.0.uname).await;

    if res.is_err() {
        // 没有此用户
        return Res::error(AuthErr::NoUser.into());
    };

    let (psw, salt,uid) = res.unwrap();

    // 检查密码是否正确
    let calc_psw = into_md5_psw(&params.0.psw, &salt);

    if !psw.eq(&calc_psw) {
        // 密码错误
        return Res::error(AuthErr::PasswordError.into());
    };

    // 签发 token
    let res = jwt::authorize(&params.0.uname,uid);

    if res.is_err() {
        return Res::error(CustErr::UnexpectedError("token 签发失败".into()).into())
    };

    // TODO refresh token

    // res
    let (token,exp) = res.unwrap();
    let dto = LoginSuccDto {
        access_token:token,
        refresh_token:"".into(),
        expire:exp
    };

    return Res::success(dto);
}
