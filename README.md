# hyper demo

## How to use docker

First build docker container.

```sh
$ docker build --rm --force-rm -t "hyper-demo:version" .
```

And then start docker container. Now visit to `http://localhost:3001`.

```sh
$ docker run -d --rm --name hyper-demo -v $PWD:/usr/src/app -p 3001:3001 hyper-demo:1
```
