#ifndef NodeHandle_H
#define NodeHandle_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "NodeHandle.d.h"






NodeHandle* NodeHandle_new_ephemeral(void);

uint64_t NodeHandle_submit_event(NodeHandle* self, DiplomatU8View payload);

uint64_t NodeHandle_current_version(const NodeHandle* self);

void NodeHandle_destroy(NodeHandle* self);





#endif // NodeHandle_H
