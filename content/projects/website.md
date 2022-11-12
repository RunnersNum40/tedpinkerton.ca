---
title: "tedpinkerton.ca"
summary: This website is hosted on Netlify and built using Hugo's PaperMod theme
description: This website is hosted on Netlify and built using Hugo's PaperMod theme
tags: ["Web Development"]
---
# Motivation
I built and maintain this website for several reasons. I wanted to experiment with creating my own custom website without using some companies propratary platform, I wanted somewhere public to document my work and experience, and several people I respect highly have done the same.

# How it's built
## Static Site Generation
This website is built using [Hugo](https://gohugo.io/), a static site generator. Hugo takes collections of documents and applies themes to them. Once the documents are themed it compiles them all into a single static page with only one style sheet. Because the site is build only once per version it can be delivered immediately and hosted easily.

I am using the [PaperMod](https://github.com/adityatelange/hugo-PaperMod/) theme with some slight modifications.

## Netlify
[Netlify](https://www.netlify.com/) is a platform that hosts sites. It supports building websites with Hugo through Git repositories. All my code and content is hosted in a [GitHub repository](https://github.com/RunnersNum40/tedpinkerton.ca) and if any changes are pushed Netlify automatically rebuilds and hosts the new content.
![demo](/images/Netlify_Screenshot.png)

## Future Plans
I'd like to add more reactive content and build my own custom theme.

