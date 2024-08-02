use arrow::datatypes::{DataType};

#[derive(Clone)]
struct Field {
    name: String,
    data_type: DataType,
    nullable: bool
}

impl Field {
    fn to_arrow(&self) -> arrow::datatypes::Field {
        return arrow::datatypes::Field::new(&self.name, self.data_type.clone(), self.nullable);
    }
}

struct Schema {
    fields: Vec<Field>
}

impl Schema {
    fn to_arrow(&self) -> arrow::datatypes::Schema {
        return arrow::datatypes::Schema::new(
            self.fields.iter().map(|x| x.to_arrow()).collect());
    }

    fn project(&self, indices: Vec<usize>) -> Schema {
        return Schema {
            fields: indices.iter().clone().map(|i| self.fields[*i].clone()).collect()
        }
    }

    fn select(&self, names: Vec<String>) -> Schema {
        let mut selected = Vec::new();

        for name in names {
            let matched_fields: Vec<&Field> = self.fields.iter().filter(|&field| field.name == *name).collect();       
            
            if matched_fields.len() == 1 {
                selected.push(matched_fields[0].clone()); 
            } else {
                panic!{"IllegalArgumentException"}; 
            }
        }

        Schema {fields: selected} 
    }
}