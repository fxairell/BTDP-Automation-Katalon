Disclaimer
Perhatian file ini berisi ringkasan dari file project Katalon pada Folder Assesment3_F.X. Airell
V.S.W._KSAT006ONL013 yang terdapat pada Folder Sesi 10 bagian dari Automation Testing with Katalon
Studio. Adapun ringkasan merupakan deskripsi singkat mengenai hal yang tercantum pada file excel
ScenarioTestDocs-20220709-F.X. Airell Valerio Satrio W.xlsx.

Nama			: F.X. Airell Valerio Satrio Wibowo
Kode Peserta	: KSAT006ONL013
Link Github		: https://github.com/fxairell/BTDP-Automation-Katalon
Ringkasan		:

Sheet yang terdapat dalam file:
- User
- Test Case Branch
- Test Case Flow

Ringkasan:
a) User
	User merupakan sheet yang digunakan untuk melihat berbagai Test Case dengan Test Scenarionya,
dilengkapi dengan Step, Test Data, dan nilai Expected-Actual untuk penentuan Passed, Failed, atau Not
Executed. Functional Test digunakan sebagai pemecah alur Test Case dan membaginya menjadi beberapa
kelompok.
b) Test Case Branch
	Test Case Branch merupakan sheet yang digunakan untuk menjabarkan percabangan skenario yang
digunakan dalam berbagai Test Case. Branch diberi nomor untuk memudahkan menentukan alur dari Test Case.
Total terdapat 51 cabang yang digunakan dalam berbagai Test Case. Test Scenario Branch digunakan untuk
menjabarkan Functional Test dari test web sederhana. Test Case ID dibentuk dari perpaduan singkatan
Test Scenario Branch dan penomoran urut dari alur yang digunakan.
c) Test Case Flow
	Test Case Flow merupakan sheet yang digunakan untuk mendefinisikan alur dari setiap Test Case.
Total terdapat 30 Test Case berbeda beserta alur dan penjabaran Positif atau Negatifnya. Alur juga
dilengkapi dengan Pre Condition untuk mempersingkat penulisan alur Test Case.

Beberapa catatan tambahan:
a) Percobaan pertama	: Automate Test Case tanpa Delay
   Permasalahan		: Automation terlalu cepat hingga kadang lebih dahulu dibandingkan kecepatan
				  aplikasi berjalan. Hal ini membuat automasi cenderung lebih sering gagal.
b) Percobaan kedua	: Automate Test Case dengan Delay pada beberapa bagian
   Permasalahan		: Kecepatan aplikasi tidak bisa diprediksi, kadang lebih lama, kadang pada waktu
				  yang tepat. Penambahan mengurangi kecenderungan gagal dari percobaan pertama.
c) Percobaan ketiga	: Pengaturan Delay untuk hampir setiap Test Case
   Permasalahan		: Permasalahan yang dihadapi masih sama. Kecenderungan gagal kini paling banyak
				  terfokus pada 1 bagian, yaitu .volley TimeOut error pada bagian akhir checkout.
   Solusi			: Galat merupakan galat random, sehingga automasi tidak disarankan menggunakan Test
				  Suite, cukup pada tingkat Test Case. Generasi Report Test Suite dapat dijadikan
				  sebagai bukti pencobaan serial dari functional test yang sama, mampu menghasilkan
				  galat yang serupa, dengan kadang ada beberapa variasi kegagalan tidak bersumber
				  dari .volley TimeOut error.