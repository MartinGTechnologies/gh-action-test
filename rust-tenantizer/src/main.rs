use std::env;
use std::fs;
use std::path::{Path};
use tera::{Tera, Context};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: <tenant_name> <base_url> <bundleId>");
        return;
    }

    let tenant_name = &args[1];
    let base_url = format!("https://{}", &args[2]);
    let bundle_id = &args[3];

    let parent_folder = Path::new("tenants");
    let folders = vec![
        parent_folder.join(tenant_name),
        parent_folder.join(format!("{}-TEST", tenant_name)),
    ];

    let file_templates = vec![
        ("environment.tenant.ts", "environment.tenant.ts"),
        ("apple.appid.json", "apple.appid.json"),
        ("datadog.json", "datadog.json"),
    ];


    //create an instance of Tera
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");

    for folder in folders {
        if folder.exists() {
            println!("Folder {} already exists. Skipping creation.", folder.display());
            continue;
        }

        fs::create_dir_all(&folder).expect("Failed to create directory");

        let site_name = folder.file_name().unwrap().to_str().unwrap();

        for (template_name, file_name) in &file_templates {
            let file_path = folder.join(file_name);

            let app_id_suffix = if site_name.ends_with("-TEST") {
                format!("{}.test", tenant_name.to_lowercase())
            } else {
                tenant_name.to_lowercase()
            };

            let mut context = Context::new();
            context.insert("base_url", &base_url);
            context.insert("site_name", site_name);
            context.insert("bundle_id", bundle_id)
            context.insert("app_id_suffix", &app_id_suffix);

            let content = tera.render(template_name, &context).expect("Failed to render template");
            fs::write(&file_path, content).expect("Failed to write file");
        }
    }
}