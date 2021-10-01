CREATE TABLE `htb_team_rank` (
    `entry_id` INT NOT NULL AUTO_INCREMENT,
    `rank` INT NOT NULL,
    `points` INT NOT NULL,
    `timestamp` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`entry_id`)
) ENGINE = InnoDB;