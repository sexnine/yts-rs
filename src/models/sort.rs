use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SortBy {
    #[serde(rename = "sort_by")]
    pub sort_value: SortValue,
    pub order_by: Order,
}

impl SortBy {
    pub fn new(sort_value: SortValue, order_by: Order) -> Self {
        SortBy {
            sort_value,
            order_by,
        }
    }

    pub fn desc(sort_value: SortValue) -> Self {
        SortBy {
            sort_value,
            order_by: Order::Descending,
        }
    }

    pub fn asc(sort_value: SortValue) -> Self {
        SortBy {
            sort_value,
            order_by: Order::Ascending,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum SortValue {
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "rating")]
    Rating,
    #[serde(rename = "peers")]
    PeersCount,
    #[serde(rename = "seeds")]
    SeedsCount,
    #[serde(rename = "download_count")]
    DownloadCount,
    #[serde(rename = "like_count")]
    LikeCount,
    #[serde(rename = "date_added")]
    DateAdded,
}

#[derive(Debug, Clone, Serialize)]
pub enum Order {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}
