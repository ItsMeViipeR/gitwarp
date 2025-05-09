# Gitwarp

# What is Gitwarp?

Gitwarp is a tool that helps you to manage git repositories by simplifying all the commands.

## Cloning

Git:

```bash
git clone <url>
```

Gitwarp:

```bash
gitwarp clone <url>
```

## Commiting

With Gitwarp, you can add, commit and push in one command. Let's see for 2 ways: committing all files and committing one file.

Add files:

Git:

```bash
git add --all
git commit -m "message"
git push
```

Gitwarp:

```bash
gitwarp commit -a -m "message" -p
```

Committing one file (or selected file):

Git:

```bash
git add file.txt
git commit -m "message"
git push
```

Gitwarp:

```bash
gitwarp commit -f file.txt -m "message" -p
```

Arguments doesn't have order

## Merging

Gitwarp, doesn't provide an easier way to merge branches

```bash
gitwarp merge <branch>
```

It'll merge the specified branch in your current branch.

### Pulling changes

To simplify changes pulling, gitwarp provide an unique command that does 2 things in 1.

Git:

```bash
git fetch
git pull [branch]
```

Gitwarp:

```bash
gitwarp pull [branch]
```
