// table "users" {
//   schema = schema.main

//   column "id" {
//     type = int
//     auto_increment = true
//   }

//   column "name" {
//     null = true
//     type = varchar(100)
//   }

//   primary_key {
//     columns = [column.id]
//   }
// }

table "sets" {
    schema = schema.main

    column "id" {
        type = character(10)
    }

    column "name" {
        type = varchar(100)
    }

    primary_key {
        columns = [column.id]
    }
}

table "terms" {
    schema = schema.main

    column "id" {
        type = integer
        auto_increment = true
    }

    column "term" {
        type = text
    }
    
    column "definition" {
        type = text
    }

    column "set_id" {
        type = character(10)
    }

    foreign_key "set" {
        columns = [column.set_id]
        ref_columns = [table.sets.column.id]
    }

    primary_key {
        columns = [column.id]
    }
}

schema "main" {
}
