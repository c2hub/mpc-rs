#include <stdarg.h>
#include <stdlib.h>

#include "mpc.h"

typedef enum
{
	ok,
	err
} res_t;

typedef struct 
{
	mpc_ast_t* ok;
	mpc_err_t* err;
	res_t res;
} parse_result;

parse_result glue_parse(const char* filename, const char* string, mpc_parser_t* p)
{
	mpc_result_t r;
	if (mpc_parse(filename, string, p, &r))
		return (parse_result){ ok: r.output, err: NULL, res: ok };
	else
		return (parse_result){ ok: NULL, err: r.error, res: err};
}
