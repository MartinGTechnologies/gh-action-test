use std::env;
use std::fs;
use std::path::{PathBuf};
use color_eyre::eyre::Result;
use tera::{Tera, Context};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: <tenant_name> <base_url> <bundleId>");
        return Ok(());
    }

    let tenant_name = &args[1];
    let base_url = format!("https://{}", &args[2]);
    let bundle_id = &args[3];

    // Adjust the paths to be relative to the project root directory
    let project_root = PathBuf::from(".."); // This assumes the binary is run from rust-tenantizer
    //TODO: binary should be able to be run from any place, should this matter?
    let parent_folder = project_root.join("tenants");
    let template_folder = project_root.join("templates");

    let folders = vec![
        parent_folder.join(tenant_name),
        parent_folder.join(format!("{}-TEST", tenant_name)),
    ];

    let file_templates = vec![
        ("environment.tenant.ts", "environment.tenant.ts"),
        ("apple.appid.json", "apple.appid.json"),
        ("datadog.json", "datadog.json"),
    ];

    // Create an instance of Tera
    let tera = Tera::new(&format!("{}/**/*", template_folder.to_string_lossy()))
        .expect("Failed to initialize Tera");

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
            context.insert("bundle_id", bundle_id);
            context.insert("app_id_suffix", &app_id_suffix);

            let content = tera
                .render(template_name, &context)
                .expect("Failed to render template");
            fs::write(&file_path, content).expect("Failed to write file");
        }
    }
    Ok(())
}