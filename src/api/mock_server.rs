use mockito::{Mock, Server, ServerGuard};

pub struct MockMatrix {
    pub server: ServerGuard,

    pub profile_endpoint: Mock,
    pub login_endpoint: Mock,

    pub room_members_endpoint: Mock,
    pub join_room_endpoint: Mock,

    pub send_message_endpoint: Mock,
}

impl MockMatrix {
    pub fn new(room: &str, full_username: &str) -> Self {
        let mut server = Server::new();
        let base_url = format!("http://{}", server.host_with_port());

        let profile_endpoint =
            MockMatrix::generate_profile_endpoint(&mut server, base_url.as_str(), full_username);
        let login_endpoint = MockMatrix::generate_login_endpoint(&mut server, base_url.as_str());
        let room_members_endpoint = MockMatrix::generate_room_members_endpoint(
            &mut server,
            base_url.as_str(),
            room,
            full_username,
        );
        let join_room_endpoint =
            MockMatrix::generate_join_room_endpoint(&mut server, base_url.as_str(), room);
        let send_message_endpoint =
            MockMatrix::generate_send_message_endpoint(&mut server, base_url.as_str(), room);

        Self {
            server,
            profile_endpoint,
            login_endpoint,
            room_members_endpoint,
            join_room_endpoint,
            send_message_endpoint,
        }
    }

    fn generate_profile_endpoint(
        server: &mut ServerGuard,
        base_url: &str,
        full_username: &str,
    ) -> Mock {
        let url = crate::config::build_profile_url(base_url, full_username);
        let stripped_url = url
            .strip_prefix(base_url)
            .expect("Base URL missing from built url");

        let response_body = format!(
            r#"
{{
}}
"#,
        );

        server
            .mock("GET", stripped_url)
            .with_status(200)
            .with_body(response_body.as_str())
            .create()
    }

    fn generate_login_endpoint(server: &mut ServerGuard, base_url: &str) -> Mock {
        let url = crate::config::build_login_url(base_url);
        let stripped_url = url
            .strip_prefix(base_url)
            .expect("Base URL missing from built url");

        let response_body = format!(
            r#"
{{
    "access_token": "testtoken",
    "user_id": "testuser",
    "home_server": "testmatrix",
    "device_id": "testingdevice"
}}
"#,
        );

        server
            .mock("POST", stripped_url)
            .with_status(200)
            .with_body(response_body.as_str())
            .create()
    }

    fn generate_room_members_endpoint(
        server: &mut ServerGuard,
        base_url: &str,
        room: &str,
        full_username: &str,
    ) -> Mock {
        let url = crate::config::build_room_members_url(base_url, room);
        let stripped_url = url
            .strip_prefix(base_url)
            .expect("Base URL missing from built url");

        let response_body = format!(
            r#"
{{
    "joined": {{
        "{}": {{
            "display_name": "Test",
            "avatar_url": ""
        }}
    }}
}}
"#,
            full_username
        );

        server
            .mock("GET", stripped_url)
            .with_status(200)
            .with_body(response_body.as_str())
            .create()
    }

    fn generate_join_room_endpoint(server: &mut ServerGuard, base_url: &str, room: &str) -> Mock {
        let url = crate::config::build_join_room_url(base_url, room);
        let stripped_url = url
            .strip_prefix(base_url)
            .expect("Base URL missing from built url");

        let response_body = format!(
            r#"
{{
}}
"#,
        );

        server
            .mock("POST", stripped_url)
            .with_status(200)
            .with_body(response_body.as_str())
            .create()
    }

    fn generate_send_message_endpoint(
        server: &mut ServerGuard,
        base_url: &str,
        full_username: &str,
    ) -> Mock {
        let url = crate::config::build_send_message_url(base_url, full_username);
        let stripped_url = url
            .strip_prefix(base_url)
            .expect("Base URL missing from built url");

        let response_body = format!(
            r#"
{{
}}
"#,
        );

        server
            .mock("POST", stripped_url)
            .with_status(200)
            .with_body(response_body.as_str())
            .create()
    }
}
