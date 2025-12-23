# mtext

Markup for generation of a book with TeX, or anything else.

## Overview

A markup file consists of lines starting with a semantic name.
The name corresponds to a template, which the Rust program will look for
when it processes the line.

The line may continue with:

- Nothing - no parameters will be passed
- A string - a single parameter will be passed to the template, with a name indicating that it's the first parameter.
  The template can reassign the parameter to a variable with a more meaningful name.
- Multiple strings enclosed in brackets `[]` - each will be passed as a parameter with ordered names.  The number of parameters allowed will be limited.
- Multiple strings enclosed in brackets `[]`, interspersed with named parameters in the form `[:foo bar]`. Parameters without a name
  will be passed with ordered names (the count ignoring keyword parameters), while keyword parameters will be passed to the template by name.

Paragraph breaks are indicated by two (2) consecutive blank lines.  Paragraph breaks need a template.  The Rust program will look for the special
name `mtext-paragraph-break`.  This also means that a paragraph break can be inserted by starting a line with `mtext-paragraph-break`.

The semantic names used to start lines can consist of letters (not case sensitive), numbers, and dashes.  The only other limit to the names
is that you need to have a corresponding template defined.

### Counting occurrences

The Rust program will pass each template variables indicating whether this is the first, second, etc. time that this template has been
used.  If you would like your templates to be more independent, you can ignore this parameter and provide a custom parameter.

- `mtext_occurrence` - an integer representing the count of times this template has been used, starting at 1
- `mtext_occurrence_roman_numeral` - a Roman numeral representation of `mtext_occurrence`

#### Counting occurrences example

chapter.tera

```tera
Chapter {{ mtext_occurrence_roman_numeral }}
```

### Examples

```
text This is a line of text.
text This is the second line in this paragraph.


text This is a new paragraph.
quote This is text that I want formatted specially.
```

```
chapter The first chapter
subtitle A beginning
```

## Emacs editing

Load the major mode by adding to Emacs:

```lisp
(add-to-list 'load-path ~/path/to/mtext-mode-directory)
(require 'mtext-mode)
```

### Collapsing paragraph whitespace

The `M-p` sequence is mapped to the function `mtext-paragraph-collapse`.

1. C+y to paste
2. C+x C+x to re-select pasted region
3. M-p to collapse whitespace in selected region

## License

See [LICENSE.md](./LICENSE.md) for the license.

For the source of the MIT No AI license, see [bitmc/mit-no-ai-license](https://github.com/bmitc/mit-no-ai-license),
commit [2ebfc3f1](https://github.com/bmitc/mit-no-ai-license/commit/2ebfc3f1347268f392147b42b3bdc7dd54963c25).