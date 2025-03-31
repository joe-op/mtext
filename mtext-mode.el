;;;; mtext Emacs mode
;;;; https://www.emacswiki.org/emacs/ModeTutorial

(defvar mtext-mode-hook nil)

(add-to-list 'auto-mode-alist '("\\.mtxt\\'" . mtext-mode))
(add-to-list 'auto-mode-alist '("\\.mtext\\'" . mtext-mode))

;;; keymap
(defun mtext-paragraph-collapse ()
  (interactive)
  "Collapse whitespace in the highlighed paragraph"
  (if (use-region-p)
      (progn
	(replace-regexp "\\([[:space:]]\\|\n\\)+" " " nil (region-beginning) (region-end)))))

(defvar mtext-mode-map
  (let ((map (make-keymap)))
    (define-key map "\M-p" 'mtext-paragraph-collapse)
    map)
  "Keymap for MTEXT major mode")

;;; highlighting
;; Pattern to allow changing the level of highlighting
;; '1' represents the minimum level
(defconst mtext-font-lock-keywords-1
  (list
   '("^[A-Za-z0-9\\-]+" . font-lock-doc-markup-face)
   '("\\[\\(\\:[A-Za-z0-9\\-]+\\)" . (1 font-lock-keyword-face))
   '("\\<\\(true\\|false\\)\\>" . font-lock-constant-face))
  "Highlight semantic names in mtext mode")

;; Now select level 1
(defvar mtext-font-lock-keywords mtext-font-lock-keywords-1
  "Default highlighting level for mtext mode")

;; syntax
(defvar mtext-mode-syntax-table
  (let ((st (make-syntax-table)))
    (modify-syntax-entry ?# "< b" st)
    (modify-syntax-entry ?\n "> b" st)
    st))

;;; indentation
(defun mtext-indent-line ()
  "Indent current line as mtext"
  (beginning-of-line))

(defun mtext-mode ()
  "Major mode for editing semantically marked-up text"
  (interactive)
  (kill-all-local-variables)
  (set-syntax-table mtext-mode-syntax-table)
  (use-local-map mtext-mode-map)
  (set (make-local-variable 'font-lock-defaults) '(mtext-font-lock-keywords))
  (set (make-local-variable 'indent-line-function) 'mtext-indent-line)
  (setq major-mode 'mtext-mode)
  (setq mode-name "MTEXT")
  (run-hooks 'mtext-mode-hook))

(provide 'mtext-mode)
