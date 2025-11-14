# A11y-Term: Exhaustive ARIA 1.2 Attribute Design Document

## 1. Objective

To create an exhaustive, type-safe Rust data model in `a11y.rs` that fully represents all states and properties of the WAI-ARIA 1.2 standard. This model will serve as the foundation for the terminal's accessibility features, enabling compile-time correctness and ensuring strict compliance with the ARIA specification.

## 2. High-Level Implementation Strategy

1.  **Data Model (`a11y.rs`):** An exhaustive `AriaData` struct will be the single source of truth. It will contain a field for every `aria-*` attribute, each wrapped in an `Option`.
2.  **Builder (`a11y.rs`):** A `AriaDataBuilder` will be implemented to provide a clean, fluent API for constructing an `AriaData` object. This encapsulates the creation logic.
3.  **Parser (`perform.rs`):** The `osc_dispatch` function will contain a large `match` statement. It will parse the `key=value` arguments from the OSC command and use the `AriaDataBuilder` to configure the object.

## 3. Core Data Type Enumerations

These enums provide type-safety for ARIA attributes that accept a limited set of token values.

```rust
// In a11y.rs

/// Represents attributes that can be true, false, or "mixed".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriState { True, False, Mixed }

/// Represents the `aria-orientation` property.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation { Vertical, Horizontal }

/// Represents the `aria-sort` property.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDirection { Ascending, Descending, None, Other }

/// Represents the `aria-live` property for live regions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiveSetting { Off, Polite, Assertive }

/// Represents items in the `aria-relevant` property for live regions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Relevance { Additions, Removals, Text, All }

/// Represents the `aria-current` property.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Current { Page, Step, Location, Date, Time, True, False }

/// Represents the `aria-haspopup` property.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PopupType { True, False, Menu, ListBox, Tree, Grid, Dialog }

/// Represents the `aria-invalid` property.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvalidState { False, True, Grammar, Spelling }

/// Represents the `aria-autocomplete` property.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AutocompleteType { Inline, List, Both, None }
```

## 4. Exhaustive `AriaData` Struct Definition

This struct contains all 46 non-deprecated attributes from the WAI-ARIA 1.2 specification.

```rust
// In a11y.rs
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AriaData {
    // --- Global Attributes (19) ---
    pub atomic: Option<bool>,
    pub busy: Option<bool>,
    pub controls: Option<Vec<String>>,
    pub current: Option<Current>,
    pub describedby: Option<Vec<String>>,
    pub details: Option<String>,
    pub disabled: Option<bool>,
    pub errormessage: Option<String>,
    pub flowto: Option<Vec<String>>,
    pub haspopup: Option<PopupType>,
    pub hidden: Option<bool>,
    pub invalid: Option<InvalidState>,
    pub keyshortcuts: Option<String>,
    pub label: Option<String>,
    pub labelledby: Option<Vec<String>>,
    pub live: Option<LiveSetting>,
    pub owns: Option<Vec<String>>,
    pub relevant: Option<Vec<Relevance>>,
    pub roledescription: Option<String>,

    // --- Widget States & Properties (18) ---
    pub autocomplete: Option<AutocompleteType>,
    pub checked: Option<TriState>,
    pub expanded: Option<bool>,
    pub level: Option<i32>,
    pub modal: Option<bool>,
    pub multiline: Option<bool>,
    pub multiselectable: Option<bool>,
    pub orientation: Option<Orientation>,
    pub placeholder: Option<String>,
    pub pressed: Option<TriState>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub selected: Option<bool>,
    pub sort: Option<SortDirection>,
    pub valuemax: Option<f64>,
    pub valuemin: Option<f64>,
    pub valuenow: Option<f64>,
    pub valuetext: Option<String>,

    // --- Relationship & Collection Attributes (9) ---
    pub activedescendant: Option<String>,
    pub colcount: Option<i32>,
    pub colindex: Option<i32>,
    pub colspan: Option<i32>,
    pub posinset: Option<i32>,
    pub rowcount: Option<i32>,
    pub rowindex: Option<i32>,
    pub rowspan: Option<i32>,
    pub setsize: Option<i32>,
}
```

## 5. WAI-ARIA 1.2 Attribute to Rust Type Mapping

This table provides the definitive mapping for all 46 non-deprecated attributes.

### 1. Global Attributes (19 Attributes)
| Attribute | Value Type | Description |
| :--- | :--- | :--- |
| `aria-atomic` | `true` / `false` | If `true`, screen readers present the *entire* region when it changes, not just the changed node. |
| `aria-busy` | `true` / `false` | If `true`, tells AT to ignore updates until the element finishes loading. |
| `aria-controls` | ID reference list | Points to the ID(s) of the element this widget controls (e.g., a tab controls a tabpanel). |
| `aria-current` | `page` / `step` / `location` / `date` / `time` / `true` / `false` | Indicates the current item in a set (e.g., the active link in a nav bar). |
| `aria-describedby`| ID reference list | Points to an element that provides a long description (read after the label). |
| `aria-details` | ID reference | Points to an element that provides complex details (more semantic than describedby). |
| `aria-disabled` | `true` / `false` | Indicates the element is perceivable but not editable/operable. |
| `aria-errormessage`| ID reference | Points to the element displaying the error message for this object. |
| `aria-flowto` | ID reference list | Overrides the reading order (very rare, use cautiously). |
| `aria-haspopup` | `true` / `menu` / `listbox` / `tree` / `grid` / `dialog` | Indicates this element triggers a popup interaction. |
| `aria-hidden` | `true` / `false` | **Critical for TUI.** Hides purely decorative characters (like ASCII borders) from screen readers. |
| `aria-invalid` | `grammar` / `spelling` / `true` / `false` | Indicates the value does not conform to expectations. |
| `aria-keyshortcuts`| string | A space-delimited list of keyboard shortcuts (e.g., "Control+S"). |
| `aria-label` | string | A direct string label. Use this if there is no visible text on the screen to point to. |
| `aria-labelledby` | ID reference list | **Best Practice.** Points to the ID of the text span that acts as the label. |
| `aria-live` | `off` / `polite` / `assertive` | Defines how aggressively updates should be announced. |
| `aria-owns` | ID reference list | **Critical for TUI.** tells the accessibility tree that an element visually located elsewhere is actually a child of this element (fixes DOM nesting issues). |
| `aria-relevant` | `additions` / `removals` / `text` / `all` | Restricts what types of "live" updates are announced. |
| `aria-roledescription`| string | Allows you to override the role name spoken (e.g., changing "Group" to "Slide"). |

### 2. Widget States & Properties (18 Attributes)
| Attribute | Value Type | Role Context |
| :--- | :--- | :--- |
| `aria-autocomplete` | `inline` / `list` / `both` / `none` | Combobox, Textbox |
| `aria-checked` | `true` / `false` / `mixed` | Checkbox, Radio, Switch, Menuitemcheckbox |
| `aria-expanded` | `true` / `false` | Button, Combobox, Treeitem (Menu expand/collapse) |
| `aria-level` | integer | Heading, Treeitem (Depth of nesting) |
| `aria-modal` | `true` / `false` | Dialog, Alertdialog (Traps focus inside) |
| `aria-multiline` | `true` / `false` | Textbox (Single vs TextArea) |
| `aria-multiselectable`| `true` / `false` | Grid, Listbox, Tablist, Tree |
| `aria-orientation` | `horizontal` / `vertical` | Scrollbar, Separator, Slider, Tablist, Toolbar |
| `aria-placeholder` | string | Textbox (Ghost text) |
| `aria-pressed` | `true` / `false` / `mixed` | Button (Toggle buttons) |
| `aria-readonly` | `true` / `false` | Grid, Textbox (Content is selectable but not editable) |
| `aria-required` | `true` / `false` | Inputs (Form validation) |
| `aria-selected` | `true` / `false` | Gridcell, Option, Tab, Treeitem, Row |
| `aria-sort` | `ascending` / `descending` / `none` / `other` | Columnheader, Rowheader |
| `aria-valuemax` | number | Range widgets (Slider, Spinbutton, Progressbar) |
| `aria-valuemin` | number | Range widgets |
| `aria-valuenow` | number | Range widgets |
| `aria-valuetext` | string | Range widgets (Human readable value, e.g., "High" instead of "10") |

### 3. Relationship & Collection Attributes (9 Attributes)
| Attribute | Value Type | Description |
| :--- | :--- | :--- |
| `aria-activedescendant`| ID reference | **The TUI "Holy Grail".** Allows focus to remain on a container (like a Grid) while the "virtual" focus moves to children IDs. Used to avoid `tabindex` management hell. |
| `aria-colcount` | integer | Total columns in the *entire* dataset (not just what's rendered). |
| `aria-colindex` | integer | The logical column index of the current cell. |
| `aria-colspan` | integer | How many columns this cell merges. |
| `aria-posinset` | integer | "Item X of Y". Used in virtual lists to tell the user their position even if only 5 items are in the DOM. |
| `aria-rowcount` | integer | Total rows in the *entire* dataset. |
| `aria-rowindex` | integer | The logical row index of the current row. |
| `aria-rowspan` | integer | How many rows this cell merges. |
| `aria-setsize` | integer | The total number of items in the list (Y in "Item X of Y"). |

## 6. WAI-ARIA 1.2 Roles (82 Usable Roles)

This section provides a complete reference for all 82 concrete (usable) roles in the WAI-ARIA 1.2 specification, categorized by function.

### 1. The "Application" Role
*   `application`: Declares that the element is a web application, passing all keystrokes to it.

### 2. Widget Roles (27 Roles)
*   `alertdialog`: A dialog containing an alert message.
*   `button`: A clickable button.
*   `checkbox`: A checkable input.
*   `dialog`: A descendant window of the primary window.
*   `gridcell`: A cell within a grid or treegrid.
*   `link`: An interactive reference to a resource.
*   `menuitem`: An item within a menu.
*   `menuitemcheckbox`: A menu item with a checkable state.
*   `menuitemradio`: A menu item within a group of radio buttons.
*   `option`: A selectable item in a listbox or combobox.
*   `progressbar`: Displays the progress status for a task.
*   `radio`: A checkable input in a group of radio roles.
*   `scrollbar`: A graphical object that controls the viewing area.
*   `searchbox`: A type of textbox intended for specifying search criteria.
*   `separator` (focusable): A divider that separates and distinguishes sections of content.
*   `slider`: An input where the user selects a value from within a given range.
*   `spinbutton`: An input that expects a value from a discrete set of values.
*   `switch`: A type of checkbox that represents on/off values.
*   `tab`: A grouping label for a tab.
*   `tabpanel`: A container for the resources associated with a tab.
*   `textbox`: Input that allows free-form text entry.
*   `treeitem`: An option item in a tree.

### 3. Composite Roles (9 Roles)
*   `combobox`: An input that controls another element, such as a listbox or grid.
*   `grid`: A composite widget containing a collection of cells.
*   `listbox`: A widget that allows the user to select one or more items from a list.
*   `menu`: A widget that offers a list of choices to the user.
*   `menubar`: A presentation of menu that usually remains visible.
*   `radiogroup`: A group of radio buttons.
*   `tablist`: A list of tab elements.
*   `tree`: A widget that allows the user to select one or more items from a hierarchy.
*   `treegrid`: A grid whose rows can be expanded and collapsed.

### 4. Document Structure Roles (37 Roles)
*   `article`: A self-contained composition.
*   `blockquote`: A section of content quoted from another source.
*   `caption`: A visible name or title for a table, grid, or figure.
*   `cell`: A cell in a tabular container.
*   `code`: A fragment of computer code.
*   `comment`: A comment on the content.
*   `definition`: A definition of a term or concept.
*   `deletion`: Content that is marked as removed.
*   `directory`: A list of references to members of a group, such as a static table of contents.
*   `document`: Content that assistive technology users may want to browse in a reading mode.
*   `emphasis`: Text that has stress emphasis.
*   `feed`: A scrollable list of articles.
*   `figure`: A perceivable section of content, often with a caption.
*   `generic`: A nameless container element with no semantic meaning.
*   `group`: A set of user interface objects not intended for page summary.
*   `heading`: A heading for a section of the page.
*   `img`: A container for a collection of elements that form an image.
*   `insertion`: Content that is marked as added.
*   `list`: A section containing listitem elements.
*   `listitem`: A single item in a list or directory.
*   `mark`: A highlight of content.
*   `math`: A mathematical expression.
*   `meter`: Represents a scalar measurement within a known range.
*   `none` (or `presentation`): An element whose implicit native role semantics will not be mapped.
*   `note`: A section with parenthetic or ancillary content.
*   `paragraph`: A paragraph of content.
*   `row`: A row of cells in a tabular container.
*   `rowgroup`: A group of rows in a tabular container.
*   `rowheader`: A cell containing header information for a row.
*   `strong`: Content that is important, serious, or urgent.
*   `subscript`: Text displayed lower than the main text.
*   `suggestion`: An suggested correction for an error.
*   `superscript`: Text displayed higher than the main text.
*   `table`: A section containing data arranged in rows and columns.
*   `term`: A word or phrase with a corresponding definition.
*   `toolbar`: A collection of commonly used function buttons or controls.
*   `tooltip`: A contextual popup that displays a description for an element.

### 5. Landmark Roles (8 Roles)
*   `banner`: Site-oriented content, rather than page-specific content.
*   `complementary`: A supporting section of the document.
*   `contentinfo`: Information about the parent document (like a footer).
*   `form`: A landmark region that contains a collection of items to create a form.
*   `main`: The main content of the document.
*   `navigation`: A collection of navigational elements.
*   `region`: A generic landmark region that requires a label.
*   `search`: A landmark region for search functionality.

### 6. Live Region Roles (5 Roles)
*   `alert`: A message with important, and usually time-sensitive, information.
*   `log`: A live region where new information is added in meaningful order.
*   `marquee`: A live region where non-essential information changes frequently.
*   `status`: Advisory information for the user that is not an alert.
*   `timer`: A live region containing a numerical counter.

### 7. Abstract Roles (Do Not Use)
These roles define the ontology but should **never** be used in the `role` attribute.
*   `command`
*   `composite`
*   `input`
*   `landmark`
*   `range`
*   `roletype`
*   `section`
*   `sectionhead`
- `select`
- `structure`
- `widget`
- `window`
