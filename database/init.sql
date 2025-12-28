create table if not exists feed (
    id uuid,
    url varchar(2000) NOT NULL,
    title varchar(1000) NOT NULL,
    description text NULL,
    active bool DEFAULT true,
    PRIMARY KEY(id)
);

create table if not exists article (
    id uuid,
    feed_id uuid,
    url varchar(2000) NOT NULL,
    title varchar(1000) NOT NULL,
    content text NOT NULL,
    read bool DEFAULT false,
    published timestamptz NOT NULL,
    PRIMARY KEY(id),
    FOREIGN KEY (feed_id) REFERENCES feed(id)
)