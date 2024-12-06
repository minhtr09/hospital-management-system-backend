-- Specialties
INSERT INTO tn_specialities (name, description, image) VALUES
('Cardiology', 'Heart and cardiovascular system specialists', 'cardiology.jpg'),
('Dermatology', 'Skin, hair, and nail specialists', 'dermatology.jpg'),
('Pediatrics', 'Medical care for children and adolescents', 'pediatrics.jpg'),
('Orthopedics', 'Bone and joint specialists', 'orthopedics.jpg');

-- Rooms
INSERT INTO tn_rooms (name, location) VALUES
('Room 101', 'First Floor, East Wing'),
('Room 102', 'First Floor, East Wing'),
('Room 201', 'Second Floor, West Wing'),
('Room 202', 'Second Floor, West Wing');

-- Doctors
INSERT INTO tn_doctors (email, phone, password, name, description, price, role, active, avatar, create_at, update_at, speciality_id, room_id) VALUES
('dr.smith@example.com', '1234567890', 'hashed_password_1', 'Dr. John Smith', 'Experienced cardiologist with 15 years of practice', 150, 'doctor', 1, 'smith.jpg', '2024-01-01', '2024-01-01', 1, 1),
('dr.jones@example.com', '2345678901', 'hashed_password_2', 'Dr. Sarah Jones', 'Pediatric specialist focusing on early childhood care', 120, 'doctor', 1, 'jones.jpg', '2024-01-01', '2024-01-01', 3, 2),
('dr.wilson@example.com', '3456789012', 'hashed_password_3', 'Dr. Michael Wilson', 'Dermatologist specializing in skin cancer treatment', 140, 'doctor', 1, 'wilson.jpg', '2024-01-01', '2024-01-01', 2, 3);

-- Patients
INSERT INTO tn_patients (email, phone, password, name, gender, birthday, address, avatar, create_at, update_at) VALUES
('patient1@example.com', '4567890123', 'hashed_password_4', 'Alice Brown', 0, '1990-05-15', '123 Main St, City', 'alice.jpg', '2024-01-01', '2024-01-01'),
('patient2@example.com', '5678901234', 'hashed_password_5', 'Bob Wilson', 1, '1985-08-22', '456 Oak Ave, Town', 'bob.jpg', '2024-01-01', '2024-01-01'),
('patient3@example.com', '6789012345', 'hashed_password_6', 'Carol Davis', 0, '1995-03-10', '789 Pine Rd, Village', 'carol.jpg', '2024-01-01', '2024-01-01');

-- Services
INSERT INTO tn_services (name, image, description) VALUES
('General Checkup', 'checkup.jpg', 'Comprehensive health examination'),
('Heart Screening', 'heart.jpg', 'Cardiovascular health assessment'),
('Skin Consultation', 'skin.jpg', 'Dermatological examination and consultation');

-- Doctor and Service Relationships
INSERT INTO tn_doctor_and_service (service_id, doctor_id) VALUES
(1, 1), (2, 1), -- Dr. Smith provides checkups and heart screenings
(1, 2), -- Dr. Jones provides checkups
(1, 3), (3, 3); -- Dr. Wilson provides checkups and skin consultations

-- Bookings
INSERT INTO tn_booking (service_id, patient_id, booking_name, booking_phone, name, gender, birthday, address, reason, appointment_date, appointment_time, status, create_at, update_at) VALUES
(1, 1, 'Alice Brown', '4567890123', 'Alice Brown', 0, '1990-05-15', '123 Main St, City', 'Annual checkup', '2024-02-01', '09:00', 'confirmed', '2024-01-15', '2024-01-15'),
(2, 2, 'Bob Wilson', '5678901234', 'Bob Wilson', 1, '1985-08-22', '456 Oak Ave, Town', 'Heart palpitations', '2024-02-02', '10:30', 'confirmed', '2024-01-16', '2024-01-16');

-- Appointments
INSERT INTO tn_appointments (booking_id, doctor_id, patient_id, patient_name, patient_birthday, patient_reason, patient_phone, numerical_order, position, appointment_time, date, status, create_at, update_at) VALUES
(1, 1, 1, 'Alice Brown', '1990-05-15', 'Annual checkup', '4567890123', 1, 1, '09:00', '2024-02-01', 'scheduled', '2024-01-15', '2024-01-15'),
(2, 1, 2, 'Bob Wilson', '1985-08-22', 'Heart palpitations', '5678901234', 2, 2, '10:30', '2024-02-02', 'scheduled', '2024-01-16', '2024-01-16');

-- Appointment Records
INSERT INTO tn_appointment_records (appointment_id, reason, description, status_before, status_after, create_at, update_at) VALUES
(1, 'Schedule confirmation', 'Appointment scheduled successfully', 'pending', 'scheduled', '2024-01-15', '2024-01-15'),
(2, 'Schedule confirmation', 'Appointment scheduled successfully', 'pending', 'scheduled', '2024-01-16', '2024-01-16');

-- Medical Records
INSERT INTO tn_medical_records (appointment_id, payment_status, patient_id, doctor_id, diagnosis) VALUES
(1, 1, 1, 1, 'Healthy, no significant issues found'),
(2, 1, 2, 1, 'Mild tachycardia, prescribed beta blockers');

-- Invoices
INSERT INTO tn_invoices (medical_record_id, time, total_price) VALUES
(1, '2024-02-01 10:00:00', 150),
(2, '2024-02-02 11:30:00', 200);

-- Medicine
INSERT INTO tn_medicine (name, price, unit, description, manufacture_date, expiry_date, side_effects, dosage) VALUES
('Aspirin', 10, 'tablet', 'Pain reliever and blood thinner', '2023-01-01', '2025-01-01', 'May cause stomach upset', '1-2 tablets every 4-6 hours'),
('Metoprolol', 25, 'tablet', 'Beta blocker for heart conditions', '2023-06-01', '2025-06-01', 'May cause fatigue, dizziness', '1 tablet twice daily');

-- Medicine Prescriptions
INSERT INTO medicine_of_prescription (medical_record_id, medicine_id, quantity) VALUES
(2, 2, 60); -- Prescribing Metoprolol for patient with tachycardia

-- Notifications
INSERT INTO tn_notifications (message, record_id, record_type, patient_id, is_read, create_at, update_at) VALUES
('Your appointment has been confirmed', 1, 'appointment', 1, 0, '2024-01-15', '2024-01-15'),
('Your appointment has been confirmed', 2, 'appointment', 2, 0, '2024-01-16', '2024-01-16');

-- Booking Photos
INSERT INTO tn_booking_photo (url, booking_id) VALUES
('medical_report1.jpg', 1),
('ecg_results.jpg', 2);