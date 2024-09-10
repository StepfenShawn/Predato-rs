use crate::arrow_storage::arrow::ToArrow;

use super::dtypes::Dtype;

#[derive(Clone)]
pub struct Field<'a> {
    name: &'a str,
    data_type: Dtype,
    nullable: bool,
}

impl Field<'_> {
    fn to_arrow(&self) -> arrow::datatypes::Field {
        return arrow::datatypes::Field::new(
            &self.name,
            self.data_type.to_arrow_type(),
            self.nullable,
        );
    }
}

pub struct Schema<'a> {
    fields: Vec<Field<'a>>,
}

impl Schema<'_> {
    fn to_arrow(&self) -> arrow::datatypes::Schema {
        return arrow::datatypes::Schema::new(self.fields.iter().map(|x| x.to_arrow()).collect());
    }

    fn project(&self, indices: Vec<usize>) -> Schema {
        return Schema {
            fields: indices
                .iter()
                .clone()
                .map(|i| self.fields[*i].clone())
                .collect(),
        };
    }

    fn select(&self, names: Vec<String>) -> Schema {
        let mut selected = Vec::new();

        for name in names {
            let matched_fields: Vec<&Field> = self
                .fields
                .iter()
                .filter(|&field| field.name == name.as_str())
                .collect();

            if matched_fields.len() == 1 {
                selected.push(matched_fields[0].clone());
            } else {
                panic! {"IllegalArgumentException"};
            }
        }

        Schema { fields: selected }
    }
}
