#include "../../rust/rust_ffi/bindings.h"
#include <dlfcn.h>
#include <stdio.h>

int main()
{
  // Call the function `add`
  int result = add(35, 34);
  printf("Result from external addition of 35 and 34: %d\n", result);

  // Call and concat
  char *s1 = "Hello, ";
  char *s2 = "world!";
  char *concatenated = concatenate_strings(s1, s2);
  printf("%s\n", concatenated);
  free_concatenated_string(concatenated);

  // VecCommands *leaked = leak();
  // printf("oh\n");
  // if (leaked != NULL) {
  //   printf("hi\n");
  //   struct TestCommand *commands = (struct TestCommand *)leaked->commands;
  //   CommandId *cmd = (CommandId *)commands->command;
  //   printf(">enum: %d\n", *cmd);
  //   printf("#enum: %d\n", *(CommandId *)commands->command);
  //   printf("Commands: %p\n", commands);
  //   printf("len %zu\n", leaked->len);

  //   for (size_t i = 0; i < leaked->len; ++i) {
  //     struct TestCommand *command = &commands[i];
  //     printf("Command: %d, Arg1: %s, Arg2: %s\n", command->command,
  //            command->arg1, command->arg2);
  //   }
  /*    for (size_t i = 0; i < leaked->len; ++i) {
       printf("i: %zu\n", i);
       printf("Command P: %p\n", &commands[i]);


       Accéder aux éléments de la structure TestCommand
       Vous pouvez utiliser command->command, command->arg1, command->arg2 ici
       printf("Command D: %d\n", *(CommandId *)commands[i].command);
       if (commands[i].arg1 != NULL && commands[i].arg2 != NULL) {
         printf("not null\n");
         Accéder aux éléments de la structure TestCommand
         printf("Arg1: %s, Arg2: %s\n", commands[i].arg1, commands[i].arg2);
       } else {
         printf("Error: NULL pointer found in TestCommand\n");
       }

       printf("Arg1: %s, Arg2: %s\n",  command.arg1, command.arg1);

     }
  */
  //   destroy_leak(leaked);
  // }

  TestCommandList test2 = get_test2();
  printf("len: %zu\n", test2.len);

  for (size_t i = 0; i < test2.len; i++)
  {
    printf("C FILE i: %zu\n", i);
    struct TestCommand cur_command = test2.test_commands[i];
    printf("C FILE cur command id %d\n", cur_command.command);
    printf("C FILE cur command arg1 %s\n", cur_command.arg1);
    printf("C FILE cur command arg2 %s\n", cur_command.arg2);
  }

  free_test(test2);

  return 0;
}