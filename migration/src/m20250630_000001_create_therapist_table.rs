use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
    sea_orm::{EnumIter, Iterable, Statement},
};

#[derive(DeriveIden)]
pub enum Therapist {
    Table,
    Id,
    DateOfBirth,
    Email,
    FirstName,
    LastName,
    LicenseNumber,
    Password, // as a salted hash
    Phone,
    Role,            // Role array
    Specializations, // string array
    TherapistStatus,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden, EnumIter)]
pub enum TherapistStatus {
    Active,
    Inactive,
}

#[derive(Iden, EnumIter)]
pub enum Role {
    Admin,
    Contractor,
    FullTime,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create enum types first
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("role"))
                    .values(Role::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("therapist_status"))
                    .values(TherapistStatus::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Therapist::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Therapist::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT gen_random_uuid()"),
                    )
                    .col(date(Therapist::DateOfBirth))
                    .col(string(Therapist::Email))
                    .col(string(Therapist::FirstName))
                    .col(string(Therapist::LastName))
                    .col(string(Therapist::LicenseNumber))
                    .col(string(Therapist::Password))
                    .col(string(Therapist::Phone))
                    .col(enumeration(
                        Therapist::Role,
                        Alias::new("role"),
                        Role::iter(),
                    ))
                    .col(ColumnDef::new(Therapist::Specializations).array(ColumnType::Text))
                    .col(enumeration(
                        Therapist::TherapistStatus,
                        Alias::new("therapist_status"),
                        TherapistStatus::iter(),
                    ))
                    .col(
                        timestamp_with_time_zone(Therapist::CreatedAt)
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        timestamp_with_time_zone(Therapist::UpdatedAt)
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
                CREATE TRIGGER update_therapist_table_updated_at
                    BEFORE UPDATE ON therapist
                    FOR EACH ROW
                    EXECUTE FUNCTION update_updated_at_column();
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Therapist::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(Alias::new("therapist_status")).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(Alias::new("role")).to_owned())
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
