#!/usr/bin/env perl

my $idle_seconds_command = 'echo $((`ioreg -c IOHIDSystem | sed -e \'/HIDIdleTime/ !{ d\' -e \'t\' -e \'}\' -e \'s/.* = //g\' -e \'q\'` / 1000000000))';

print "Counting seconds of inactivity... Command + Period (.) to quit\n\n";

do {

    my $idle_seconds = `$idle_seconds_command`;
    chomp($idle_seconds);
    print "Idle for $idle_seconds seconds.\n";

    sleep(1);

} while(1);
