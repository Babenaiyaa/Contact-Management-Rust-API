use crate::app::dto::mobile::MobileDto;
use crate::app::repository::mobile_repository::MobileRepository;

pub struct MobileService;

impl MobileService {

    pub fn create_mobile(
        repo: &mut MobileRepository,
        person_id: u64,
        number: String,
    ) -> MobileDto {

        repo.create(person_id, number)
    }

    pub fn list_mobiles(
        repo: &MobileRepository,
        person_id: u64,
    ) -> Vec<MobileDto> {

        repo.find_by_person(person_id)
    }

    pub fn delete_mobile(
        repo: &mut MobileRepository,
        id: u64,
    ) -> bool {

        repo.delete(id)
    }
}