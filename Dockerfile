FROM node:20-slim AS base

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"

RUN corepack enable

FROM base AS build
COPY . /usr/src/app
RUN mv /usr/src/app/.npmrc-docker /usr/src/app/.npmrc
WORKDIR /usr/src/app

RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile --shamefully-hoist

RUN pnpm run --filter=protobufs build
RUN pnpm run --filter=ai build
RUN pnpm run --filter=common build
RUN pnpm run -r build
RUN pnpm deploy --filter=@bosca/content --prod /prod/content
RUN pnpm deploy --filter=@bosca/graphql --prod /prod/graphql
RUN pnpm deploy --filter=@bosca/imageproxy --prod /prod/imageproxy
RUN pnpm deploy --filter=@bosca/workflow --prod /prod/workflow
RUN pnpm deploy --filter=@bosca/workflow-queue --prod /prod/workflow-queue
RUN pnpm deploy --filter=@bosca/workflow-workers --prod /prod/workflow-workers
RUN pnpm deploy --filter=@bosca/workflow-dashboard --prod /prod/workflow-dashboard
RUN pnpm deploy --filter=@bosca/bible-graphql --prod /prod/bible-graphql
RUN pnpm deploy --filter=@bosca/ui --prod /prod/ui

FROM base AS content
COPY --from=build /prod/content /prod/content
WORKDIR /prod/content
EXPOSE 7000
CMD [ "pnpm", "start" ]

FROM base AS graphql
COPY --from=build /prod/graphql /prod/graphql
WORKDIR /prod/graphql
EXPOSE 9000
CMD [ "pnpm", "start" ]

FROM base AS bible-graphql
COPY --from=build /prod/bible-graphql /prod/bible-graphql
WORKDIR /prod/bible-graphql
EXPOSE 2000
CMD [ "pnpm", "start" ]

FROM base AS imageproxy
COPY --from=build /prod/imageproxy /prod/imageproxy
WORKDIR /prod/imageproxy
EXPOSE 8002
CMD [ "pnpm", "start" ]

FROM base AS workflow
COPY --from=build /prod/workflow /prod/workflow
WORKDIR /prod/workflow
EXPOSE 7100
CMD [ "pnpm", "start" ]

FROM base AS workflow-queue
COPY --from=build /prod/workflow-queue /prod/workflow-queue
WORKDIR /prod/workflow-queue
EXPOSE 7200
CMD [ "pnpm", "start" ]

FROM base AS workflow-workers
COPY --from=build /prod/workflow-workers /prod/workflow-workers
WORKDIR /prod/workflow-workers
CMD [ "pnpm", "start" ]

FROM base AS workflow-workers-ml
ENV POETRY_HOME="/poetry"
WORKDIR /poetry
RUN apt update && apt install -y python3 curl ffmpeg
RUN curl -sSL https://install.python-poetry.org | python3 -
ENV PATH="$PATH:/poetry/bin"
ENV POETRY_PROGRAM="/poetry/bin/poetry"
ENV MEDIA_PYTHON_DIR="/prod/workflow-workers-py"

COPY --from=build /usr/src/app/workspace/workflow/media-py /prod/workflow-workers-py

RUN cd /prod/workflow-workers-py && poetry run pip install torch=="2.0.0+cpu" torchaudio=="2.0.0+cpu" -f https://download.pytorch.org/whl/torch_stable.html
RUN cd /prod/workflow-workers-py && LLVM_CONFIG=/usr/bin/llvm-config-14 poetry install

COPY --from=build /prod/workflow-workers /prod/workflow-workers
WORKDIR /prod/workflow-workers
CMD [ "pnpm", "start" ]

FROM base AS workflow-dashboard
COPY --from=build /prod/workflow-dashboard /prod/workflow-dashboard
WORKDIR /prod/workflow-dashboard
CMD [ "pnpm", "start" ]

FROM base AS ui
COPY --from=build /prod/ui /prod/ui
WORKDIR /prod/ui
CMD [ "pnpm", "start" ]