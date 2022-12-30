// Asks user whether to use '.env' file. Deletes the '.env' file accordingly.
let use_dotenv = variable::prompt("Do you want to add .env file with default values?", true);
if !use_dotenv {
    file::delete(".env");
}

// Sets a variable called 'use_dotenv' for further use.
variable::set("use_dotenv", use_dotenv);