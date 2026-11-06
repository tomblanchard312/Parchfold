# Parchfold UI design

Parchfold serves people reading, editing, reviewing, and signing PDFs. It follows the
[Eridu Design System](https://github.com/tomblanchard312/EriduOs/tree/53133f86434676a5bdcb249556d3c11b50e600e4/design-system)
workspace pattern. The toolbar and document tabs sit above a central PDF canvas, with
tool navigation and inspectors beside it. Use the File menu or Ctrl+O to open a
document, and the command palette to find tools by name. Native menu shortcuts follow
the platform.

- First run shows the home view with Open and recent documents.
- An open PDF owns the canvas. Tabs identify the active document, and panels expose tools
  and document details without changing the document’s typefaces.
- Open and save failures surface through the existing error UI. Closing edited documents
  offers a save decision. Enki speech is optional and belongs to the document workspace.

Parchfold maps the light and dark semantic colors into its egui theme, including copper
primary actions and River Blue accents. It uses Inter and JetBrains Mono for UI and code,
with existing language fallbacks. Desktop theme and interface scaling remain user-controlled.

Parchfold supports Linux, FreeBSD, Windows, macOS, and the web. Platform-specific acceptance is tracked separately. The home heading uses Source Serif 4
Display and page and zoom counters use Inter Tabular. Shared widget and menu spacing
follow the Eridu grid. Controls use Phosphor Regular SVGs and Phosphor Regular and Fill glyphs. The Windows PDF association artwork is attributed separately. Native accessibility
and visual review are tracked separately from the automated UI harness.
