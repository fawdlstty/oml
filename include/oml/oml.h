#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Try parse string and get oml-expr pointer
 */
int oml_expr_from_str(const char *psrc, void **ppexpr, const char **pperr);

int oml_expr_evalute(void *pexpr, const char *ppath, void **ppval, const char **pperr);

int oml_value_is_none(void *pval, const char *ppath);

int oml_value_is_bool(void *pval, const char *ppath);

int oml_value_as_bool(void *pval, const char *ppath);

int oml_value_is_int(void *pval, const char *ppath);

long long oml_value_as_int(void *pval, const char *ppath);

int oml_value_is_float(void *pval, const char *ppath);

double oml_value_as_float(void *pval, const char *ppath);

int oml_value_is_str(void *pval, const char *ppath);

const char *oml_value_as_str(void *pval, const char *ppath);

int oml_value_is_array(void *pval, const char *ppath);

int oml_value_get_array_length(void *pval, const char *ppath);

int oml_value_is_map(void *pval, const char *ppath);

int oml_value_get_map_length(void *pval, const char *ppath);

const char *oml_value_get_map_key(void *pval, const char *ppath, int index);

void oml_release_expr(const void *pexpr);

void oml_release_value(const void *pval);

void oml_release_str(const char *pstr);
