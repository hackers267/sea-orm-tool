use sea_orm::{ConnectionTrait, DbErr, Paginator, SelectorTrait};
use serde::{Deserialize, Serialize};

/// 分页查询数据框架
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PageData<T> {
    pub list: Vec<T>,
    pub page_num: u64,
    pub total: u64,
    pub total_page: u64,
    pub page_size: u64,
}

/// 转换paginator为PageData结构
pub async fn to_page_data<'db, C, S, T>(
    mut paginator: Paginator<'db, C, S>,
    current: u64,
    page_size: u64,
) -> Result<PageData<T>, DbErr>
where
    C: ConnectionTrait,
    S: SelectorTrait<Item = T> + 'db,
{
    let range = 1..current;
    range.for_each(|_| {
        paginator.next();
    });
    let total_page = paginator.num_pages().await?;
    let total = paginator.num_items().await?;
    let list = paginator.fetch().await?;
    let page_num = paginator.cur_page() + 1;
    let data = PageData {
        list,
        page_num,
        total,
        total_page,
        page_size,
    };
    Ok(data)
}
