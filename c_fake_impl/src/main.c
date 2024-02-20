#include "test_tree.h"
#include <dlfcn.h>
#include <stdio.h>

// Define a struct that implements the ITestTree
typedef struct {
  ITestTree interface;
  // Other fields for the implementation
  // for this fake implementation we just put a counter that will be incremented
  // by all fake function implementations
  size_t counter;
} FakeTestTree;

static FakeTestTree *to_impl(ITestTree *self) { return (FakeTestTree *)self; }

// Implement the functions for the FakeTestTree struct
Result fake_test_tree_insert(ITestTree *tree, const Key *key,
                             const Value *value) {
  FakeTestTree *impl = to_impl(tree);

  const char *key_str = key_to_cstring(key);
  const char *value_str = value_to_cstring(value);
  printf("Insert operation: %zu, %s, %s\n", impl->counter, key_str, value_str);
  free((void *)key_str);

  impl->counter++;

  return OK_RESULT();
}

Result fake_test_tree_remove(ITestTree *tree, const Key *key) {
  FakeTestTree *impl = to_impl(tree);

  const char *key_str = key_to_cstring(key);

  printf("Remove operation: %zu, %s\n", impl->counter, key_str);
  free((void *)key_str);

  impl->counter++;

  return OK_RESULT();
}

Result fake_test_tree_get(ITestTree *tree, const Key *key) {
  FakeTestTree *impl = to_impl(tree);

  const char *key_str = key_to_cstring(key);
  printf("Get operation: %zu, %s\n", impl->counter, key_str);
  free((void *)key_str);

  impl->counter++;

  Value res = {0, 0};
  return OK_VALUE_RESULT(res);
}

Result fake_test_tree_contains(ITestTree *tree, const Key *key) {
  FakeTestTree *impl = to_impl(tree);

  const char *key_str = key_to_cstring(key);
  printf("Contains operation: %zu, %s\n", impl->counter, key_str);
  free((void *)key_str);

  impl->counter++;

  bool res = true;
  return OK_BOOL_RESULT(res);
}

Result fake_test_tree_commit(ITestTree *tree) {
  FakeTestTree *impl = to_impl(tree);

  printf("Commit operation: %zu\n", impl->counter);

  impl->counter++;

  return OK_RESULT();
}

Result fake_test_tree_root_hash(ITestTree *tree) {
  FakeTestTree *impl = to_impl(tree);

  printf("Root hash operation: %zu\n", impl->counter);

  impl->counter++;

  Value res = {0, 0};
  return OK_VALUE_RESULT(res);
}

// Create a function that creates a FakeTestTree and conforms to this type:
// typedef ITestTree *(*TestTreeCreator)();
ITestTree *create_fake_test_tree() {

  FakeTestTree *tree = malloc(sizeof(FakeTestTree));
  if (tree == NULL) {
    printf("Error: malloc failed\n");
    exit(1);
  }

  // populate the Interface
  tree->interface.insert = fake_test_tree_insert;
  tree->interface.remove = fake_test_tree_remove;
  tree->interface.get = fake_test_tree_get;
  tree->interface.contains = fake_test_tree_contains;
  tree->interface.commit = fake_test_tree_commit;
  tree->interface.root_hash = fake_test_tree_root_hash;

  // initialize the operation counter aka fake tree
  tree->counter = 1;

  return (ITestTree *)tree;
}

int main(int argc, char *argv[]) {
  return init_runner(argc, argv, create_fake_test_tree);
}