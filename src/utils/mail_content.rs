pub fn subject() -> String {
    "Sending email with Rust".to_string()
}

pub fn mail_content() -> String {
    r#"<!DOCTYPE html>
 <html lang="en">
 <head>
     <meta charset="UTF-8">
     <meta name="viewport" content="width=device-width, initial-scale=1.0">
     <title>Hello from Lettre!</title>
 </head>
 <body>
     <div style="display: flex; flex-direction: column; align-items: center;">
         <h2 style="font-family: Arial, Helvetica, sans-serif;">Hello from Lettre!</h2>
         <h4 style="font-family: Arial, Helvetica, sans-serif;">A mailer library for Rust</h4>
     </div>
 </body>
 </html>

 "#
    .to_string()
}
