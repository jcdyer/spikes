use user::Role;

pub mod errors {
    pub struct AccessError;
}

pub mod user {
    use crate::errors;
    use http::Request;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Role {
        Admin,
        Unauthorized,
    }

    pub struct User {
        role: Role,
    }

    impl User {
        pub fn new(role: Role) -> User {
            User { role }
        }

        pub fn from_request<B>(req: &Request<B>) -> Result<User, errors::AccessError> {
            let admin_token = std::env::var("ADMIN_TOKEN").unwrap();

            let auth_value = auth_header(req);
            let auth_value = auth_value.as_deref();
            match auth_value {
                Some(["Bearer", token]) if token == &admin_token => {
                    Ok(User { role: Role::Admin })
                }
                Some(_) => Err(errors::AccessError),
                None => Ok(User {
                    role: Role::Unauthorized,
                }),
            }
        }

        pub fn role(&self) -> Role {
            self.role
        }
    }

    pub fn auth_header<'rq, B>(req: &'rq Request<B>) -> Option<Vec<&'rq str>> {
        req.headers()
            .get("Authorization")
            .and_then(|val| val.to_str().ok())
            .map(|val| val.split(' '))
            .map(|split| split.collect::<Vec<&str>>())
    }
}



pub mod auth_user {
    use crate::errors;
    use http::Request;

    pub struct AdminUser {
        _private: (),
    }

    impl AdminUser {
        fn new() -> AdminUser {
            AdminUser { _private: () }
        }
    }

    pub enum User {
        Admin(AdminUser),
        Unauthorized,
    }
    impl User {
        pub fn from_request<B>(req: &Request<B>) -> Result<User, errors::AccessError> {
            let admin_token = std::env::var("ADMIN_TOKEN").unwrap();
            let auth_value = auth_header(req);
            let auth_value = auth_value.as_deref();
            match auth_value {
                Some(["Bearer", token]) if token == &admin_token => {
                    Ok(User::Admin(AdminUser::new()))
                }
                Some(_) => Err(errors::AccessError),
                None => Ok(User::Unauthorized),
            }
        }
        #[cfg(test)]
        pub fn admin_user() -> User {
            User::Admin(AdminUser::new())
        }

        #[cfg(test)]
        pub fn unauthorized_user() -> User {
            User::Unauthorized
        }
    }

    fn auth_header<'rq, B>(req: &'rq Request<B>) -> Option<Vec<&'rq str>> {
        req.headers()
            .get("Authorization")
            .and_then(|val| val.to_str().ok())
            .map(|val| val.split(' '))
            .map(|split| split.collect::<Vec<&str>>())
    }
}

fn protected_route(user: user::User) -> Result<&'static str, errors::AccessError> {
    if user.role() == Role::Admin {
        Ok("Authorized")
    } else {
        Err(errors::AccessError)
    }
}

fn protected_route_2(user: &auth_user::AdminUser) -> &'static str {
    "Authorized"
}
