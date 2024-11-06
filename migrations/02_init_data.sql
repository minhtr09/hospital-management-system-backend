
INSERT INTO tn_rooms (name, location)
VALUES ('Phòng 101', 'Khu A, tầng 1');

INSERT INTO tn_specialities (name, description, image)
VALUES ('Unknown', 'Chưa phân loại chuyên khoa', '');

-- ############################################################################
-- ## bác sĩ với 3 vai trò khác nhau: ADMIN - SUPPORTER - MEMBER
-- ############################################################################

INSERT INTO tn_doctors (email, phone, password, name, description, price, role, active, avatar, speciality_id, room_id, recovery_token)
VALUES ('phongkaster@gmail.com', '0366253623', '$2y$10$nODDRAA4Y8OhAnrVoWtkuuTepjjGMAjsCUQWsX4zHU1JiGrguVdoK', 
'Phong Kaster', 'Bác sĩ Phong', 159000, 'admin', 1,'', 1, 1, '');

INSERT INTO tn_doctors (email, phone, password, name, description, price, role, active, avatar, speciality_id, room_id, recovery_token)
VALUES ('phongkaster1@gmail.com', '0366253623', '$2y$10$nODDRAA4Y8OhAnrVoWtkuuTepjjGMAjsCUQWsX4zHU1JiGrguVdoK', 
'Phong Kaster', 'Bác sĩ Phong', 159000, 'supporter', 1,'', 1, 1, '');

INSERT INTO tn_doctors (email, phone, password, name, description, price, role, active, avatar, speciality_id, room_id, recovery_token)
VALUES ('phongkaster2@gmail.com', '0366253623', '$2y$10$nODDRAA4Y8OhAnrVoWtkuuTepjjGMAjsCUQWsX4zHU1JiGrguVdoK', 
'Phong Kaster', 'Bác sĩ Phong', 159000, 'member', 1,'', 1, 1, '');

INSERT INTO tn_patients (email, phone, password, name, gender, birthday, address, avatar)
VALUES ('emma@gmail.com', '0794104124', '$2y$10$nODDRAA4Y8OhAnrVoWtkuuTepjjGMAjsCUQWsX4zHU1JiGrguVdoK', 
'Emma', 0, '2000-05-01', 'USA', '');

-- ############################################################################
-- ## 10 lịch hẹn được tạo ra để dễ làm kiểm thử
-- ############################################################################

UPDATE tn_appointments
SET date = '2022-11-03';

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (2, 1, 'Bệnh nhân 1', '2022-02-22', 'Mệt mỏi', '0791234567', 1, 1,'', '2022-11-02', 'cancelled');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (2, 1, 'Bệnh nhân 2', '2022-02-22', 'Mệt mỏi', '0791234567', 2, 2,'', '2022-11-02', 'cancelled');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 3', '2022-02-22', 'Mệt mỏi', '0791234567', 3, 3,'', '2022-11-02', 'verified');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 4', '2022-02-22', 'Mệt mỏi', '0791234567', 4, 4,'', '2022-11-02', 'verified');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 5', '2022-02-22', 'Mệt mỏi', '0791234567', 5, 5,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 6', '2022-02-22', 'Mệt mỏi', '0791234567', 6, 6,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 7', '2022-02-22', 'Mệt mỏi', '0791234567', 7, 7,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 8', '2022-02-22', 'Mệt mỏi', '0791234567', 8, 8,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 9', '2022-02-22', 'Mệt mỏi', '0791234567', 9, 9,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 10', '2022-02-22', 'Mệt mỏi', '0791234567', 10, 10,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 11', '2022-02-22', 'Mệt mỏi', '0791234567', 11, 11,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 12', '2022-02-22', 'Mệt mỏi', '0791234567', 12, 12,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 13', '2022-02-22', 'Mệt mỏi', '0791234567', 13, 13,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 14', '2022-02-22', 'Mệt mỏi', '0791234567', 14, 14,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 15', '2022-02-22', 'Mệt mỏi', '0791234567', 15, 15,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 16', '2022-02-22', 'Mệt mỏi', '0791234567', 16, 16,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 17', '2022-02-22', 'Mệt mỏi', '0791234567', 17, 17,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 18', '2022-02-22', 'Mệt mỏi', '0791234567', 18, 18,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 19', '2022-02-22', 'Mệt mỏi', '0791234567', 19, 19,'', '2022-11-02', 'processing');

INSERT INTO tn_appointments (doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date ,status)
VALUES (3, 1, 'Bệnh nhân 20', '2022-02-22', 'Mệt mỏi', '0791234567', 20, 20,'', '2022-11-02', 'processing');

-- ############################################################################
-- ## 7 lịch hẹn khám - BOOKING 
-- ############################################################################
update tn_booking
set appointment_date = '2022-11-08';

update tn_booking
set status = 'processing';

update tn_booking
set booking_name = 'Phong';

INSERT INTO tn_booking (service_id, patient_id, booking_name, booking_phone, name, gender, birthday, address, reason, appointment_date, appointment_time ,status)
VALUES (1, 1, 'Phong', '0794104124', 'Emma', 0, '2000-01-01', 'Sweden', 'Khám thai định kì', '2022-11-06', '09:00', 'done');

INSERT INTO tn_booking (service_id, patient_id, booking_name, booking_phone, name, gender, birthday, address, reason, appointment_date, appointment_time ,status)
VALUES (1, 1, 'Phong', '0794104124', 'Jessica', 0, '2000-01-01', 'Sweden', 'Khám thai định kì', '2022-11-06', '09:00', 'done');

INSERT INTO tn_booking (service_id, patient_id, booking_name, booking_phone, name, gender, birthday, address, reason, appointment_date, appointment_time ,status)
VALUES (1, 2, 'Phong', '0794104124', 'Jeffrey Dahmer', 0, '2000-01-01', 'Sweden', 'Khám sức khỏe', '2022-11-06', '09:00', 'processing');

INSERT INTO tn_booking (service_id, patient_id, booking_name, booking_phone, name, gender, birthday, address, reason, appointment_date, appointment_time ,status)
VALUES (1, 2, 'Phong', '0794104124', 'Johnwick', 0, '2000-01-01', 'Sweden', 'Khám sức khỏe', '2022-11-06', '09:00', 'processing');

INSERT INTO tn_booking (service_id, patient_id, booking_name, booking_phone, name, gender, birthday, address, reason, appointment_date, appointment_time ,status)
VALUES (1, 2, 'Phong', '0794104124', 'Jeffrey Snow', 0, '2000-01-01', 'Sweden', 'Khám sức khỏe', '2022-11-06', '09:00', 'processing');

INSERT INTO tn_booking (service_id, patient_id, booking_name, booking_phone, name, gender, birthday, address, reason, appointment_date, appointment_time ,status)
VALUES (1, 2, 'Phong', '0794104124', 'John Snow', 0, '2000-01-01', 'Sweden', 'Khám sức khỏe', '2022-11-06', '09:00', 'processing');

INSERT INTO tn_booking (service_id, patient_id, booking_name, booking_phone, name, gender, birthday, address, reason, appointment_date, appointment_time ,status)
VALUES (1, 2, 'Phong', '0794104124', 'Bravo', 0, '2000-01-01', 'Sweden', 'Khám sức khỏe', '2022-11-06', '09:00', 'processing');

-- ############################################################################
-- ## Các dịch vụ khám bệnh
-- ############################################################################

INSERT INTO tn_services(name)
VALUES('Khám tổng quát');

INSERT INTO tn_services(name)
VALUES('Nhi khoa');

INSERT INTO tn_services(name)
VALUES('Chủng ngừa');

INSERT INTO tn_services(name)
VALUES('Sản phụ khoa');

INSERT INTO tn_services(name)
VALUES('Sức khỏe phụ nữ');

INSERT INTO tn_services(name)
VALUES('Nha khoa');

INSERT INTO tn_services(name)
VALUES('Nhãn khoa');

INSERT INTO tn_services(name)
VALUES('Tai mũi họng');

INSERT INTO tn_services(name)
VALUES('Tim mạch');

INSERT INTO tn_services(name)
VALUES('Nội tiết');

INSERT INTO tn_services(name)
VALUES('Thận & Tiết Niệu');

INSERT INTO tn_services(name)
VALUES('Tiêu hóa');

INSERT INTO tn_services(name)
VALUES('Hô hấp');

INSERT INTO tn_services(name)
VALUES('Thần kinh');

INSERT INTO tn_services(name)
VALUES('Nội khớp');

INSERT INTO tn_services(name)
VALUES('Da liệu');

INSERT INTO tn_services(name)
VALUES('Tầm soát STD');

INSERT INTO tn_services(name)
VALUES('Xét nghiệm PCR COVID-19');