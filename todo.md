1. TODO: make parser_error! return a token that will be used to fit in before the error token based on expects.
    - Note: needs to be ordered by priority so doesn't do infinite loop.
    - Ex: Got Semi but expected Comma or list end token. If comma is picked, then it will add an expression, then comma again. list end token should be higher priority.
2. block, block_expr, fn_decl_stmt
