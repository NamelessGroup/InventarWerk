# syntax=docker/dockerfile:1
#
# Frontend + reverse proxy for all backend services.
# The build context is the repository root:
#   docker build -f frontend/Dockerfile --build-arg VERSION=$(git rev-parse --short=6 HEAD) .

FROM node:24-alpine AS build
WORKDIR /app
COPY frontend/package.json frontend/package-lock.json ./
# --ignore-scripts: the "prepare" script installs git hooks (husky) and needs the repo
RUN npm ci --ignore-scripts
COPY frontend/ .
# Replaces fetchVersion.cjs, which needs .git
ARG VERSION=dev
RUN echo "export const version = '${VERSION}'" > src/utils/version.ts \
 && npm run type-check \
 && npm run build-only:prod

FROM nginx:1.28-alpine
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80
