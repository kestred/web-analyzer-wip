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