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
![rotate-color:g:0.5 all apply save](images/00.png)

```
$ degenerate autosave x apply cross apply load:0.png save
```
![autosave x apply cross apply load:0.png save](images/01.png)

```
$ degenerate scale:2 x apply save
```
![scale:2 x apply save](images/02.png)

```
$ degenerate comment:slow x rotate-color:g:0.07 rotate:0.07 for:10 apply loop rotate-color:b:0.09 rotate:0.09 for:10 apply loop save
```
![comment:slow x rotate-color:g:0.07 rotate:0.07 for:10 apply loop rotate-color:b:0.09 rotate:0.09 for:10 apply loop save](images/03.png)

```
$ degenerate rotate-color:red:0.5 all apply save
```
![rotate-color:red:0.5 all apply save](images/04.png)

```
$ degenerate comment:slow seed:19798 rotate-color:g:0.01 rotate:0.01 for:100 random-mask apply loop rotate-color:b:0.01 rotate:0.01 for:100 random-mask apply loop save
```
![comment:slow seed:19798 rotate-color:g:0.01 rotate:0.01 for:100 random-mask apply loop rotate-color:b:0.01 rotate:0.01 for:100 random-mask apply loop save](images/05.png)

```
$ degenerate top apply save
```
![top apply save](images/06.png)

```
$ degenerate rotate-color:green:1.0 all save
```
![rotate-color:green:1.0 all save](images/07.png)

```
$ degenerate all apply save
```
![all apply save](images/08.png)

```
$ degenerate rotate:1.0 square apply save
```
![rotate:1.0 square apply save](images/09.png)

```
$ degenerate rotate-color:red:1.0 all save
```
![rotate-color:red:1.0 all save](images/10.png)

```
$ degenerate square apply top apply save
```
![square apply top apply save](images/11.png)

```
$ degenerate rows:1:1 apply save
```
![rows:1:1 apply save](images/12.png)

```
$ degenerate comment:slow rotate:0.3333 rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop save
```
![comment:slow rotate:0.3333 rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop save](images/13.png)

```
$ degenerate seed:2 random-mask apply save
```
![seed:2 random-mask apply save](images/14.png)

```
$ degenerate rotate:0.05 square for:2 apply loop x for:1 apply loop save
```
![rotate:0.05 square for:2 apply loop x for:1 apply loop save](images/15.png)

```
$ degenerate rotate-color:b:0.5 all apply save
```
![rotate-color:b:0.5 all apply save](images/16.png)

```
$ degenerate comment:slow circle scale:0.5 for:8 apply loop save
```
![comment:slow circle scale:0.5 for:8 apply loop save](images/17.png)

```
$ degenerate random-mask apply save
```
![random-mask apply save](images/18.png)

```
$ degenerate rotate:0.125 square apply save
```
![rotate:0.125 square apply save](images/19.png)

```
$ degenerate rotate:0.05 scale:2 x apply save
```
![rotate:0.05 scale:2 x apply save](images/20.png)

```
$ degenerate rotate-color:blue:1.0 all apply save
```
![rotate-color:blue:1.0 all apply save](images/21.png)

```
$ degenerate comment:slow rotate:0.111 for:16 square apply circle apply loop save
```
![comment:slow rotate:0.111 for:16 square apply circle apply loop save](images/22.png)

```
$ degenerate comment:slow circle scale:0.5 for:8 apply wrap loop save
```
![comment:slow circle scale:0.5 for:8 apply wrap loop save](images/23.png)

```
$ degenerate rotate-color:blue:0.5 all apply save
```
![rotate-color:blue:0.5 all apply save](images/24.png)

```
$ degenerate scale:0.5 circle wrap apply save
```
![scale:0.5 circle wrap apply save](images/25.png)

```
$ degenerate comment:slow seed:12462 rotate-color:g:0.1 rotate:0.1 for:10 random-mask apply loop rotate-color:b:0.1 rotate:0.1 for:10 random-mask apply loop save
```
![comment:slow seed:12462 rotate-color:g:0.1 rotate:0.1 for:10 random-mask apply loop rotate-color:b:0.1 rotate:0.1 for:10 random-mask apply loop save](images/26.png)

```
$ degenerate circle apply save
```
![circle apply save](images/27.png)

```
$ degenerate resize:512 save
```
![resize:512 save](images/28.png)

```
$ degenerate autosave resize:256 x apply load:0.png save
```
![autosave resize:256 x apply load:0.png save](images/29.png)

```
$ degenerate mod:3:0 apply save
```
![mod:3:0 apply save](images/30.png)

```
$ degenerate apply save
```
![apply save](images/31.png)

```
$ degenerate resize:512:256 rotate:0.05 x apply save load save
```
![resize:512:256 rotate:0.05 x apply save load save](images/32.png)

```
$ degenerate rotate-color:green:0.5 all apply save
```
![rotate-color:green:0.5 all apply save](images/33.png)

```
$ degenerate cross apply save
```
![cross apply save](images/34.png)

```
$ degenerate rows:18446744073709551615:18446744073709551615 apply save
```
![rows:18446744073709551615:18446744073709551615 apply save](images/35.png)

```
$ degenerate resize:3 default:0:255:0 scale:0.5 apply save
```
![resize:3 default:0:255:0 scale:0.5 apply save](images/36.png)

```
$ degenerate rotate:0.05 square for:2 apply loop save
```
![rotate:0.05 square for:2 apply loop save](images/37.png)

```
$ degenerate default:0:255:0 resize:512 save
```
![default:0:255:0 resize:512 save](images/38.png)

```
$ degenerate comment:slow rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop save
```
![comment:slow rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate:0.8333 rotate-color:b:0.05 for:8 apply loop save](images/39.png)

```
$ degenerate x apply save
```
![x apply save](images/40.png)

```
$ degenerate comment:slow scale:0.99 circle for:100 apply loop save
```
![comment:slow scale:0.99 circle for:100 apply loop save](images/41.png)

```
$ degenerate comment:slow generate save
```
![comment:slow generate save](images/42.png)

```
$ degenerate scale:0.5 circle apply save
```
![scale:0.5 circle apply save](images/43.png)

```
$ degenerate comment:slow rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop save
```
![comment:slow rotate-color:g:0.05 circle scale:0.5 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop save](images/44.png)

```
$ degenerate rotate-color:r:0.5 all apply save
```
![rotate-color:r:0.5 all apply save](images/45.png)

```
$ degenerate resize:512:256 save
```
![resize:512:256 save](images/46.png)

```
$ degenerate scale:0.5 circle apply all scale:0.9 wrap apply save
```
![scale:0.5 circle apply all scale:0.9 wrap apply save](images/47.png)

```
$ degenerate comment:slow alpha:0.75 circle scale:0.5 for:8 apply wrap loop save
```
![comment:slow alpha:0.75 circle scale:0.5 for:8 apply wrap loop save](images/48.png)

```
$ degenerate comment:foo save
```
![comment:foo save](images/49.png)

```
$ degenerate x apply scale:0.5 wrap identity all apply save
```
![x apply scale:0.5 wrap identity all apply save](images/50.png)

```
$ degenerate rotate:0.05 x apply save
```
![rotate:0.05 x apply save](images/51.png)

```
$ degenerate scale:2 rotate:0.05 x apply save
```
![scale:2 rotate:0.05 x apply save](images/52.png)

```
$ degenerate square apply save
```
![square apply save](images/53.png)

```
$ degenerate resize:512:256 all apply save all apply load
```
![resize:512:256 all apply save all apply load](images/54.png)

```
$ degenerate alpha:0.5 x apply save
```
![alpha:0.5 x apply save](images/55.png)

```
$ degenerate read save
```
![read save](images/56.png)

```
$ degenerate comment:slow x scale:0.5 for:8 apply wrap loop save
```
![comment:slow x scale:0.5 for:8 apply wrap loop save](images/57.png)

```
$ degenerate comment:slow rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop save
```
![comment:slow rotate-color:g:0.05 circle scale:0.75 wrap for:8 apply loop rotate-color:b:0.05 for:8 apply loop save](images/58.png)

```
$ degenerate autosave square apply load:0.png x apply load:1.png save
```
![autosave square apply load:0.png x apply load:1.png save](images/59.png)

```
$ degenerate save
```
![save](images/60.png)

```
$ degenerate comment:slow x scale:0.5 for:8 apply loop save
```
![comment:slow x scale:0.5 for:8 apply loop save](images/61.png)
