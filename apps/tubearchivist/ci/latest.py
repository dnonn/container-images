#!/usr/bin/env python

def get_latest(channel):
    return "0.4.10"

if __name__ == "__main__":
    import sys
    channel = sys.argv[1]
    print(get_latest(channel))
