<h1 align="center">
mediarepo-daemon
</h1>
<h3 align="center" style="color:red">This repository is a work in progress</h3>

- - -

This repository contains a media repository daemon that allows one to manage their media (mainly images).
It works similar to image boards (boorus) as it allows one to assign tags to media entries and
search for entries by using those tags. It is heavily inspired by [hydrus](https://github.com/hydrusnetwork/hydrus/).

## Usage

1. Initialize an empty repository
```
mediarepo --repo "where/your/repo/should/be" init
```

2. Import some images
```
mediarepo --repo "path/to/your/repo" import "path/to/your/files/as/**/glob/*.png"
``` 

3. Start the daemon
```
mediarepo --repo "path/to/your/repo start
```

4. Open the mediarepo-ui and connect to the repository


## License

GPL-3