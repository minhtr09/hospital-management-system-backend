create table tn_specialities
(
	id serial primary key,
	name varchar(30),
	description varchar(255),
	image varchar(255)
);

create table tn_rooms
(
	id serial primary key,
	name varchar(15),
	location varchar(255)
);

create table tn_doctors
(
	id serial primary key,
	email varchar(255) unique,
	phone varchar(15),
	password varchar(255),
	name varchar(50),
	description varchar(255),
	role varchar(10),
	active int,
	avatar varchar(255),
	create_at timestamp,
	update_at timestamp,
	speciality_id int,
	room_id int,
	recovery_token varchar(255),
	FOREIGN KEY (speciality_id) REFERENCES tn_specialities(id)
);

create table tn_patients
(
	id serial primary key,
	email varchar(255) unique,
	phone varchar(15),
	password varchar(255),
	name varchar(50),
	gender int,
	birthday varchar(10),
	address varchar(255),
	avatar varchar(255),
	create_at timestamp,
	update_at timestamp
);

create table tn_appointments
(
	id serial primary key,
	patient_id int,
	patient_name varchar(50),
	patient_birthday varchar(10),
	patient_reason varchar(255),
	patient_phone varchar(15),
	numerical_order int,
	appointment_time varchar(20),
	date date,
	status varchar(15),
	create_at timestamp,
	update_at timestamp,
	speciality_id int,
	FOREIGN KEY (patient_id) REFERENCES tn_patients(id)
);

create table tn_appointment_records
(
	id serial primary key,
	appointment_id int,
	doctor_id int,
	room_id int,
	reason varchar(100),
	description text,
	status_before varchar(255),
	status_after varchar(255),
	create_at timestamp,
	update_at timestamp,
	FOREIGN KEY (appointment_id) REFERENCES tn_appointments(id)
);

create table tn_medical_records
(
	id serial primary key,
	appointment_id int,
	FOREIGN KEY (appointment_id) REFERENCES tn_appointments(id),
	payment_status int,
	patient_id int,
	doctor_id int,
	FOREIGN KEY (patient_id) REFERENCES tn_patients(id),
	FOREIGN KEY (doctor_id) REFERENCES tn_doctors(id),
	diagnosis varchar(255)
);

create table tn_invoices
(
	id serial primary key,
	medical_record_id int,
	FOREIGN KEY (medical_record_id) REFERENCES tn_medical_records(id),
	time timestamp,
	total_price int
);

create table tn_services
(
	id serial primary key,
    name varchar(255),
	image varchar(255),
	description TEXT,
	price int,
);

create table tn_doctor_and_service
(
	id serial primary key,
	service_id int,
	doctor_id int,
    FOREIGN KEY (doctor_id) REFERENCES tn_doctors(id),
	FOREIGN KEY (service_id) REFERENCES tn_services(id)
);

create table tn_booking
(
	id serial primary key,
	service_id int,
	patient_id int,
	booking_name varchar(50),
	booking_phone varchar(15),
	name varchar(50),
	gender int,
	birthday varchar(10),
	address varchar(255),
	reason varchar(255),
	appointment_date varchar(10),
	appointment_time varchar(5),
	status varchar(15),
	create_at timestamp,
	update_at timestamp,
	FOREIGN KEY (patient_id) REFERENCES tn_patients(id),
	FOREIGN KEY (service_id) REFERENCES tn_services(id)
);

create table tn_notifications
(
	id serial primary key,
	message TEXT,
	record_id int,
	record_type varchar(20),
	patient_id int,
	is_read int,
	create_at timestamp,
	update_at timestamp,
	FOREIGN KEY (patient_id) REFERENCES tn_patients(id)
);

create table tn_booking_photo
(
	id serial primary key,
	url varchar(255),
	booking_id int,
	FOREIGN KEY (booking_id) REFERENCES tn_booking(id)
);

create table tn_medicine
(
	id serial primary key,
	name varchar(255),
	price int,
	unit varchar(20),
	description text,
	manufacture_date timestamp,
	expiry_date timestamp,
	side_effects varchar(255),
	dosage varchar(255)
);

create table medicine_of_prescription
(
	id serial primary key,
	medical_record_id int,
	medicine_id int,
	quantity int,
	FOREIGN KEY (medical_record_id) REFERENCES tn_medical_records(id),
	FOREIGN KEY (medicine_id) REFERENCES tn_medicine(id)
);

create table tn_admins
(
	id serial primary key,
	email varchar(255) unique,
	phone varchar(15),
	password varchar(255),
	name varchar(50),
	gender int,
	birthday varchar(10),
	address varchar(255),
	avatar varchar(255),
	create_at timestamp,
	update_at timestamp
);

-- Add column price to table tn_services
ALTER TABLE tn_services ADD COLUMN price INTEGER;

ALTER TABLE tn_invoices ADD COLUMN service_ids integer[];

ALTER TABLE tn_appointments ADD COLUMN treatment_status varchar(15);

CREATE TABLE tn_vital_signs (
    id SERIAL PRIMARY KEY, -- or AUTO_INCREMENT for MySQL
	medical_record_id int,
    temperature int,
    blood_pressure_systolic int,
    blood_pressure_diastolic int,
    heart_rate int,
    spO2 int,
    weight int,
    height int,
	FOREIGN KEY (medical_record_id) REFERENCES tn_medical_records(id)
);


