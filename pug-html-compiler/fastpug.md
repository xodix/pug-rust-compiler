# Fastpug intermediate compilation

# File format bin

1 - Check if template is dumb or smart.

2 - Array of 'smart' snippets.

3 - Plain html that should be copy pasted.

4 - Snippet that is in separate scope that are handled by separate threads.

# 1

'Dumb template' is a normal html file with added fastpug headers.

'Smart template' is a normal html file with pug scripts.

# 2

This part of file contains positioning of smart snippets.

# 3

This part of file contains plain html.

example:

```html
<html>
  <head></head>
  <body></body>
</html>
```

# 4

This part of file contains smart snippets / templates.

```pug
	each i in [1,3,5,6]
		p i
```

# Order

File always starts with 1, than 2

followed by various combination of 3 and 4.
