use std::collections::BTreeMap;
use crate::availability::Availability;
use crate::ast::config::Config;
use crate::ast::config_declaration::ConfigDeclaration;
use crate::ast::data_set::DataSet;
use crate::ast::decorator_declaration::DecoratorDeclaration;
use crate::ast::handler::{HandlerDeclaration, HandlerGroupDeclaration};
use crate::ast::handler_template_declaration::HandlerTemplateDeclaration;
use crate::ast::interface::InterfaceDeclaration;
use crate::ast::middleware::MiddlewareDeclaration;
use crate::ast::model::Model;
use crate::ast::namespace::Namespace;
use crate::ast::node::Node;
use crate::ast::pipeline_item_declaration::PipelineItemDeclaration;
use crate::ast::r#enum::Enum;
use crate::ast::source::Source;
use crate::ast::struct_declaration::StructDeclaration;
use crate::traits::identifiable::Identifiable;

#[derive(Debug)]
pub struct Schema {
    pub sources: BTreeMap<usize, Source>,
    pub references: SchemaReferences,
}

impl Schema {

    pub fn main_source(&self) -> &Source {
        self.source(self.references.main_source.unwrap()).unwrap()
    }

    pub fn source(&self, id: usize) -> Option<&Source> {
        self.sources.get(&id)
    }

    pub fn source_at_path(&self, path: &str) -> Option<&Source> {
        self.sources().iter().find_map(|s| if s.file_path.as_str() == path { Some(*s) } else { None })
    }

    pub fn builtin_sources(&self) -> Vec<&Source> {
        self.references.builtin_sources.iter().map(|id| self.source(*id).unwrap()).collect()
    }

    pub fn user_sources(&self) -> Vec<&Source> {
        self.references.user_sources.iter().map(|id| self.source(*id).unwrap()).collect()
    }

    pub fn std_source(&self) -> &Source {
        if self.builtin_sources().is_empty() {
            self.sources().first().unwrap()
        } else {
            self.builtin_sources().first().unwrap()
        }
    }

    pub fn find_config_declaration_by_name(&self, name: &str, availability: Availability) -> Option<&ConfigDeclaration> {
        for config_declaration in self.config_declarations() {
            if config_declaration.identifier().name() == name && config_declaration.define_availability.contains(availability) {
                return Some(config_declaration)
            }
        }
        None
    }

    pub fn find_top_by_path(&self, path: &Vec<usize>) -> Option<&Node> {
        if path.len() < 2 {
            return None;
        }
        if let Some(source) = self.source(*path.get(0).unwrap()) {
            source.find_top_by_path(path)
        } else {
            None
        }
    }

    // Public APIs

    pub fn sources(&self) -> Vec<&Source> {
        self.sources.values().collect()
    }

    pub fn configs(&self) -> Vec<&Config> {
        self.references.config_declarations.iter().map(|path| self.find_top_by_path(path).unwrap().as_config().unwrap()).collect()
    }

    pub fn server(&self) -> Option<&Config> {
        self.references.server.as_ref().map(|path| self.find_top_by_path(path).unwrap().as_config().unwrap())
    }

    pub fn debug(&self) -> Option<&Config> {
        self.references.debug.as_ref().map(|path| self.find_top_by_path(path).unwrap().as_config().unwrap())
    }

    pub fn test(&self) -> Option<&Config> {
        self.references.test.as_ref().map(|path| self.find_top_by_path(path).unwrap().as_config().unwrap())
    }

    pub fn connectors(&self) -> Vec<&Config> {
        self.references.connectors.iter().map(|path| self.find_top_by_path(path).unwrap().as_config().unwrap()).collect()
    }

    pub fn entities(&self) -> Vec<&Config> {
        self.references.entities.iter().map(|path| self.find_top_by_path(path).unwrap().as_config().unwrap()).collect()
    }

    pub fn clients(&self) -> Vec<&Config> {
        self.references.clients.iter().map(|path| self.find_top_by_path(path).unwrap().as_config().unwrap()).collect()
    }

    pub fn enums(&self) -> Vec<&Enum> {
        self.references.enums.iter().map(|path| self.find_top_by_path(path).unwrap().as_enum().unwrap()).collect()
    }

    pub fn models(&self) -> Vec<&Model> {
        self.references.models.iter().map(|path| self.find_top_by_path(path).unwrap().as_model().unwrap()).collect()
    }

    pub fn data_sets(&self) -> Vec<&DataSet> {
        self.references.data_sets.iter().map(|path| self.find_top_by_path(path).unwrap().as_data_set().unwrap()).collect()
    }

    pub fn interfaces(&self) -> Vec<&InterfaceDeclaration> {
        self.references.interfaces.iter().map(|path| self.find_top_by_path(path).unwrap().as_interface_declaration().unwrap()).collect()
    }

    pub fn namespaces(&self) -> Vec<&Namespace> {
        self.references.namespaces.iter().map(|path| self.find_top_by_path(path).unwrap().as_namespace().unwrap()).collect()
    }

    pub fn config_declarations(&self) -> Vec<&ConfigDeclaration> {
        self.references.config_declarations.iter().map(|path| self.find_top_by_path(path).unwrap().as_config_declaration().unwrap()).collect()
    }

    pub fn decorator_declarations(&self) -> Vec<&DecoratorDeclaration> {
        self.references.decorator_declarations.iter().map(|path| self.find_top_by_path(path).unwrap().as_decorator_declaration().unwrap()).collect()
    }

    pub fn pipeline_item_declarations(&self) -> Vec<&PipelineItemDeclaration> {
        self.references.pipeline_item_declarations.iter().map(|path| self.find_top_by_path(path).unwrap().as_pipeline_item_declaration().unwrap()).collect()
    }

    pub fn middleware_declarations(&self) -> Vec<&MiddlewareDeclaration> {
        self.references.middlewares.iter().map(|path| self.find_top_by_path(path).unwrap().as_middleware_declaration().unwrap()).collect()
    }

    pub fn handler_declarations(&self) -> Vec<&HandlerDeclaration> {
        self.references.handlers.iter().map(|path| self.find_top_by_path(path).unwrap().as_handler_declaration().unwrap()).collect()
    }

    pub fn handler_template_declarations(&self) -> Vec<&HandlerTemplateDeclaration> {
        self.references.handler_templates.iter().map(|path| self.find_top_by_path(path).unwrap().as_handler_template_declaration().unwrap()).collect()
    }

    pub fn handler_group_declarations(&self) -> Vec<&HandlerGroupDeclaration> {
        self.references.handler_groups.iter().map(|path| self.find_top_by_path(path).unwrap().as_handler_group_declaration().unwrap()).collect()
    }

    pub fn struct_declarations(&self) -> Vec<&StructDeclaration> {
        self.references.struct_declarations.iter().map(|path| self.find_top_by_path(path).unwrap().as_struct_declaration().unwrap()).collect()
    }
}

#[derive(Debug, Clone)]
pub struct SchemaReferences {
    pub builtin_sources: Vec<usize>,
    pub user_sources: Vec<usize>,
    pub main_source: Option<usize>,
    pub configs: Vec<Vec<usize>>,
    pub server: Option<Vec<usize>>,
    pub debug: Option<Vec<usize>>,
    pub test: Option<Vec<usize>>,
    pub connectors: Vec<Vec<usize>>,
    pub entities: Vec<Vec<usize>>,
    pub clients: Vec<Vec<usize>>,
    pub enums: Vec<Vec<usize>>,
    pub models: Vec<Vec<usize>>,
    pub data_sets: Vec<Vec<usize>>,
    pub interfaces: Vec<Vec<usize>>,
    pub namespaces: Vec<Vec<usize>>,
    pub config_declarations: Vec<Vec<usize>>,
    pub decorator_declarations: Vec<Vec<usize>>,
    pub pipeline_item_declarations: Vec<Vec<usize>>,
    pub middlewares: Vec<Vec<usize>>,
    pub handlers: Vec<Vec<usize>>,
    pub handler_templates: Vec<Vec<usize>>,
    pub handler_groups: Vec<Vec<usize>>,
    pub struct_declarations: Vec<Vec<usize>>,
    pub use_middlewares_blocks: Vec<Vec<usize>>,
}

impl SchemaReferences {

    pub fn new() -> Self {
        Self {
            builtin_sources: vec![],
            user_sources: vec![],
            main_source: None,
            connectors: vec![],
            configs: vec![],
            server: None,
            entities: vec![],
            clients: vec![],
            enums: vec![],
            models: vec![],
            data_sets: vec![],
            debug: None,
            test: None,
            interfaces: vec![],
            namespaces: vec![],
            config_declarations: vec![],
            decorator_declarations: vec![],
            pipeline_item_declarations: vec![],
            middlewares: vec![],
            handlers: vec![],
            handler_templates: vec![],
            handler_groups: vec![],
            struct_declarations: vec![],
            use_middlewares_blocks: vec![],
        }
    }

    pub fn add_config(&mut self, config: &Config) {
        self.configs.push(config.path().clone());
        if config.keyword().is_client() {
            self.clients.push(config.path().clone());
        } else if config.keyword().is_connector() {
            self.connectors.push(config.path().clone());
        } else if config.keyword().is_server() {
            self.server = Some(config.path().clone());
        } else if config.keyword().is_entity() {
            self.entities.push(config.path().clone());
        } else if config.keyword().is_test() {
            self.test = Some(config.path().clone());
        } else if config.keyword().is_debug() {
            self.debug = Some(config.path().clone());
        }
    }
}