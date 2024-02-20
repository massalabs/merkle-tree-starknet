#include "test_tree.h"
#include <assert.h>
#include <dirent.h>
#include <libgen.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

/**
 * Converts a Key to a C-style string representation.
 *
 * @param key The Key to convert.
 * @return A C-style string representation of the Key. The string is allocated
 *         dynamically and have to be freed by the caller using free().
 */
const char *key_to_cstring(const Key *key) {
  // key is an array or bytes.
  // key.len is the length of the array and is at most 32
  // we want to print the key as follows: [12,34,56,100,...], each number is a
  // byte in the array and is printed in decimal
  // hence we need to allocate 4 bytes for each byte in the array, plus 2 bytes
  // (for [ and ]). we also need to add 1 byte for the null terminator
  // so the total length of the string is 4 * key->len + 3

  char *str = malloc(4 * key->len + 3);

  if (str == NULL) {
    return NULL;
  }

  const char *const ret = str;
  str[0] = '[';

  str += 1;
  for (size_t i = 0; i < key->len; i++) {
    // print the number
    const char *FORMAT = "%d,";
    const int FORMAT_LEN = 4;

    str += snprintf(str, FORMAT_LEN, FORMAT, key->ptr[i]);
  }
  // overwrite the last comma with a closing bracket
  str[-1] = ']';
  str[0] = '\0';

  return ret;
}

/**
 * Converts a Value object to a C-style string.
 *
 * @param value The Value object to convert.
 * @return The C-style string representation of the Value object.
 */
const char *value_to_cstring(const Value *value) {
  // value is an array of bytes that contains either the ascii representation in
  // hex of a number, a hash, that is at most 32 bytes long, or a string
  // representing a boolean value ("true" or "false")
  // hence no conversion should be needed.
  // we just need to check that value.ptr is null terminated, that
  // strlen(value.ptr) == value.len then return value.ptr

  if (value == NULL || value->ptr == NULL) {
    return NULL;
  }

  assert(value->ptr[value->len-1] == '\0');

  return (const char *)value->ptr;
}

typedef struct {
  char *scenario_dir;
  char *scenario_file;
} Cli;

static void start_test(const char *path, const char *filename,
                       TestTreeCreator test_env_creator);
static int run_test(const CommandList *command_list, ITestTree *tree);

static void print_usage(const char *progname) {
  fprintf(stderr, "Usage: %s [-d SCENARIO_DIR_PATH] [-f SCENARIO_FILE_PATH]\n",
          progname);
  exit(EXIT_FAILURE);
}

/**
 * Parses the command-line arguments and returns a Cli struct.
 *
 * @param argc The number of command-line arguments.
 * @param argv An array of strings representing the command-line arguments.
 * @return A Cli struct containing the parsed command-line arguments.
 */
static Cli parse_cli(int argc, char *argv[]) {
  Cli cli = {NULL, NULL};

  int opt;
  while ((opt = getopt(argc, argv, "d:f:")) != -1) {
    switch (opt) {
    case 'd':
      cli.scenario_dir = optarg;
      break;
    case 'f':
      cli.scenario_file = optarg;
      break;
    default:
      print_usage(argv[0]);
    }
  }

  if (cli.scenario_dir == NULL && cli.scenario_file == NULL) {
    print_usage(argv[0]);
  }

  if (cli.scenario_dir != NULL) {
    printf("scenario_dir: %s\n", cli.scenario_dir);
  }

  if (cli.scenario_file != NULL) {
    printf("scenario_file: %s\n", cli.scenario_file);
  }

  return cli;
}

/**
 * Initializes the runner with the specified arguments and test tree creator.
 *
 * @param argc The number of command-line arguments.
 * @param argv An array of command-line argument strings.
 * @param creator The test tree creator function.
 * @return An integer indicating the success or failure of the initialization.
 */
int init_runner(int argc, char *argv[], TestTreeCreator creator) {
  Cli config = parse_cli(argc, argv);

  if (config.scenario_dir != NULL) {
    struct dirent *entry;
    DIR *dir = opendir(config.scenario_dir);
    while ((entry = readdir(dir)) != NULL) {
      if (entry->d_type == DT_REG) {
        char *dot = strrchr(entry->d_name, '.');
        if (dot != NULL && strcmp(dot, ".yml") == 0) {
          char path[1024];
          snprintf(path, sizeof(path), "%s/%s", config.scenario_dir,
                   entry->d_name);
          start_test(path, entry->d_name, creator);
        }
      }
    }
    closedir(dir);
  } else if (config.scenario_file != NULL) {
    char *dot = strrchr(config.scenario_file, '.');
    if (dot != NULL && strcmp(dot, ".yml") == 0) {
      start_test(config.scenario_file, basename(config.scenario_file), creator);
    }
  }

  return 0;
}

/**
 * Starts a test with the given path and filename.
 *
 * @param path The path of the test.
 * @param filename The filename of the test.
 */
static void start_test(const char *path, const char *filename,
                       TestTreeCreator test_env_creator) {
  ITestTree *test_env = test_env_creator();

  CommandList command_list = load_scenario(path);

  if (run_test(&command_list, test_env) != 0) {
    printf("Test %s %s\n", filename, "FAIL");
  } else {
    printf("Test %s %s\n", filename, "SUCCESS");
  }

  free_scenario(command_list);
  free(test_env);
}

/**
 * Executes the specified command and updates the test tree.
 *
 * @param command The command to be executed.
 * @param tree The test tree to be updated.
 * @return The result of the command execution.
 */
static int run_command(const Command *command, ITestTree *tree) {
  Result res;

  switch (command->id) {

  case Insert: {
    res = tree->insert(tree, &command->arg1, &command->arg2);
    break;
  }

  case Remove: {
    res = tree->remove(tree, &command->arg1);
    break;
  }

  case CheckRootHash: {
    res = tree->root_hash(tree);
    switch (res.status) {
    case RESULT_OK:
      if (command->arg1.len != res.ok.value.len ||
          memcmp(command->arg1.ptr, res.ok.value.ptr, res.ok.value.len) != 0) {
        printf("Error: Expected root hash %s, got %s\n",
               key_to_cstring(&command->arg1), key_to_cstring(&res.ok.value));
        return -1;
      }
      return 0;
    case RESULT_ERR:
      printf("Error: %s\n", res.err);
      return -1;
    }

    break;
  }

  case Get: {
    res = tree->get(tree, &command->arg1);

    switch (res.status) {
    case RESULT_OK:
      if (command->arg2.len != res.ok.value.len ||
          memcmp(command->arg2.ptr, res.ok.value.ptr, res.ok.value.len) != 0) {
        printf("Error: Expected value %s, got %s\n",
               value_to_cstring(&command->arg2),
               value_to_cstring(&res.ok.value));
        return -1;
      }
      return 0;
    case RESULT_ERR:
      printf("Error: %s\n", res.err);
      return -1;
    }

    break;
  }

  case Contains: {
    res = tree->contains(tree, &command->arg1);
    switch (res.status) {
    case RESULT_OK:
      if (res.ok.value.len != command->arg2.len ||
          memcmp(command->arg2.ptr, res.ok.value.ptr, res.ok.value.len) != 0) {
        printf("Error: Expected %s, got %s\n",
               command->arg2.len == 1 ? "true" : "false",
               value_to_cstring(&res.ok.value));
        return -1;
      }
      return 0;
    case RESULT_ERR:
      printf("Error: %s\n", res.err);
      return -1;
    }
    break;
  }

  default:
    printf("Error: Unknown command id: %d\n", command->id);
    return -1;
  }

  switch (res.status) {
  case RESULT_OK:
    return 0;
  case RESULT_ERR:
    printf("Error: %s\n", res.err);
    return -1;
  }

  return -1;
}

/**
 * Runs a test using the provided command list and test tree.
 *
 * @param command_list The command list to be used for the test.
 * @param tree The test tree to be used for the test.
 * @return An integer representing the result of the test.
 */
static int run_test(const CommandList *command_list, ITestTree *tree) {
  for (size_t i = 0; i < command_list->len; i++) {
    const Command *command = &(command_list->test_commands[i]);
    if (run_command(command, tree) != 0) {
      return -1;
    }
  }

  return 0;
}
