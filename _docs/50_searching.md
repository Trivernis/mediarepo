---
layout: doc
title: Searching
sort_index: 50
---

Now that our files have tags assigned we can use those tags for searching. 
Searching is only possible in the *Files* tab so if you haven't already you
need to open one now.
We need to switch our side menu to *Search* mode (1).

![](/assets/images/mediarepo-search.png)

Now you can enter filter expressions in the corresponding input field (2).
You can enter any of the tags you've previously assigned. If it exists
you should see it in the suggestions.
After entering a tag it will appear in the line above (3) and start
searching. You can prefix your query with a minus `-` to negate it.

## Wildcards

Searching with wildcards *\** is also possible. A wildcard must be positioned
at the end of a query entry to be interpreted as one. Valid entries would be:

- `sno*` - searching by all tags beginning with *sno*
- `season:*` - searching by all tags within the *season* namespace

## OR Expressions

The more tags you enter the more restricted the search result becomes.
But in some cases you might want to filter by either of some tags.
This is possible by separating two entries with the keyword *OR*.
For example:

- `season:winter OR river`
- `snow OR rabbit OR forest`

![](/assets/images/search-with-or.png)

## Search dialog

You can get a better overview of your current search query by pressing the
funnel icon in the search input element.
This will open a search dialog displaying the current search expression as
a list rather than in one line.

![](/assets/images/filter-dialog.png)

Expressions with an OR combinator are displayed in dark boxes (1).
Stand-alone expressinos are displayed outside these boxes (2).
You an enter additional queries in the bottom input field (3) and filter
by those once you're ready (4).

You're also able to select the single entries and by right clicking one 
open a menu with additional options.

![](/assets/images/search-dialog-contextmenu.png)

All entries can be removed (1). You can convert the selected entries to other
expression types removing them from their original expression (2 and 3) or
copy them to a new expression without removing them (4 and 5).

## Searching by properties

Files are stored with several properties which can be used in searches too.
To search by file properties you have to prefix your query with a dot `.` .
Properties that can be used in searches are:

- status
- file size
- imported time
- changed time
- created time
- tag count
- content descriptor (CD)

Depending on the property different types of comparators are valid.
Status and content descriptor can only be searched by an equal `=` comparator
while the remaining entries can also additionally 
be searched with the less `<` and greater `>` comparator e.g.

- `.Status = Imported`
- `.FileSize > 10MB`
- `.FileImportedTime < 2022-01-12T00:00:00`
- `.TagCount > 15`
- `.ContentDescriptor = 28g5gfpdvcvlua15gq6etq8e65tol4113e0kudcjjajkgmsj9aefimg`

![](/assets/images/property-filters.png)