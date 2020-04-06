use crate::reader::Reader;
use crate::tables::TypeDef;
use crate::type_limits::TypeLimits;
use crate::type_tree::TypeTree;
use crate::types::Type;

use std::collections::*;

#[derive(Default, Debug)]
pub struct TypeStage(pub BTreeMap<TypeDef, Type>);

impl TypeStage {
    pub fn from_limits(reader: &Reader, limits: &TypeLimits) -> TypeStage {
        let mut stage: TypeStage = Default::default();

        for namespace in &limits.0 {
            for def in reader.namespace_types(&namespace) {
                stage.insert(reader, *def);
            }
        }

        stage
    }

    fn insert(&mut self, reader: &Reader, def: TypeDef) {
        if !self.0.contains_key(&def) {
            let info = def.into_type(reader);
            let depends = info.dependencies();
            self.0.insert(def, info);
            for def in depends {
                self.insert(reader, def);
            }
        }
    }

    pub fn into_tree(self) -> TypeTree {
        let mut tree: TypeTree = Default::default();
        self.0
            .into_iter()
            .for_each(|(_, t)| tree.insert(t.name().namespace.clone(), t));
        tree
    }
}