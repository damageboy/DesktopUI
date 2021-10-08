// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((guint) G_BINDING_BIDIRECTIONAL);
    PRINT_CONSTANT((guint) G_BINDING_DEFAULT);
    PRINT_CONSTANT((guint) G_BINDING_INVERT_BOOLEAN);
    PRINT_CONSTANT((guint) G_BINDING_SYNC_CREATE);
    PRINT_CONSTANT((guint) G_CONNECT_AFTER);
    PRINT_CONSTANT((guint) G_CONNECT_SWAPPED);
    PRINT_CONSTANT((guint) G_PARAM_CONSTRUCT);
    PRINT_CONSTANT((guint) G_PARAM_CONSTRUCT_ONLY);
    PRINT_CONSTANT((guint) G_PARAM_DEPRECATED);
    PRINT_CONSTANT((guint) G_PARAM_EXPLICIT_NOTIFY);
    PRINT_CONSTANT((guint) G_PARAM_LAX_VALIDATION);
    PRINT_CONSTANT(G_PARAM_MASK);
    PRINT_CONSTANT((guint) G_PARAM_PRIVATE);
    PRINT_CONSTANT((guint) G_PARAM_READABLE);
    PRINT_CONSTANT((guint) G_PARAM_READWRITE);
    PRINT_CONSTANT((guint) G_PARAM_STATIC_BLURB);
    PRINT_CONSTANT((guint) G_PARAM_STATIC_NAME);
    PRINT_CONSTANT((guint) G_PARAM_STATIC_NICK);
    PRINT_CONSTANT(G_PARAM_STATIC_STRINGS);
    PRINT_CONSTANT(G_PARAM_USER_SHIFT);
    PRINT_CONSTANT((guint) G_PARAM_WRITABLE);
    PRINT_CONSTANT((guint) G_SIGNAL_ACTION);
    PRINT_CONSTANT((guint) G_SIGNAL_DEPRECATED);
    PRINT_CONSTANT((guint) G_SIGNAL_DETAILED);
    PRINT_CONSTANT(G_SIGNAL_FLAGS_MASK);
    PRINT_CONSTANT((guint) G_SIGNAL_MATCH_CLOSURE);
    PRINT_CONSTANT((guint) G_SIGNAL_MATCH_DATA);
    PRINT_CONSTANT((guint) G_SIGNAL_MATCH_DETAIL);
    PRINT_CONSTANT((guint) G_SIGNAL_MATCH_FUNC);
    PRINT_CONSTANT((guint) G_SIGNAL_MATCH_ID);
    PRINT_CONSTANT(G_SIGNAL_MATCH_MASK);
    PRINT_CONSTANT((guint) G_SIGNAL_MATCH_UNBLOCKED);
    PRINT_CONSTANT((guint) G_SIGNAL_MUST_COLLECT);
    PRINT_CONSTANT((guint) G_SIGNAL_NO_HOOKS);
    PRINT_CONSTANT((guint) G_SIGNAL_NO_RECURSE);
    PRINT_CONSTANT((guint) G_SIGNAL_RUN_CLEANUP);
    PRINT_CONSTANT((guint) G_SIGNAL_RUN_FIRST);
    PRINT_CONSTANT((guint) G_SIGNAL_RUN_LAST);
    PRINT_CONSTANT((guint) G_TYPE_DEBUG_INSTANCE_COUNT);
    PRINT_CONSTANT((guint) G_TYPE_DEBUG_MASK);
    PRINT_CONSTANT((guint) G_TYPE_DEBUG_NONE);
    PRINT_CONSTANT((guint) G_TYPE_DEBUG_OBJECTS);
    PRINT_CONSTANT((guint) G_TYPE_DEBUG_SIGNALS);
    PRINT_CONSTANT((guint) G_TYPE_FLAG_ABSTRACT);
    PRINT_CONSTANT((guint) G_TYPE_FLAG_CLASSED);
    PRINT_CONSTANT((guint) G_TYPE_FLAG_DEEP_DERIVABLE);
    PRINT_CONSTANT((guint) G_TYPE_FLAG_DERIVABLE);
    PRINT_CONSTANT((guint) G_TYPE_FLAG_INSTANTIATABLE);
    PRINT_CONSTANT(G_TYPE_FLAG_RESERVED_ID_BIT);
    PRINT_CONSTANT((guint) G_TYPE_FLAG_VALUE_ABSTRACT);
    PRINT_CONSTANT(G_TYPE_FUNDAMENTAL_MAX);
    PRINT_CONSTANT(G_TYPE_FUNDAMENTAL_SHIFT);
    PRINT_CONSTANT(G_TYPE_RESERVED_BSE_FIRST);
    PRINT_CONSTANT(G_TYPE_RESERVED_BSE_LAST);
    PRINT_CONSTANT(G_TYPE_RESERVED_GLIB_FIRST);
    PRINT_CONSTANT(G_TYPE_RESERVED_GLIB_LAST);
    PRINT_CONSTANT(G_TYPE_RESERVED_USER_FIRST);
    PRINT_CONSTANT(G_VALUE_INTERNED_STRING);
    PRINT_CONSTANT(G_VALUE_NOCOPY_CONTENTS);
    return 0;
}
