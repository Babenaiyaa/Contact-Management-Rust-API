use crate::app::dto::person::PersonDto;
use crate::app::repository::person_repository::PersonRepository;

pub struct PersonService;

impl PersonService {

    pub fn create_person(repo: &mut PersonRepository, name: String) -> PersonDto {
        repo.create(name)
    }

    pub fn list_persons(repo: &PersonRepository) -> Vec<PersonDto> {
        repo.list()
    }

    pub fn get_person(repo: &PersonRepository, id: u64) -> Option<PersonDto> {
        repo.find(id)
    }

    pub fn update_person(repo: &mut PersonRepository, id: u64, name: String) -> Option<PersonDto> {
        repo.update(id, name)
    }

    pub fn delete_person(repo: &mut PersonRepository, id: u64) -> bool {
        repo.delete(id)
    }
}