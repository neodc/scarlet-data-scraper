use mysql::{Pool, Value};
use scarlet_data::ScarletData;

pub struct Database {
    pool: Pool,
}

impl Database {
    pub fn new(url: &str) -> Self {
        Database {
            pool: Pool::new(url).expect("Could not connect to the database"),
        }
    }

    pub fn add_scarlet_data(&self, scarlet_data: &ScarletData) {
        let period_id = self.get_period_id(scarlet_data);

        self.pool.prep_exec("INSERT INTO data (period_id, transfert_volume, max_volume, days_left) VALUE (?, ?, ?, ?)", (period_id, scarlet_data.transfert_volume(), scarlet_data.max_volume(), scarlet_data.days_left())).unwrap();
    }

    fn get_period_id(&self, scarlet_data: &ScarletData) -> u64 {
        if let Some(new_days_left) = scarlet_data.days_left() {
            let tmp = self.pool.first_exec("SELECT period_id, days_left
FROM data
ORDER BY id DESC
LIMIT 1", ()).unwrap();

            if let Some(data) = tmp {
                if let (&Value::Int(ref days_left), &Value::Int(ref period_id)) = (&data["days_left"], &data["period_id"]) {
                    if *days_left as u32 >= new_days_left {
                        return *period_id as u64
                    }
                }
            }
        }

        self.pool.prep_exec("INSERT INTO period () VALUE ()", ()).unwrap().last_insert_id()
    }

    pub fn get_consomation_since_last_day(&self, scarlet_data: &ScarletData) -> f64 {
        let transfert_volume_last_day: f64 =
            if let Some(Ok(mut result)) = self.pool.prep_exec("SELECT MAX(transfert_volume) FROM data WHERE id = (SELECT MAX(id) FROM data WHERE creation_time <= NOW() - INTERVAL 1 DAY)", ())
                .unwrap()
                .next() {
            result.take(0).unwrap()
        } else {
            0f64
        };

        scarlet_data.transfert_volume() - transfert_volume_last_day
    }
}