use crate::variables::{bloc_variables::BlocVariables, var_type};
use crate::instructions::type_instruction::generate_type_instruction;

pub fn generate_operation_instruction(bloc_variables: &mut BlocVariables) -> String {
    let mut instruction: String = String::new();
    
    let chosen_type = var_type::random_basic_type();
    
    let variables_used = bloc_variables.get_variables_by_types([chosen_type.clone()].to_vec());

    match bloc_variables.get_random_variable([chosen_type.clone()].to_vec(), Some(true)) {
        Some(assigned_var) => instruction = format!("{}{} = ", instruction, assigned_var.name()),
        None => {
            let assigned_var = bloc_variables.new_variable([chosen_type.clone()].to_vec(), None);
            instruction = format!("{}{} = ", instruction, assigned_var.initialise());
        },
    }

    format!("{}{};\n", instruction, generate_type_instruction(&variables_used, chosen_type))

}