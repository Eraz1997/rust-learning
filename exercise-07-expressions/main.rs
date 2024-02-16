fn main () {
    /*

    EXPRESSIONS

     */

    let x = 5; // variable binding statement

    // Expression statements
    x;
    6;
    x + 1;

    // Block expressions
    let y = {
        x + 2;
        56;
        x - 3 // returns because there's no ";" => y = 2
    };
}