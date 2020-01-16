FROM nimlang/nim:1.0.4 as development

ENV environment=dev

WORKDIR /app/src/pachamama

COPY pachamama.nimble /app/src/pachamama
COPY ./src ./src

RUN nimble test
RUN nimble build
