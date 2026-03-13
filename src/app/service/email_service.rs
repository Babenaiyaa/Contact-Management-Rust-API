use crate::app::dto::email::EmailDto;
use crate::app::repository::email_repository::EmailRepository;

pub struct EmailService;

impl EmailService {

    pub fn create_email(
        repo: &mut EmailRepository,
        person_id: u64,
        email: String,
    ) -> EmailDto {

        repo.create(person_id, email)
    }

    pub fn list_emails(
        repo: &EmailRepository,
        person_id: u64,
    ) -> Vec<EmailDto> {

        repo.find_by_person(person_id)
    }

    pub fn delete_email(
        repo: &mut EmailRepository,
        id: u64,
    ) -> bool {

        repo.delete(id)
    }
}