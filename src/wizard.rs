use dialoguer::{Input, Select, Confirm};
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub fn run_wizard() {
    println!("Welcome to Systemd Wizard! Let's create a systemd service file.");

    let service_name: String = Input::new()
        .with_prompt("What is the name of your service? (Example: my-app)")
        .interact_text()
        .unwrap();

    let exec_start: String = Input::new()
        .with_prompt("What is the command to start your application? (Example: /usr/bin/my-app)")
        .interact_text()
        .unwrap();

    let user: String = Input::new()
        .with_prompt("Which user should run this service? (Example: root or ubuntu)")
        .default("root".to_string())
        .interact_text()
        .unwrap();

    let restart_policy = Select::new()
        .with_prompt("What should happen if your service crashes?")
        .items(&["always (recommended)", "on-failure", "never"])
        .default(0)
        .interact()
        .unwrap();

    let restart_value = match restart_policy {
        0 => "always",
        1 => "on-failure",
        _ => "never",
    };

    let working_directory: String = Input::new()
        .with_prompt("Where should the service run? (Example: /home/user/)")
        .default("/".to_string())
        .interact_text()
        .unwrap();

    let service_content = format!(
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

    let filename = format!("/etc/systemd/system/{}.service", service_name);

    if Confirm::new()
        .with_prompt(format!("Save service file to {}?", filename))
        .default(true)
        .interact()
        .unwrap()
    {
        let mut file = File::create(&filename).expect("Failed to create systemd service file");
        file.write_all(service_content.as_bytes()).expect("Failed to write service file");

        println!("Service file created successfully!");

        if Confirm::new()
            .with_prompt("Would you like to enable and start the service now?")
            .default(true)
            .interact()
            .unwrap()
        {
            Command::new("systemctl")
                .arg("daemon-reload")
                .output()
                .expect("Failed to reload systemd");

            Command::new("systemctl")
                .arg("enable")
                .arg(&service_name)
                .output()
                .expect("Failed to enable service");

            Command::new("systemctl")
                .arg("start")
                .arg(&service_name)
                .output()
                .expect("Failed to start service");

            println!("Service has been enabled and started.");
        }
    } else {
        println!("Operation cancelled. No file was created.");
    }
}
