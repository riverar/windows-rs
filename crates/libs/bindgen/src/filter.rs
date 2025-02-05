use super::*;

#[derive(Debug, Default)]
pub struct Filter(Vec<(String, bool)>);

impl Filter {
    pub fn new(include: &[&str], exclude: &[&str]) -> Self {
        let mut rules = vec![];

        for filter in include {
            push_filter(&mut rules, filter, true);
        }

        for filter in exclude {
            push_filter(&mut rules, filter, false)
        }

        debug_assert!(!rules.is_empty());

        rules.sort_unstable_by(|left, right| {
            let left = (left.0.len(), !left.1);
            let right = (right.0.len(), !right.1);
            left.cmp(&right).reverse()
        });

        Self(rules)
    }

    pub fn includes_namespace(&self, namespace: &str) -> bool {
        for rule in &self.0 {
            if rule.1 {
                // include
                if namespace_starts_with(&rule.0, namespace) {
                    return true;
                }
                if namespace_starts_with(namespace, &rule.0) {
                    return true;
                }
            } else {
                // exclude
                if namespace_starts_with(namespace, &rule.0) {
                    return false;
                }
            }
        }

        false
    }

    pub fn includes_type_name(&self, name: TypeName) -> bool {
        for rule in &self.0 {
            if match_type_name(&rule.0, name.namespace(), name.name()) {
                return rule.1;
            }
        }

        false
    }

    pub fn excludes_type_name(&self, name: TypeName) -> bool {
        for rule in &self.0 {
            if match_type_name(&rule.0, name.namespace(), name.name()) {
                return !rule.1;
            }
        }

        false
    }
}

fn push_filter(rules: &mut Vec<(String, bool)>, filter: &str, include: bool) {
    let reader = reader();

    if reader.contains_key(filter) {
        rules.push((filter.to_string(), include));
        return;
    }

    if let Some((namespace, name)) = filter.rsplit_once('.') {
        if reader.with_full_name(namespace, name).next().is_some() {
            rules.push((filter.to_string(), include));
            return;
        }
    }

    let mut pushed = false;

    for (namespace, types) in reader.iter() {
        if types.get(filter).is_some() {
            rules.push((format!("{namespace}.{filter}"), include));
            pushed = true;
        }
    }

    if pushed {
        return;
    }

    if reader
        .keys()
        .any(|namespace| namespace_starts_with(namespace, filter))
    {
        rules.push((filter.to_string(), include));
        return;
    }

    panic!("type not found: `{filter}`");
}

fn match_type_name(rule: &str, namespace: &str, name: &str) -> bool {
    if rule.len() <= namespace.len() {
        return namespace.starts_with(rule);
    }

    if !rule.starts_with(namespace) {
        return false;
    }

    if rule.as_bytes()[namespace.len()] != b'.' {
        return false;
    }

    name == &rule[namespace.len() + 1..]
}
