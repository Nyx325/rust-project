CREATE TABLE Logs(
    id_log INTEGER PRIMARY KEY AUTOINCREMENT,
    log_active INTEGER NOT NULL,
    msg TEXT NOT NULL
);

CREATE TABLE Standard(
    id_standard INTEGER PRIMARY KEY AUTOINCREMENT,
    std_active INTEGER NOT NULL,
    std_name TEXT NOT NULL,
    units TEXT NOT NULL,
    window_type INTEGER NOT NULL
);

CREATE TABLE Signatary(
    id_signatary INTEGER PRIMARY KEY AUTOINCREMENT,
    signatary_active INTEGER NOT NULL,
    first_name TEXT NOT NULL,
    second_name TEXT NOT NULL,
    father_last_name TEXT NOT NULL,
    mother_last_name TEXT NOT NULL,
    user TEXT NOT NULL,
    passwd TEXT NOT NULL
);

CREATE TABLE Parameter(
    id_parameter INTEGER PRIMARY KEY AUTOINCREMENT,
	parameter_active INTEGER NOT NULL,
    parameter_name TEXT NOT NULL
);

CREATE TABLE Client(
    id_client INTEGER PRIMARY KEY AUTOINCREMENT,
    client_active INTEGER NOT NULL,
    client_name TEXT NOT NULL
);

CREATE TABLE Site(
    id_site INTEGER PRIMARY KEY AUTOINCREMENT,
	site_active INTEGER NOT NULL,
    site_key TEXT NOT NULL,
    site_name TEXT NOT NULL,
    watershed TEXT,
    aquifer_key TEXT,
    aquifer TEXT,
    watershed_org TEXT,
    local_dir TEXT,
    site_state TEXT,
    site_municipality TEXT,
    body_of_water TEXT,
    body_of_water_type TEXT,
    latitude TEXT NOT NULL,
    longitude TEXT NOT NULL,
    site_use TEXT,
    sampling_site TEXT,
    id_client INTEGER,
    FOREIGN KEY (id_client)
    	REFERENCES Client(id_client)
    	ON DELETE SET NULL
        ON UPDATE CASCADE
);

CREATE TABLE Analysis(
    id_analysis INTEGER PRIMARY KEY AUTOINCREMENT,
	analysis_active INTEGER NOT NULL,
    analysis_name TEXT NOT NULL,
    id_parameter INTEGER,
    FOREIGN KEY (id_parameter)
        REFERENCES Parameter(id_parameter)
        ON DELETE SET NULL
        ON UPDATE CASCADE
);

CREATE TABLE StandardDetail(
    id_std_detail INTEGER PRIMARY KEY AUTOINCREMENT,
    id_standard INTEGER,
    id_analysis INTEGER,
    FOREIGN KEY (id_standard)
        REFERENCES Standard(id_standard)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (id_analysis)
        REFERENCES Analysis(id_analysis)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE SignataryDetail(
    id_sig_detail INTEGER PRIMARY KEY AUTOINCREMENT,
    id_signatary INTEGER NOT NULL,
    id_analysis INTEGER NOT NULL,
    FOREIGN KEY (id_signatary)
        REFERENCES Signatary(id_signatary)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    FOREIGN KEY (id_analysis)
        REFERENCES Analysis(id_analysis)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE Sampler(
    id_sampler INTEGER PRIMARY KEY AUTOINCREMENT,
	sampler_active INTEGER NOT NULL,
    id_signatary INTEGER NOT NULL,
    FOREIGN KEY (id_signatary)
        REFERENCES Signatary(id_signatary)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE Sample(
    id_sample INTEGER PRIMARY KEY AUTOINCREMENT,
	sample_active INTEGER NOT NULL,
    control_number TEXT NOT NULL,
    project TEXT NOT NULL,
    sampling_date DATE NOT NULL,
    sampling_hour TEXT NOT NULL,
    reception DATE NOT NULL,
    id_sampler INTEGER,
    id_site INTEGER,
    FOREIGN KEY (id_sampler)
        REFERENCES Sampler(id_sampler)
        ON DELETE SET NULL
        ON UPDATE CASCADE,
    FOREIGN KEY (id_site)
        REFERENCES Site(id_site)
        ON DELETE SET NULL
        ON UPDATE CASCADE
);

CREATE TABLE SampleResult(
    id_sample_result INTEGER PRIMARY KEY AUTOINCREMENT,
	result_active INTEGER NOT NULL,
    result TEXT NOT NULL,
    analysis_date DATE NOT NULL,
    id_signatary INTEGER,
    id_analysis INTEGER,
    id_standard INTEGER,
    id_sample INTEGER,
    FOREIGN KEY (id_signatary)
        REFERENCES Signatary(id_signatary)
        ON DELETE SET NULL
        ON UPDATE CASCADE,
    FOREIGN KEY (id_analysis)
        REFERENCES Analysis(id_analysis)
        ON DELETE SET NULL
        ON UPDATE CASCADE,
    FOREIGN KEY (id_standard)
        REFERENCES Standard(id_standard)
        ON DELETE SET NULL
        ON UPDATE CASCADE,
    FOREIGN KEY (id_sample)
        REFERENCES Sample(id_sample)
        ON DELETE SET NULL
        ON UPDATE CASCADE
);
