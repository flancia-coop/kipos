**⚠ This is a mirror of https://sr.ht/~boehs/kipos/. Pull requests are DOA, Evan likes sourcehut.**
# Kipos

> You lovin this readme? great, this is the only part of kipos that exists. At the moment, we have an idea.

<img src="./etc/jp.jpg" align="left" height="150" width="auto">

Hello, welcome to our abode. The idea and goal of Kipos is to aggregate your decentralized network of content together. For someone new in these parts, this means creating *one* source of truth, be that your Twitter feed, your markdown files, and/or your Spotify listening history — without centralizing your data.

## Theory

We have conventionally viewed, for instance, Twitter and a Blog as different things, but ultimately, truly, they are both gardens. When we begin to view our internet as a bunch of gardens, it opens up many opportunities for the representation of all these gardens blended together.

### Goals

Let's save the obvious. Sure, our flagship feature is the consolidation of all your things, but what else, and how do we do it?

#### Streams flow at different speeds

We recognize streams flow at different speeds, So we must find a good middle ground between fully static (Hugo), and fully dynamic (Twitter). Our solution is to run a daemon that, when the time is right, builds relevant pages, like new, edited, and dependents (representations). It then runs post tasks like syncing to a remote host.

#### Zero batteries ~~included~~ needed

A big frustration for those who are decentralized is dealing with services that have a huge superiority complex, telling you where your files should live. We don't make any assumptions and encourage plugin makers to do the same. Sure, provide sane defaults, but don't ever tell the user what they can and can't do, where they can and can't share.

#### Usable & Configurable

As a result of our decentralization, Kipos is, unfortunately, configuration-heavy, but we strive to make it as intuitive as possible, prioritizing file based layouts. Once we have achieved all our dreams, we will offer a web UI for configuration, making it easy for anyone to set up Kipos, without ever opening a file.

#### Slow

Speed is a silly thing to strive for if it means taking freedoms away from our users. The daemon's cache should hopefully reduce the time it takes to build significantly. Once content is fetched, it is stored and never fetched again unless your computer has some serious problems and then well sucks for you your computer has serious problems. The only thing that would be noticeably long is if you change templates causing each page to be rebuilt. If you need breathtaking speed, look at Hugo, ElderJS, and soon possibly astro.

## In practice

*Enough PR nonsense. This section describes the workflow as we hurry to implement it. It's long, and in depth*

### Node

A node is one item, a post, commonly. a node is really two concepts, views, and

#### Metadata

For each node, unstructured JSON-like data is created at will, containing whatever the plugin author wants. Typically, this will look like:

```json
{
    "metadata": {
        ...
    },
    "content": """"""
}
```

Content is whatever format makes sense, Markdown, JSON, Etc. This is for Gemini support and other abstract exports (did you know we are not a static site generator but just a static generator?).

Internally, this *is* the `node`, `nodes` is an array of these objects.

#### View

a view is a way to represent that node, under `views/`. Views support folders, etc, and the final filesystem will generally reflect the structure of views.

```
# note, most of this can be configured, if you want an unorganized pile of shit set all paths to `./` in config.
./
├─ md_section
│  ├─ md_notes
│  │  ├─ node1
│  │  │  ├─ index.html
│  │  │  ├─ preview.html
│  │  ├─ node2
│  │  │  ├─ ...
│  ├─ md_biyearly
│  │  ├─ ...
```

There is one reserved folder in `views`, that is `special`, and inside of special folders can be created with the same garden name. Depending on the garden the node comes from, the views contained within `special/nodesgarden` will be rendered as if they were outside in the normal views directory. If for some reason you need the name `special`, feel free to change the dir config to `"eeeeeeeeeeeeee"` or something.

The actual file layout looks like

```
۞
├─ layout
|  ├─ md_section
│  │  ├─ gardens
│  │  │  ├─ md_notes
│  │  │  │  ├─ config
│  │  │  ├─ md_biyearly
│  │  │  │  ├─ config
│  │  ├─ views
│  │  │  ├─ index.html
│  │  │  ├─ preview.html
│  │  │  ├─ special
│  │  │  │  ├─ markdown_biyearly
│  │  │  │  │  ├─ anything.html
```

Working our way up, we find:

### Gardens

A garden is a place, somewhere. Your Twitter account, a blog you run, anything else.

Each section can have multiple gardens, and each garden generates a list of nodes from somewhere. Not much exciting.

### Section

Sections contain gardens and views.

Lots of gardens you might have are similar. They are distinct, sure, but ought to be rendered similarly. Had we integrated views as a part of gardens, this would cause serious duplication. On the other hand, small distinctions are often needed, so we provide the `special` folder. Sections are a good middle ground.

---

## Monetization

Kipos has always, will always be freeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee.

<!-->
### Donations

I accept donations at https://flattr.com/@evan. Of course, these donations allow me to dedicate more time to Kipos, writing, and the greater flancia.

If that does not Flattr you, donate me a kind email.

#### Why flattr

It allows you to pay a single monthly fee and support many great projects at your leisure. It's hard to pick percentages! I hope that by signing up for Flattr you will be encouraged to support those who deserve it more than me.

### SaaS

We will eventually host people's daemons, should they need it. It will be offered at the user's choice of price, and will also include webhosting, although we encourage you to do that yourself @ pages.sr.ht.

Until then, consider hosting your daemon with the good people at https://uberspace.de
