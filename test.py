# https://pypi.org/project/pyobjc-framework-Quartz/

from Quartz import (
    CGEventSourceSecondsSinceLastEventType,
    kCGEventSourceStateCombinedSessionState,
    kCGAnyInputEventType
)
import time

while True:
    idle = CGEventSourceSecondsSinceLastEventType(
        kCGEventSourceStateCombinedSessionState,
        kCGAnyInputEventType
    );
    print("idle for " + str(idle));
    time.sleep(1)
