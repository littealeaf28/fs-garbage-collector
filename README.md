# FS Garbage Collector

## Purpose

The aim of FS Garbage Collector is to help notify users of any new temporarily needed files they created, but forgot to delete or organize.

In particular, FS Garbage Collector is a lightweight application that runs in the background to listen for new created files in directories that the user indicates where they will be storing temporary files.

(Also this is an excuse for me to learn about Rust).

## Use

Currently, I'm using this for my own sake, but if anyone finds this useful, feel free to reach out.

## Future Use

Right now, FS Garbage Collector relies on the user to make the decision of whether or not a file should be deleted or not. Perhaps it could be extended to incorporating machine learning or other AI techniques to recommend certain unused files to be removed.