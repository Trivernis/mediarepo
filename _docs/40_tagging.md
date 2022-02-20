---
layout: doc
title: Tagging
sort_index: 40
draft: false
---

The main feature of mediarepo is associating tags with files. 
A tag describes a feature of a file or some metadata about it.
There's unnamespaced tags, e.g. `summer` or `frog` and namespaced
tags (prepended and separated from the tag by a colon)m e.g. `weather:sunny`
or `author:trivernis`.
Tags enable you to search for files more accurately and sort them according to tag namespaces.

To assign tags we're opening a new *Files* tab. You can also stay in the import
tab from the previous step as we're probably spending more time in the *Files* tab anyway
we're switching to it in this walkthrough.

Use the **+** icon next to the rightmost tab to open a new tab and select *Files*.

![](/assets/images/mediarepo-new-tab-files.png)

This tab looks very similar to the import tab and the layout is mostly the same.
On the left side you've got search and sorting options but for tagging we don't need those
right now. To make identifying features of the images easier let's switch to the gallery view.
To open this view you can double click an image in the gallery view or select it and press `<Enter>`
or right click it and select *Open*.

![](/assets/images/mediarepo-files-open-image.png)

A full view of the selected image will open. To edit tags we have to switch to the *Edit Tags* mode
of the side menu (1).

![](/assets/images/mediarepo-edit-tags-gallery.png)

Now you can assign tags to the selected image by entering them in the text field (2). When entering
a tag and pressing `<Enter>` or the button next to the input field it gets assigned to the file. When
entering it again and pressing enter again it will be removed from this image. This is because
the default mode for assinging tags is the *Toggle* mode which add a tag when it's not assigned
 and removes it when it is. you can change the mode in the mode selection drop-down (3). Tags can also be deleted
 by pressing the **-** button next to the tag in the *Edit Tags* list.
If you want to assign tags to multiple files at once you can exit the gallery view by pressing the **x** in the top
right corner or pressing `<Escape>` twice in quick succession.

In the grid view you can select multiple files by first selecting one file and then while pressing `<Shift>` or 
`<Ctrl>` selecting another. You can also use `<Ctrl> + <A>` to select all files. The *Edit Tags* mode of the side menu
will only be visible when at least one file is selected.

![](/assets/images/edit-tags-grid-multiple.png)