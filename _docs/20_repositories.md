---
layout: doc
title: Repositories
sort_key: 20
draft: false
---

When opening mediarepo you will be greeted by an (empty) overview of configured repositories.
A repository is a contained unit of imported files, settings and tags. 
By using the *Add Repository* button you can create a new repository.

![](/assets/images/mediarepo-empty-repo-view.png)

A dialog should open where you can specify the name of the repository, the type and further related options.

![](/assets/images/add-repository-dialog.png)

When choosing the type *Local* you have to specify the path where the repository should be created. 
If no repository exists at the selected path, you can initialize it by clicking on the *Init* button.
After that you can save the configuration by clicking on *Save*.

![](/assets/images/add-remote-repository-dialog.png)

When choosing the *Remote* type you have to specify the IP address and port of the remote repository. 
You can check connection to make sure the entered address is correct.
Remote repositories are repositories that have a daemon running with TCP enabled and a fixed port.
You can read more about configuring daemons and repositories [here](50_repo_configuration).

After adding a repository it should appear in the repository overview.

![](/assets/images/mediarepo-first-repository.png)