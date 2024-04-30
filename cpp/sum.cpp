#include <cstdio>
#include <cstring>

#include <string>

extern "C" int sum(int a, int b) { return a + b; }

extern "C" double doubleSum(double a, double b) { return a + b; }

extern "C" const char *concatenateStrings(const char *str1, const char *str2) {
  std::string result = std::string(str1) + std::string(str2);
  char *cstr = new char[result.length() + 1];
  strcpy(cstr, result.c_str());
  return cstr;
}

extern "C" void noRet() { printf("%s", "hello world\n"); }

extern "C" int *createArrayi32(const int *arr, int size) {
  int *vec = (int *)malloc((size) * sizeof(int));
  for (int i = 0; i < size; i++) {
    vec[i] = arr[i];
  }
  return vec;
}

extern "C" double *createArrayDouble(const double *arr, int size) {
  double *vec = (double *)malloc((size) * sizeof(double));
  for (int i = 0; i < size; i++) {
    vec[i] = arr[i];
  }
  return vec;
}

extern "C" char **createArrayString(char **arr, int size) {
  char **vec = (char **)malloc((size) * sizeof(char *));
  for (int i = 0; i < size; i++) {
    vec[i] = arr[i];
  }
  return vec;
}

extern "C" bool return_opposite(bool input) { return !input; }

typedef struct Person {
  double *doubleArray;
  int age;
  double doubleProps;
  const char *name;
  char **stringArray;
  int *i32Array;
  int testnum;
  bool boolTrue;
  bool boolFalse;
  // struct Person *parent; // 新增parent字段
} Person;
void printPersonAttributes(Person *person) {
  printf("age: %d\n", person->age);
  printf("doubleProps: %f\n", person->doubleProps);
  printf("name: %s\n", person->name);
  printf("testnum: %d\n", person->testnum);
  printf("boolTrue: %d\n", person->boolTrue);
  printf("boolFalse: %d\n", person->boolFalse);
}
extern "C" Person *getStruct(Person *person) {
  printPersonAttributes(person);
  // printPersonAttributes(person->parent);
  return person;
}

extern "C" Person *createPerson() {
  Person *person = (Person *)malloc(sizeof(Person));

  // Allocate and initialize doubleArray
  person->doubleArray = (double *)malloc(sizeof(double) * 3);
  person->doubleArray[0] = 1.0;
  person->doubleArray[1] = 2.0;
  person->doubleArray[2] = 3.0;

  // Initialize age and doubleProps
  person->age = 30;
  person->doubleProps = 1.23;

  // Allocate and initialize name
  person->name = strdup("John Doe");

  person->stringArray = (char **)malloc(sizeof(char *) * 2);
  person->stringArray[0] = strdup("Hello");
  person->stringArray[1] = strdup("World");

  person->i32Array = (int *)malloc(sizeof(int) * 3);
  person->i32Array[0] = 1;
  person->i32Array[1] = 2;
  person->i32Array[2] = 3;
  person->testnum = 123;
  person->boolTrue = true;
  person->boolFalse = false;

  // Allocate and initialize parent
  // person->parent = (Person *)malloc(sizeof(Person));
  // person->parent->doubleArray = (double *)malloc(sizeof(double) * 3);
  // person->parent->doubleArray[0] = 4.0;
  // person->parent->doubleArray[1] = 5.0;
  // person->parent->doubleArray[2] = 6.0;
  // person->parent->age = 50;
  // person->parent->doubleProps = 4.56;
  // person->parent->name = strdup("Jane Doe");
  // person->parent->stringArray = (char **)malloc(sizeof(char *) * 2);
  // person->parent->stringArray[0] = strdup("Parent");
  // person->parent->stringArray[1] = strdup("String");
  // person->parent->i32Array = (int *)malloc(sizeof(int) * 3);
  // person->parent->i32Array[0] = 4;
  // person->parent->i32Array[1] = 5;
  // person->parent->i32Array[2] = 6;
  // person->parent->testnum = 456;
  // person->parent->boolTrue = true;
  // person->parent->boolFalse = false;

  return person;
}
typedef void (*FunctionPointer)(int a, bool b, char *c, char **d, int *e,
                                Person *p);

extern "C" void callFunction(FunctionPointer func) {
  printf("callFunction\n");

  for (int i = 0; i < 2; i++) {
    int a = 100;
    bool b = false;
    double ddd = 100.11;
    char *c = (char *)malloc(14 * sizeof(char));
    strcpy(c, "Hello, World!");

    char **stringArray = (char **)malloc(sizeof(char *) * 2);
    stringArray[0] = strdup("Hello");
    stringArray[1] = strdup("world");

    int *i32Array = (int *)malloc(sizeof(int) * 3);
    i32Array[0] = 101;
    i32Array[1] = 202;
    i32Array[2] = 303;

    Person *p = createPerson();
    func(a, b, c, stringArray, i32Array, p);
  }
}

extern "C" void bufferToFill(double bufferToFill[3]) {
  bufferToFill[0] = -0.5;
  bufferToFill[1] = 7.5;
  bufferToFill[2] = 3;
  printf("%f", bufferToFill[0]);
}

// typedef void (*CallbackType)(const char *);
// extern "C" void call_callback_async() {
//   dispatch_async(dispatch_get_main_queue(), ^{
//     printf("dispatch_async\n");
//     // callback("Hello from dispatched block");
//   });
//   // dispatch_main();
// }
// int call_callback_async(CallbackType callback) {
//   std::async(std::launch::async, [=]() { callback("Hello from async task");
//   }); return 0;
// }
