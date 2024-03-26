use crate::functions::list_functions::ListFunctions;
use crate::random::Random;
use crate::variables::bloc_data::BlocData;
use crate::variables::list_structs::ListStructs;
use crate::variables::var_type;
use crate::instructions::type_instruction::generate_type_instruction;
use crate::variables::variable::Variable;

pub fn generate_variable_declaration(random: &mut Random, bloc_variables: &mut BlocData, list_functions: &ListFunctions, list_structs: &ListStructs) -> String {

    let chosen_type = var_type::random_type(random, list_structs);

    let instr_string = generate_type_instruction(random, bloc_variables, list_functions, list_structs, &chosen_type);

    let new_variable = Variable::new(bloc_variables.next_variable_name(), random.gen_bool(), &chosen_type);
    let ret = format!("{} = {};\n", new_variable.initialize(), instr_string);
    bloc_variables.add_variable(new_variable);

    ret
}

pub fn generate_lambda_declaration(random: &mut Random, bloc_variables: &mut BlocData, list_functions: &ListFunctions, list_structs: &ListStructs) -> String {

    let chosen_type = var_type::random_type(random, list_structs);

    let new_lambda = bloc_variables.create_lambda(random, list_structs, &chosen_type);
    let ret = new_lambda.initialize(random, list_functions, list_structs);

    bloc_variables.add_lambda(new_lambda);

    ret
}