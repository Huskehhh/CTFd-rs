CREATE TABLE `htb_solves` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `user_id` INT NOT NULL,
    `username` TEXT NOT NULL,
    `challenge_id` INT NOT NULL,
    `solve_type` TEXT NOT NULL,
    `announced` BOOLEAN NOT NULL,
    `solved_time` DATETIME NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB;