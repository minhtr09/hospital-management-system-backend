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
INSERT INTO tn_doctors (email, phone, password, name, description, role, active, avatar, create_at, update_at, speciality_id, room_id, recovery_token) VALUES
('dr.smith@hospital.com', '1234567890', 'hashed_password_1', 'Dr. John Smith', 'Experienced cardiologist', 'doctor', 1, 'smith.jpg', NOW(), NOW(), (SELECT id FROM tn_specialities WHERE name = 'Cardiology'), 1, NULL),
('dr.jones@hospital.com', '2345678901', 'hashed_password_2', 'Dr. Sarah Jones', 'Pediatric specialist', 'doctor', 1, 'jones.jpg', NOW(), NOW(), (SELECT id FROM tn_specialities WHERE name = 'Pediatrics'), 2, NULL);

-- Patients
INSERT INTO tn_patients (email, phone, password, name, gender, birthday, address, avatar, create_at, update_at) VALUES
('patient1@email.com', '3456789012', 'hashed_password_3', 'Alice Brown', 1, '1990-05-15', '123 Main St', 'alice.jpg', NOW(), NOW()),
('patient2@email.com', '4567890123', 'hashed_password_4', 'Bob Wilson', 0, '1985-08-22', '456 Oak Ave', 'bob.jpg', NOW(), NOW());

-- Appointments
INSERT INTO tn_appointments (patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, appointment_time, date, status, create_at, update_at, speciality_id) VALUES
((SELECT id FROM tn_patients WHERE name = 'Alice Brown'), 'Alice Brown', '1990-05-15', 'Annual checkup', '3456789012', 1, '09:00', '2024-02-01', 'scheduled', NOW(), NOW(), (SELECT id FROM tn_specialities WHERE name = 'Cardiology')),
((SELECT id FROM tn_patients WHERE name = 'Bob Wilson'), 'Bob Wilson', '1985-08-22', 'Fever', '4567890123', 2, '10:00', '2024-02-01', 'scheduled', NOW(), NOW(), (SELECT id FROM tn_specialities WHERE name = 'Pediatrics'));

-- Appointment Records
INSERT INTO tn_appointment_records (appointment_id, doctor_id, room_id, reason, description, status_before, status_after, create_at, update_at) VALUES
((SELECT id FROM tn_appointments WHERE patient_name = 'Alice Brown' AND date = '2024-02-01'), (SELECT id FROM tn_doctors WHERE name = 'Dr. John Smith'), 1, 'Regular checkup', 'Patient reported mild symptoms', 'scheduled', 'completed', NOW(), NOW()),
((SELECT id FROM tn_appointments WHERE patient_name = 'Bob Wilson' AND date = '2024-02-01'), (SELECT id FROM tn_doctors WHERE name = 'Dr. Sarah Jones'), 2, 'Emergency', 'High fever treatment', 'scheduled', 'in-progress', NOW(), NOW());

-- Medical Records
INSERT INTO tn_medical_records (appointment_id, payment_status, patient_id, doctor_id, diagnosis) VALUES
((SELECT id FROM tn_appointments WHERE patient_name = 'Alice Brown' AND date = '2024-02-01'), 1, (SELECT id FROM tn_patients WHERE name = 'Alice Brown'), (SELECT id FROM tn_doctors WHERE name = 'Dr. John Smith'), 'Healthy, regular checkup completed'),
((SELECT id FROM tn_appointments WHERE patient_name = 'Bob Wilson' AND date = '2024-02-01'), 0, (SELECT id FROM tn_patients WHERE name = 'Bob Wilson'), (SELECT id FROM tn_doctors WHERE name = 'Dr. Sarah Jones'), 'Viral fever, prescribed medication');

--- Invoices
INSERT INTO tn_invoices (medical_record_id, time, total_price)
VALUES
    ((SELECT id FROM tn_medical_records WHERE appointment_id = (SELECT id FROM tn_appointments WHERE patient_name = 'Alice Brown' AND date = '2024-02-01')), NOW(), 500),
    ((SELECT id FROM tn_medical_records WHERE appointment_id = (SELECT id FROM tn_appointments WHERE patient_name = 'Bob Wilson' AND date = '2024-02-01')), NOW(), 300);

-- Services
INSERT INTO tn_services (name, image, description) VALUES
('General Checkup', 'checkup.jpg', 'Complete body examination and health status evaluation'),
('Vaccination', 'vaccine.jpg', 'Various vaccination services available'),
('Blood Test', 'bloodtest.jpg', 'Comprehensive blood analysis');

-- Doctor and Service
INSERT INTO tn_doctor_and_service (doctor_id, service_id)
VALUES
    ((SELECT id FROM tn_doctors WHERE name = 'Dr. John Doe'), (SELECT id FROM tn_services WHERE name = 'Consultation')),
    ((SELECT id FROM tn_doctors WHERE name = 'Dr. Jane Smith'), (SELECT id FROM tn_services WHERE name = 'Surgery'));

-- Booking
INSERT INTO tn_booking (service_id, patient_id, booking_name, booking_phone, name, gender, birthday, address, reason, appointment_date, appointment_time, status, create_at, update_at)
VALUES
    ((SELECT id FROM tn_services WHERE name = 'Consultation'), (SELECT id FROM tn_patients WHERE name = 'Alice Brown'), 'Alice Brown', '3456789012', 'Alice Brown', 1, '1990-05-15', '123 Main St', 'Annual checkup', '2024-02-15', '09:00', 'pending', NOW(), NOW()),
    ((SELECT id FROM tn_services WHERE name = 'Vaccination'), (SELECT id FROM tn_patients WHERE name = 'Bob Wilson'), 'Bob Wilson', '4567890123', 'Bob Wilson', 0, '1985-08-22', '456 Oak Ave', 'Vaccination', '2024-02-16', '10:30', 'confirmed', NOW(), NOW());

-- Notifications
INSERT INTO tn_notifications (message, record_id, record_type, patient_id, is_read, create_at, update_at)
VALUES
    ('Your appointment is tomorrow at 10:00 AM.', 123, 'Appointment', (SELECT id FROM tn_patients WHERE name = 'John Doe'), 0, NOW(), NOW()),
    ('Your lab results are ready for review.', 456, 'Lab', (SELECT id FROM tn_patients WHERE name = 'Jane Smith'), 0, NOW(), NOW());

INSERT INTO tn_booking_photo (url, booking_id) VALUES
('photo1.jpg', (SELECT id FROM tn_booking WHERE booking_name = 'Alice Brown' AND appointment_date = '2024-02-15')),
('photo2.jpg', (SELECT id FROM tn_booking WHERE booking_name = 'Bob Wilson' AND appointment_date = '2024-02-16'));

-- Medicine
INSERT INTO tn_medicine (name, price, unit, description, manufacture_date, expiry_date, side_effects, dosage) VALUES
('Paracetamol', 5, 'tablet', 'Pain reliever and fever reducer', '2023-01-01', '2025-01-01', 'Mild nausea in rare cases', '1-2 tablets every 6 hours'),
('Amoxicillin', 10, 'capsule', 'Antibiotic', '2023-06-01', '2025-06-01', 'May cause diarrhea', '1 capsule three times daily');

-- Medicine of Prescription
INSERT INTO medicine_of_prescription (medical_record_id, medicine_id, quantity) VALUES
((SELECT id FROM tn_medical_records WHERE diagnosis = 'Healthy, regular checkup completed' LIMIT 1), (SELECT id FROM tn_medicine WHERE name = 'Paracetamol'), 10),
((SELECT id FROM tn_medical_records WHERE diagnosis = 'Viral fever, prescribed medication' LIMIT 1), (SELECT id FROM tn_medicine WHERE name = 'Amoxicillin'), 15);
