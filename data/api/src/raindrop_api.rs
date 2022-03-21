pub trait RaindropApi {
    fn fetch_today_topic_list();
}

struct RaindropApiInternal {}

impl RaindropApi for RaindropApiInternal {
    fn fetch_today_topic_list() {}
}