# GitHub globs

## [Filter pattern cheat sheet](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#filter-pattern-cheat-sheet)

You can use special characters in path, branch, and tag filters.

\_: Matches zero or more characters, but does not match the / character. For example, Octo\* matches Octocat. \*\*: Matches zero or more of any character.  
?: Matches zero or one of the preceding character.  
+: Matches one or more of the preceding character.  
[] Matches one alphanumeric character listed in the brackets or included in ranges. Ranges can only include a-z, A-Z, and 0-9. For example, the range[0-9a-z] matches any digit or lowercase letter. For example, [CB]at matches Cat or Bat and [1-2]00 matches 100 and 200.  
!: At the start of a pattern makes it negate previous positive patterns. It has no special meaning if not the first character.  
The characters \_, [, and ! are special characters in YAML. If you start a pattern with \*, [, or !, you must enclose the pattern in quotes. Also, if you use a flow sequence with a pattern containing [ and/or ], the pattern must be enclosed in quotes.

```yml
# Valid
paths:
  - '**/README.md'

# Invalid - creates a parse error that
# prevents your workflow from running.
paths:
  - **/README.md

# Valid
branches: [ main, 'release/v[0-9].[0-9]' ]

# Invalid - creates a parse error
branches: [ main, release/v[0-9].[0-9] ]
```

For more information about branch, tag, and path filter syntax, see on.<push>.<branches|tags>, on.<pull_request>.<branches|tags>, and on.<push|pull_request>.paths.

## [Patterns to match file paths](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#patterns-to-match-file-paths)

Path patterns must match the whole path, and start from the repository's root.

| Pattern                                 | Description of matches                                                                                                                                                                        | Example matches                                                             |
| --------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| `'*'`                                   | The `*` wildcard matches any character, but does not match slash (`/`). The `*` character is a special character in YAML. When you start a pattern with `*`, you must use quotes.             | `README.md`<br>`server.rb`                                                  |
| `'*.jsx?'`                              | The `?` character matches zero or one of the preceding character.                                                                                                                             | `page.js`<br>`page.jsx`                                                     |
| `'**'`                                  | The `**` wildcard matches any character including slash (`/`). This is the default behavior when you don't use a `path` filter.                                                               | `all/the/files.md`                                                          |
| `'*.js'`                                | The `*` wildcard matches any character, but does not match slash (`/`). Matches all `.js` files at the root of the repository.                                                                | `app.js`<br>`index.js`                                                      |
| `'**.js'`                               | Matches all `.js` files in the repository.                                                                                                                                                    | `index.js`<br>`js/index.js`<br>`src/js/app.js`                              |
| `docs/*`                                | All files within the root of the `docs` directory only, at the root of the repository.                                                                                                        | `docs/README.md`<br>`docs/file.txt`                                         |
| `docs/**`                               | Any files in the `docs` directory and its subdirectories at the root of the repository.                                                                                                       | `docs/README.md`<br>`docs/mona/octocat.txt`                                 |
| `docs/**/*.md`                          | A file with a `.md` suffix anywhere in the `docs` directory.                                                                                                                                  | `docs/README.md`<br>`docs/mona/hello-world.md`<br>`docs/a/markdown/file.md` |
| `'**/docs/**'`                          | Any files in a `docs` directory anywhere in the repository.                                                                                                                                   | `docs/hello.md`<br>`dir/docs/my-file.txt`<br>`space/docs/plan/space.doc`    |
| `'**/README.md'`                        | A README.md file anywhere in the repository.                                                                                                                                                  | `README.md`<br>`js/README.md`                                               |
| `'**/*src/**'`                          | Any file in a folder with a `src` suffix anywhere in the repository.                                                                                                                          | `a/src/app.js`<br>`my-src/code/js/app.js`                                   |
| `'**/*-post.md'`                        | A file with the suffix `-post.md` anywhere in the repository.                                                                                                                                 | `my-post.md`<br>`path/their-post.md`                                        |
| `'**/migrate-*.sql'`                    | A file with the prefix `migrate-` and suffix `.sql` anywhere in the repository.                                                                                                               | `migrate-10909.sql`<br>`db/migrate-v1.0.sql`<br>`db/sept/migrate-v1.sql`    |
| `'*.md'`<br>`'!README.md'`              | Using an exclamation mark (`!`) in front of a pattern negates it. When a file matches a pattern and also matches a negative pattern defined later in the file, the file will not be included. | `hello.md`<br><br>Does not match:<br>`README.md`<br>`docs/hello.md`         |
| `'*.md'`<br>`'!README.md'`<br>`README*` | Patterns are checked sequentially. A pattern that negates a previous pattern will re-include file paths.                                                                                      | `hello.md`<br>`README.md`<br>`README.doc`                                   |
