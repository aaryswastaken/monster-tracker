CREATE DATABASE IF NOT EXISTS monster_tracker;

CREATE TABLE IF NOT EXISTS monster_tracker.items (
    id INT NOT NULL AUTO INCREMENT,
    name VARCHAR(150) NOT NULL,

    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS monster_tracker.vendors (
    vid INT NOT NULL AUTO INCREMENT,
    name VARCHAR(75) NOT NULL,
    internal_id INT NOT NULL
);

CREATE TABLE IF NOT EXISTS monster_tracker.specific_id (
    uid INT NOT NULL,
    vid INT NOT NULL,
    eid VARCHAR(250) NOT NULL

    FOREIGN KEY (uid)
        REFERENCES monster_tracker.items(id)
        ON DELETE NO ACTION,

    FOREIGN KEY (vid)
        REFERENCES monster_tracker.vendors(vid)
        ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS monster_tracker.shops (
    sid INT NOT NULL AUTO INCREMENT,
    name VARCHAR(250) NOT NULL,
    location VARCHAR(5000) NOT NULL,
    zip_code BIGINT,
    `latitude` DECIMAL(6,5) NOT NULL,
    `longitude` DECIMAL(6,5) NOT NULL,
    vendor INT NOT NULL,
    specific_cookie VARCHAR(5000)

    FOREIGN KEY (vendor)
        REFERENCES monster_tracker.vendors(vid)
        ON DELETE NO ACTION,
);

CREATE TABLE IF NOT EXISTS monster_tracker.prices (
    record_id BIGINT NOT NULL AUTO INCREMENT,
    item_id INT NOT NULL,
    shop_id INT NOT NULL,
    date DATETIME NOT NULL,
    value DECIMAL(8,2) NOT NULL,

    FOREIGN KEY (item_id)
        REFERENCES monster_tracker.items(id)
        ON DELETE NO ACTION,

    FOREIGN KEY (shop_id)
        REFERENCES monster_tracker.shops(sid)
        ON DELETE NO ACTION
);


