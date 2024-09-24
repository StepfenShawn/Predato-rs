use super::dtypes::Dtype;

#[derive(Clone)]
pub struct Field<'a> {
    pub name: &'a str,
    pub data_type: Dtype,
    pub nullable: bool,
}

pub struct Schema<'a> {
    pub fields: Vec<Field<'a>>,
}

impl Schema<'_> {
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
