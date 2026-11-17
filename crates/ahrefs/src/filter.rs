//! Typed builders for the `where` filter expression.
//!
//! The API filters with a small JSON language:
//!
//! ```json
//! { "and": [{ "field": "foo", "is": ["eq", 3] },
//!            { "field": "bar", "is": ["lt", 10] }] }
//! ```
//!
//! [`Filter`] mirrors that grammar so a query never has to be hand-written
//! JSON.

use serde_json::{json, Value};

/// A filter expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    /// A single field condition.
    Expr(Expr),
    /// Every child must hold.
    And(Vec<Filter>),
    /// At least one child must hold.
    Or(Vec<Filter>),
    /// The child must not hold.
    Not(Box<Filter>),
}

impl std::ops::Not for Filter {
    type Output = Filter;

    /// The filter must not hold.
    fn not(self) -> Filter {
        Filter::Not(Box::new(self))
    }
}

impl Filter {
    /// A condition on a single field.
    pub fn expr(field: impl Into<String>, condition: Condition) -> Self {
        Filter::Expr(Expr {
            field: field.into(),
            modifier: None,
            condition: Some(condition),
            list_condition: None,
        })
    }

    /// A condition on a single field, with a modifier such as `uppercase`.
    pub fn expr_modifier(
        field: impl Into<String>,
        modifier: impl Into<String>,
        condition: Condition,
    ) -> Self {
        Filter::Expr(Expr {
            field: field.into(),
            modifier: Some(modifier.into()),
            condition: Some(condition),
            list_condition: None,
        })
    }

    /// Every child must hold.
    pub fn and(filters: impl IntoIterator<Item = Filter>) -> Self {
        Filter::And(filters.into_iter().collect())
    }

    /// At least one child must hold.
    pub fn or(filters: impl IntoIterator<Item = Filter>) -> Self {
        Filter::Or(filters.into_iter().collect())
    }

    /// A condition that must hold for at least one element of an array field.
    pub fn list_any(field: impl Into<String>, condition: Filter) -> Self {
        Filter::Expr(Expr {
            field: field.into(),
            modifier: None,
            condition: None,
            list_condition: Some(ListCondition::Any(Box::new(condition))),
        })
    }

    /// A condition that must hold for every element of an array field.
    pub fn list_all(field: impl Into<String>, condition: Filter) -> Self {
        Filter::Expr(Expr {
            field: field.into(),
            modifier: None,
            condition: None,
            list_condition: Some(ListCondition::All(Box::new(condition))),
        })
    }

    /// A list condition built from operator/value pairs.
    ///
    /// ```
    /// use ahrefs::filter::Filter;
    /// let f = Filter::list_pairs("bar", &[("prefix", "Ahrefs"), ("suffix", "seo")]);
    /// ```
    pub fn list_pairs<V: Clone + Into<Value>>(
        field: impl Into<String>,
        pairs: &[(&str, V)],
    ) -> Self {
        let pairs: Vec<Value> = pairs
            .iter()
            .map(|(op, value)| json!([op, value.clone().into()]))
            .collect();
        Filter::Expr(Expr {
            field: field.into(),
            modifier: None,
            condition: None,
            list_condition: Some(ListCondition::Pairs(pairs)),
        })
    }

    /// The JSON representation sent to the API.
    pub fn to_json(&self) -> Value {
        match self {
            Filter::Expr(expr) => expr.to_json(),
            Filter::And(filters) => json!({ "and": to_array(filters) }),
            Filter::Or(filters) => json!({ "or": to_array(filters) }),
            Filter::Not(filter) => json!({ "not": filter.to_json() }),
        }
    }
}

fn to_array(filters: &[Filter]) -> Vec<Value> {
    filters.iter().map(Filter::to_json).collect()
}

/// A condition on one field.
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    /// The field alias, dot notation for nested fields.
    pub field: String,
    /// An optional modifier such as `uppercase`.
    pub modifier: Option<String>,
    /// The condition on a scalar field.
    pub condition: Option<Condition>,
    /// The condition on an array field.
    pub list_condition: Option<ListCondition>,
}

impl Expr {
    /// The JSON representation sent to the API.
    pub fn to_json(&self) -> Value {
        let mut map = serde_json::Map::new();
        map.insert("field".into(), Value::String(self.field.clone()));
        if let Some(modifier) = &self.modifier {
            map.insert("modifier".into(), Value::String(modifier.clone()));
        }
        if let Some(condition) = &self.condition {
            map.insert("is".into(), condition.to_json());
        }
        if let Some(list_condition) = &self.list_condition {
            map.insert("list_is".into(), list_condition.to_json());
        }
        Value::Object(map)
    }
}

/// The condition of a scalar field.
#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    /// Equal to the value.
    Eq(Value),
    /// Not equal to the value.
    Neq(Value),
    /// Greater than the value.
    Gt(Value),
    /// Greater than or equal to the value.
    Gte(Value),
    /// Less than the value.
    Lt(Value),
    /// Less than or equal to the value.
    Lte(Value),
    /// Contains the value as a substring.
    Substring(Value),
    /// Contains the value as a case-insensitive substring.
    Isubstring(Value),
    /// Matches the value as a phrase.
    PhraseMatch(Value),
    /// Matches the value as a case-insensitive phrase.
    IphraseMatch(Value),
    /// Starts with the value.
    Prefix(Value),
    /// Ends with the value.
    Suffix(Value),
    /// Matches the regular expression.
    Regex(Value),
    /// The field is empty.
    Empty,
    /// The field is null.
    IsNull,
}

impl Condition {
    /// The JSON representation sent to the API.
    pub fn to_json(&self) -> Value {
        match self {
            Condition::Eq(value) => json!(["eq", value]),
            Condition::Neq(value) => json!(["neq", value]),
            Condition::Gt(value) => json!(["gt", value]),
            Condition::Gte(value) => json!(["gte", value]),
            Condition::Lt(value) => json!(["lt", value]),
            Condition::Lte(value) => json!(["lte", value]),
            Condition::Substring(value) => json!(["substring", value]),
            Condition::Isubstring(value) => json!(["isubstring", value]),
            Condition::PhraseMatch(value) => json!(["phrase_match", value]),
            Condition::IphraseMatch(value) => json!(["iphrase_match", value]),
            Condition::Prefix(value) => json!(["prefix", value]),
            Condition::Suffix(value) => json!(["suffix", value]),
            Condition::Regex(value) => json!(["regex", value]),
            Condition::Empty => json!("empty"),
            Condition::IsNull => json!("is_null"),
        }
    }
}

/// The condition of an array field.
#[derive(Debug, Clone, PartialEq)]
pub enum ListCondition {
    /// At least one element matches.
    Any(Box<Filter>),
    /// Every element matches.
    All(Box<Filter>),
    /// A list of operator/value pairs.
    Pairs(Vec<Value>),
}

impl ListCondition {
    /// The JSON representation sent to the API.
    pub fn to_json(&self) -> Value {
        match self {
            ListCondition::Any(filter) => json!({ "any": filter.to_json() }),
            ListCondition::All(filter) => json!({ "all": filter.to_json() }),
            ListCondition::Pairs(pairs) => json!({ "and": pairs }),
        }
    }
}
