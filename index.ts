const IDENTIFIERS = /[0-9a-zA-Z?!_\-.]*/;
const WHITESPACE = /\s/;

enum TokenType {
  'LPAREN' = '(',
  'RPAREN' = ')',
  'OPERATOR' = 'OP',
  'INTEGER_LITERAL' = 'INT',
}

type Token = {
  type: TokenType;
  value?: string;
};

const OPERATORS = {
  PLUS: '+',
  MINUS: '-',
  TIMES: '*',
  DIVIDES: '/',
};

type PrimitiveOperator = typeof OPERATORS[keyof typeof OPERATORS];

function lexer(input: string): Token[] {
  let currentIndex = 0;
  const tokens: Token[] = [];
  const { LPAREN, RPAREN, OPERATOR, INTEGER_LITERAL } = TokenType;
  let char = input[currentIndex];
  while (currentIndex < input.length) {
    if (char === '(') {
      tokens.push({ type: LPAREN });
      advanceIndex();
      continue;
    }
    if (char === ')') {
      tokens.push({ type: RPAREN });
      advanceIndex();
      continue;
    }
    if (isPrimitiveOperator(char)) {
      tokens.push({ type: OPERATOR, value: char });
      advanceIndex();
      continue;
    }
    if (isIntegerLiteral(char)) {
      let value = '';
      while (isIntegerLiteral(char)) {
        value += char;
        advanceIndex();
      }
      tokens.push({ type: INTEGER_LITERAL, value });
      continue;
    }
    if (isWhitespace(char)) {
      advanceIndex();
      continue;
    }

    throw new Error(`Unexpected token ${char} encountered`);
  }
  return tokens;
  function advanceIndex() {
    char = input[++currentIndex];
  }
}

function isPrimitiveOperator(input: string) {
  return Object.values(OPERATORS).includes(input);
}
function isWhitespace(input: string) {
  return WHITESPACE.test(input);
}
function isIntegerLiteral(input: string) {
  const DIGIT = /[0-9]/;
  return DIGIT.test(input);
}
console.log(lexer('(+    2122 192192)'));
