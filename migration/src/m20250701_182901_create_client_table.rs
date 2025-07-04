use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
    sea_orm::{EnumIter, Iterable, Statement},
};

#[derive(DeriveIden)]
pub enum Client {
    Table,
    Id,
    ClientStatus,
    DateOfBirth,
    Email,
    FirstName,
    LastName,
    Password, // as a salted hash
    Phone,
    PreferredName,
    Pronouns,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden, EnumIter)]
pub enum ClientStatus {
    InTherapy,
    OnHold,
    Completed,
    Canceled,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS pgcrypto;")
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("client_status"))
                    .values(ClientStatus::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Client::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Client::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(enumeration(
                        Client::ClientStatus,
                        Alias::new("client_status"),
                        ClientStatus::iter(),
                    ))
                    .col(date(Client::DateOfBirth))
                    .col(string(Client::Email))
                    .col(string(Client::FirstName))
                    .col(string(Client::LastName))
                    .col(string(Client::Password))
                    .col(string(Client::Phone))
                    .col(ColumnDef::new(Client::PreferredName).string().null())
                    .col(string(Client::Pronouns))
                    .col(
                        timestamp_with_time_zone(Client::CreatedAt)
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        timestamp_with_time_zone(Client::UpdatedAt)
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .to_owned(),
            )
            .await?;

        // Create trigger function for auto-updating updated_at
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE OR REPLACE FUNCTION update_updated_at_column()
                RETURNS TRIGGER AS $$
                BEGIN
                    NEW.updated_at = CURRENT_TIMESTAMP;
                    RETURN NEW;
                END;
                $$ language 'plpgsql';
                "#,
            )
            .await?;

        // Create trigger for the table
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE TRIGGER update_client_table_updated_at
                    BEFORE UPDATE ON client
                    FOR EACH ROW
                    EXECUTE FUNCTION update_updated_at_column();
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Client::Table).to_owned())
            .await?;

        // Drop enum types
        manager
            .drop_type(Type::drop().name(Alias::new("client_status")).to_owned())
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
