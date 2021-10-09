CREATE TABLE `htb_user_id_mapping` (
    `entry_id` INT NOT NULL AUTO_INCREMENT,
    `htb_id` INT NOT NULL,
    `discord_id` BIGINT NOT NULL,
    PRIMARY KEY (`entry_id`)
) ENGINE = InnoDB;