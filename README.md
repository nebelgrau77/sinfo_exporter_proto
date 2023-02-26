### SLURM GPU data exporter prototype

Right now used "fakesinfo", which outputs data in the format of `sinfo -O GresUsed` on localhost:9199/metrics.

#### TO DO:
- replace `fakesinfo` with the `sinfo -O GresUsed` command
- correctly handle errors if that is not present, or the output is different (e.g. unknown GPUs)
- accept port as argument
- accept interval in miliseconds or seconds 

