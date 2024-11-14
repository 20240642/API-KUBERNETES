CREATE DATABASE actividades_extraescolares;

USE actividades_extraescolares;

CREATE TABLE actividades(
	id INT PRIMARY KEY AUTO_INCREMENT NOT NULL,
    encargado VARCHAR(80) NOT NULL,
    credito VARCHAR(80) NOT NULL,
    nombre VARCHAR(80) NOT NULL,
    descripcion VARCHAR(255) NOT NULL
);