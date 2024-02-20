#pragma once

#include "bindings.h"

typedef Bytes Key;
typedef Bytes Value;

/**
 * @brief Enum representing the result tag.
 *
 * This enum is used to indicate the result tag of an operation.
 * It can have two possible values: RESULT_OK and RESULT_ERR.
 */
typedef enum ResultStatus {
  RESULT_OK,
  RESULT_ERR,
} ResultStatus;

/**
 * @brief Represents the possible types of results for an operation.
 *
 * The `ResultOkType` enum defines the possible types of results for an
 * operation. It includes the following values:
 * - `OK_EMPTY`: Represents an empty result.
 * - `OK_VALUE`: Represents a result with a value.
 * - `OK_BOOL`: Represents a result with a boolean value.
 */
typedef enum ResultOkType {
  OK_EMPTY,
  OK_VALUE,
  OK_BOOL,
} ResultOkType;

/**
 * @brief Represents a successful result value in the `Result` type.
 *
 * The `ResultOkValue` struct is used to store the successful result value in
 * the `Result` type. It contains a tag indicating the type of the result value,
 * and a union that can hold different types of values.
 *
 * For the `get` and `root_hash` operations, the `value` field is used to store
 * the result value. For the `contains` operation, the `boolean` field is used
 * to store the result value. For the `insert` `remove` and `commit` as they
 * only return an empty result, the `tag` field is set to `OK_EMPTY`
 */
typedef struct ResultOkValue {
  ResultOkType type;
  union {
    Value value;  // used for get and root_hash
    bool boolean; // used for contains
  };
} ResultOkValue;

/**
 * @brief Represents the result of an operation.
 *
 * The `Result` struct is used to represent the result of an operation, which
 * can be either successful or contain an error. It consists of a `tag` field
 * indicating the type of result, and a `union` field that holds either the
 * successful value (`ok`) or the error message (`err`).
 */
typedef struct Result {
  ResultStatus status;
  union {
    ResultOkValue ok;
    const char *err;
  };
} Result;

#define OK_RESULT() ((Result){.status = RESULT_OK, {.ok = {.type = OK_EMPTY}}})
#define OK_BOOL_RESULT(val)                                                    \
  ((Result){.status = RESULT_OK, {.ok = {.type = OK_BOOL, {.boolean = (val)}}}})
#define OK_VALUE_RESULT(val)                                                   \
  ((Result){.status = RESULT_OK, {.ok = {.type = OK_VALUE, .value = (val)}}})

struct ITestTree;
/**
 * @brief Interface for a test tree.
 *
 * This interface defines the operations that can be performed on a test tree.
 * It provides methods for inserting, removing, getting, checking containment,
 * committing, and retrieving the root hash of the tree.
 */
typedef struct ITestTree {
  Result (*insert)(struct ITestTree *tree, const Key *key, const Value *value);
  Result (*remove)(struct ITestTree *tree, const Key *key);
  Result (*get)(struct ITestTree *tree, const Key *key);
  Result (*contains)(struct ITestTree *tree, const Key *key);
  Result (*commit)(struct ITestTree *tree);
  Result (*root_hash)(struct ITestTree *tree);
} ITestTree;

typedef ITestTree *(*TestTreeCreator)();

/**
 * Converts a Key object to a C-style string.
 *
 * @param key The Key object to convert.
 * @return A C-style string representation of the key.
 */
const char *key_to_cstring(const Key *key);

/**
 * Converts a Value object to a C-style string representation.
 *
 * @param value The Value object to convert.
 * @return A C-style string representation of the Value object.
 * @note The returned string is allocated on the heap and must be freed by the
 * caller.
 */
const char *value_to_cstring(const Value *value);

// test framework entry point
/**
 * Initializes the test runner, and runs the tests according to the given args.
 *
 * @param argc The number of command-line arguments.
 * @param argv An array of command-line argument strings.
 * @param creator A function pointer to the test tree creator.
 * @return An integer representing the status of the initialization.
 */
int init_runner(int argc, char *argv[], TestTreeCreator creator);