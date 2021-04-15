// clang -o reset_idle_time reset_idle_time.c -Wall -O3 -framework CoreFoundation -framework IOKit


#include <stdio.h>
#include <stdint.h>
#include <mach/mach.h>
#include <CoreFoundation/CoreFoundation.h>

extern const mach_port_t kIOMasterPortDefault;

typedef mach_port_t io_object_t;
typedef io_object_t io_service_t;
typedef io_object_t io_connect_t;

kern_return_t IOObjectRelease(io_object_t object);
CFMutableDictionaryRef IOServiceMatching(const char *name) CF_RETURNS_RETAINED;
io_service_t IOServiceGetMatchingService(mach_port_t master, CFDictionaryRef matching CF_RELEASES_ARGUMENT);
kern_return_t IOServiceOpen(io_service_t service, task_t task, uint32_t type, io_connect_t *client);
kern_return_t IOServiceClose(io_connect_t client);
kern_return_t IOConnectCallScalarMethod(io_connect_t client, uint32_t selector, const uint64_t *in, uint32_t inCnt, uint64_t *out, uint32_t *outCnt);

const uint32_t kIOHIDParamConnectType             = 1;
const uint32_t kIOHIDActivityUserIdle             = 3;
const uint32_t kIOHIDActivityReport               = 0;
const uint32_t kIOHIDParam_extSetStateForSelector = 6;

#define LOG(str, args...) do { fprintf(stderr, str "\n", ##args); } while(0)

int hid_reset(void)
{
    int retval = -1;
    kern_return_t ret = 0;
    io_service_t service = MACH_PORT_NULL;
    io_connect_t client = MACH_PORT_NULL;

    service = IOServiceGetMatchingService(kIOMasterPortDefault, IOServiceMatching("IOHIDSystem"));
    LOG("service: %x", service);
    if(!MACH_PORT_VALID(service)) goto out;

    ret = IOServiceOpen(service, mach_task_self(), kIOHIDParamConnectType, &client);
    LOG("client: %x, %s", client, mach_error_string(ret));
    if(ret != KERN_SUCCESS || !MACH_PORT_VALID(client)) goto out;

    ret = IOHIDSetStateForSelector(client, 3, 0);
    LOG("extSetStateForSelector: %s", mach_error_string(ret));
    if(ret != KERN_SUCCESS) {
        LOG("err");
        goto out;
    }

    retval = 0;

out:;
    if(MACH_PORT_VALID(client)) IOServiceClose(client);
    if(MACH_PORT_VALID(service)) IOObjectRelease(service);
    return retval;
}

int main(void)
{
    return hid_reset();
}