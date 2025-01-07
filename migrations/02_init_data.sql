-- Specialities
INSERT INTO tn_specialities (name, description, image) VALUES
('Cardiology', 'Heart and cardiovascular system specialists', 'cardiology.jpg'),
('Pediatrics', 'Medical care for children and adolescents', 'pediatrics.jpg'),
('Dermatology', 'Skin, hair, and nail specialists', 'dermatology.jpg');

-- Rooms
INSERT INTO tn_rooms (name, location) VALUES
('Room 101', 'First Floor, East Wing'),
('Room 102', 'First Floor, East Wing'),
('Room 201', 'Second Floor, West Wing');

-- Doctors
INSERT INTO tn_doctors (email, phone, password, name, description, price, role, active, avatar, create_at, update_at, speciality_id, room_id, recovery_token) VALUES
('dr.smith@hospital.com', '1234567890', 'hashed_password_1', 'Dr. John Smith', 'Experienced cardiologist', 150, 'doctor', 1, 'smith.jpg', NOW(), NOW(), 1, 1, NULL),
('dr.jones@hospital.com', '2345678901', 'hashed_password_2', 'Dr. Sarah Jones', 'Pediatric specialist', 120, 'doctor', 1, 'jones.jpg', NOW(), NOW(), 2, 2, NULL);

-- Patients
INSERT INTO tn_patients (email, phone, password, name, gender, birthday, address, avatar, create_at, update_at) VALUES
('patient1@email.com', '3456789012', 'hashed_password_3', 'Alice Brown', 1, '1990-05-15', '123 Main St', 'alice.jpg', NOW(), NOW()),
('patient2@email.com', '4567890123', 'hashed_password_4', 'Bob Wilson', 0, '1985-08-22', '456 Oak Ave', 'bob.jpg', NOW(), NOW());

-- Appointments
INSERT INTO tn_appointments (patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, appointment_time, date, status, create_at, update_at, speciality_id) VALUES
(1, 'Alice Brown', '1990-05-15', 'Annual checkup', '3456789012', 1, '09:00', '2024-02-01', 'scheduled', NOW(), NOW(), 1),
(2, 'Bob Wilson', '1985-08-22', 'Fever', '4567890123', 2, '10:00', '2024-02-01', 'scheduled', NOW(), NOW(), 2);

-- Appointment Records
INSERT INTO tn_appointment_records (appointment_id, doctor_id, room_id, reason, description, status_before, status_after, create_at, update_at) VALUES
(1, 1, 1, 'Regular checkup', 'Patient reported mild symptoms', 'scheduled', 'completed', NOW(), NOW()),
(2, 2, 2, 'Emergency', 'High fever treatment', 'scheduled', 'in-progress', NOW(), NOW());

-- Medical Records
INSERT INTO tn_medical_records (appointment_id, payment_status, patient_id, doctor_id, diagnosis) VALUES
(1, 1, 1, 1, 'Healthy, regular checkup completed'),
(2, 0, 2, 2, 'Viral fever, prescribed medication');

-- Invoices
INSERT INTO tn_invoices (medical_record_id, time, total_price) VALUES
(1, NOW(), 150),
(2, NOW(), 120);

-- Services
INSERT INTO tn_services (name, image, description) VALUES
('General Checkup', 'checkup.jpg', 'Complete body examination and health status evaluation'),
('Vaccination', 'vaccine.jpg', 'Various vaccination services available'),
('Blood Test', 'bloodtest.jpg', 'Comprehensive blood analysis');

-- Doctor and Service
INSERT INTO tn_doctor_and_service (service_id, doctor_id) VALUES
(1, 1),
(1, 2),
(2, 2);

-- Booking
INSERT INTO tn_booking (service_id, patient_id, booking_name, booking_phone, name, gender, birthday, address, reason, appointment_date, appointment_time, status, create_at, update_at) VALUES
(1, 1, 'Alice Brown', '3456789012', 'Alice Brown', 1, '1990-05-15', '123 Main St', 'Annual checkup', '2024-02-15', '09:00', 'pending', NOW(), NOW()),
(2, 2, 'Bob Wilson', '4567890123', 'Bob Wilson', 0, '1985-08-22', '456 Oak Ave', 'Vaccination', '2024-02-16', '10:30', 'confirmed', NOW(), NOW());

-- Notifications
INSERT INTO tn_notifications (message, record_id, record_type, patient_id, is_read, create_at, update_at) VALUES
('Your appointment has been confirmed', 1, 'appointment', 1, 0, NOW(), NOW()),
('Reminder: Upcoming appointment tomorrow', 2, 'appointment', 2, 0, NOW(), NOW());

-- Booking Photo
INSERT INTO tn_booking_photo (url, booking_id) VALUES
('photo1.jpg', 1),
('photo2.jpg', 2);

-- Medicine
INSERT INTO tn_medicine (name, price, unit, description, manufacture_date, expiry_date, side_effects, dosage) VALUES
('Paracetamol', 5, 'tablet', 'Pain reliever and fever reducer', '2023-01-01', '2025-01-01', 'Mild nausea in rare cases', '1-2 tablets every 6 hours'),
('Amoxicillin', 10, 'capsule', 'Antibiotic', '2023-06-01', '2025-06-01', 'May cause diarrhea', '1 capsule three times daily');

-- Medicine of Prescription
INSERT INTO medicine_of_prescription (medical_record_id, medicine_id, quantity) VALUES
(1, 1, 10),
(2, 2, 15);
