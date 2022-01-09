# degenerate

Degenerate is an algorithmic image generator inspired by
[blaster](https://github.com/casey/blaster).

(N.B. blaster is written in idiosyncratic Objective-C++.)

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
$ degenerate comment:slow resize:256 generate save
```
![comment:slow resize:256 generate save](images/comment%3Aslow%20resize%3A256%20generate%20save.png)

```
$ degenerate comment:slow resize:256 rotate-color:red:0.083333 rotate:0.1 for:12 circle apply cross apply x apply loop save
```
![comment:slow resize:256 rotate-color:red:0.083333 rotate:0.1 for:12 circle apply cross apply x apply loop save](images/comment%3Aslow%20resize%3A256%20rotate-color%3Ared%3A0.083333%20rotate%3A0.1%20for%3A12%20circle%20apply%20cross%20apply%20x%20apply%20loop%20save.png)

```
$ degenerate comment:slow resize:256 rotate:0.111 for:16 square apply circle apply loop save
```
![comment:slow resize:256 rotate:0.111 for:16 square apply circle apply loop save](images/comment%3Aslow%20resize%3A256%20rotate%3A0.111%20for%3A16%20square%20apply%20circle%20apply%20loop%20save.png)

```
$ degenerate comment:slow resize:256 scale:0.99 circle for:100 apply loop save
```
![comment:slow resize:256 scale:0.99 circle for:100 apply loop save](images/comment%3Aslow%20resize%3A256%20scale%3A0.99%20circle%20for%3A100%20apply%20loop%20save.png)

```
$ degenerate comment:slow resize:256 seed:12462 rotate-color:g:0.1 rotate:0.1 for:10 random-mask apply loop rotate-color:b:0.1 rotate:0.1 for:10 random-mask apply loop save
```
![comment:slow resize:256 seed:12462 rotate-color:g:0.1 rotate:0.1 for:10 random-mask apply loop rotate-color:b:0.1 rotate:0.1 for:10 random-mask apply loop save](images/comment%3Aslow%20resize%3A256%20seed%3A12462%20rotate-color%3Ag%3A0.1%20rotate%3A0.1%20for%3A10%20random-mask%20apply%20loop%20rotate-color%3Ab%3A0.1%20rotate%3A0.1%20for%3A10%20random-mask%20apply%20loop%20save.png)

```
$ degenerate comment:slow resize:256 seed:19798 rotate-color:g:0.01 rotate:0.01 for:100 random-mask apply loop rotate-color:b:0.01 rotate:0.01 for:100 random-mask apply loop save
```
![comment:slow resize:256 seed:19798 rotate-color:g:0.01 rotate:0.01 for:100 random-mask apply loop rotate-color:b:0.01 rotate:0.01 for:100 random-mask apply loop save](images/comment%3Aslow%20resize%3A256%20seed%3A19798%20rotate-color%3Ag%3A0.01%20rotate%3A0.01%20for%3A100%20random-mask%20apply%20loop%20rotate-color%3Ab%3A0.01%20rotate%3A0.01%20for%3A100%20random-mask%20apply%20loop%20save.png)

```
$ degenerate comment:slow resize:256 x rotate-color:g:0.07 rotate:0.07 for:10 apply loop rotate-color:b:0.09 rotate:0.09 for:10 apply loop save
```
![comment:slow resize:256 x rotate-color:g:0.07 rotate:0.07 for:10 apply loop rotate-color:b:0.09 rotate:0.09 for:10 apply loop save](images/comment%3Aslow%20resize%3A256%20x%20rotate-color%3Ag%3A0.07%20rotate%3A0.07%20for%3A10%20apply%20loop%20rotate-color%3Ab%3A0.09%20rotate%3A0.09%20for%3A10%20apply%20loop%20save.png)

```
$ degenerate default:0:255:0 resize:256 save
```
![default:0:255:0 resize:256 save](images/default%3A0%3A255%3A0%20resize%3A256%20save.png)

```
$ degenerate resize:256 all apply save
```
![resize:256 all apply save](images/resize%3A256%20all%20apply%20save.png)

```
$ degenerate resize:256 apply save
```
![resize:256 apply save](images/resize%3A256%20apply%20save.png)

```
$ degenerate resize:256 circle apply save
```
![resize:256 circle apply save](images/resize%3A256%20circle%20apply%20save.png)

```
$ degenerate resize:256 comment:foo save
```
![resize:256 comment:foo save](images/resize%3A256%20comment%3Afoo%20save.png)

```
$ degenerate resize:256 cross apply save
```
![resize:256 cross apply save](images/resize%3A256%20cross%20apply%20save.png)

```
$ degenerate resize:256 mod:3:0 apply save
```
![resize:256 mod:3:0 apply save](images/resize%3A256%20mod%3A3%3A0%20apply%20save.png)

```
$ degenerate resize:256 random-mask apply save
```
![resize:256 random-mask apply save](images/resize%3A256%20random-mask%20apply%20save.png)

```
$ degenerate resize:256 rotate-color:b:0.5 all apply save
```
![resize:256 rotate-color:b:0.5 all apply save](images/resize%3A256%20rotate-color%3Ab%3A0.5%20all%20apply%20save.png)

```
$ degenerate resize:256 rotate-color:blue:0.5 all apply save
```
![resize:256 rotate-color:blue:0.5 all apply save](images/resize%3A256%20rotate-color%3Ablue%3A0.5%20all%20apply%20save.png)

```
$ degenerate resize:256 rotate-color:blue:1.0 all apply save
```
![resize:256 rotate-color:blue:1.0 all apply save](images/resize%3A256%20rotate-color%3Ablue%3A1.0%20all%20apply%20save.png)

```
$ degenerate resize:256 rotate-color:g:0.5 all apply save
```
![resize:256 rotate-color:g:0.5 all apply save](images/resize%3A256%20rotate-color%3Ag%3A0.5%20all%20apply%20save.png)

```
$ degenerate resize:256 rotate-color:green:0.5 all apply save
```
![resize:256 rotate-color:green:0.5 all apply save](images/resize%3A256%20rotate-color%3Agreen%3A0.5%20all%20apply%20save.png)

```
$ degenerate resize:256 rotate-color:green:1.0 all save
```
![resize:256 rotate-color:green:1.0 all save](images/resize%3A256%20rotate-color%3Agreen%3A1.0%20all%20save.png)

```
$ degenerate resize:256 rotate-color:r:0.5 all apply save
```
![resize:256 rotate-color:r:0.5 all apply save](images/resize%3A256%20rotate-color%3Ar%3A0.5%20all%20apply%20save.png)

```
$ degenerate resize:256 rotate-color:red:0.5 all apply save
```
![resize:256 rotate-color:red:0.5 all apply save](images/resize%3A256%20rotate-color%3Ared%3A0.5%20all%20apply%20save.png)

```
$ degenerate resize:256 rotate-color:red:1.0 all save
```
![resize:256 rotate-color:red:1.0 all save](images/resize%3A256%20rotate-color%3Ared%3A1.0%20all%20save.png)

```
$ degenerate resize:256 rotate:0.05 scale:2 x apply save
```
![resize:256 rotate:0.05 scale:2 x apply save](images/resize%3A256%20rotate%3A0.05%20scale%3A2%20x%20apply%20save.png)

```
$ degenerate resize:256 rotate:0.05 x apply save
```
![resize:256 rotate:0.05 x apply save](images/resize%3A256%20rotate%3A0.05%20x%20apply%20save.png)

```
$ degenerate resize:256 rotate:0.125 square apply save
```
![resize:256 rotate:0.125 square apply save](images/resize%3A256%20rotate%3A0.125%20square%20apply%20save.png)

```
$ degenerate resize:256 rotate:1.0 square apply save
```
![resize:256 rotate:1.0 square apply save](images/resize%3A256%20rotate%3A1.0%20square%20apply%20save.png)

```
$ degenerate resize:256 rows:18446744073709551615:18446744073709551615 apply save
```
![resize:256 rows:18446744073709551615:18446744073709551615 apply save](images/resize%3A256%20rows%3A18446744073709551615%3A18446744073709551615%20apply%20save.png)

```
$ degenerate resize:256 rows:1:1 apply save
```
![resize:256 rows:1:1 apply save](images/resize%3A256%20rows%3A1%3A1%20apply%20save.png)

```
$ degenerate resize:256 save
```
![resize:256 save](images/resize%3A256%20save.png)

```
$ degenerate resize:256 scale:0.5 circle apply all scale:0.9 wrap apply save
```
![resize:256 scale:0.5 circle apply all scale:0.9 wrap apply save](images/resize%3A256%20scale%3A0.5%20circle%20apply%20all%20scale%3A0.9%20wrap%20apply%20save.png)

```
$ degenerate resize:256 scale:0.5 circle apply save
```
![resize:256 scale:0.5 circle apply save](images/resize%3A256%20scale%3A0.5%20circle%20apply%20save.png)

```
$ degenerate resize:256 scale:0.5 circle wrap apply save
```
![resize:256 scale:0.5 circle wrap apply save](images/resize%3A256%20scale%3A0.5%20circle%20wrap%20apply%20save.png)

```
$ degenerate resize:256 scale:2 rotate:0.05 x apply save
```
![resize:256 scale:2 rotate:0.05 x apply save](images/resize%3A256%20scale%3A2%20rotate%3A0.05%20x%20apply%20save.png)

```
$ degenerate resize:256 scale:2 x apply save
```
![resize:256 scale:2 x apply save](images/resize%3A256%20scale%3A2%20x%20apply%20save.png)

```
$ degenerate resize:256 seed:2 random-mask apply save
```
![resize:256 seed:2 random-mask apply save](images/resize%3A256%20seed%3A2%20random-mask%20apply%20save.png)

```
$ degenerate resize:256 square apply save
```
![resize:256 square apply save](images/resize%3A256%20square%20apply%20save.png)

```
$ degenerate resize:256 square apply top apply save
```
![resize:256 square apply top apply save](images/resize%3A256%20square%20apply%20top%20apply%20save.png)

```
$ degenerate resize:256 top apply save
```
![resize:256 top apply save](images/resize%3A256%20top%20apply%20save.png)

```
$ degenerate resize:256 x apply save
```
![resize:256 x apply save](images/resize%3A256%20x%20apply%20save.png)

```
$ degenerate resize:3 default:0:255:0 scale:0.5 apply save
```
![resize:3 default:0:255:0 scale:0.5 apply save](images/resize%3A3%20default%3A0%3A255%3A0%20scale%3A0.5%20apply%20save.png)

```
$ degenerate resize:512:256 all apply save all apply load
```
![resize:512:256 all apply save all apply load](images/resize%3A512%3A256%20all%20apply%20save%20all%20apply%20load.png)

```
$ degenerate resize:512:256 rotate:0.05 x apply save load save
```
![resize:512:256 rotate:0.05 x apply save load save](images/resize%3A512%3A256%20rotate%3A0.05%20x%20apply%20save%20load%20save.png)

```
$ degenerate resize:512:256 save
```
![resize:512:256 save](images/resize%3A512%3A256%20save.png)
