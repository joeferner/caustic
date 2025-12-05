use std::{collections::HashMap, rc::Rc};

use crate::parser::{
    CallArgument, CallArgumentWithPosition, ChildStatement, ChildStatementWithPosition, Expr,
    ModuleId, ModuleInstantiation, ModuleInstantiationWithPosition, SingleModuleInstantiation,
    SingleModuleInstantiationWithPosition, Statement, StatementWithPosition,
};

#[derive(Debug, Clone, Copy)]
pub enum Module {
    Cube,
}

#[derive(Debug)]
pub struct ModuleInstance {
    pub module: Module,
    pub arguments: Vec<ModuleArgument>,
}

#[derive(Debug)]
pub struct ModuleInstanceTree {
    pub instance: ModuleInstance,
    pub children: Vec<Rc<ModuleInstanceTree>>,
}

#[derive(Debug)]
pub enum ModuleArgument {
    Positional(ModuleArgumentValue),
}

#[derive(Debug)]
pub enum ModuleArgumentValue {
    Number(f64),
}

struct Interpreter {
    modules: HashMap<String, Module>,
    stack: Vec<Rc<ModuleInstanceTree>>,
    results: Vec<Rc<ModuleInstanceTree>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut modules = HashMap::new();
        modules.insert("cube".to_string(), Module::Cube);

        Self {
            modules,
            stack: vec![],
            results: vec![],
        }
    }

    fn interpret(mut self, statements: Vec<StatementWithPosition>) -> Vec<Rc<ModuleInstanceTree>> {
        for statement in statements {
            self.process_statement(statement);
        }
        self.results
    }

    fn process_statement(&mut self, statement: StatementWithPosition) {
        match statement.item {
            Statement::Empty => (),
            Statement::ModuleInstantiation {
                module_instantiation,
            } => self.process_module_instantiation(module_instantiation),
        }
    }

    fn process_module_instantiation(
        &mut self,
        module_instantiation: ModuleInstantiationWithPosition,
    ) {
        match module_instantiation.item {
            ModuleInstantiation::SingleModuleInstantiation {
                single_module_instantiation,
                child_statement,
            } => {
                self.process_single_module_instantiation(single_module_instantiation);
                self.process_child_statement(child_statement)
            }
        }
    }

    fn process_single_module_instantiation(
        &mut self,
        single_module_instantiation: SingleModuleInstantiationWithPosition,
    ) {
        match single_module_instantiation.item {
            SingleModuleInstantiation::Module {
                module_id,
                call_arguments,
            } => match module_id.item {
                ModuleId::For => todo!(),
                ModuleId::Identifier(identifier) => {
                    if let Some(module) = self.modules.get(&identifier) {
                        let instance = ModuleInstance {
                            module: *module,
                            arguments: self.process_call_arguments(call_arguments),
                        };
                        self.append_instance(instance);
                    } else {
                        todo!("handle unknown module");
                    }
                }
            },
        }
    }

    fn process_call_arguments(
        &self,
        call_arguments: Vec<CallArgumentWithPosition>,
    ) -> Vec<ModuleArgument> {
        let mut results: Vec<ModuleArgument> = vec![];

        for call_argument in call_arguments {
            match call_argument.item {
                CallArgument::Expr { expr } => match expr.item {
                    Expr::Number(number) => results.push(ModuleArgument::Positional(
                        ModuleArgumentValue::Number(number),
                    )),
                },
            }
        }

        results
    }

    fn process_child_statement(&mut self, child_statement: ChildStatementWithPosition) {
        match child_statement.item {
            ChildStatement::Empty => {
                self.stack.clear();
            }
        }
    }

    fn append_instance(&mut self, instance: ModuleInstance) {
        if self.stack.is_empty() {
            let tree = Rc::new(ModuleInstanceTree {
                instance,
                children: vec![],
            });
            self.results.push(tree.clone());
            self.stack.push(tree);
        } else {
            todo!();
        }
    }
}

pub fn openscad_interpret(statements: Vec<StatementWithPosition>) -> Vec<Rc<ModuleInstanceTree>> {
    let it = Interpreter::new();
    it.interpret(statements)
}
