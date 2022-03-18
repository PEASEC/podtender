FROM alpine:latest
COPY run.sh /
RUN chmod +x /run.sh
HEALTHCHECK --timeout=3s CMD whoami
CMD ["/run.sh"]