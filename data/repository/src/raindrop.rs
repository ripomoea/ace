use ace_api::raindrop::RaindropApi;
use ace_kernel::repository::raindrop::RaindropRepository;

struct RaindropDataRepository {
    api: dyn RaindropApi,
}

impl RaindropRepository for RaindropDataRepository {
    fn fetch_today_topic_list() {}
}
