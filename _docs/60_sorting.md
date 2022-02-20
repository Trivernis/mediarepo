---
layout: doc
title: Sorting
sort_index: 60
---

After searching you might want to change the order in which search entries appear.
To do that you can apply a sorting configuration to the search. By default
results are sorted by the time they were imported but this can be changed
by clicking on the sort button.

![](/assets/images/mediarepo-sort-button.png)

This will open up a dialog with the current sorting configuration.

![](/assets/images/sort-dialog.png)

The key used for sorting can be configured in the first drop-down (1). You can
specify which sort direction should be applied for the specific key (2). When selecting
the *Namespace* key you have to additionally provide the namespaces that shuld be used (3).
Sorting by namespace uses the tags with the specific namespace assigned to the image as
sorting values. For example if you've got files that have tags of the `page` namespaced
assigned like `page:1`, `page:2`, `page:3` and so on you can sort by the `page` namespace
to get them in the correct order. The value is first interpreted as a numerical value and
if that fails as a string value and sorted alphabetically.
To add new search entries you can use the plus button (5). Entries can be removed by using
the minus button (4).
You can change the order in which search entries are applied by dragging and dropping entries
in the right position with their handles (6). If you want to save a specific sorting configuration
so that it doesn't need to be configured for each page you can use the *Save new* button in the bottom left (7).

![](/assets/images/sort-dialog-preset.png)

This will create a new entry which can be selected via a drop-down (11) at the top which is only visible
when there's sorting configurations stored in the repository. The selected configuration can also be overwritten
with the *Save* button (9) or deleted with the *Delete* button (10).

When you're satisfied with the sorting configuration you can apply it by pressing the *Sort* button (8).