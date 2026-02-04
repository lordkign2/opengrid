#ifndef OpenGridEngine_H
#define OpenGridEngine_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "NodeHandle.d.h"

#include "OpenGridEngine.d.h"






OpenGridEngine* OpenGridEngine_new(void);

NodeHandle* OpenGridEngine_create_node(const OpenGridEngine* self, DiplomatStringView _name);

void OpenGridEngine_destroy(OpenGridEngine* self);





#endif // OpenGridEngine_H
