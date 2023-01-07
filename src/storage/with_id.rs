use diesel::prelude::*;
use uuid::Uuid;

pub trait WithId {
    type Id: diesel::Expression<SqlType = diesel::sql_types::Uuid>;

    fn id() -> Self::Id;

    fn with_id(&self) -> (&Self, diesel::dsl::Eq<Self::Id, Uuid>) {
        (self, id_for(Self::id()))
    }
}

fn id_for<Id>(id: Id) -> diesel::dsl::Eq<Id, Uuid>
where
    Id: diesel::Expression<SqlType = diesel::sql_types::Uuid>,
{
    id.eq(Uuid::new_v4())
}
