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
        let tmp = self.pool.first_exec("SELECT period_id, days_left
FROM data
ORDER BY id DESC
LIMIT 1", ()).unwrap();

        if let Some(data) = tmp {
            if let (&Value::Int(ref days_left), &Value::Int(ref period_id)) = (&data["days_left"], &data["period_id"]) {
                if *days_left as u32 >= scarlet_data.days_left() {
                    return *period_id as u64
                }
            }
        }

        self.pool.prep_exec("INSERT INTO period () VALUE ()", ()).unwrap().last_insert_id()
    }
}