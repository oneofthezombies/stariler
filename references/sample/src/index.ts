const singleQuote = 'single quote';
const singleQuoteWithEscape = 'single quote with escape \'';
const doubleQuote = "double quote";
const doubleQuoteWithEscape = "double quote with escape \"";
const backtick = `backtick`;
const backtickWithEscape = `backtick with escape \``;
const backtickWithTemplate = `backtick with template ${singleQuote} ${doubleQuote} ${backtick}`;
const backtickWithMultiline = `backtick with multiline
    ${singleQuote}
    ${doubleQuote}
    ${backtick}`;
const regExp = /regexp/;
const regExpWithEscape = /regexp\//;
const regExpWithFlags = /regexp/gi;
const regExpWithFlagsAndSpaces = /reg exp/gi;
class A {
    foo() {
    }
};
const instance = new A();
instance .foo();
