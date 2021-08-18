CREATE DATABASE IF NOT EXISTS `railway`
DEFAULT CHARACTER SET utf8mb4
COLLATE 'utf8mb4_general_ci' ;
USE `railway` ;

CREATE TABLE IF NOT EXISTS `railway`.`train_type` (
  `id` TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(32) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `name_UNIQUE` (`name` ASC))
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `railway`.`train` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `num` INT UNSIGNED NOT NULL,
  `train_type_id` TINYINT UNSIGNED NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `num_UNIQUE` (`num` ASC))
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `railway`.`voyage` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `train_id` INT UNSIGNED NOT NULL,
  `departure_datetime_absolute` DATETIME NOT NULL,
  `late_by` TIME NOT NULL DEFAULT '00:00:00',
  `carriage_num_start` ENUM('head', 'tail') NULL,
  `track_num` TINYINT NULL,
  `platform_num` TINYINT NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_train_id_idx` (`train_id` ASC),
  CONSTRAINT `fk_voyage_train_id`
    FOREIGN KEY (`train_id`)
    REFERENCES `railway`.`train` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `railway`.`carriage_type` (
  `id` TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(32) NOT NULL,
  `seats_count` TINYINT NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `railway`.`carriage` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `voyage_id` INT UNSIGNED NOT NULL,
  `carriage_type_id` INT UNSIGNED NOT NULL,
  `number` TINYINT UNSIGNED NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `railway`.`station` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(32) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `name_UNIQUE` (`name` ASC))
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `railway`.`train_station` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `train_id` INT UNSIGNED NOT NULL,
  `station_id` INT UNSIGNED NOT NULL,
  `arrival_time_relative` TIME NULL,
  `departure_time_relative` TIME NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_train_id_idx` (`train_id` ASC),
  INDEX `fk_station_id_idx` (`station_id` ASC),
  CONSTRAINT `fk_train_station_train_id`
    FOREIGN KEY (`train_id`)
    REFERENCES `railway`.`train` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_train_station_station_id`
    FOREIGN KEY (`station_id`)
    REFERENCES `railway`.`station` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `railway`.`carriage_station` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `station_id` INT UNSIGNED NOT NULL,
  `carriage_id` INT UNSIGNED NOT NULL,
  `seats_state` BINARY(8) NOT NULL,
  `seat_price` DECIMAL(6,2) NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_carriage_id_idx` (`carriage_id` ASC),
  INDEX `fk_station_id_idx` (`station_id` ASC),
  CONSTRAINT `fk_carriage_station_station_id`
    FOREIGN KEY (`station_id`)
    REFERENCES `railway`.`station` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_carriage_station_carriage_id`
    FOREIGN KEY (`carriage_id`)
    REFERENCES `railway`.`carriage` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `railway`.`passenger` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `passport_num` CHAR(9) NOT NULL,
  `first_name` VARCHAR(64) CHARACTER SET 'utf8' NOT NULL,
  `last_name` VARCHAR(64) CHARACTER SET 'utf8' NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `email` (`passport_num` ASC))
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `railway`.`user` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `email` VARCHAR(255) NOT NULL,
  `pass` VARCHAR(255) NOT NULL,
  `is_admin` TINYINT NOT NULL DEFAULT 0,
  `is_active` TINYINT NOT NULL DEFAULT 1,
  PRIMARY KEY (`id`))
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `railway`.`ticket` (
  `id` INT UNSIGNED NOT NULL AUTO_INCREMENT,
  `user_id` INT UNSIGNED NOT NULL,
  `passenger_id` INT UNSIGNED NOT NULL,
  `carriage_station_id` INT UNSIGNED NOT NULL,
  `seat_num` TINYINT UNSIGNED NOT NULL,
  `price` DECIMAL(6,2) UNSIGNED NOT NULL,
  `sell_datetime` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `cancel_datetime` DATETIME NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_user_id_idx` (`user_id` ASC),
  CONSTRAINT `fk_ticket_user_id`
    FOREIGN KEY (`user_id`)
    REFERENCES `railway`.`user` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;
