FROM nimlang/nim:1.0.4 as development

ENV environment=dev

RUN apt-get update && apt-get install curl build-essential -y -qq

# Install entr
RUN curl -L http://eradman.com/entrproject/code/entr-4.3.tar.gz -o /tmp/entr-4.3 \
    && tar xvf /tmp/entr-4.3 -C /tmp \
    && cd /tmp/entr-4.3 \
    && ./configure; make \
    && ln -s /tmp/entr-4.3/entr /usr/local/bin

WORKDIR /app/src/pachamama

COPY pachamama.nimble /app/src/pachamama
COPY ./src ./src

RUN nimble test
RUN nimble build

CMD [ "nimble", "watch" ]
