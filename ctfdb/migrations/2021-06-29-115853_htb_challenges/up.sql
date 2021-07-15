CREATE TABLE `htb_challenges` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `htb_id` INT NOT NULL,
    `name` TEXT NOT NULL,
    `difficulty` TEXT NOT NULL,
    `points` TEXT NOT NULL,
    `release_date` TEXT NOT NULL,
    `challenge_category` INT NOT NULL,
    `working` TEXT NULL,
    `machine_avatar` TEXT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB;