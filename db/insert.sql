INSERT INTO `railway`.`train_type` (`name`)
VALUES
	("IRL"),
	("RL"),
	("CL");

INSERT INTO `railway`.`train` (`num`, `train_type_id`)
VALUES
	(321, 1),
	(331, 1),
	(201, 2),
	(100, 3),
	(101, 3);

INSERT INTO `railway`.`voyage` (`train_id`, `departure_datetime_absolute`)
VALUES
	(1, '2020-01-03 07:00:00'),
	(2, '2020-01-03 10:00:00'),
	(3, '2020-01-03 10:00:00'),
	(3, '2020-01-03 15:00:00'),
	(3, '2020-01-03 19:30:00'),
	(4, '2020-01-03 10:45:00'),
	(4, '2020-01-03 11:00:00'),
	(4, '2020-01-03 11:45:00'),
	(4, '2020-01-03 12:00:00'),
	(4, '2020-01-03 12:15:00'),
	(5, '2020-01-03 11:30:00'),
	(5, '2020-01-03 11:45:00'),
	(5, '2020-01-03 12:00:00'),
	(5, '2020-01-03 12:15:00');

INSERT INTO `railway`.`station` (`name`)
VALUES
	("Аэропорт Минск"),
	("Минск"),
	("Лебяжий"),
	("Ждановичи"),
	("Гродно"),
	("Гомель"),
	("Брест"),
	("Жлобин"),
	("Красный Берег");

INSERT INTO `railway`.`train_station` (`train_id`, `station_id`, `arrival_time_relative`, departure_time_relative)
VALUES
	(1, 5, NULL, '00:00:00'),
	(1, 2, '03:39:00', '03:45:00'),
	(1, 6, '07:00:00', NULL),

	(2, 2, NULL, '00:00:00'),
	(2, 7, '03:00:00', NULL),

	(3, 6, NULL, '00:00:00'),
	(3, 8, '01:00:00', '01:03:00'),
	(3, 9, '01:15:00', NULL),

	(4, 4, NULL, '00:00:00'),
	(4, 3, '00:03:00', '00:04:00'),
	(4, 2, '00:08:00', '00:11:00'),
	(4, 1, '00:50:00', NULL),

	(5, 1, NULL, '00:00:00'),
	(5, 2, '00:38:00', '00:40:00'),
	(5, 3, '00:44:00', '00:45:00'),
	(5, 4, '00:48:00', NULL);

INSERT INTO `railway`.`carriage_type` (`name`, `seats_count`)
VALUES
	("Спальный", 20),
	("Купе", 30),
	("Плацкарт", 40),
	("Сидячий", 50);

INSERT INTO `railway`.`carriage` (`voyage_id`, `carriage_type_id`, `number`)
VALUES
	(1, 1, 1),
	(1, 2, 2),
	(1, 2, 3),
	(1, 3, 4),
	(2, 2, 1),
	(2, 2, 2),
	(2, 3, 3),
	(2, 4, 5),
	(2, 4, 6);

-- INSERT INTO `railway`.`carriage_station` (`station_id`, `carriage_id`, `seats_state`, `seat_price`)
-- VALUES
-- 	(),

INSERT INTO `railway`.`passenger` (`passport_num`, `first_name`, `last_name`)
VALUES
	("HB2001011", "John", "Smith"),
	("MP2121216", "Иван", "Иванов"),
	("MP2401013", "Пётр", "Петров"),
	("MP1051014", "Сидор", "Сидоров"),
	("MP8055010", "Jeremiah", "Smith"),
	("HB7677435", "Anonymous", "Unknown");

-- INSERT INTO `railway`.`user`

-- INSERT INTO `railway`.`ticket`
