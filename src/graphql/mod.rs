use serde::Serialize;

#[derive(Serialize, Default)]
pub struct GQLRequest {
    query: String,
    variables: Variables,
}

impl GQLRequest {
    pub fn json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Default)]
struct Variables {
    object_id: String,
    stream_id: String,
    my_query: Vec<SpeckleQuery>,
    my_select: Vec<String>,
}

#[derive(Serialize, Clone)]
struct SpeckleQuery {
    field: String,
    value: String,
    operator: Operator,
}

#[derive(Serialize, Clone)]
enum Operator {
    #[serde(rename = "=")]
    Equals,
    #[serde(rename = "!=")]
    Different,
}

#[derive(Default)]
pub struct QueryBuilder {
    _stream_id: String,
    _object_id: String,
    _queries: Vec<SpeckleQuery>,
    _select: Vec<String>,
}

impl QueryBuilder {
    pub fn new(_stream_id: impl Into<String>, _object_id: impl Into<String>) -> Self {
        Self {
            _stream_id: _stream_id.into(),
            _object_id: _object_id.into(),
            ..QueryBuilder::default()
        }
    }

    /// Where @field is equal to @value.
    pub fn where_equals(
        &mut self,
        field: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut Self {
        let query = SpeckleQuery {
            field: field.into(),
            value: value.into(),
            operator: Operator::Equals,
        };

        self._queries.push(query);
        self
    }

    /// Where @field is not equal to @value
    pub fn where_not_equals(
        &mut self,
        field: impl Into<String>,
        value: impl Into<String>,
    ) -> &mut Self {
        let query = SpeckleQuery {
            field: field.into(),
            value: value.into(),
            operator: Operator::Different,
        };

        self._queries.push(query);
        self
    }

    pub fn select(&mut self, attribute: impl Into<String>) -> &mut Self {
        self._select.push(attribute.into());
        self
    }

    pub fn build(&self) -> GQLRequest {
        GQLRequest {
            query: BASE_QUERY.to_string(),
            variables: Variables {
                object_id: self._object_id.clone(),
                stream_id: self._stream_id.clone(),
                my_query: self._queries.clone(),
                my_select: self._select.clone(),
            },
        }
    }
}

const BASE_QUERY: &str = "
query ($stream_id: String!, $object_id: String!, $my_query:[JSONObject!], $my_select: [String]) {
  stream(id: $stream_id) {
    object(id: $object_id) {
      children(query: $my_query select: $my_select) {
        objects {
          data
        }
      }
    }
  }
}
";
