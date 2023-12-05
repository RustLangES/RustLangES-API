use sqlx::PgPool;

use crate::{errors::Errors, models::dtos::visitwithdomain::VisitWithDomain};

pub struct TrackService;

impl TrackService {
    /// # Errors
    /// `Errors::DatabaseError`
    pub async fn count_a_visit(pool: &PgPool, reference: String) -> Result<(), Errors> {
        Self::insert_visit_if_exists(pool, &reference).await?;
        Self::insert_visit_and_domain(pool, &reference).await?;
        Ok(())
    }

    /// # Errors
    /// `Errors::DatabaseError`
    async fn insert_visit_if_exists(pool: &PgPool, reference: &String) -> Result<(), Errors> {
        let result = sqlx::query!("SELECT id FROM domains WHERE domain = $1", reference)
            .fetch_one(&pool.clone())
            .await?;

        sqlx::query!("INSERT INTO visits (domain_id) VALUES ($1)", result.id)
            .execute(&pool.clone())
            .await?;

        Ok(())
    }

    async fn insert_visit_and_domain(pool: &PgPool, reference: &String) -> Result<(), Errors> {
        let result = sqlx::query!(
            "INSERT INTO domains (domain) VALUES ($1) RETURNING id",
            reference
        )
        .fetch_one(&pool.clone())
        .await?;

        sqlx::query!("INSERT INTO visits (domain_id) VALUES ($1)", result.id)
            .execute(&pool.clone())
            .await?;

        Ok(())
    }

    /// # Errors
    /// `Errors::DatabaseError`
    pub async fn get_visits_by_domain(db_pool: &PgPool) -> Result<Vec<VisitWithDomain>, Errors> {
        let result = sqlx::query_as!(
            VisitWithDomain,
            "SELECT domain, COUNT(visits.id) as visits
            FROM domains
            INNER JOIN visits ON domains.id = visits.domain_id
            GROUP BY domain"
        )
        .fetch_all(db_pool)
        .await?;

        Ok(result)
    }
}
