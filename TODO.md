# TO DO

Update the design.  Currently, calculating the SHA of all files becomes expensive when the file set is large.  And it is unnecessary.

A file's unique value can be calculated as:

  - file size
  - a small sample of file contents
  - the sha256 digest of the full file

The cool part is that we only need to calculate the later values if there is a tie on the simpler.  So for example:

  - if no two files have the same size, that's
    all we need to know to say all are unique.
  - [optional] for any two that are the same size, we could
    sample a few bytes to see if those are the same.
  - if there is still a "tie", we should calculate the sha256 digest.
    if those match, the files can be considered identical.
