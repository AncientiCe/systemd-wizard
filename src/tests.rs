#[cfg(test)]
mod tests {
    #[test]
    fn test_service_content_format() {
        let service_name = "test-service";
        let exec_start = "/usr/bin/test-app";
        let working_directory = "/home/test";
        let user = "test-user";
        let restart_value = "always";

        let expected_content = format!(
            "[Unit]
Description={0} service
After=network.target

[Service]
ExecStart={1}
WorkingDirectory={2}
User={3}
Restart={4}

[Install]
WantedBy=multi-user.target",
            service_name, exec_start, working_directory, user, restart_value
        );

        assert!(expected_content.contains(service_name));
        assert!(expected_content.contains(exec_start));
        assert!(expected_content.contains(working_directory));
        assert!(expected_content.contains(user));
        assert!(expected_content.contains(restart_value));
    }
}
