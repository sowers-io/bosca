Status: Alpha

This is a direct port of the TypeScript parser.  As such, it's not very "rusty".  And, probably has a memory leak somewhere.

But, it's good enough to run in the Bible processing workflow (so long as we don't allocate a bunch of them).

Once its feature set is mature (as in, it does reasonable things with multiple Bibles), cleanup will follow.