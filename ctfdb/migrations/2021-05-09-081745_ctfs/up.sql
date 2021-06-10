CREATE TABLE `ctfs` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `name` TEXT NOT NULL,
    `base_url` TEXT NOT NULL,
    `api_url` TEXT NOT NULL,
    `api_key` TEXT NOT NULL,
    `channel_id` BIGINT NOT NULL,
    `active` BOOLEAN NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB;