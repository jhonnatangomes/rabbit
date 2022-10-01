import { inspect } from 'util';
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
  lineNumber: number;
  columnNumber: number;
  value?: string;
};

const OPERATORS = {
  PLUS: '+',
  MINUS: '-',
  TIMES: '*',
  DIVIDES: '/',
} as const;

type PrimitiveOperator = typeof OPERATORS[keyof typeof OPERATORS];

type StatementType = 'Expression' | 'IntegerLiteral';

type Statement<T extends StatementType> = {
  type: T;
};

type IntegerLiteral = Statement<'IntegerLiteral'> & {
  value: number;
};

type ExpressionStatement = Statement<'Expression'> & {
  operator: PrimitiveOperator;
  arguments: (ExpressionStatement | IntegerLiteral)[];
};

type Ast = {
  type: 'program';
  body: ExpressionStatement[];
};

function lexer(input: string): Token[] {
  const lines = input.split('\n');
  const { LPAREN, RPAREN, OPERATOR, INTEGER_LITERAL } = TokenType;
  return lines.flatMap((line, i) => {
    let currentIndex = 0;
    const tokens: Token[] = [];
    let char = line[currentIndex];
    while (currentIndex < line.length) {
      if (char === '(') {
        tokens.push({
          type: LPAREN,
          lineNumber: i + 1,
          columnNumber: currentIndex + 1,
        });
        advanceIndex();
        continue;
      }
      if (char === ')') {
        tokens.push({
          type: RPAREN,
          lineNumber: i + 1,
          columnNumber: currentIndex + 1,
        });
        advanceIndex();
        continue;
      }
      if (isPrimitiveOperator(char)) {
        tokens.push({
          type: OPERATOR,
          value: char,
          lineNumber: i + 1,
          columnNumber: currentIndex + 1,
        });
        advanceIndex();
        continue;
      }
      if (isIntegerLiteral(char)) {
        let value = '';
        const initialColumnNumber = currentIndex + 1;
        while (isIntegerLiteral(char)) {
          value += char;
          advanceIndex();
        }
        tokens.push({
          type: INTEGER_LITERAL,
          value,
          lineNumber: i + 1,
          columnNumber: initialColumnNumber,
        });
        continue;
      }
      if (isWhitespace(char)) {
        advanceIndex();
        continue;
      }
      let value = '';
      const initialColumnNumber = currentIndex + 1;
      while (currentIndex < input.length && !isWhitespace(char)) {
        value += char;
        advanceIndex();
      }
      throw new Error(
        `Unexpected token ${value} encountered in line ${
          i + 1
        }, column ${initialColumnNumber}`
      );
    }
    return tokens;
    function advanceIndex() {
      char = line[++currentIndex];
    }
  });
}

function parser(tokens: Token[]) {
  let currentIndex = 0;
  const { LPAREN, INTEGER_LITERAL, RPAREN } = TokenType;
  let token = tokens[currentIndex];
  function walk(): ExpressionStatement | IntegerLiteral {
    if (token.type === INTEGER_LITERAL) {
      return {
        type: 'IntegerLiteral',
        value: parseInt(token.value),
      };
    }
    if (token.type === LPAREN) {
      advanceToken();
      if (!isPrimitiveOperator(token.value)) {
        throw new Error(
          `Expected operator in line ${token.lineNumber}, column ${token.columnNumber}`
        );
      }
      const node: ExpressionStatement = {
        type: 'Expression',
        operator: token.value,
        arguments: [],
      };
      advanceToken();
      //@ts-ignore
      while (currentIndex < tokens.length && token.type !== RPAREN) {
        node.arguments.push(walk());
        //@ts-ignore
        if (token.type !== RPAREN) {
          advanceToken();
        }
      }
      advanceToken();
      return node;
    }
    console.log({ token });
    throw new Error(
      `Unexpected token ${token.value || token.type} in line ${
        token.lineNumber
      }, column ${token.columnNumber}`
    );
  }

  const ast: Ast = {
    type: 'program',
    body: [],
  };
  while (currentIndex < tokens.length) {
    const statement = walk();
    if (statement.type !== 'Expression') {
      throw new Error(
        `line ${token.lineNumber} should start with an expression`
      );
    }
    ast.body.push(statement);
  }
  return ast;
  // function peekToken() {
  //   return tokens[currentIndex + 1];
  // }
  function advanceToken() {
    token = tokens[++currentIndex];
  }
}
function evaluator(ast: Ast) {
  function traverse(node: ExpressionStatement | IntegerLiteral): number {
    if (node.type === 'IntegerLiteral') {
      return node.value;
    }
    if (node.type === 'Expression') {
      if (node.operator === '+') {
        return node.arguments.reduce((prev, curr) => prev + traverse(curr), 0);
      }
      if (node.operator === '-') {
        const numbers = node.arguments.map(traverse);
        return numbers.slice(1).reduce((prev, curr) => prev - curr, numbers[0]);
      }
      if (node.operator === '*') {
        return node.arguments.reduce((prev, curr) => prev * traverse(curr), 1);
      }
      if (node.operator === '/') {
        const numbers = node.arguments.map(traverse);
        return numbers.slice(1).reduce((prev, curr) => prev / curr, numbers[0]);
      }
    }
    throw new Error(`Invalid node type ${node.type}`);
  }
  return ast.body.map(traverse);
}

function isPrimitiveOperator(input: string): input is PrimitiveOperator {
  return Object.values<string>(OPERATORS).includes(input);
}
function isWhitespace(input: string) {
  return WHITESPACE.test(input);
}
function isIntegerLiteral(input: string) {
  const DIGIT = /[0-9]/;
  return DIGIT.test(input);
}
const a = lexer(`
(+ 1 (* 5 3))
(- 10 5)
`);
const b = parser(a);
const c = evaluator(b);
console.log(a);
console.log(inspect(b, { depth: null }));
console.log(c);
