CREATE TABLE `challenges` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `ctf_id` INT NOT NULL,
    `name` TEXT NOT NULL,
    `category` TEXT NOT NULL,
    `solved` BOOLEAN NOT NULL,
    `working` TEXT NULL,
    `solver` TEXT NULL,
    `points` INT NOT NULL,
    `solved_time` DATETIME NULL DEFAULT NULL,
    `announced_solve` BOOLEAN NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB;