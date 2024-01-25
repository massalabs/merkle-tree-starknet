#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum CommandId {
  End = 0,
  Insert = 1,
  Remove = 2,
  Commit = 3,
  RootHash = 4,
} CommandId;

typedef enum TestId {
  Test1,
  Test2,
  Test3,
  Count,
} TestId;

typedef struct TestCases TestCases;

typedef struct TestCommand {
  enum CommandId command;
  const char *arg1;
  const char *arg2;
} TestCommand;

typedef struct TestCommandList {
  const struct TestCommand *test_commands;
  uintptr_t len;
} TestCommandList;

int32_t add(int32_t a, int32_t b);

char *concatenate_strings(const char *s1, const char *s2);

void free_concatenated_string(char *s);

void print_string(const char *s);

char *get_string(void);

const uint8_t *ffi_string(void);

struct TestCommandList get_test1(void);

struct TestCommandList get_test2(void);

void free_test(struct TestCommandList cmd);

struct TestCases get_test_cases(void);

struct TestCommandList get_test(enum TestId id);
