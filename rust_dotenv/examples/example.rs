use rust_dotenv::dotenv::DotEnv;

fn main() {
    // Initialize DotEnv with a specific environment (e.g., "development")
    // This loads environment variables from `.env.development` or falls back to `.env.development.local` if it exists
    let dotenv: DotEnv = DotEnv::new("development");
    println!("Loaded variables for 'development': {:?}", dotenv.all_vars());

    // Access a specific variable by key (case-insensitive) and print its value if it exists
    if let Some(database_url) = dotenv.get_var("DATABASE_URL".to_string()) {
        println!("DATABASE_URL = {}", database_url);
    } else {
        println!("DATABASE_URL is not set.");
    }

    // Check if a specific variable exists in the environment configuration
    if dotenv.has_var("SECRET_KEY".to_string()) {
        println!("SECRET_KEY is set.");
    } else {
        println!("SECRET_KEY is not set.");
    }

    // Initialize DotEnv with another environment (e.g., "test") to demonstrate adding variables dynamically
    let mut dotenv_test: DotEnv = DotEnv::new("test");
    println!("Loaded variables for 'test': {:?}", dotenv_test.all_vars());

    // Add a new variable to the environment configuration without overwriting any existing ones
    // If "NEW_VARIABLE" does not exist, it will be added with the specified value; otherwise, it will be ignored
    let added = dotenv_test.set_var("NEW_VARIABLE".to_string(), "value".to_string());
    if added {
        println!("NEW_VARIABLE added successfully.");
    } else {
        println!("NEW_VARIABLE already exists and was not overwritten.");
    }

    // Verify that "NEW_VARIABLE" was successfully added by retrieving its value and printing it
    if let Some(new_variable) = dotenv_test.get_var("NEW_VARIABLE".to_string()) {
        println!("NEW_VARIABLE = {}", new_variable);
    } else {
        println!("NEW_VARIABLE is not set.");
    }
}