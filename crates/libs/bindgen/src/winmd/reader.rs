use super::*;
use std::sync::atomic::{AtomicPtr, Ordering};

static READER: AtomicPtr<Reader> = AtomicPtr::new(std::ptr::null_mut());

pub fn reader() -> &'static Reader {
    let ptr = READER.load(Ordering::Relaxed);

    if ptr.is_null() {
        panic!();
    } else {
        unsafe { &*ptr }
    }
}

fn insert(types: &mut HashMap<&'static str, Vec<Type>>, name: &'static str, ty: Type) {
    types.entry(name).or_default().push(ty);
}

pub struct Reader(HashMap<&'static str, HashMap<&'static str, Vec<Type>>>);

impl std::ops::Deref for Reader {
    type Target = HashMap<&'static str, HashMap<&'static str, Vec<Type>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Reader {
    pub fn init(files: Vec<File>) {
        let reader = Box::leak(Box::new(Self(HashMap::new())));

        for file in files {
            let file = Box::leak(Box::new(file));
            let mut nested = HashMap::<TypeDef, Vec<TypeDef>>::new();

            for key in file.table::<NestedClass>() {
                let inner = key.inner();
                nested.entry(key.outer()).or_default().push(inner);
            }

            for def in file.table::<TypeDef>() {
                let type_name = def.type_name();

                if type_name.namespace().is_empty() {
                    // This skips the nested types as we've already retrieved them.
                    continue;
                }

                if Type::remap(type_name) != Remap::None {
                    continue;
                }

                let types = reader.0.entry(type_name.namespace()).or_default();
                let category = Category::new(def);

                if def.flags().contains(TypeAttributes::WindowsRuntime) {
                    let ty = match category {
                        Category::Attribute => continue,
                        Category::Class => Type::Class(Class { def }),
                        Category::Delegate => Type::Delegate(Delegate {
                            def,
                            generics: def.generics(),
                        }),
                        Category::Enum => Type::Enum(Enum { def }),
                        Category::Interface => Type::Interface(Interface {
                            def,
                            generics: def.generics(),
                            kind: InterfaceKind::None,
                        }),
                        Category::Struct => {
                            // Skip marker types representing API contracts.
                            if def.has_attribute("ApiContractAttribute") {
                                continue;
                            }

                            Type::Struct(Struct { def })
                        }
                    };

                    insert(types, type_name.1, ty);
                } else {
                    match category {
                        Category::Attribute => continue,
                        Category::Class => {
                            if type_name.1 == "Apis" {
                                for method in def.methods() {
                                    if let Some(map) = method.impl_map() {
                                        // Skip inline and ordinal functions.
                                        if map.scope().name() == "FORCEINLINE"
                                            || map.import_name().starts_with("#")
                                        {
                                            continue;
                                        }
                                    }

                                    let name = method.name();
                                    insert(
                                        types,
                                        name,
                                        Type::CppFn(CppFn {
                                            namespace: def.namespace(),
                                            method,
                                        }),
                                    );
                                }

                                for field in def.fields() {
                                    let name = field.name();
                                    insert(
                                        types,
                                        name,
                                        Type::CppConst(CppConst {
                                            namespace: def.namespace(),
                                            field,
                                        }),
                                    );
                                }
                            }
                        }
                        Category::Delegate => {
                            insert(types, type_name.1, Type::CppDelegate(CppDelegate { def }));
                        }
                        Category::Enum => {
                            insert(types, type_name.1, Type::CppEnum(CppEnum { def }));

                            if !def.has_attribute("ScopedEnumAttribute") {
                                for field in def.fields() {
                                    if field.flags().contains(FieldAttributes::Literal) {
                                        let name = field.name();
                                        insert(
                                            types,
                                            name,
                                            Type::CppConst(CppConst {
                                                namespace: def.namespace(),
                                                field,
                                            }),
                                        );
                                    }
                                }
                            }
                        }
                        Category::Interface => {
                            insert(types, type_name.1, Type::CppInterface(CppInterface { def }));
                        }
                        Category::Struct => {
                            fn make(
                                def: TypeDef,
                                name: &'static str,
                                nested: &HashMap<TypeDef, Vec<TypeDef>>,
                            ) -> CppStruct {
                                let mut ty = CppStruct {
                                    def,
                                    name,
                                    nested: BTreeMap::new(),
                                };

                                for (index, def) in
                                    nested.get(&def).into_iter().flatten().enumerate()
                                {
                                    ty.nested.insert(
                                        def.name(),
                                        make(*def, format!("{}_{index}", ty.name).leak(), nested),
                                    );
                                }

                                ty
                            }

                            insert(
                                types,
                                type_name.1,
                                Type::CppStruct(make(def, def.name(), &nested)),
                            );
                        }
                    };
                }
            }
        }

        READER.store(reader, Ordering::Relaxed);
    }

    #[track_caller]
    pub fn unwrap_full_name(&self, namespace: &str, name: &str) -> Type {
        if let Some(ty) = self.with_full_name(namespace, name).next() {
            ty
        } else {
            panic!("type not found: {namespace}.{name}")
        }
    }

    /// Gets all types matching the given namespace and name.
    pub fn with_full_name(&self, namespace: &str, name: &str) -> impl Iterator<Item = Type> + '_ {
        self.get(namespace)
            .and_then(|types| types.get(name))
            .into_iter()
            .flatten()
            .cloned()
    }
}

enum Category {
    Interface,
    Class,
    Enum,
    Struct,
    Delegate,
    Attribute,
}

impl Category {
    fn new(def: TypeDef) -> Self {
        if let Some(extends) = def.extends() {
            if extends.namespace() == "System" {
                match extends.name() {
                    "Enum" => Self::Enum,
                    "MulticastDelegate" => Self::Delegate,
                    "ValueType" => Self::Struct,
                    "Attribute" => Self::Attribute,
                    _ => Self::Class,
                }
            } else {
                Self::Class
            }
        } else {
            Self::Interface
        }
    }
}
