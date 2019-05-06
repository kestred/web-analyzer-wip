use crate::ast::Program;
use crate::syntax_kind::*;

pub const SAMPLE_1: &'static str = r#"
var rows = prompt("How many rows for your multiplication table?");
var cols = prompt("How many columns for your multiplication table?");
if(rows == "" || rows == null) rows = 10;
if(cols== "" || cols== null) cols = 10;
createTable(rows, cols);
function createTable(rows, cols) {
  var j=1;
  var output = "<table border='1' width='500' cellspacing='0'cellpadding='5'>";
  for(i=1;i<=rows;i++) {
    output = output + "<tr>";
    while(j<=cols) {
      output = output + "<td>" + i*j + "</td>";
      j = j+1;
    }
    output = output + "</tr>";
    j = 1;
  }
  output = output + "</table>";
  document.write(output);
}
"#;

#[test]
fn test_parse_sample1() {
    let (root, tail) = Program::parse(SAMPLE_1);
    let success = root.errors().is_empty() && tail.trim().is_empty();
    assert!(success, "failed to parse:\n\n{}\n\nerror:\n\t{:?}\n\n", tail, root.errors()[0]);
    assert_eq!(root.syntax.kind(), PROGRAM);
    assert_eq!(root.syntax.children().map(|n| n.kind()).collect::<Vec<_>>(), &[
        VARIABLE_DECLARATION,
        VARIABLE_DECLARATION,
        IF_STATEMENT,
        IF_STATEMENT,
        EXPRESSION_STATEMENT,
        FUNCTION_DECLARATION,
    ]);
}

pub const SAMPLE_2: &'static str = r#"
import Vue from "vue";

export default Vue.extend({
  props: { id: { type: String, required: true } },
  data() {
    return { party: {} };
  },
  watch: {
    id() {
      this.load();
    }
  },
  created() {
    this.load();
  },
  methods: {
    async load() {
      try {
        const variables = { id: this.id };
        const query = `query ($id: PartyId!) {
          userParty(id: $id) {
            id
            status
            partyNumber
            date
            partyType
            partyMethod
            user { id, name, type: __typename }
          }
        }`;
        const resp = await this.$http.post("/graphql", { query, variables });
        if (resp.errors) {
          throw resp.errors[0];
        }
        this.party = resp.data.userParty;
      } catch (error) {
        alert(error.message ? error.message : JSON.stringify(error));
        console.error(error);
        return;
      }
    }
  }
});
"#;

#[test]
fn test_parse_sample2() {
    let (root, tail) = Program::parse(SAMPLE_2);
    let success = root.errors().is_empty() && tail.trim().is_empty();
    assert!(success, "failed to parse:\n\n{}\n\nerror:\n\t{:?}\n\n", tail, root.errors()[0]);
    assert_eq!(root.syntax.kind(), PROGRAM);
    assert_eq!(root.syntax.children().map(|n| n.kind()).collect::<Vec<_>>(), &[
        IMPORT_DECLARATION,
        EMPTY_STATEMENT,
        EXPORT_DEFAULT_DECLARATION,
    ]);
    let export_decl = root.syntax.children().nth(2).unwrap();
    assert_eq!(export_decl.children().map(|n| n.kind()).collect::<Vec<_>>(), &[
        CALL_EXPRESSION
    ]);
    let component = export_decl.children().nth(0).unwrap();
    assert_eq!(component.children().map(|n| n.kind()).collect::<Vec<_>>(), &[
        MEMBER_EXPRESSION,
        OBJECT_EXPRESSION
    ]);
}
