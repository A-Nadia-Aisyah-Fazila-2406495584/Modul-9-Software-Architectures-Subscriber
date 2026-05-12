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

## Simulating Slow Subscriber
- RabbitMQ Dashboard:
    ![Simulating subscriber image](/assets/images/SimulatingSlowSubscriber.png)
    - Explanation: 
        - Pada percobaan ini, subscriber dibuat lebih lambat dengan menambahkan delay 1 detik untuk tiap pemrosesan message. Sementara itu, publisher tetap mengirim 5 message sekaligus setiap kali dijalankan.
        - Terlihat di chart "Queued messages" jumlah message yang mengantri sempat mencapai sekitar 17 - 18. Hal tsb terjadi karena publisher mengirim message jauh lebih cepat daripada subscriber memprosesnya. Angka 17 - 18 ini berasal dari akumulasi beberapa kali publisher dijalankan sebelum subscriber sempat menghabiskan antriannya. Ini membuktikan bahwa message broker berfungsi sebagai buffer, message tidak hilang meskipun subscribernya lambat, tetapi tetap tersimpan di queue dan diproses secara satu per satu.

