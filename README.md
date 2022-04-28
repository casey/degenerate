# degenerate

Degenerate is an algorithmic image generator inspired by
[blaster](https://github.com/casey/blaster).

(N.B. blaster is written in idiosyncratic Objective-C++.)

## Compiling

`degenerate` can render to a terminal or to a window. To render to a window,
`degenerate` must be built with the optional `window` feature.

## Usage

```bash
$ degenerate [COMMAND]...
```

`COMMAND`s may take zero or more `:`-separated arguments, and are currently
undocumented. The best way to learn what they do is to peruse the [image
tests](images). The name of each image is the `degenerate` program that
produced it. The image tests are reproduced below, with each preceded by its
`degenerate` invocation.

## Gallery

```
$ degenerate rotate-color:g:0.5 all apply save

```
![rotate-color:g:0.5 all apply save
](images/rotate-color%3Ag%3A0.5%20all%20apply%20save%0A.png)

```
$ degenerate autosave x apply cross apply load:0.png save

```
![autosave x apply cross apply load:0.png save
](images/autosave%20x%20apply%20cross%20apply%20load%3A0.png%20save%0A.png)

```
$ degenerate scale:2 x apply save

```
![scale:2 x apply save
](images/scale%3A2%20x%20apply%20save%0A.png)

```
$ degenerate comment:slow x rotate-color:g:0.07 rotate:0.07 for:10 apply loop rotate-color:b:0.09 rotate:0.09 for:10 apply loop save

```
![comment:slow x rotate-color:g:0.07 rotate:0.07 for:10 apply loop rotate-color:b:0.09 rotate:0.09 for:10 apply loop save
](images/comment%3Aslow%20x%20rotate-color%3Ag%3A0.07%20rotate%3A0.07%20for%3A10%20apply%20loop%20rotate-color%3Ab%3A0.09%20rotate%3A0.09%20for%3A10%20apply%20loop%20save%0A.png)

```
$ degenerate rotate-color:red:0.5 all apply save

```
![rotate-color:red:0.5 all apply save
](images/rotate-color%3Ared%3A0.5%20all%20apply%20save%0A.png)

```
$ degenerate comment:slow seed:19798 rotate-color:g:0.01 rotate:0.01 for:100 random-mask apply loop rotate-color:b:0.01 rotate:0.01 for:100 random-mask apply loop save

```
![comment:slow seed:19798 rotate-color:g:0.01 rotate:0.01 for:100 random-mask apply loop rotate-color:b:0.01 rotate:0.01 for:100 random-mask apply loop save
](images/comment%3Aslow%20seed%3A19798%20rotate-color%3Ag%3A0.01%20rotate%3A0.01%20for%3A100%20random-mask%20apply%20loop%20rotate-color%3Ab%3A0.01%20rotate%3A0.01%20for%3A100%20random-mask%20apply%20loop%20save%0A.png)

```
$ degenerate top apply save

```
![top apply save
](images/top%20apply%20save%0A.png)

```
$ degenerate rotate-color:green:1.0 all save

```
![rotate-color:green:1.0 all save
](images/rotate-color%3Agreen%3A1.0%20all%20save%0A.png)

```
$ degenerate all apply save

```
![all apply save
](images/all%20apply%20save%0A.png)

```
$ degenerate rotate:1.0 square apply save

```
![rotate:1.0 square apply save
](images/rotate%3A1.0%20square%20apply%20save%0A.png)

```
$ degenerate rotate-color:red:1.0 all save

```
![rotate-color:red:1.0 all save
](images/rotate-color%3Ared%3A1.0%20all%20save%0A.png)

```
$ degenerate square apply top apply save

```
![square apply top apply save
](images/square%20apply%20top%20apply%20save%0A.png)

```
$ degenerate rows:1:1 apply save

```
![rows:1:1 apply save
](images/rows%3A1%3A1%20apply%20save%0A.png)

```
$ degenerate comment:slow rotate:0.3333 rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop save

```
![comment:slow rotate:0.3333 rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop save
](images/comment%3Aslow%20rotate%3A0.3333%20rotate-color%3Ag%3A0.05%20circle%20scale%3A0.5%20wrap%20for%3A8%20apply%20loop%20rotate%3A0.8333%20rotate-color%3Ab%3A0.05%20for%3A8%20apply%20loop%20save%0A.png)

```
$ degenerate seed:2 random-mask apply save

```
![seed:2 random-mask apply save
](images/seed%3A2%20random-mask%20apply%20save%0A.png)

```
$ degenerate rotate:0.05 square for:2 apply loop x for:1 apply loop save

```
![rotate:0.05 square for:2 apply loop x for:1 apply loop save
](images/rotate%3A0.05%20square%20for%3A2%20apply%20loop%20x%20for%3A1%20apply%20loop%20save%0A.png)

```
$ degenerate rotate-color:b:0.5 all apply save

```
![rotate-color:b:0.5 all apply save
](images/rotate-color%3Ab%3A0.5%20all%20apply%20save%0A.png)

```
$ degenerate comment:slow circle scale:0.5 for:8 apply loop save

```
![comment:slow circle scale:0.5 for:8 apply loop save
](images/comment%3Aslow%20circle%20scale%3A0.5%20for%3A8%20apply%20loop%20save%0A.png)

```
$ degenerate random-mask apply save

```
![random-mask apply save
](images/random-mask%20apply%20save%0A.png)

```
$ degenerate rotate:0.125 square apply save

```
![rotate:0.125 square apply save
](images/rotate%3A0.125%20square%20apply%20save%0A.png)

```
$ degenerate rotate:0.05 scale:2 x apply save

```
![rotate:0.05 scale:2 x apply save
](images/rotate%3A0.05%20scale%3A2%20x%20apply%20save%0A.png)

```
$ degenerate rotate-color:blue:1.0 all apply save

```
![rotate-color:blue:1.0 all apply save
](images/rotate-color%3Ablue%3A1.0%20all%20apply%20save%0A.png)

```
$ degenerate comment:slow rotate:0.111 for:16 square apply circle apply loop save

```
![comment:slow rotate:0.111 for:16 square apply circle apply loop save
](images/comment%3Aslow%20rotate%3A0.111%20for%3A16%20square%20apply%20circle%20apply%20loop%20save%0A.png)

```
$ degenerate comment:slow circle scale:0.5 for:8 apply wrap loop save

```
![comment:slow circle scale:0.5 for:8 apply wrap loop save
](images/comment%3Aslow%20circle%20scale%3A0.5%20for%3A8%20apply%20wrap%20loop%20save%0A.png)

```
$ degenerate rotate-color:blue:0.5 all apply save

```
![rotate-color:blue:0.5 all apply save
](images/rotate-color%3Ablue%3A0.5%20all%20apply%20save%0A.png)

```
$ degenerate scale:0.5 circle wrap apply save

```
![scale:0.5 circle wrap apply save
](images/scale%3A0.5%20circle%20wrap%20apply%20save%0A.png)

```
$ degenerate comment:slow seed:12462 rotate-color:g:0.1 rotate:0.1 for:10 random-mask apply loop rotate-color:b:0.1 rotate:0.1 for:10 random-mask apply loop save

```
![comment:slow seed:12462 rotate-color:g:0.1 rotate:0.1 for:10 random-mask apply loop rotate-color:b:0.1 rotate:0.1 for:10 random-mask apply loop save
](images/comment%3Aslow%20seed%3A12462%20rotate-color%3Ag%3A0.1%20rotate%3A0.1%20for%3A10%20random-mask%20apply%20loop%20rotate-color%3Ab%3A0.1%20rotate%3A0.1%20for%3A10%20random-mask%20apply%20loop%20save%0A.png)

```
$ degenerate circle apply save

```
![circle apply save
](images/circle%20apply%20save%0A.png)

```
$ degenerate resize:512 save

```
![resize:512 save
](images/resize%3A512%20save%0A.png)

```
$ degenerate autosave resize:256 x apply load:0.png save

```
![autosave resize:256 x apply load:0.png save
](images/autosave%20resize%3A256%20x%20apply%20load%3A0.png%20save%0A.png)

```
$ degenerate mod:3:0 apply save

```
![mod:3:0 apply save
](images/mod%3A3%3A0%20apply%20save%0A.png)

```
$ degenerate apply save

```
![apply save
](images/apply%20save%0A.png)

```
$ degenerate resize:512:256 rotate:0.05 x apply save load save

```
![resize:512:256 rotate:0.05 x apply save load save
](images/resize%3A512%3A256%20rotate%3A0.05%20x%20apply%20save%20load%20save%0A.png)

```
$ degenerate rotate-color:green:0.5 all apply save

```
![rotate-color:green:0.5 all apply save
](images/rotate-color%3Agreen%3A0.5%20all%20apply%20save%0A.png)

```
$ degenerate cross apply save

```
![cross apply save
](images/cross%20apply%20save%0A.png)

```
$ degenerate rows:18446744073709551615:18446744073709551615 apply save

```
![rows:18446744073709551615:18446744073709551615 apply save
](images/rows%3A18446744073709551615%3A18446744073709551615%20apply%20save%0A.png)

```
$ degenerate resize:3 default:0:255:0 scale:0.5 apply save

```
![resize:3 default:0:255:0 scale:0.5 apply save
](images/resize%3A3%20default%3A0%3A255%3A0%20scale%3A0.5%20apply%20save%0A.png)

```
$ degenerate rotate:0.05 square for:2 apply loop save

```
![rotate:0.05 square for:2 apply loop save
](images/rotate%3A0.05%20square%20for%3A2%20apply%20loop%20save%0A.png)

```
$ degenerate default:0:255:0 resize:512 save

```
![default:0:255:0 resize:512 save
](images/default%3A0%3A255%3A0%20resize%3A512%20save%0A.png)

```
$ degenerate comment:slow rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop save

```
![comment:slow rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop save
](images/comment%3Aslow%20rotate-color%3Ag%3A0.05%20circle%20scale%3A0.75%20wrap%20for%3A8%20apply%20loop%20rotate%3A0.8333%20rotate-color%3Ab%3A0.05%20for%3A8%20apply%20loop%20save%0A.png)

```
$ degenerate x apply save

```
![x apply save
](images/x%20apply%20save%0A.png)

```
$ degenerate comment:slow scale:0.99 circle for:100 apply loop save

```
![comment:slow scale:0.99 circle for:100 apply loop save
](images/comment%3Aslow%20scale%3A0.99%20circle%20for%3A100%20apply%20loop%20save%0A.png)

```
$ degenerate comment:slow generate save

```
![comment:slow generate save
](images/comment%3Aslow%20generate%20save%0A.png)

```
$ degenerate scale:0.5 circle apply save

```
![scale:0.5 circle apply save
](images/scale%3A0.5%20circle%20apply%20save%0A.png)

```
$ degenerate comment:slow rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop save

```
![comment:slow rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop save
](images/comment%3Aslow%20rotate-color%3Ag%3A0.05%20circle%20scale%3A0.5%20wrap%20for%3A8%20apply%20loop%20rotate-color%3Ab%3A0.05%20for%3A8%20apply%20loop%20save%0A.png)

```
$ degenerate rotate-color:r:0.5 all apply save

```
![rotate-color:r:0.5 all apply save
](images/rotate-color%3Ar%3A0.5%20all%20apply%20save%0A.png)

```
$ degenerate resize:512:256 save

```
![resize:512:256 save
](images/resize%3A512%3A256%20save%0A.png)

```
$ degenerate scale:0.5 circle apply all scale:0.9 wrap apply save

```
![scale:0.5 circle apply all scale:0.9 wrap apply save
](images/scale%3A0.5%20circle%20apply%20all%20scale%3A0.9%20wrap%20apply%20save%0A.png)

```
$ degenerate comment:slow alpha:0.75 circle scale:0.5 for:8 apply wrap loop save

```
![comment:slow alpha:0.75 circle scale:0.5 for:8 apply wrap loop save
](images/comment%3Aslow%20alpha%3A0.75%20circle%20scale%3A0.5%20for%3A8%20apply%20wrap%20loop%20save%0A.png)

```
$ degenerate comment:foo save

```
![comment:foo save
](images/comment%3Afoo%20save%0A.png)

```
$ degenerate x apply scale:0.5 wrap identity all apply save

```
![x apply scale:0.5 wrap identity all apply save
](images/x%20apply%20scale%3A0.5%20wrap%20identity%20all%20apply%20save%0A.png)

```
$ degenerate rotate:0.05 x apply save

```
![rotate:0.05 x apply save
](images/rotate%3A0.05%20x%20apply%20save%0A.png)

```
$ degenerate scale:2 rotate:0.05 x apply save

```
![scale:2 rotate:0.05 x apply save
](images/scale%3A2%20rotate%3A0.05%20x%20apply%20save%0A.png)

```
$ degenerate square apply save

```
![square apply save
](images/square%20apply%20save%0A.png)

```
$ degenerate resize:512:256 all apply save all apply load

```
![resize:512:256 all apply save all apply load
](images/resize%3A512%3A256%20all%20apply%20save%20all%20apply%20load%0A.png)

```
$ degenerate alpha:0.5 x apply save

```
![alpha:0.5 x apply save
](images/alpha%3A0.5%20x%20apply%20save%0A.png)

```
$ degenerate read save

```
![read save
](images/read%20save%0A.png)

```
$ degenerate comment:slow x scale:0.5 for:8 apply wrap loop save

```
![comment:slow x scale:0.5 for:8 apply wrap loop save
](images/comment%3Aslow%20x%20scale%3A0.5%20for%3A8%20apply%20wrap%20loop%20save%0A.png)

```
$ degenerate comment:slow rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop save

```
![comment:slow rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop save
](images/comment%3Aslow%20rotate-color%3Ag%3A0.05%20circle%20scale%3A0.75%20wrap%20for%3A8%20apply%20loop%20rotate-color%3Ab%3A0.05%20for%3A8%20apply%20loop%20save%0A.png)

```
$ degenerate autosave square apply load:0.png x apply load:1.png save

```
![autosave square apply load:0.png x apply load:1.png save
](images/autosave%20square%20apply%20load%3A0.png%20x%20apply%20load%3A1.png%20save%0A.png)

```
$ degenerate save

```
![save
](images/save%0A.png)

```
$ degenerate comment:slow x scale:0.5 for:8 apply loop save

```
![comment:slow x scale:0.5 for:8 apply loop save
](images/comment%3Aslow%20x%20scale%3A0.5%20for%3A8%20apply%20loop%20save%0A.png)
