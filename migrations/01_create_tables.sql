-- Account table
CREATE TABLE account (
    account_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT FALSE,
    last_login TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    account_type VARCHAR(10) NOT NULL CHECK (account_type IN ('patient', 'doctor', 'staff', 'nurse'))
);

-- Patient Information
CREATE TABLE patient (
    patient_id SERIAL PRIMARY KEY,
    account_id INTEGER UNIQUE REFERENCES account(account_id),
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    date_of_birth DATE NOT NULL,
    gender VARCHAR(10),
    address TEXT,
    phone VARCHAR(20)
);

-- Emergency Contact
CREATE TABLE emergency_contact (
    contact_id SERIAL PRIMARY KEY,
    patient_id INTEGER REFERENCES patient(patient_id),
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    relationship VARCHAR(50)
);

-- Create an enumerated type for permissions
CREATE TYPE permission_type AS ENUM (
    'Full Access',
    'Read Only',
    'Patient Data Access',
    'Administrative Access',
    'Pharmacy Access'
);

-- Role table
CREATE TYPE role_type AS ENUM ('doctor', 'nurse', 'staff', 'patient', 'admin');

CREATE TABLE role (
    role_id SERIAL PRIMARY KEY,
    role_name VARCHAR(50) UNIQUE NOT NULL,
    role_type role_type NOT NULL,
    description TEXT,
    permission permission_type NOT NULL
);

-- Doctor Information
CREATE TABLE doctor (
    doctor_id SERIAL PRIMARY KEY,
    account_id INTEGER UNIQUE REFERENCES account(account_id),
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    specialty VARCHAR(50),
    phone VARCHAR(20),
    role_id INTEGER REFERENCES role(role_id)
);

-- Nurse Information
CREATE TABLE nurse (
    nurse_id SERIAL PRIMARY KEY,
    account_id INTEGER UNIQUE REFERENCES account(account_id),
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    specialization VARCHAR(50),
    phone VARCHAR(20),
    role_id INTEGER REFERENCES role(role_id)
);

-- Medical Staff
CREATE TABLE medical_staff (
    staff_id SERIAL PRIMARY KEY,
    account_id INTEGER UNIQUE REFERENCES account(account_id),
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    position VARCHAR(50),
    phone VARCHAR(20),
    role_id INTEGER REFERENCES role(role_id)
);

-- Appointment
CREATE TABLE appointment (
    appointment_id SERIAL PRIMARY KEY,
    patient_id INTEGER REFERENCES patient(patient_id),
    doctor_id INTEGER REFERENCES doctor(doctor_id),
    appointment_datetime TIMESTAMP NOT NULL,
    status VARCHAR(20),
    notes TEXT
);

-- Medical Record
CREATE TABLE medical_record (
    record_id SERIAL PRIMARY KEY,
    patient_id INTEGER REFERENCES patient(patient_id),
    doctor_id INTEGER REFERENCES doctor(doctor_id),
    creation_date TIMESTAMP NOT NULL,
    symptoms TEXT,
    diagnosis TEXT,
    treatment_plan TEXT,
    follow_up_instructions TEXT
);

-- Prescription
CREATE TABLE prescription (
    prescription_id SERIAL PRIMARY KEY,
    medical_record_id INTEGER REFERENCES medical_record(record_id),
    doctor_id INTEGER REFERENCES doctor(doctor_id),
    prescription_date TIMESTAMP NOT NULL
);

-- Medication
CREATE TABLE medication (
    medication_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    generic_name VARCHAR(100),
    manufacturer VARCHAR(100),
    dosage_form VARCHAR(50),
    strength VARCHAR(50),
    unit VARCHAR(20),
    price DECIMAL(10, 2)
);

-- Prescription Details
CREATE TABLE prescription_detail (
    prescription_id INTEGER REFERENCES prescription(prescription_id),
    medication_id INTEGER REFERENCES medication(medication_id),
    dosage VARCHAR(50),
    frequency VARCHAR(50),
    duration VARCHAR(50),
    instructions TEXT,
    PRIMARY KEY (prescription_id, medication_id)
);

-- ICD Diagnosis
CREATE TABLE icd (
    code VARCHAR(20) UNIQUE PRIMARY KEY,
    description VARCHAR(255) NOT NULL
);

-- Healthcare Service
CREATE TABLE healthcare_service (
    service_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    price DECIMAL(10, 2)
);

-- Payment
CREATE TABLE payment (
    payment_id SERIAL PRIMARY KEY,
    patient_id INTEGER REFERENCES patient(patient_id),
    amount DECIMAL(10, 2) NOT NULL,
    payment_date TIMESTAMP NOT NULL,
    payment_method VARCHAR(50),
    status VARCHAR(20)
);

-- User Evaluation
CREATE TABLE user_evaluation (
    evaluation_id SERIAL PRIMARY KEY,
    patient_id INTEGER REFERENCES patient(patient_id),
    doctor_id INTEGER REFERENCES doctor(doctor_id),
    rating INTEGER CHECK (rating BETWEEN 1 AND 5),
    feedback TEXT,
    evaluation_date TIMESTAMP NOT NULL
);

-- Asset and Equipment
CREATE TABLE asset (
    asset_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    type VARCHAR(50),
    acquisition_date DATE,
    purchase_price DECIMAL(10, 2),
    current_value DECIMAL(10, 2),
    location VARCHAR(50),
    status VARCHAR(20)
);

-- Maintenance
CREATE TABLE maintenance (
    maintenance_id SERIAL PRIMARY KEY,
    asset_id INTEGER REFERENCES asset(asset_id),
    maintenance_date TIMESTAMP NOT NULL,
    performed_by INTEGER REFERENCES medical_staff(staff_id),
    description TEXT,
    cost DECIMAL(10, 2)
);

-- Inpatient Admission
CREATE TABLE inpatient_admission (
    admission_id SERIAL PRIMARY KEY,
    patient_id INTEGER REFERENCES patient(patient_id),
    admitting_doctor_id INTEGER REFERENCES doctor(doctor_id),
    primary_nurse_id INTEGER REFERENCES nurse(nurse_id),
    admission_datetime TIMESTAMP NOT NULL,
    discharge_datetime TIMESTAMP,
    room_number VARCHAR(20),
    diagnosis TEXT,
    treatment_summary TEXT
);

-- Medical History
CREATE TABLE medical_history (
    history_id SERIAL PRIMARY KEY,
    patient_id INTEGER REFERENCES patient(patient_id),
    record_date TIMESTAMP NOT NULL,
    condition VARCHAR(100),
    notes TEXT
);

-- Care Plan
CREATE TABLE care_plan (
    plan_id SERIAL PRIMARY KEY,
    patient_id INTEGER REFERENCES patient(patient_id),
    created_by_nurse_id INTEGER REFERENCES nurse(nurse_id),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    goals TEXT,
    interventions TEXT,
    evaluation_criteria TEXT
);