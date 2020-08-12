FROM postgres

RUN apt-get update && apt-get install -y postgresql-contrib

ADD ./docker/postgres/createExtension.sh /docker-entrypoint-initdb.d/
RUN chmod 755 /docker-entrypoint-initdb.d/createExtension.sh