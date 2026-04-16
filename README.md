# Typelings

Advanced type-level programming exercises.

**Task**: Run tests and pass them.

## Initial and Final Encodings

Start with the initial encodings and then proceed onto the tagless final exercises.

Initial encoding is a pattern whereby we define EDSLs by their syntax. In Rust we can do that with enums/sum types.

Tagless final is a pattern whereby we define EDSLs by their semantics (i.e. trait interfaces) and leave them "tagless" as they have no syntactic "tag" associated with them.

There are advantages and disadvantages of both patterns which hopefully become clearer through the exercises.
