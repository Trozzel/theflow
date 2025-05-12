FROM postgres:16

# Set environment for postgres user
ENV POSTGRES_USER=gford
ENV POSTGRES_PASSWORD=secret
ENV POSTGRES_DB=theflow

# Copy schema file into init directory
COPY init/ /docker-entrypoint-initdb.d/
