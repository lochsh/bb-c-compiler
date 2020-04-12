#include <stdbool.h>
#include <stdint.h>

void led_toggle() {
    while (true) {
        volatile uint32_t* bsrr = (uint32_t*)0x40020018;
        *bsrr = 0x100;
        *bsrr = 0x1000000;
    }
}
