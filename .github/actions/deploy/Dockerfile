FROM alpine
RUN apk add --update ansible
COPY entrypoint.sh /.
ENTRYPOINT ["entrypoint.sh"]