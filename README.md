# Tutorial A

## Try to answer the following questions, and write the answer in the new file readme.md in your repository.
**a. What is amqp?**
- Protokol komunikasi standard untuk message broker. Intinya, ini itu bahasa yang dipakai oleh aplikasi2 untuk saling berkirim pesan lewat perantara seperti RabbitMQ. 
- Dengan AMQP, pengirim dan penerima pesan tidak perlu terhubung langsung. Pengirim cukup lempar pesan ke broker, nanti broker yang akan urus pengirimannya ke penerima yang tepat.

**b. What does it mean? guest:guest@localhost:5672, what is the first guest, and what is the second guest, and what is localhost:5672 is for?**
- Itu adalah connection string untuk connect ke RabbitMQ, formatnya: username:password@host:port
    - guest pertama: username untuk login ke RabbitMQ
    - guest kedua: passwordnya 
    - localhost: RabbitMQnya lokal dan bukan di server lain
    - 5672: port yang dipakai RabbitMQ untuk menerima koneksi AMQP
