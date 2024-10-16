use std::collections::HashSet;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique_emails = HashSet::new();

        for email in emails {
            let parts: Vec<&str> = email.split('@').collect();
            let mut local = parts[0].split('+').next().unwrap().replace('.', "");
            let domain = parts[1];

            let unique_email = format!("{}@{}", local, domain);
            unique_emails.insert(unique_email);
        }
        unique_emails.len() as i32
    }
}
