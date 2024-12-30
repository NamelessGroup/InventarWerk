#!/bin/bash

# Diesel setup ausführen
echo "Starte diesel setup..."
diesel setup

# Anwendung starten
echo "Starte die Anwendung..."
exec ./backend