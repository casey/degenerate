# degenerate

Degenerate is an algorithmic image generator base on image filter chains. It is inspired by [blaster](https://github.com/casey/blaster).

## Usage

```bash
$ degenerate [COMMAND]...
```

## Gallery

```
$ degenerate resize:256:256 random-filter save
```
![resize:256:256 random-filter save](images/resize%3A256%3A256%20random-filter%20save.png)

```
$ degenerate resize:512:256 all save all load
```
![resize:512:256 all save all load](images/resize%3A512%3A256%20all%20save%20all%20load.png)

```
$ degenerate resize:256:256 rotate-color:green:0.5 all save
```
![resize:256:256 rotate-color:green:0.5 all save](images/resize%3A256%3A256%20rotate-color%3Agreen%3A0.5%20all%20save.png)

```
$ degenerate resize:256:256 rows:1:1 save
```
![resize:256:256 rows:1:1 save](images/resize%3A256%3A256%20rows%3A1%3A1%20save.png)

```
$ degenerate resize:256:256 rotate-color:blue:0.5 all save
```
![resize:256:256 rotate-color:blue:0.5 all save](images/resize%3A256%3A256%20rotate-color%3Ablue%3A0.5%20all%20save.png)

```
$ degenerate resize:256:256 save
```
![resize:256:256 save](images/resize%3A256%3A256%20save.png)

```
$ degenerate resize:256:256 comment:foo save
```
![resize:256:256 comment:foo save](images/resize%3A256%3A256%20comment%3Afoo%20save.png)

```
$ degenerate resize:256:256 rotate:0.05 scale:2 x save
```
![resize:256:256 rotate:0.05 scale:2 x save](images/resize%3A256%3A256%20rotate%3A0.05%20scale%3A2%20x%20save.png)

```
$ degenerate resize:256:256 rows:18446744073709551615:18446744073709551615 save
```
![resize:256:256 rows:18446744073709551615:18446744073709551615 save](images/resize%3A256%3A256%20rows%3A18446744073709551615%3A18446744073709551615%20save.png)

```
$ degenerate resize:256:256 rotate:0.05 x save
```
![resize:256:256 rotate:0.05 x save](images/resize%3A256%3A256%20rotate%3A0.05%20x%20save.png)

```
$ degenerate comment:ignore resize:256:256 rotate-color:g:0.07 rotate:0.07 for:10 x loop rotate-color:b:0.09 rotate:0.09 for:10 x loop save
```
![comment:ignore resize:256:256 rotate-color:g:0.07 rotate:0.07 for:10 x loop rotate-color:b:0.09 rotate:0.09 for:10 x loop save](images/comment%3Aignore%20resize%3A256%3A256%20rotate-color%3Ag%3A0.07%20rotate%3A0.07%20for%3A10%20x%20loop%20rotate-color%3Ab%3A0.09%20rotate%3A0.09%20for%3A10%20x%20loop%20save.png)

```
$ degenerate comment:ignore resize:256:256 scale:0.99 for:100 circle loop save
```
![comment:ignore resize:256:256 scale:0.99 for:100 circle loop save](images/comment%3Aignore%20resize%3A256%3A256%20scale%3A0.99%20for%3A100%20circle%20loop%20save.png)

```
$ degenerate resize:256:256 scale:2 rotate:0.05 x save
```
![resize:256:256 scale:2 rotate:0.05 x save](images/resize%3A256%3A256%20scale%3A2%20rotate%3A0.05%20x%20save.png)

```
$ degenerate resize:256:256 rotate-color:red:1.0 all save
```
![resize:256:256 rotate-color:red:1.0 all save](images/resize%3A256%3A256%20rotate-color%3Ared%3A1.0%20all%20save.png)

```
$ degenerate comment:ignore resize:256:256 rotate-color:red:0.083333 rotate:0.1 for:12 circle cross x loop save
```
![comment:ignore resize:256:256 rotate-color:red:0.083333 rotate:0.1 for:12 circle cross x loop save](images/comment%3Aignore%20resize%3A256%3A256%20rotate-color%3Ared%3A0.083333%20rotate%3A0.1%20for%3A12%20circle%20cross%20x%20loop%20save.png)

```
$ degenerate resize:256:256 top save
```
![resize:256:256 top save](images/resize%3A256%3A256%20top%20save.png)

```
$ degenerate resize:256:256 mod:3:0 save
```
![resize:256:256 mod:3:0 save](images/resize%3A256%3A256%20mod%3A3%3A0%20save.png)

```
$ degenerate resize:256:256 rotate:1.0 square save
```
![resize:256:256 rotate:1.0 square save](images/resize%3A256%3A256%20rotate%3A1.0%20square%20save.png)

```
$ degenerate resize:256:256 square top save
```
![resize:256:256 square top save](images/resize%3A256%3A256%20square%20top%20save.png)

```
$ degenerate resize:256:256 rotate-color:r:0.5 all save
```
![resize:256:256 rotate-color:r:0.5 all save](images/resize%3A256%3A256%20rotate-color%3Ar%3A0.5%20all%20save.png)

```
$ degenerate resize:256:256 rotate:0.125 square save
```
![resize:256:256 rotate:0.125 square save](images/resize%3A256%3A256%20rotate%3A0.125%20square%20save.png)

```
$ degenerate resize:256:256 rotate-color:red:0.5 all save
```
![resize:256:256 rotate-color:red:0.5 all save](images/resize%3A256%3A256%20rotate-color%3Ared%3A0.5%20all%20save.png)

```
$ degenerate resize:256:256 cross save
```
![resize:256:256 cross save](images/resize%3A256%3A256%20cross%20save.png)

```
$ degenerate resize:512:256 rotate:0.05 x save load save
```
![resize:512:256 rotate:0.05 x save load save](images/resize%3A512%3A256%20rotate%3A0.05%20x%20save%20load%20save.png)

```
$ degenerate resize:256:256 square save
```
![resize:256:256 square save](images/resize%3A256%3A256%20square%20save.png)

```
$ degenerate resize:256:256 x save
```
![resize:256:256 x save](images/resize%3A256%3A256%20x%20save.png)

```
$ degenerate resize:256:256 rotate-color:b:0.5 all save
```
![resize:256:256 rotate-color:b:0.5 all save](images/resize%3A256%3A256%20rotate-color%3Ab%3A0.5%20all%20save.png)

```
$ degenerate resize:256:256 scale:2 x save
```
![resize:256:256 scale:2 x save](images/resize%3A256%3A256%20scale%3A2%20x%20save.png)

```
$ degenerate resize:256:256 seed:2 random-filter save
```
![resize:256:256 seed:2 random-filter save](images/resize%3A256%3A256%20seed%3A2%20random-filter%20save.png)

```
$ degenerate resize:256:256 rotate-color:g:0.5 all save
```
![resize:256:256 rotate-color:g:0.5 all save](images/resize%3A256%3A256%20rotate-color%3Ag%3A0.5%20all%20save.png)

```
$ degenerate resize:512:256 save
```
![resize:512:256 save](images/resize%3A512%3A256%20save.png)

```
$ degenerate resize:256:256 rotate-color:blue:1.0 all save
```
![resize:256:256 rotate-color:blue:1.0 all save](images/resize%3A256%3A256%20rotate-color%3Ablue%3A1.0%20all%20save.png)

```
$ degenerate resize:256:256 rotate-color:green:1.0 all save
```
![resize:256:256 rotate-color:green:1.0 all save](images/resize%3A256%3A256%20rotate-color%3Agreen%3A1.0%20all%20save.png)

```
$ degenerate resize:256:256 all save
```
![resize:256:256 all save](images/resize%3A256%3A256%20all%20save.png)

```
$ degenerate comment:ignore resize:256:256 rotate:0.111 for:16 square circle loop save
```
![comment:ignore resize:256:256 rotate:0.111 for:16 square circle loop save](images/comment%3Aignore%20resize%3A256%3A256%20rotate%3A0.111%20for%3A16%20square%20circle%20loop%20save.png)

```
$ degenerate comment:ignore resize:256:256 seed:12462 rotate-color:g:0.1 rotate:0.1 for:10 random-filter loop rotate-color:b:0.1 rotate:0.1 for:10 random-filter loop save
```
![comment:ignore resize:256:256 seed:12462 rotate-color:g:0.1 rotate:0.1 for:10 random-filter loop rotate-color:b:0.1 rotate:0.1 for:10 random-filter loop save](images/comment%3Aignore%20resize%3A256%3A256%20seed%3A12462%20rotate-color%3Ag%3A0.1%20rotate%3A0.1%20for%3A10%20random-filter%20loop%20rotate-color%3Ab%3A0.1%20rotate%3A0.1%20for%3A10%20random-filter%20loop%20save.png)

```
$ degenerate comment:ignore resize:256:256 seed:19798 rotate-color:g:0.01 rotate:0.01 for:100 random-filter loop rotate-color:b:0.01 rotate:0.01 for:100 random-filter loop save
```
![comment:ignore resize:256:256 seed:19798 rotate-color:g:0.01 rotate:0.01 for:100 random-filter loop rotate-color:b:0.01 rotate:0.01 for:100 random-filter loop save](images/comment%3Aignore%20resize%3A256%3A256%20seed%3A19798%20rotate-color%3Ag%3A0.01%20rotate%3A0.01%20for%3A100%20random-filter%20loop%20rotate-color%3Ab%3A0.01%20rotate%3A0.01%20for%3A100%20random-filter%20loop%20save.png)

```
$ degenerate resize:256:256 scale:0.5 circle save
```
![resize:256:256 scale:0.5 circle save](images/resize%3A256%3A256%20scale%3A0.5%20circle%20save.png)

```
$ degenerate resize:256:256 circle save
```
![resize:256:256 circle save](images/resize%3A256%3A256%20circle%20save.png)
