
# This is my blog

I want to build a blog using the ACTIX framework in rust.
It should serve static content which will be generated from markdown files 
during compilation. The compilation should happen in a CI/CD pipeline.

### Tech stack

* The web server will be implemented using ACTIX in native rust
* The compilation will most likely be done using Pandoc within a Docker container (or CI/CD runner), 
rusts documentation generation might be an alternative, more investigation needs to be done here
* the generation should be wrapped within a shell script
* the generated pages will be saved to the filesystem and served from there
* the actual blog entries will be written in pure markdown
* any additional content (e.g. pictures) will be saved on a CDN.
### Motivation

I wanted to tinker with Rust, so this is a good project.
The static serving/generation and usage of the CDN for other content came to me after 
[this](https://www.youtube.com/watch?v=mE-DGW0CcAk&t=1786s) talk by Scott Hanselman.


