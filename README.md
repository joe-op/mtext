# mtext

Markup for generation of a book with TeX, or anything else.

## Emacs editing

Load the major mode by adding to Emacs:

```lisp
(add-to-list 'load-path ~/path/to/mtext-mode-directory)
(require 'mtext-mode)

### Collapsing paragraph whitespace

The `M-p` sequence is mapped to the function `mtext-paragraph-collapse`.

1. C+y to paste
2. C+x C+x to re-select pasted region
3. M-p to collapse whitespace in selected region
