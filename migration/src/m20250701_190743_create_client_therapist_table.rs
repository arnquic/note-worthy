use super::{
    m20250630_000001_create_therapist_table::Therapist,
    m20250701_182901_create_client_table::Client,
};
use sea_orm_migration::{prelude::*, schema::*, sea_orm::Statement};

#[derive(DeriveIden)]
enum ClientTherapist {
    Table,
    Id,
    ClientId,
    TherapistId,
    CreatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ClientTherapist::Table)
                    .if_not_exists()
                    .col(pk_auto(ClientTherapist::Id))
                    .col(uuid(ClientTherapist::ClientId))
                    .col(uuid(ClientTherapist::TherapistId))
                    .col(
                        timestamp_with_time_zone(ClientTherapist::CreatedAt)
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("client_therapist_client_id")
                            .from(ClientTherapist::Table, ClientTherapist::ClientId)
                            .to(Client::Table, Client::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("client_therapist_therapist_id")
                            .from(ClientTherapist::Table, ClientTherapist::TherapistId)
                            .to(Therapist::Table, Therapist::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ClientTherapist::Table).to_owned())
            .await?;

        // Check if function is still referenced by any triggers
        let result = manager
            .get_connection()
            .query_one(Statement::from_string(
                manager.get_database_backend(),
                "SELECT COUNT(*) as count FROM information_schema.triggers
             WHERE event_object_schema = current_schema()
             AND action_statement LIKE '%update_updated_at_column%'"
                    .to_string(),
            ))
            .await?;

        if let Some(row) = result {
            let count: i64 = row.try_get("", "count")?;
            if count == 0 {
                // Safe to drop the function
                manager
                    .get_connection()
                    .execute_unprepared("DROP FUNCTION IF EXISTS update_updated_at_column()")
                    .await?;
            }
        }

        Ok(())
    }
}
