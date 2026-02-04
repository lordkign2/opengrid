#ifndef TestStruct_H
#define TestStruct_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "TestStruct.d.h"






TestStruct* TestStruct_new(void);

uint32_t TestStruct_hello(const TestStruct* self);

void TestStruct_destroy(TestStruct* self);





#endif // TestStruct_H
