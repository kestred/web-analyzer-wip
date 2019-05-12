use crate::ast::Program;
use crate::syntax_kind::*;
use grammar_utils::ast;
use test_utils::assert_diff;

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

#[test]
fn test_parse_sample1() {
    let (root, tail) = Program::parse(SAMPLE_1);
    let success = root.errors().is_empty() && tail.trim().is_empty();
    assert!(success, "failed to parse:\n\n{}\n\nerror:\n\t{:?}\n\n", tail, root.errors()[0]);
    let syntax = ast::debug_dump(&root.syntax, root.errors(), |k| as_debug_repr(k).unwrap().name);
    assert_diff!(
      syntax.trim(),
      r#"
PROGRAM@[0; 621)
  WHITESPACE@[0; 1)
  VARIABLE_DECLARATION@[1; 66)
    VAR_KW@[1; 4)  "var"
    WHITESPACE@[4; 5)
    VARIABLE_DECLARATOR@[5; 66)
      IDENTIFIER@[5; 9)
        IDENTIFIER@[5; 9)  "rows"
      WHITESPACE@[9; 10)
      EQ@[10; 11)  "="
      WHITESPACE@[11; 12)
      CALL_EXPRESSION@[12; 66)
        IDENTIFIER@[12; 18)
          IDENTIFIER@[12; 18)  "prompt"
        L_PAREN@[18; 19)  "("
        LITERAL@[19; 65)
          STRING_LITERAL@[19; 65)
        R_PAREN@[65; 66)  ")"
  SEMICOLON@[66; 67)  ";"
  WHITESPACE@[67; 68)
  VARIABLE_DECLARATION@[68; 136)
    VAR_KW@[68; 71)  "var"
    WHITESPACE@[71; 72)
    VARIABLE_DECLARATOR@[72; 136)
      IDENTIFIER@[72; 76)
        IDENTIFIER@[72; 76)  "cols"
      WHITESPACE@[76; 77)
      EQ@[77; 78)  "="
      WHITESPACE@[78; 79)
      CALL_EXPRESSION@[79; 136)
        IDENTIFIER@[79; 85)
          IDENTIFIER@[79; 85)  "prompt"
        L_PAREN@[85; 86)  "("
        LITERAL@[86; 135)
          STRING_LITERAL@[86; 135)
        R_PAREN@[135; 136)  ")"
  SEMICOLON@[136; 137)  ";"
  WHITESPACE@[137; 138)
  IF_STATEMENT@[138; 179)
    IF_KW@[138; 140)  "if"
    L_PAREN@[140; 141)  "("
    LOGICAL_EXPRESSION@[141; 167)
      BINARY_EXPRESSION@[141; 151)
        IDENTIFIER@[141; 145)
          IDENTIFIER@[141; 145)  "rows"
        WHITESPACE@[145; 146)
        EQEQ@[146; 148)  "=="
        WHITESPACE@[148; 149)
        LITERAL@[149; 151)
          STRING_LITERAL@[149; 151)  "\"\""
      WHITESPACE@[151; 152)
      OR@[152; 154)  "||"
      WHITESPACE@[154; 155)
      BINARY_EXPRESSION@[155; 167)
        IDENTIFIER@[155; 159)
          IDENTIFIER@[155; 159)  "rows"
        WHITESPACE@[159; 160)
        EQEQ@[160; 162)  "=="
        WHITESPACE@[162; 163)
        LITERAL@[163; 167)
          NULL_KW@[163; 167)  "null"
    R_PAREN@[167; 168)  ")"
    WHITESPACE@[168; 169)
    EXPRESSION_STATEMENT@[169; 179)
      ASSIGNMENT_EXPRESSION@[169; 178)
        IDENTIFIER@[169; 173)
          IDENTIFIER@[169; 173)  "rows"
        WHITESPACE@[173; 174)
        EQ@[174; 175)  "="
        WHITESPACE@[175; 176)
        LITERAL@[176; 178)
          NUMBER_LITERAL@[176; 178)  "10"
      SEMICOLON@[178; 179)  ";"
  WHITESPACE@[179; 180)
  IF_STATEMENT@[180; 219)
    IF_KW@[180; 182)  "if"
    L_PAREN@[182; 183)  "("
    LOGICAL_EXPRESSION@[183; 207)
      BINARY_EXPRESSION@[183; 192)
        IDENTIFIER@[183; 187)
          IDENTIFIER@[183; 187)  "cols"
        EQEQ@[187; 189)  "=="
        WHITESPACE@[189; 190)
        LITERAL@[190; 192)
          STRING_LITERAL@[190; 192)  "\"\""
      WHITESPACE@[192; 193)
      OR@[193; 195)  "||"
      WHITESPACE@[195; 196)
      BINARY_EXPRESSION@[196; 207)
        IDENTIFIER@[196; 200)
          IDENTIFIER@[196; 200)  "cols"
        EQEQ@[200; 202)  "=="
        WHITESPACE@[202; 203)
        LITERAL@[203; 207)
          NULL_KW@[203; 207)  "null"
    R_PAREN@[207; 208)  ")"
    WHITESPACE@[208; 209)
    EXPRESSION_STATEMENT@[209; 219)
      ASSIGNMENT_EXPRESSION@[209; 218)
        IDENTIFIER@[209; 213)
          IDENTIFIER@[209; 213)  "cols"
        WHITESPACE@[213; 214)
        EQ@[214; 215)  "="
        WHITESPACE@[215; 216)
        LITERAL@[216; 218)
          NUMBER_LITERAL@[216; 218)  "10"
      SEMICOLON@[218; 219)  ";"
  WHITESPACE@[219; 220)
  EXPRESSION_STATEMENT@[220; 244)
    CALL_EXPRESSION@[220; 243)
      IDENTIFIER@[220; 231)
        IDENTIFIER@[220; 231)  "createTable"
      L_PAREN@[231; 232)  "("
      IDENTIFIER@[232; 236)
        IDENTIFIER@[232; 236)  "rows"
      COMMA@[236; 237)  ","
      WHITESPACE@[237; 238)
      IDENTIFIER@[238; 242)
        IDENTIFIER@[238; 242)  "cols"
      R_PAREN@[242; 243)  ")"
    SEMICOLON@[243; 244)  ";"
  WHITESPACE@[244; 245)
  FUNCTION_DECLARATION@[245; 621)
    FUNCTION_KW@[245; 253)  "function"
    WHITESPACE@[253; 254)
    IDENTIFIER@[254; 265)
      IDENTIFIER@[254; 265)  "createTable"
    L_PAREN@[265; 266)  "("
    ASSIGNMENT_PATTERN@[266; 270)
      IDENTIFIER@[266; 270)
        IDENTIFIER@[266; 270)  "rows"
    COMMA@[270; 271)  ","
    WHITESPACE@[271; 272)
    ASSIGNMENT_PATTERN@[272; 276)
      IDENTIFIER@[272; 276)
        IDENTIFIER@[272; 276)  "cols"
    R_PAREN@[276; 277)  ")"
    WHITESPACE@[277; 278)
    BLOCK_STATEMENT@[278; 621)
      L_CURLY@[278; 279)  "{"
      WHITESPACE@[279; 282)
      VARIABLE_DECLARATION@[282; 289)
        VAR_KW@[282; 285)  "var"
        WHITESPACE@[285; 286)
        VARIABLE_DECLARATOR@[286; 289)
          IDENTIFIER@[286; 287)
            IDENTIFIER@[286; 287)  "j"
          EQ@[287; 288)  "="
          LITERAL@[288; 289)
            NUMBER_LITERAL@[288; 289)  "1"
      SEMICOLON@[289; 290)  ";"
      WHITESPACE@[290; 293)
      VARIABLE_DECLARATION@[293; 369)
        VAR_KW@[293; 296)  "var"
        WHITESPACE@[296; 297)
        VARIABLE_DECLARATOR@[297; 369)
          IDENTIFIER@[297; 303)
            IDENTIFIER@[297; 303)  "output"
          WHITESPACE@[303; 304)
          EQ@[304; 305)  "="
          WHITESPACE@[305; 306)
          LITERAL@[306; 369)
            STRING_LITERAL@[306; 369)
      SEMICOLON@[369; 370)  ";"
      WHITESPACE@[370; 373)
      FOR_STATEMENT@[373; 619)
        FOR_KW@[373; 376)  "for"
        L_PAREN@[376; 377)  "("
        ASSIGNMENT_EXPRESSION@[377; 380)
          IDENTIFIER@[377; 378)
            IDENTIFIER@[377; 378)  "i"
          EQ@[378; 379)  "="
          LITERAL@[379; 380)
            NUMBER_LITERAL@[379; 380)  "1"
        SEMICOLON@[380; 381)  ";"
        BINARY_EXPRESSION@[381; 388)
          IDENTIFIER@[381; 382)
            IDENTIFIER@[381; 382)  "i"
          LT_EQ@[382; 384)  "<="
          IDENTIFIER@[384; 388)
            IDENTIFIER@[384; 388)  "rows"
        SEMICOLON@[388; 389)  ";"
        UPDATE_EXPRESSION@[389; 392)
          IDENTIFIER@[389; 390)
            IDENTIFIER@[389; 390)  "i"
          INCREMENT@[390; 392)  "++"
        R_PAREN@[392; 393)  ")"
        WHITESPACE@[393; 394)
        BLOCK_STATEMENT@[394; 561)
          L_CURLY@[394; 395)  "{"
          WHITESPACE@[395; 400)
          EXPRESSION_STATEMENT@[400; 425)
            ASSIGNMENT_EXPRESSION@[400; 424)
              IDENTIFIER@[400; 406)
                IDENTIFIER@[400; 406)  "output"
              WHITESPACE@[406; 407)
              EQ@[407; 408)  "="
              WHITESPACE@[408; 409)
              BINARY_EXPRESSION@[409; 424)
                IDENTIFIER@[409; 415)
                  IDENTIFIER@[409; 415)  "output"
                WHITESPACE@[415; 416)
                PLUS@[416; 417)  "+"
                WHITESPACE@[417; 418)
                LITERAL@[418; 424)
                  STRING_LITERAL@[418; 424)  "\"<tr>\""
            SEMICOLON@[424; 425)  ";"
          WHITESPACE@[425; 430)
          WHILE_STATEMENT@[430; 515)
            WHILE_KW@[430; 435)  "while"
            L_PAREN@[435; 436)  "("
            BINARY_EXPRESSION@[436; 443)
              IDENTIFIER@[436; 437)
                IDENTIFIER@[436; 437)  "j"
              LT_EQ@[437; 439)  "<="
              IDENTIFIER@[439; 443)
                IDENTIFIER@[439; 443)  "cols"
            R_PAREN@[443; 444)  ")"
            WHITESPACE@[444; 445)
            BLOCK_STATEMENT@[445; 515)
              L_CURLY@[445; 446)  "{"
              WHITESPACE@[446; 453)
              EXPRESSION_STATEMENT@[453; 494)
                ASSIGNMENT_EXPRESSION@[453; 493)
                  IDENTIFIER@[453; 459)
                    IDENTIFIER@[453; 459)  "output"
                  WHITESPACE@[459; 460)
                  EQ@[460; 461)  "="
                  WHITESPACE@[461; 462)
                  BINARY_EXPRESSION@[462; 493)
                    IDENTIFIER@[462; 468)
                      IDENTIFIER@[462; 468)  "output"
                    WHITESPACE@[468; 469)
                    PLUS@[469; 470)  "+"
                    WHITESPACE@[470; 471)
                    BINARY_EXPRESSION@[471; 493)
                      LITERAL@[471; 477)
                        STRING_LITERAL@[471; 477)  "\"<td>\""
                      WHITESPACE@[477; 478)
                      PLUS@[478; 479)  "+"
                      WHITESPACE@[479; 480)
                      BINARY_EXPRESSION@[480; 493)
                        BINARY_EXPRESSION@[480; 483)
                          IDENTIFIER@[480; 481)
                            IDENTIFIER@[480; 481)  "i"
                          ASTERISK@[481; 482)  "*"
                          IDENTIFIER@[482; 483)
                            IDENTIFIER@[482; 483)  "j"
                        WHITESPACE@[483; 484)
                        PLUS@[484; 485)  "+"
                        WHITESPACE@[485; 486)
                        LITERAL@[486; 493)
                          STRING_LITERAL@[486; 493)  "\"</td>\""
                SEMICOLON@[493; 494)  ";"
              WHITESPACE@[494; 501)
              EXPRESSION_STATEMENT@[501; 509)
                ASSIGNMENT_EXPRESSION@[501; 508)
                  IDENTIFIER@[501; 502)
                    IDENTIFIER@[501; 502)  "j"
                  WHITESPACE@[502; 503)
                  EQ@[503; 504)  "="
                  WHITESPACE@[504; 505)
                  BINARY_EXPRESSION@[505; 508)
                    IDENTIFIER@[505; 506)
                      IDENTIFIER@[505; 506)  "j"
                    PLUS@[506; 507)  "+"
                    LITERAL@[507; 508)
                      NUMBER_LITERAL@[507; 508)  "1"
                SEMICOLON@[508; 509)  ";"
              WHITESPACE@[509; 514)
              R_CURLY@[514; 515)  "}"
          WHITESPACE@[515; 520)
          EXPRESSION_STATEMENT@[520; 546)
            ASSIGNMENT_EXPRESSION@[520; 545)
              IDENTIFIER@[520; 526)
                IDENTIFIER@[520; 526)  "output"
              WHITESPACE@[526; 527)
              EQ@[527; 528)  "="
              WHITESPACE@[528; 529)
              BINARY_EXPRESSION@[529; 545)
                IDENTIFIER@[529; 535)
                  IDENTIFIER@[529; 535)  "output"
                WHITESPACE@[535; 536)
                PLUS@[536; 537)  "+"
                WHITESPACE@[537; 538)
                LITERAL@[538; 545)
                  STRING_LITERAL@[538; 545)  "\"</tr>\""
            SEMICOLON@[545; 546)  ";"
          WHITESPACE@[546; 551)
          EXPRESSION_STATEMENT@[551; 557)
            ASSIGNMENT_EXPRESSION@[551; 556)
              IDENTIFIER@[551; 552)
                IDENTIFIER@[551; 552)  "j"
              WHITESPACE@[552; 553)
              EQ@[553; 554)  "="
              WHITESPACE@[554; 555)
              LITERAL@[555; 556)
                NUMBER_LITERAL@[555; 556)  "1"
            SEMICOLON@[556; 557)  ";"
          WHITESPACE@[557; 560)
          R_CURLY@[560; 561)  "}"
        WHITESPACE@[561; 564)
        EXPRESSION_STATEMENT@[564; 593)
          ASSIGNMENT_EXPRESSION@[564; 592)
            IDENTIFIER@[564; 570)
              IDENTIFIER@[564; 570)  "output"
            WHITESPACE@[570; 571)
            EQ@[571; 572)  "="
            WHITESPACE@[572; 573)
            BINARY_EXPRESSION@[573; 592)
              IDENTIFIER@[573; 579)
                IDENTIFIER@[573; 579)  "output"
              WHITESPACE@[579; 580)
              PLUS@[580; 581)  "+"
              WHITESPACE@[581; 582)
              LITERAL@[582; 592)
                STRING_LITERAL@[582; 592)  "\"</table>\""
          SEMICOLON@[592; 593)  ";"
        WHITESPACE@[593; 596)
        EXPRESSION_STATEMENT@[596; 619)
          CALL_EXPRESSION@[596; 618)
            MEMBER_EXPRESSION@[596; 610)
              IDENTIFIER@[596; 604)
                IDENTIFIER@[596; 604)  "document"
              DOT@[604; 605)  "."
              IDENTIFIER@[605; 610)
                IDENTIFIER@[605; 610)  "write"
            L_PAREN@[610; 611)  "("
            IDENTIFIER@[611; 617)
              IDENTIFIER@[611; 617)  "output"
            R_PAREN@[617; 618)  ")"
          SEMICOLON@[618; 619)  ";"
      WHITESPACE@[619; 620)
      R_CURLY@[620; 621)  "}"
"#.trim()
    );
}

#[test]
fn test_parse_sample2() {
    let (root, tail) = Program::parse(SAMPLE_2);
    let success = root.errors().is_empty() && tail.trim().is_empty();
    assert!(success, "failed to parse:\n\n{}\n\nerror:\n\t{:?}\n\n", tail, root.errors()[0]);
    let syntax = ast::debug_dump(&root.syntax, root.errors(), |k| as_debug_repr(k).unwrap().name);
    assert_diff!(
      syntax.trim(),
      r#"
PROGRAM@[0; 942)
  WHITESPACE@[0; 1)
  IMPORT_DECLARATION@[1; 23)
    IMPORT_KW@[1; 7)  "import"
    WHITESPACE@[7; 8)
    IMPORT_DEFAULT_SPECIFIER@[8; 11)
      IDENTIFIER@[8; 11)
        IDENTIFIER@[8; 11)  "Vue"
    WHITESPACE@[11; 12)
    FROM_KW@[12; 16)
      IDENTIFIER@[12; 16)  "from"
    WHITESPACE@[16; 17)
    LITERAL@[17; 22)
      STRING_LITERAL@[17; 22)  "\"vue\""
    SEMICOLON@[22; 23)  ";"
  WHITESPACE@[23; 25)
  EXPORT_DEFAULT_DECLARATION@[25; 942)
    EXPORT_KW@[25; 31)  "export"
    WHITESPACE@[31; 32)
    DEFAULT_KW@[32; 39)  "default"
    WHITESPACE@[39; 40)
    CALL_EXPRESSION@[40; 941)
      MEMBER_EXPRESSION@[40; 50)
        IDENTIFIER@[40; 43)
          IDENTIFIER@[40; 43)  "Vue"
        DOT@[43; 44)  "."
        IDENTIFIER@[44; 50)
          IDENTIFIER@[44; 50)  "extend"
      L_PAREN@[50; 51)  "("
      OBJECT_EXPRESSION@[51; 940)
        L_CURLY@[51; 52)  "{"
        WHITESPACE@[52; 55)
        PROPERTY@[55; 102)
          IDENTIFIER@[55; 60)
            IDENTIFIER@[55; 60)  "props"
          COLON@[60; 61)  ":"
          WHITESPACE@[61; 62)
          OBJECT_EXPRESSION@[62; 102)
            L_CURLY@[62; 63)  "{"
            WHITESPACE@[63; 64)
            PROPERTY@[64; 100)
              IDENTIFIER@[64; 66)
                IDENTIFIER@[64; 66)  "id"
              COLON@[66; 67)  ":"
              WHITESPACE@[67; 68)
              OBJECT_EXPRESSION@[68; 100)
                L_CURLY@[68; 69)  "{"
                WHITESPACE@[69; 70)
                PROPERTY@[70; 82)
                  IDENTIFIER@[70; 74)
                    IDENTIFIER@[70; 74)  "type"
                  COLON@[74; 75)  ":"
                  WHITESPACE@[75; 76)
                  IDENTIFIER@[76; 82)
                    IDENTIFIER@[76; 82)  "String"
                COMMA@[82; 83)  ","
                WHITESPACE@[83; 84)
                PROPERTY@[84; 98)
                  IDENTIFIER@[84; 92)
                    IDENTIFIER@[84; 92)  "required"
                  COLON@[92; 93)  ":"
                  WHITESPACE@[93; 94)
                  LITERAL@[94; 98)
                    TRUE_KW@[94; 98)  "true"
                WHITESPACE@[98; 99)
                R_CURLY@[99; 100)  "}"
            WHITESPACE@[100; 101)
            R_CURLY@[101; 102)  "}"
        COMMA@[102; 103)  ","
        WHITESPACE@[103; 106)
        PROPERTY@[106; 144)
          FUNCTION_EXPRESSION@[106; 144)
            IDENTIFIER@[106; 110)
              IDENTIFIER@[106; 110)  "data"
            L_PAREN@[110; 111)  "("
            R_PAREN@[111; 112)  ")"
            WHITESPACE@[112; 113)
            BLOCK_STATEMENT@[113; 144)
              L_CURLY@[113; 114)  "{"
              WHITESPACE@[114; 119)
              RETURN_STATEMENT@[119; 140)
                RETURN_KW@[119; 125)  "return"
                WHITESPACE@[125; 126)
                OBJECT_EXPRESSION@[126; 139)
                  L_CURLY@[126; 127)  "{"
                  WHITESPACE@[127; 128)
                  PROPERTY@[128; 137)
                    IDENTIFIER@[128; 133)
                      IDENTIFIER@[128; 133)  "party"
                    COLON@[133; 134)  ":"
                    WHITESPACE@[134; 135)
                    OBJECT_EXPRESSION@[135; 137)
                      L_CURLY@[135; 136)  "{"
                      R_CURLY@[136; 137)  "}"
                  WHITESPACE@[137; 138)
                  R_CURLY@[138; 139)  "}"
                SEMICOLON@[139; 140)  ";"
              WHITESPACE@[140; 143)
              R_CURLY@[143; 144)  "}"
        COMMA@[144; 145)  ","
        WHITESPACE@[145; 148)
        PROPERTY@[148; 196)
          IDENTIFIER@[148; 153)
            IDENTIFIER@[148; 153)  "watch"
          COLON@[153; 154)  ":"
          WHITESPACE@[154; 155)
          OBJECT_EXPRESSION@[155; 196)
            L_CURLY@[155; 156)  "{"
            WHITESPACE@[156; 161)
            PROPERTY@[161; 192)
              FUNCTION_EXPRESSION@[161; 192)
                IDENTIFIER@[161; 163)
                  IDENTIFIER@[161; 163)  "id"
                L_PAREN@[163; 164)  "("
                R_PAREN@[164; 165)  ")"
                WHITESPACE@[165; 166)
                BLOCK_STATEMENT@[166; 192)
                  L_CURLY@[166; 167)  "{"
                  WHITESPACE@[167; 174)
                  EXPRESSION_STATEMENT@[174; 186)
                    CALL_EXPRESSION@[174; 185)
                      MEMBER_EXPRESSION@[174; 183)
                        THIS_EXPRESSION@[174; 178)
                          THIS_KW@[174; 178)  "this"
                        DOT@[178; 179)  "."
                        IDENTIFIER@[179; 183)
                          IDENTIFIER@[179; 183)  "load"
                      L_PAREN@[183; 184)  "("
                      R_PAREN@[184; 185)  ")"
                    SEMICOLON@[185; 186)  ";"
                  WHITESPACE@[186; 191)
                  R_CURLY@[191; 192)  "}"
            WHITESPACE@[192; 195)
            R_CURLY@[195; 196)  "}"
        COMMA@[196; 197)  ","
        WHITESPACE@[197; 200)
        PROPERTY@[200; 232)
          FUNCTION_EXPRESSION@[200; 232)
            IDENTIFIER@[200; 207)
              IDENTIFIER@[200; 207)  "created"
            L_PAREN@[207; 208)  "("
            R_PAREN@[208; 209)  ")"
            WHITESPACE@[209; 210)
            BLOCK_STATEMENT@[210; 232)
              L_CURLY@[210; 211)  "{"
              WHITESPACE@[211; 216)
              EXPRESSION_STATEMENT@[216; 228)
                CALL_EXPRESSION@[216; 227)
                  MEMBER_EXPRESSION@[216; 225)
                    THIS_EXPRESSION@[216; 220)
                      THIS_KW@[216; 220)  "this"
                    DOT@[220; 221)  "."
                    IDENTIFIER@[221; 225)
                      IDENTIFIER@[221; 225)  "load"
                  L_PAREN@[225; 226)  "("
                  R_PAREN@[226; 227)  ")"
                SEMICOLON@[227; 228)  ";"
              WHITESPACE@[228; 231)
              R_CURLY@[231; 232)  "}"
        COMMA@[232; 233)  ","
        WHITESPACE@[233; 236)
        PROPERTY@[236; 938)
          IDENTIFIER@[236; 243)
            IDENTIFIER@[236; 243)  "methods"
          COLON@[243; 244)  ":"
          WHITESPACE@[244; 245)
          OBJECT_EXPRESSION@[245; 938)
            L_CURLY@[245; 246)  "{"
            WHITESPACE@[246; 251)
            PROPERTY@[251; 934)
              FUNCTION_EXPRESSION@[251; 934)
                ASYNC_KW@[251; 256)
                  IDENTIFIER@[251; 256)  "async"
                WHITESPACE@[256; 257)
                IDENTIFIER@[257; 261)
                  IDENTIFIER@[257; 261)  "load"
                L_PAREN@[261; 262)  "("
                R_PAREN@[262; 263)  ")"
                WHITESPACE@[263; 264)
                BLOCK_STATEMENT@[264; 934)
                  L_CURLY@[264; 265)  "{"
                  WHITESPACE@[265; 272)
                  TRY_STATEMENT@[272; 928)
                    TRY_KW@[272; 275)  "try"
                    WHITESPACE@[275; 276)
                    BLOCK_STATEMENT@[276; 788)
                      L_CURLY@[276; 277)  "{"
                      WHITESPACE@[277; 286)
                      VARIABLE_DECLARATION@[286; 319)
                        CONST_KW@[286; 291)  "const"
                        WHITESPACE@[291; 292)
                        VARIABLE_DECLARATOR@[292; 319)
                          IDENTIFIER@[292; 301)
                            IDENTIFIER@[292; 301)  "variables"
                          WHITESPACE@[301; 302)
                          EQ@[302; 303)  "="
                          WHITESPACE@[303; 304)
                          OBJECT_EXPRESSION@[304; 319)
                            L_CURLY@[304; 305)  "{"
                            WHITESPACE@[305; 306)
                            PROPERTY@[306; 317)
                              IDENTIFIER@[306; 308)
                                IDENTIFIER@[306; 308)  "id"
                              COLON@[308; 309)  ":"
                              WHITESPACE@[309; 310)
                              MEMBER_EXPRESSION@[310; 317)
                                THIS_EXPRESSION@[310; 314)
                                  THIS_KW@[310; 314)  "this"
                                DOT@[314; 315)  "."
                                IDENTIFIER@[315; 317)
                                  IDENTIFIER@[315; 317)  "id"
                            WHITESPACE@[317; 318)
                            R_CURLY@[318; 319)  "}"
                      SEMICOLON@[319; 320)  ";"
                      WHITESPACE@[320; 329)
                      VARIABLE_DECLARATION@[329; 590)
                        CONST_KW@[329; 334)  "const"
                        WHITESPACE@[334; 335)
                        VARIABLE_DECLARATOR@[335; 590)
                          IDENTIFIER@[335; 340)
                            IDENTIFIER@[335; 340)  "query"
                          WHITESPACE@[340; 341)
                          EQ@[341; 342)  "="
                          WHITESPACE@[342; 343)
                          TEMPLATE_LITERAL@[343; 590)
                      SEMICOLON@[590; 591)  ";"
                      WHITESPACE@[591; 600)
                      VARIABLE_DECLARATION@[600; 668)
                        CONST_KW@[600; 605)  "const"
                        WHITESPACE@[605; 606)
                        VARIABLE_DECLARATOR@[606; 668)
                          IDENTIFIER@[606; 610)
                            IDENTIFIER@[606; 610)  "resp"
                          WHITESPACE@[610; 611)
                          EQ@[611; 612)  "="
                          WHITESPACE@[612; 613)
                          AWAIT_EXPRESSION@[613; 668)
                            AWAIT_KW@[613; 618)  "await"
                            WHITESPACE@[618; 619)
                            CALL_EXPRESSION@[619; 668)
                              MEMBER_EXPRESSION@[619; 634)
                                MEMBER_EXPRESSION@[619; 629)
                                  THIS_EXPRESSION@[619; 623)
                                    THIS_KW@[619; 623)  "this"
                                  DOT@[623; 624)  "."
                                  IDENTIFIER@[624; 629)
                                    IDENTIFIER@[624; 629)  "$http"
                                DOT@[629; 630)  "."
                                IDENTIFIER@[630; 634)
                                  IDENTIFIER@[630; 634)  "post"
                              L_PAREN@[634; 635)  "("
                              LITERAL@[635; 645)
                                STRING_LITERAL@[635; 645)  "\"/graphql\""
                              COMMA@[645; 646)  ","
                              WHITESPACE@[646; 647)
                              OBJECT_EXPRESSION@[647; 667)
                                L_CURLY@[647; 648)  "{"
                                WHITESPACE@[648; 649)
                                PROPERTY@[649; 654)
                                  IDENTIFIER@[649; 654)
                                    IDENTIFIER@[649; 654)  "query"
                                COMMA@[654; 655)  ","
                                WHITESPACE@[655; 656)
                                PROPERTY@[656; 665)
                                  IDENTIFIER@[656; 665)
                                    IDENTIFIER@[656; 665)  "variables"
                                WHITESPACE@[665; 666)
                                R_CURLY@[666; 667)  "}"
                              R_PAREN@[667; 668)  ")"
                      SEMICOLON@[668; 669)  ";"
                      WHITESPACE@[669; 678)
                      IF_STATEMENT@[678; 738)
                        IF_KW@[678; 680)  "if"
                        WHITESPACE@[680; 681)
                        L_PAREN@[681; 682)  "("
                        MEMBER_EXPRESSION@[682; 693)
                          IDENTIFIER@[682; 686)
                            IDENTIFIER@[682; 686)  "resp"
                          DOT@[686; 687)  "."
                          IDENTIFIER@[687; 693)
                            IDENTIFIER@[687; 693)  "errors"
                        R_PAREN@[693; 694)  ")"
                        WHITESPACE@[694; 695)
                        BLOCK_STATEMENT@[695; 738)
                          L_CURLY@[695; 696)  "{"
                          WHITESPACE@[696; 707)
                          THROW_STATEMENT@[707; 728)
                            THROW_KW@[707; 712)  "throw"
                            WHITESPACE@[712; 713)
                            MEMBER_EXPRESSION@[713; 727)
                              MEMBER_EXPRESSION@[713; 724)
                                IDENTIFIER@[713; 717)
                                  IDENTIFIER@[713; 717)  "resp"
                                DOT@[717; 718)  "."
                                IDENTIFIER@[718; 724)
                                  IDENTIFIER@[718; 724)  "errors"
                              L_SQUARE@[724; 725)  "["
                              LITERAL@[725; 726)
                                NUMBER_LITERAL@[725; 726)  "0"
                              R_SQUARE@[726; 727)  "]"
                            SEMICOLON@[727; 728)  ";"
                          WHITESPACE@[728; 737)
                          R_CURLY@[737; 738)  "}"
                      WHITESPACE@[738; 747)
                      EXPRESSION_STATEMENT@[747; 780)
                        ASSIGNMENT_EXPRESSION@[747; 779)
                          MEMBER_EXPRESSION@[747; 757)
                            THIS_EXPRESSION@[747; 751)
                              THIS_KW@[747; 751)  "this"
                            DOT@[751; 752)  "."
                            IDENTIFIER@[752; 757)
                              IDENTIFIER@[752; 757)  "party"
                          WHITESPACE@[757; 758)
                          EQ@[758; 759)  "="
                          WHITESPACE@[759; 760)
                          MEMBER_EXPRESSION@[760; 779)
                            MEMBER_EXPRESSION@[760; 769)
                              IDENTIFIER@[760; 764)
                                IDENTIFIER@[760; 764)  "resp"
                              DOT@[764; 765)  "."
                              IDENTIFIER@[765; 769)
                                IDENTIFIER@[765; 769)  "data"
                            DOT@[769; 770)  "."
                            IDENTIFIER@[770; 779)
                              IDENTIFIER@[770; 779)  "userParty"
                        SEMICOLON@[779; 780)  ";"
                      WHITESPACE@[780; 787)
                      R_CURLY@[787; 788)  "}"
                    WHITESPACE@[788; 789)
                    CATCH_CLAUSE@[789; 928)
                      CATCH_KW@[789; 794)  "catch"
                      WHITESPACE@[794; 795)
                      L_PAREN@[795; 796)  "("
                      IDENTIFIER@[796; 801)
                        IDENTIFIER@[796; 801)  "error"
                      R_PAREN@[801; 802)  ")"
                      WHITESPACE@[802; 803)
                      BLOCK_STATEMENT@[803; 928)
                        L_CURLY@[803; 804)  "{"
                        WHITESPACE@[804; 813)
                        EXPRESSION_STATEMENT@[813; 874)
                          CALL_EXPRESSION@[813; 873)
                            IDENTIFIER@[813; 818)
                              IDENTIFIER@[813; 818)  "alert"
                            L_PAREN@[818; 819)  "("
                            CONDITIONAL_EXPRESSION@[819; 872)
                              MEMBER_EXPRESSION@[819; 832)
                                IDENTIFIER@[819; 824)
                                  IDENTIFIER@[819; 824)  "error"
                                DOT@[824; 825)  "."
                                IDENTIFIER@[825; 832)
                                  IDENTIFIER@[825; 832)  "message"
                              WHITESPACE@[832; 833)
                              QUESTION@[833; 834)  "?"
                              WHITESPACE@[834; 835)
                              MEMBER_EXPRESSION@[835; 848)
                                IDENTIFIER@[835; 840)
                                  IDENTIFIER@[835; 840)  "error"
                                DOT@[840; 841)  "."
                                IDENTIFIER@[841; 848)
                                  IDENTIFIER@[841; 848)  "message"
                              WHITESPACE@[848; 849)
                              COLON@[849; 850)  ":"
                              WHITESPACE@[850; 851)
                              CALL_EXPRESSION@[851; 872)
                                MEMBER_EXPRESSION@[851; 865)
                                  IDENTIFIER@[851; 855)
                                    IDENTIFIER@[851; 855)  "JSON"
                                  DOT@[855; 856)  "."
                                  IDENTIFIER@[856; 865)
                                    IDENTIFIER@[856; 865)  "stringify"
                                L_PAREN@[865; 866)  "("
                                IDENTIFIER@[866; 871)
                                  IDENTIFIER@[866; 871)  "error"
                                R_PAREN@[871; 872)  ")"
                            R_PAREN@[872; 873)  ")"
                          SEMICOLON@[873; 874)  ";"
                        WHITESPACE@[874; 883)
                        EXPRESSION_STATEMENT@[883; 904)
                          CALL_EXPRESSION@[883; 903)
                            MEMBER_EXPRESSION@[883; 896)
                              IDENTIFIER@[883; 890)
                                IDENTIFIER@[883; 890)  "console"
                              DOT@[890; 891)  "."
                              IDENTIFIER@[891; 896)
                                IDENTIFIER@[891; 896)  "error"
                            L_PAREN@[896; 897)  "("
                            IDENTIFIER@[897; 902)
                              IDENTIFIER@[897; 902)  "error"
                            R_PAREN@[902; 903)  ")"
                          SEMICOLON@[903; 904)  ";"
                        WHITESPACE@[904; 913)
                        RETURN_STATEMENT@[913; 920)
                          RETURN_KW@[913; 919)  "return"
                          SEMICOLON@[919; 920)  ";"
                        WHITESPACE@[920; 927)
                        R_CURLY@[927; 928)  "}"
                  WHITESPACE@[928; 933)
                  R_CURLY@[933; 934)  "}"
            WHITESPACE@[934; 937)
            R_CURLY@[937; 938)  "}"
        WHITESPACE@[938; 939)
        R_CURLY@[939; 940)  "}"
      R_PAREN@[940; 941)  ")"
    SEMICOLON@[941; 942)  ";"
"#.trim()
    );
}
