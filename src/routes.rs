use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![
        controller::user_controller::create_user,
        controller::user_controller::get_user,
        controller::user_controller::get_list,
        controller::user_controller::update_user,
        controller::user_controller::delete_user,
    ]
}
