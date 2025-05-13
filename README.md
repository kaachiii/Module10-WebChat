# YewChat 💬

> Source code for [Let’s Build a Websocket Chat Project With Rust and Yew 0.19 🦀](https://fsjohnny.medium.com/lets-build-a-websockets-project-with-rust-and-yew-0-19-60720367399f)

## Install

1. Install the required toolchain dependencies:
   ```npm i```

2. Follow the YewChat post!

# Module 10 - WebChat using yew
* Nama : Ischika Afrilla
* NPM : 2306227955

### Experiment 3.1: Original code
Login page
![Login page](images/Screenshot%20(1087).png)

Chat page
![Chat page](images/Screenshot%20(1089).png)

Not found page
![Not found page](images/Screenshot%20(1092).png)

### Experiment 3.2: Be Creative!
Profile button
![Profile button](images/Screenshot%20(1093).png)

Profile page
![Profile page](images/Screenshot%20(1094).png)

Saya menambahkan tombol di halaman chat yang bisa digunakan untuk melihat profil pengguna. Saat tombol diklik, pengguna akan diarahkan ke halaman `/profile`. Di halaman ini, saya menampilkan foto profil default sebagai avatar agar tampilannya lebih menarik. Selain itu, saya juga menampilkan nama pengguna yang sedang login, yang diambil dari konteks `User`. Agar lebih santai, saya menggunakan format sapaan `("Hello, {}!", username)`.