use super::{Field, FieldAttribute, Model};

struct PasswordsModel { }

impl Model<3> for PasswordsModel {
    const TABLE_NAME: &'static str = "passwords";
    const TABLE_COLUMNS: [Field; 3] = 
        [Field::new("id", &[FieldAttribute::Integer, FieldAttribute::PrimaryKey, FieldAttribute::Autoincrement]),
         Field::new("name", &[FieldAttribute::Text, FieldAttribute::NotNull]),
         Field::new("password", &[FieldAttribute::Text, FieldAttribute::NotNull])
        ];
}