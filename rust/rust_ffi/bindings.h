#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum CommandId {
  End = 0,
  Insert = 1,
  Remove = 2,
  Commit = 3,
  CheckRootHash = 4,
} CommandId;

typedef enum TestId {
  Test1,
  Test2,
  Test3,
  Count,
} TestId;

typedef struct TestCases TestCases;

typedef struct Command {
  enum CommandId command;
  const char *arg1;
  const char *arg2;
} Command;

typedef struct CommandList {
  const struct Command *test_commands;
  uintptr_t len;
} CommandList;

int32_t add(int32_t a, int32_t b);

char *concatenate_strings(const char *s1, const char *s2);

void free_concatenated_string(char *s);

void print_string(const char *s);

char *get_string(void);

const uint8_t *ffi_string(void);

struct CommandList get_test1(void);

struct CommandList get_test2(void);

void free_test(struct CommandList cmd);

struct TestCases get_test_cases(void);

struct CommandList get_test(enum TestId id);
