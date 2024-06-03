# dsa-tools-rust
CLI Tools für DSA 4.1, geschrieben in Rust
Es stehen Hilfedialoge und Beispiele zur Verfügung

### Ausführen
Es ist für spezifische Maschinen kompiliert, achtet also darauf, die richtige Version zu erwischen.
Ausführen könnt ihr es beispielsweise mit `dsa-wetter [OPTIONS]` oder `dsa-wetter.exe [OPTIONS]` unter Windows. Als erste Option empfehle ich immer `--help`.

### Bugs
Keine Chance, dass der Code vollständig korrekt ist. Bisher war er für mich ausreichend.
Wenn ihr Bugs findet, macht gerne nen Issue auf. Ich habe nicht ausgiebig getestet.

---

## dsa-wetter
Eine CLI Utility für "schnell mal eben nachm Wetter gucken".
Es richtet sich nach den DSA 4.1 Regeln im WdE, S.156ff.

### Beispiele
- `dsa-wetter` - Wetter für einen Sommertag im Mittelreich. Die Ausgabe ist direkt und weitestgehend unformatiert.
- `dsa-wetter -n 7 -f csv -o wetter.csv` - Wetter für sieben Sommertage im Mittelreich. Die Ausgabe erfolgt als CSV Tabelle formatiert in die Datei "wetter.csv".
- `dsa-wetter -n 3 -r "Khom" -d -s herbst -f md` - Wetter für drei Herbsttage in der Khomwüste. Die Ausgabe erfolgt direkt als Markdown Stichpunktliste.
- `dsa-wetter -v -n 365 -f csv -o "der-bericht.csv" -x 4711 -d -s winter -r "Höhen des Ehernen Schwerts"` - Wetter für einen windigen Winter, der ganzes Jahr lang hält, auf den wüstenüberzogenen Spitzen des ehernen Schwertes, gespeichert als CSV Tabelle, mit dem Seed 4711 replizierbar und mit Debugausgabe. Kann man machen, muss man nicht.

## dsa-gift
Eine CLI Utility zum zufälligen Erstellen von Giften.
Grundlage sind die Regeln zu Giftpflanzen im ZBA, S217.
Wichtige Info dabei: Die angegebene Dauer sollte nur als Zeitraum des Schadens, nicht der Symptome, interpretiert werden - ansonsten fallen die Regeln bei hohen Giftstufen auseinander.

### Beispiele
- `dsa-gift` - Ein zufälliges Gift der Stufe 1. Die Ausgabe ist direkt und weitestgehend unformatiert.
- `dsa-gift -l 4 -f json -o butterblume.json` - Ein zufälliges Gift der Stufe 4, gespeichert im Format JSON unter dem Namen _butterblume_.
- `dsa-gift -v -l 20 -f csv -o "das ende.csv" -x 42069` - Ein Gift der Stufe 20, replizierbar mit dem Seed 42069, gespeichert unter dem Namen _das ende_ und mit Debugausgabe. Kann man machen, muss man nicht.