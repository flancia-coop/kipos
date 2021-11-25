# Kipos

<img src="./etc/jp.jpg" align="left" height="150" width="auto">

Hello, welcome to our abode. The idea and goal of Kipos is to aggregate your decentralized network of content together. For someone new in these parts, this means creating *one* source of truth, be that your Twitter feed, your markdown files, and/or your Spotify listening history — without centralizing your data.

## Theory

We have conventionally viewed, for instance, Twitter and a Blog as different things, but ultimately, truly, they are both gardens. When we begin to view our internet as a bunch of gardens, it opens up many opportunities for the representation of all these gardens blended together.

### Goals

Let's save the obvious. Sure, our flagship feature is the consolidation of all your things, but what else, and how do we do it?

#### Streams flow at different speeds

We recognize streams flow at different streams, So we must find a good middle ground between fully static (Hugo), and fully dynamic (Twitter). Our solution is to run a daemon that, when the time is right, builds relevant pages, like new, edited, and dependents (representations). then runs post tasks like syncing to a remote host.

#### Zero batteries ~~included~~ needed

A big frustration for those who are decentralized is dealing with services that have a huge superiority complex, telling you where your files should live. We don't make any assumptions, and encourage plugin makers to do the same. Sure, provide sane defaults, but don't ever tell the user what they can and can't do, where they can and can't share.

#### Usable & Configurable

As a result of our decentralization, Kipos is, unfortunately, configuration-heavy. This is a reality we must accept, but one that we can help reduce. We provide sane defaults through the `std.yml` import, and encourage garden plugin makers to do the same. Once we have achieved all our dreams, we will offer a web UI for configuration, making it easy for anyone to set up kipos, without ever opening a file.

#### Slow

Speed is a silly thing to strive for, if it means taking freedoms away from our users. The daemon's cashe should hopefully reduce the time it takes to build significantly. Once content is fetched, it is stored and never fetched again unless your computer has some serious problems and then well sucks for you your computer has serious problems. The only thing that would be noticeably long is if you change templates causing each page to be rebuilt. If you need breathtaking speed, look at Hugo, ElderJS, and soon possibly astro.

## In practice

*Enough PR nonsense. This section describes the workflow as we hurry to implement it. It's long, and in debth*

On boot, the dameon will form an single configuration from your decentralized configs, if you have any. It will load all the imported plugin's into memory.

Before we continue, some important concepts

### Node

a node is one item, a post, commonly. a node is made up of

```
*
├─ section
│  ├─ gardens
│  │  ├─ markdown_notes
│  │  │  ├─ config
│  │  ├─ markdown_biyearly
│  │  │  ├─ config
│  ├─ templates
│  │  ├─ index.html
│  │  ├─ preview.html
│  │  ├─ special
│  │  │  ├─ markdown_biyearly
│  │  │  │  ├─ anything.html
```

## Monitization

### Donations

I accept donations at https://flattr.com/@evan.

If that does not flattr you than donate me a kind email.

#### Why flattr

It allows you to pay a single monthly fee and support many great projects at your leasure. It's hard to pick precentages! I hope that in signing up for flattr you will be encouraged to suppport those who deserve it even more than me.

### SaaS

We will eventually host people's daemons, should they need it. It will be offered at the user's choice of price, and will also include webhosting, although we encourage you to do that yourself @ pages.sr.ht.

Until then, consider hosting your daemon with the good people at https://uberspace.de
