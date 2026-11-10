// Setiap bahasa pemrograman biasanya akan mengggunakan file ex yang sudah diyntukan,
//  rust menggunakan file extension .rs
// sehingga nanti di tiap kode program rust yang akan kita buat kita akan menggunakan extension .rs

// Masalah dengan Main Function
// dalam rust, satu project hanya bia menggunakan satu main function
// agak sulit untuk sambil belajar menggunakan main function 
//  oleh karena itu alternative lain nya adalah menggunakan unit test

// Unit test adalah kode yang memang dikhusu kan untuk melakukan percobaan 
// cara membuat unit test di rust sanagt mudah, kita bisa membuat funciton dengan nama apapun, lalu ditambahakan attribute test pada function

use core::{fmt, panic};
use std::{collections::{BTreeMap, HashMap, HashSet, LinkedList, VecDeque, hash_map::IntoIter}, ffi::os_str::Display, fmt::{Debug, format}, ops::{Add, Deref, DerefMut, Range, RangeInclusive}, process::Output, rc::Rc};

fn main(){
    println!("Hello, world");
}


#[test] // tambahkan attribute ini untuk menandakan function unit test
fn hello_test(){
    println!("Hello test");
}

//  Variable
// Variable adalah tempat untuk menyimpan data
// cara membuat variable di rust bisa menggunakan kata kunci 'let'
// setelah variable diisi, data maka data dalam variable nya tak akan bisa diubah lagi

#[test]
pub fn belajar_variable(){
    let nama = "Faiz Hannan Hakim";
    println!("Hello {}", nama) 
}

// Mutable 
// Seperti yang sudah dijelaskan sebelumnya, variable yang sudah diisi datanya tidak bisa diubah lagi, atau disebut immutable
// namun rust juga memperbolehkan jika kita ingin membuat variable yang bia diubah lagi, atau disebut mutable
// caranya kita bisa gunakan kata kunci let 'mut' ketika membuat variable

#[test]
fn belajar_mutable(){
    let mut nama = "Faiz Hannan Hakim";
    println!("ini nama {}", nama);

    nama = "Faiz Hannan Hakim Mutable";
    println!("ini nama {}", nama);
}

// Static Typing
//  Rust adalah bahasa yang static typing, artinya setiap kita membuat variable dengan jenis data tertentu, maka dia tidak akan bisa berubah menjadi tipe data lain nya
// sebelumnya kita membuat variable dengan tipe text/string, kita tidak bisa mengubah variable tersebut dengan data angka/number contohnya
#[test]
fn static_typing(){
    let mut nama = "Faiz Hanan Hakim";
    println!("nama: {}", nama);
    
    // nama = 10; //ini akan menimbulkan error
    println!("nama: {}", nama);
}

// Shadowing
// Shadowing di rust, kita bisa membuat variable dengan nama yang sama
// namin saat kita membuat variable dengan nama yang sama, maka variable sebelumnya akan tertutup atau disebut shadowing
// praktek ini mungkin kurang baik jika dilakukan trlalu sering karena bisa membuingungkan
// namun ini diperbolehkan di rust


fn shadowing(){
    let name = "Faiz Hannan Hakim";
    println!("Hello {}", name);

    let name = 100;
    println!("Hello {}", name);

    // kedua name ini berbeda dan tak ada hubungan nya
    // bahkan lokasi penyimpanan nya berbeda
    // namun perlu diketahui blok setelahnya dalam pemanggilan nama
    // akan mengambil value dari variable nama terakhir yang di inisialisasi
    // dalam hal ini adalah 100
}




/*
// ///////////
// TIPE DATA
// //////////
*/

// setiap nilai di rust memiliki tipe data secara garis besar rust membagi tipe data menjadi dua subsets,
// scalar dan compound
// Scalar adalah type mempresentasikan "single value", yaitu integer, float, boolean, char dll
// Compound adalah type mempresentsikan beberapa value (bisa lebih dari satu), dalam satu type, yaitu tuple, string, array dll

// Scalar Type List:
// Integer type: yaitu tipe data number dalam bilangan bulat
// Float type, yaitu tipe data number dalam desimal
// Boolean type, yaitu tipe data yang hanya bernilai true (benar) atau false (salah)
// char type, yaitu tipe data karakter

#[test]
fn scalar_type(){
    // Tipe Integer
    let SignedInteger8: i8 = 12; let UnsignedInteger8: u8 = 12;// panjang 8-bit
    // Rentang i8: -128 s.d. 127 | u8: 0 s.d. 255
    let SignedInteger16: i16 = 12; let UnsignedInteger16: u16 = 12;// panjang 16-bit
    // Rentang i16: -32.768 s.d. 32.767 | u16: 0 s.d. 65.535
    let SignedInteger32: i32 = 12; let UnsignedInteger32: u32 = 12; //panjang 32-bit
    // Rentang i32: -2.147.483.648 s.d. 2.147.483.647 | u32: 0 s.d. 4.294.967.295
    let SignedInteger64: i64 = 12; let UnsignedInteger64: u64 = 12;// panjang 64 bit 
    // Rentang i64: -9.223.372.036.854.775.808 s.d. 9.223.372.036.854.775.807 | u64: 0 s.d. 18.446.744.073.709.551.615
    let SignedInteger128: i128 = 12; let UnsignedInteger128: u128 = 12;// panjang 128-bit
    // Rentang i128: -170.141.183.460.469.231.731.687.303.715.884.105.728 s.d. 170.141.183.460.469.231.731.687.303.715.884.105.727 | u128: 0 s.d. 340.282.366.920.938.463.463.374.607.431.768.211.455\
    let isize: isize = 12; let usize: usize = 12; // ini tipe data yang cukup unik karna akan mengkikuti platform misal 32 bit atau 64 bit
    // Rentang (Platform 32-bit) -> isize: -2.147.483.648 s.d. 2.147.483.647 | usize: 0 s.d. 4.294.967.295
    // Rentang (Platform 64-bit) -> isize: -9.223.372.036.854.775.808 s.d. 9.223.372.036.854.775.807 | usize: 0 s.d. 18.446.744.073.709.551.615


    // Tipe Float
    let Float32: f32 = 3.14; // panjang 32-bit
    // Rentang f32: ±1.18 × 10^-38 s.d. ±3.40 × 10^38
    let Float64: f64 = 3.14; // panjang 64-bit
    // Rentang f64: ±2.23 × 10^-308 s.d. ±1.80 × 10^308

    // Tipe Boolean
    let boolean: bool = true;
    // Nilai bool: true atau false

    // Tipe Char
    let char1: char = 'a';
    // Nilai char: Unicode scalar value (U+0000 s.d. U+D7FF dan U+E000 s.d. U+10FFFF)

}


//  Konversi Tipe Data Number
// Rust bisa melakukan konversi tipe data dari tipe data number yang ukruan nya kecil ke ukuran nya lebih besar atau sebaliknya 
// namun perlu diperhatikan, jika kita lakukan konversi tipe data number dari ukuran besar ke kecil, maka bisa terjadi yang namanya integer overflow, yaitu kondisi dimana nilai number tidak bisa ditampung oleh tipe data tujuan konversi, 
// misal kita punya number 1000000 dalam bentuk i32 lalu kita konversi ke bentuk i8, maka akan terjadi integer overflow, 
// karena i8 tidak bisa menampung nilai tersebut
// untuk melakukan konversi, kita bisa menggunakan kata kunci 'as'

#[test]
fn konversi_tipe_data(){
    let a: i8 = 10;

    let b: i32 = a as i32;

    println!("{}", b); // berhasil

    let d: i64 = 1000000000;

    let e: i8 = d as i8;

    println!("{}", e) // terjadi integer overflow
}

// Operasi Numerik
//  Rust mendukung semua operasi numerik
// operator yang digunakan hampir sama dengan kebanyakan bahasa pemrograman lain nya
// + -> pertambahan
// - -> pengurangan
// / -> pembagian
// * -> perkalian
// % -> sisa bagi / modulo
#[test]
fn operasi_numerik(){
    let a: i8 = 12;
    let b: i32 = 29;
    let c: i16 = 499;
    println!("{}", a as i32 + b * c as i32)
}

// Augmented Assignment
// augmented assignment adalah operasi langsung pada variable sendiri
// augmented hanya bisa dilakukan pada mutable variable
// contoh augemented assignment pada variable a 
// a+= 10
// a-= 10
// a*= 10
// a/=10
// a%=10

#[test]
fn augmented_assignment(){
    let mut a: i16 = 0;
    a+= 100;
    a-= 10;
    a*= 10;
    a/=10;
    a%=4;
    println!("{}", a)
}

// Comparison Operator
// Comparison Operator(operator perbandingan) adalah operator menghasilkan nilai boolean (benar/salah)
// > -> lebih besar dari 
// < -> lebih kecil dari
// >= -> lebih besar sama dengan
// <=  -> lebih kecil sama dengan
// == -> sama dengan


#[test]
fn comparison(){
    let result: bool = 10 > 20;
    println!("{}", result)
}

// Operasi Boolean 
// operasi boolean bekerja dengan membandingkan 2 nilai boolean atau lebih
// && -> Dan
// || -> Atau
// ! -> Kebalikan

#[test]
fn operasi_boolean(){
    let resultdan: bool = true && false;
    let resultatau: bool = true || false;
    let kebalikanresultdan: bool = !resultdan;
    println!("{}", resultdan);
    println!("{}", resultatau);
    println!("{}", kebalikanresultdan)
}

// Operasi Char
// Tipe data char di Rust dapat dibandingkan menggunakan operator perbandingan (berdasarkan nilai Unicode-nya) 
// dan dapat dikonversi ke tipe data numerik seperti u32 untuk melihat kode Unicode-nya.

#[test]
fn operasi_char(){
    let char1: char = 'a';
    let char2: char = 'z';
    
    // Perbandingan antar char
    let lebih_kecil: bool = char1 < char2;
    
    // Konversi char ke u32 (Unicode scalar value)
    let kode_unicode: u32 = char1 as u32;

    println!("{}", lebih_kecil);
    println!("{}", kode_unicode);
}

// Compound Type List:
// Tuple type, yaitu kumpulan beberapa data tang bisa berbeda tipe data
// Array type, yaitu kumpulan beberapa data yang harus bertipe data yang sama

#[test]
fn compound_type(){
    // Tipe Tupple
    // Jumlah data di tuple sudah final, jadi tak bisa diubah lagi jumlah nya
    let tuple: (i32, f64, bool) = (10, 12.4, true);
    println!("{:?}", tuple);


   

}


// Mengakses data tuple 
    // Untuk mengakses tiap data di tuple, kita bisa gunakan .(titik), lalu diikuti dengan nomor index (lokasi) data nya
    // Index di Tuple dimulai dari nomor 0
#[test]
fn buat_dan_akses_tuple() {
    // Tipe Tupple
    let tuple: (i32, f64, bool) = (10, 12.4, true);
    println!("{:?}", tuple);


    // Tipe Array
    let array: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", array)

}

   // Destructuring Tuple
    // Kadang ketika kita ingin mengakses seluruh data tuple dan menyimpan nya di variable, akan sangat menyulitkan jika kita akses satu satu menggunakan nomor indexnya
    // tuple mendukung destructuring, yaitu membongkar isi tuple, dan menyimpan nya dalam variable
    // jika ada data di tuple yang tidak kita butuhkan kita bisa gunakan tanda _(underscore), ketika melakukan destructuring

#[test]
fn destructuring_tuple() {
    let tuple: (i32, f64, bool) = (10, 12.4, true);

 
    let (a1, a2, a3) = tuple;
    println!("{} {} {}", a1, a2, a3);
}

// Mutable Tuple
    // Saat kita buat immutable variable, secara otomatis tuple yang sudah dibuat tidak bisa diubah lagi isi datanya
    // namun jika kita membuat variable dalam bentuk mutable maka data tuple bisa kita ubah
    // untuk mengubah data tuple, kita cukup gunakan nomor index dan = (sama dengan) mirip seperti mengubah data variable biasanya
#[test]
fn mutable_tuple() {
    let tuple: (i32, f64, bool) = (10, 12.4, true);

    
    let mut mutable_tuple: (i32, f64, bool) = tuple;
    
    mutable_tuple.0 = 12;
    mutable_tuple.1 = 100.23;
    mutable_tuple.2 = false;

    let (a1, a2, a3) = mutable_tuple;
    println!("{} {} {}", a1, a2, a3);
}

// Unit 
// Unit adalah tipe tuple tanpa nilai apapun, ditulisnya ()
// Hal ini mungkin kelihatan tak ada gunanya
//  Biasanya Unit digunakan untuk function function yang tidak membutuhkan hasil data apapun

fn unit(){
    return ()
    // ini adalah yang sebenernya terjadi, dan ini adalah unit sebetulnya
}
#[test]
fn test_unit() {
    let result: () = unit();
    return result // ini adalah yang sebenernya terjadi, dan ini adalah unit sebetulnya
}

//  Mengakses Array
//  Untuk mengakses array sama seperti tuple, kita perlu menenrtukan nomor index yang akan diakses dimulai dari 0
//  namun cara mengaksesnya berbeda, tidak menggunakan .(titik), melainkan menggunakan [index](kurung kotak)

#[test]
fn mengakses_array(){
    let array: [i8; 5] = [1,2,3,4,5];

    println!("{:?}", array);

    let a = array[0];
    let b: i8 = array[1];

    println!("{} {}", a, b);
}



// Mutable Array
// sama seperti tuple, jika kita membuat variable array defaultnya adalah immutable, secara otomatis data array tidak bisa diubah
// jika data array ingin bisa diubah, kita harus buat variable dalam bentuk mutable 
// kita bisa mengubah data array dengan menggunakan [index] = diisi dengan nilai baru

#[test]
fn mutable_array(){
    let mut array: [i8; 5] = [1,2,3,4,5];

    array[0] = 12;
    array[1] = 13;

    println!("{:?}", array);
}

#[test]
fn lihat_panjang_array(){
    let array: [i8; 5] = [1,2,3,4,5];

    let length: usize = array.len();
    println!("{}", length)
}

// Constant 
// constant adalah variable immutable yang menggunakan kata kunci const
// yang membedakan antara const dan let adalah const tidak bisa mut, dan const harus di deklarasikan ketika kode program dibuat bukan "saat program dijalankan", 
// oleh karena itu nilai const tidak bisa hasil dari kalkulasi nilai lain yang belum jelas hasilnya
// untuk membuat const, wajib disebutkan tipe datanya secara eksplisit
// nama const di Rust harus huruf besar, dan biasanya pemisah kata menggunakan garis bawah

const NILAI_MAX: u8 = 100;

#[test]
fn constant(){
    println!("{}", NILAI_MAX)
}

// Variable Scope
//  Dalam rust, variable scope mendefinisikan area dimana variable bisa digunakan
// variable bisa digunakan di dalam scope tempat definisi variable, dan juga di inner scoper
// namun variable tidak bisa digunakan di outer scope

#[test]
fn variable_scope(){
    let anancuy: i8 = 1;

    {
        println!("{}", anancuy);
        let faiz: i8 = 10;
        println!("{}", faiz);
    }

    // println!("{}", faiz) // ini akan error
}

// Memory management
// Garbage collection adalah fitur yang banyak digunakan bahasa pemrograman untuk melakukan manajemen memory, seperti di java atau golang atau js dl;
// secara berkala garbage collection akan memantau data yang sudah tidak digunakan lag di memory, dan menghapusnya secara otomatis
// atau di bahasa pemrograman tanpa garbage collection, yang biasanya melakukan manajemen memory manual, seperti c/ cpp
// tanpa garbage collection, kita harus mengalukasikan data secara manual di memory, begitu juga ketika sudah tidak buuth, kita harus menghapus data dari memory
// rust memiliki pendekatan berbedam rust tidak menggunakan garbage collection, rust juga tidak menggunakan manual memory management

// Stack dan Heap
// rust membagi data di memory dalam dua bagian, stack dan heap
// Stack adalah bagian dimana data disimpan dalam strukur data tumpukan, last in first out, semua data di stack harus yang fixed size (artinya ukuran data sudah pasti)
// Heap berbeda, heap seperti tempat untuk menympan data, dimana untuk menyimpan data di heap kita akan melakukan request ke Heap, lalu di dalam heap terdapat memory allocator yang bertugas untuk menemukan area kosong untuk untuk menyimpan dan mengalokasikan data ke area tersebut
// setelah setelah itu kita akan diberi pointer (penunjuk) ke lokasi dimana data tu berada di heap
// Pointer dari heap berukuran fix sized, oleh karena itu pointer akan disimpan di stack

fn function_a(){
    let a : i8 = 10; // ini akan disimpan di stack
    let b: String = String::from("Faiz"); //-> string bisa bertambah atau mengecil maka akan disimpan di heap

    println!("{} {}", a, b)
}
fn function_b(){
    let a : i8 = 10;
    let b: String = String::from("Hannan");
    
    println!("{} {}", a, b)
}

#[test]
fn stack_heap(){
    function_a();
    function_b();
}

// Drop function
// saat variable keluar dari scope nya, yang artinya tidak bisa diakses lagi, secara otomatis rust akan memanggil drop function
// drop function adalah function untuk menghapus data sehingga akan dibersihkan dari heap
// dan jika rust function() sudah selesai di eksekusi, maka function() tersebut akan dihapus pula dari stack frame
// oleh karena itu, rust tidak membutuhkan garbage collection atau manual memory management


// &str dan String
// Rust memiliki tipe data text yang fixed size, yaitu &str (string slice), dan yang bisa mengembang ukuran nya, yaitu String
// &str karena ukuran nya fixed size, jadi rust akan menyimpan nya di stack, 
// String karena bisa mengembang maka akan disimpan di heap
// String di rust merupakan tipe data text UTF-8, dan bisa berkembang ukuran nya
// Ketika kita buat dalam bentuk immutable variable, maka String tidak bisa berkembang, namun tetap disimpoan di Heap
// Ketikakita buat dalam bentuk mutable variable, maka String bisa berkembang di Heap
// String juga memiliki method / function untuk memanipulasi data, namun perlu diperhatikan ada method yang digunakan untuk mengubah datanya sendiri, ada juga method untuk mengubah dalam bentuk data baru tanpa memodifikasi data aslinya
//  Unutk lengkapnya
// https://doc.rust-lang.org/std/string/struct.String.html

#[test]
fn string(){
    let name: &str = "Faiz Hannan Hakim";
    let trim: &str = name.trim();


    let mut nama: String = String::from("Faiz");
    println!("{}", nama);

    nama.push_str("Anancuy");

    println!("{}", nama);

    let namaBaru :String = name.replace("Hannan", "Mabar");

    println!("{}", namaBaru);
}

// Ownership
// Rust menggunakan Ownership untuk melkukn data management di Memory
// Ownership adalah salah satu fitur unik di rust yang mungkin jarang ada di bahasa pemrograman lain
// Ownership wjib dimengerti karena akan berdampak ke hampir semua fitur di rust
// wonership adalah fitur yang digunkan oleh rust untuk menjadikan rust menjadi bahasa pemrograman yang aman dalam mengelola data di memory
// tanpa harus adanya fitur garbage collection taupun manual memory management
// karena ownership adalah konsep yang baru untuk kebanyakan programmer, maka kadang kita butuh waktu untuk memahami nya


// Data Copy 
// Sesuai aturan di ownership rules, setiap value harus dimiliki oleh satu owner pada satu waktu atau pada 1 blok
// ketika kita berinteraksi dengan data, maka data akan dimiliki hanya oleh satu owner
// semua data yang bersifat dixed size (yang disimpan di stack), ketika kita tambahkan ke variable berbeda (owner baru), maka hasilnya adalah data akan di copy, sehingga variable baru (owner baru) akan memiliki data hasil copy dari variable lama (owner lama)
// Oleh karena itu, tiap data akan selalu dimiliki oleh satu owner pada satu waktu

#[test]
fn data_copy(){
    let a: i8 = 10;
    let b: i8 = a; // ini sebenernya bukan b memiliki a, namun nilai a di pass by value menuju b, jadi b hanya menerima salinan, alias membuat alokasi data baru di memori
    println!("{} {}", a, b);
}

// Ownership Movement
// Namun data copy tidak terjadi untuk tipe data yang disimpan di Heap
// ketika kita coba buat variable baru, dari variable lama, yang terjadi bukanlah pass by value (copy), melainkan transfer ownership dari owner lama ke owner baru

#[test]
fn ownership_movement(){
    let mut nama: String = String::from("Faiz");
    
    {
        let mut namaubah = nama;
        //  println!("{}", nama) // ini tak akan bisa karna saat ini nama sudah milik namaubah
    }

    // println!("{}", nama) // disini juga tak bisa ingat nama sudah diberikan pada namaubah dan namaubah sudah keluar scope (hilang), artinya nama sudah tidaklah ada

   
}

// Clone
// Sekarang kita tahu bahwa data di stack akan di copy sedangkan data di heap akan dipindahkan ownership nya
// Lantas bagaimana jika kita juga ingin melakukan cpy untuk data di Heap?
// maka kita harus melakukan clone
// String memmiliki method clone()
// saat kita memannggil method clone, maka method tersebut akan meng copy data String menjadi data String baru
// Semua tipe data yang disimpan di Heap di rust memiliki method clone 

#[test]
fn clone(){
    let nama:String = String::from("Faiz Hannan");

    {
        let namaorang:String = nama.clone();
    }

    println!("{}", nama) // disini tidak error karna namaorang tidak menjadi owner barunya, tetapi yang dioper ke namaorang adalah value clone (copy) an nya
}

// 3 ATURAN OWNERSHIP
// 1. Setiap Value di rust harus punya owner (variable pemilik value)
// 2. Dalam satu waktu, hanya boleh ada satu owner
// 3. Ketika owner keluar scope, value akan dihapus

#[test]
fn ownership_rules() {
    let mut a: i8 = 10;

    // --- 1. SCOPE PEMINJAMAN BIASA (IMMUTABLE BORROW) ---
    {
        // Anda boleh membuat banyak referensi pinjaman biasa secara bersamaan
        let r1: &i8 = &a;
        let r2: &i8 = &a;
        
        println!("Peminjaman biasa r1: {}, r2: {}", r1, r2);
        // Data 'a' hanya bisa dibaca, tidak bisa diubah lewat r1 atau r2
    } 
    // Scope peminjaman biasa selesai, referensi r1 dan r2 sudah hilang

    // --- 2. SCOPE PEMINJAMAN MUTABEL (MUTABLE BORROW) ---
    {
        // Hanya boleh ada SATU peminjam mutabel dalam satu waktu
        let m_ref: &mut i8 = &mut a;
        
        // Mengubah nilai asli 'a' melalui mutabel referensi (*m_ref)
        *m_ref += 15; 
        println!("Peminjaman mutabel mengubah a menjadi: {}", *m_ref);
    } 
    // Scope peminjaman mutabel selesai, hak akses kembali ke 'a'

    // Nilai 'a' sekarang sudah berubah karena sempat dipinjam secara mutabel di atas
    println!("Nilai akhir a di luar scope: {}", a); // Output: 25

    // =========================================================================
    // --- BUKTI ATURAN 1 & 2: KEPEMILIKAN TUNGGAL & MOVE SEMANTICS ---
    // =========================================================================
    {
        let s1 = String::from("Belajar Rust"); // s1 adalah owner dari String
        let s2 = s1; // Ownership berpindah (moved) dari s1 ke s2

        // Aturan 2 dibuktikan di sini:
        // Sekarang s2 adalah satu-satunya owner. s1 sudah tidak valid/mati.
        println!("Owner baru (s2): {}", s2);

        // Jika baris di bawah ini di-uncomment, Rust akan ERROR (borrow checker):
        // println!("s1 lama: {}", s1); 
        // Error: value borrowed here after move
    }

    // =========================================================================
    // --- BUKTI ATURAN 3: KETIKA OWNER KELUAR SCOPE, VALUE AKAN DIHAPUS ---
    // =========================================================================
    {
        // Kita buat Struct bantuan untuk melihat kapan memori/value dihancurkan
        struct LoggerDrop;
        impl Drop for LoggerDrop {
            fn drop(&mut self) {
                println!("-> ATURAN 3 BUKTI: Owner keluar scope, value otomatis di-drop/dihapus!");
            }
        }

        let _item = LoggerDrop; // _item menjadi owner
        println!("_item masih hidup di dalam scope ini.");
    } // <- Akhir scope blok ini. Variabel _item mati, trait `drop` otomatis terpanggil di sini.

    println!("Blok pengujian ownership selesai.");
}

// If Expression
// Sama seperti bahas pemrograman yang lain, rust juga mendukung if expression
// if expression digunakan untuk membuat percabangan kode sesuai dengan kondisi, 
// jika kondisi terpenuhi maka blok kode id akan dieksekusi, jika kondisi tidak terpenuhi
// blok kode if tidak akan dieksekusi

#[test]
fn if_expression(){
    let valuenya: i8 = 9;

    if valuenya >= 4 {
        println!("Lebih besar dari 4")
    } else {
        
    }
}

//  Else expression
// ketika kondisi if tidak terpenuhi kadang kita ingin melakukan sesuatu
// kita bisa lakukan tu dengan else expression
// blok di else akan dieksekusi jika kondisi if tidak terpenuhi
#[test]
fn else_expression(){
    let lulus: bool = false ;

    if lulus {
        println!("lulus");
    } else {
        println!("tidak lulus");
    }
}

// Else If Condisition
// saat membuat if expression, kadang kita ingin membuat beberapa kondisi
// untuk membuat beberapa kondisi kita bisa gabungkan dengan else if expression

#[test]
fn else_if_expression(){
    let nama: &str = "Hannan";

    if nama == "Faiz"{
        println!("anjay namanya faiz")
    } else if nama == "Hannan"{
        println!("namanye anan")
    } else {
        println!("Namanya yang lain")
    }

}

// Let Statement
// If di rust adalah sebuah expression,artinya bisa menghailkan value dan bisa digunakan dengan let statement untuk mengisi data di variable
// ini sangat berguna sehingga kita tidak perlu memasukan nilai ke variable terpisah dengan deklarasi variable nya
#[test]
fn let_statement(){
    let nilai: u8 = 70;
    let result: &str = if nilai >=100 as u8 {
        "Excellent"
    } else if nilai >70 {
        "Mantap"
    } else if nilai > 50 {
        "Rata Rata "
    } else {
        "Goblok"
    };

    println!("{}", result)


}

// Loop 
// Setiap bahasa pemrograman biasanya memiliki fitur untuk melakukn perulangan
// rust mendukung beberapa cara untuk melakukan perulangan, pertama kita akan bahas tentang loop
// Loop merupakan perintah di rust untuk melakukan perulangan terus menerus sampai kita memerinahkan nya untuk berhenti
// jika kita tidak memerintahkan untuk berhenti, maka loop tidak akan pernah berhenti melakuka perulangan
// Untuk menghentikan perulangan kita bisa menggunakan'break'
// selain break ada juga perintah continue, yang artinya menghentikan perulangan saat ini, dan langsung dilanjutkan ke perulangan berikutnya

#[test]
fn loop_expression(){
    let mut counter: u8 = 0;
    loop {
        counter += 1; 
        if counter >=254{
            break;
        } else if counter %2 == 0 {
            continue;
        }
    }

    println!("Counter: {}", counter);
}

// FYI sama seperti di if expression, loop juga bisa mengembalikan nilai, sehingga bisa disimpan dalam variable dengan let expression
// caranya kita bisa gunakan break lalu diikuti dengan nilai yang akandikembalikan di loop

#[test]
fn loop_let_expression(){
    let mut counter: u8 = 0;
    let hasil: u8 = loop {
        counter += 1; 
        if counter >=254{
            break counter;
        } else if counter %2 == 0 {
            continue;
        }
    };

    println!("Counter: {}", hasil);
}

// Loop Label
// Kadang kita sering membuat loop dalam loop, dan ketika ingin menghentikan loop paling atas dari loop yang ada di dalam, maka hal itu tak bisa dilakukan
// loop memiliki fitur label, dimana kita bis amemberi nama pada loop
// keuntungan memberi label loop adalah kita bisa mengentikan loop yang ingin kita hentikan dimanapun dengan cara menyebutkan label nya
// untuk memberi nama label adalah dengan menggunakan petik 1
// petik 1 nya 1 kali saja
// 'contoh misalkan

#[test]
fn loop_label(){
    let mut number: u8 = 1;
    'outer: loop {
        let mut i: u8 = 1; 
        
        loop {
            println!("{} x {} = {}", number, i, number * i); 
            i += 1;
            
            if i > 10 {
                break; // Keluar dari loop dalam (i loop)
            }
        }
        
        number += 1;
        
        if number > 10 {
            break 'outer; // Keluar dari loop luar ('outer) setelah number mencapai 10
        }
    }
}


// While Loop
// While loop adalah jenis perulangan dimana memiliki kondisi
// jika kondisi masih tepenuhi maka perulangan akan dilanjutkan
// namun jika kondisi tidak terpenuhi, maka perulangan akan dihentikan
// while loop mirip seperti loop bisa dihentikan menggunakan break dan continue

#[test]
fn while_loop(){
    let mut counter :u8 = 0;
    while counter <= 10 {
        if counter %2 == 0 {
            println!("Counter: {}", counter)
        }
        counter += 1;
    }

}

// For Loop
// salah satu yang biasa kita lakukan ketika menggunakan array adalah melakukan pengambilan data di array dar data awal sampai paling akhir
// di rust for loop akan sangat berguna dalam case seperti ini

#[test]
fn for_loop(){
    let KataKata: [&str; 5] = ["Kocak lu", "beyir", "goks", "mabar bero", "23"];

    for kata in KataKata {
        println!("Value: {}", kata)
    }
}

// Range 
// Rust memiliki tipe data bernama range 
// range adalah jarak antara start dan end
// range merupakan tipe data collection seperti array, sehingga bisa dilakukan pengulangan for loop
// data range akan dimulai dari start dan akan diakhiri sebelum end (exclusive)
// yang biasa disebut range exclusive karna data terakhir nya tak akan diambil dalam perulangan
// cara penulisan nya adalah start .. end
// https://doc.rust-lang.org/std/ops/struct.Range.html

#[test]
fn data_range() {
    let rangedata: Range<u8> =0..5 as u8;
    println!("Start: {}", rangedata.start);
    println!("End: {}", rangedata.end);
    // disini start dan end adalah attribute


    for angka in rangedata {
        println!("Value: {}", angka);
    }
}

// Range Inclusive 
// Selain range yang exclusive, rust juga memiliki tipe data range inclusive
// implementasinya berbeda dengan range sebelumnya 
// perbedaan range inclusive sama range exclusive adalah
// range incluseive data terakhirnya akan tetap diambil di akhir perulangan 
// cara penulisan nya adalah start ..= end 


#[test]
fn range_inclusive(){
    let rangedata: RangeInclusive<u8> = 0..=10 as u8;
    println!("Start: {}", rangedata.start());
    println!("End: {}", rangedata.end());
    // disini start dan end nya bukan lagi attribute namun funciton
    for angka in rangedata {
        println!("Value: {}", angka);
    }
}

// Function
// Function adalah kumpulan kode yang memiliki nama dan kegunaan nya adalah agar bisa dipanggil dengan nama tersebut
// sebelumnya kita sudah tau tentang main function, yaitu function yang dipanggil oleh ketika aplikasi berjalan
// untuk membuat function, kita menggunakan kata kunci fn, dan diikuti dengan nama function nya
// tradisi nama function / variable di rust menggunakan format snake case, 
// untuk memanggil function kita bisa langsung sebutkan nama function nya 

fn functionnih(){
    println!("ini adalah function")
}
#[test]
fn contohFunction(){
    functionnih();
}

// Parameter
// Saat kita membuat function kita bisa menambahkan parameter yatu variable yang menjadi bagian dari definisi function
// ketika function memiliki parameter, maka kita wajib memberi value untuk parameter tersebut ketika memanggil function nya
// beberapa orang memanggil parameter menjadi argument, jadi jangan terlalu bingung
// parameter di functon bisa satu atau lebih, dan tap parameter bisa menggunakan tipe data apapun yang kita inginkan


fn parameter_nama_mu(nama_depan: &str, nama_belakang: &str){
    println!("Hallo {} {}", nama_depan, nama_belakang);
}


#[test]
fn test_parameter(){
    parameter_nama_mu("Faiz", "Hannan");
    parameter_nama_mu("Eko", "Khannedy");
}

// Return value
// saat membuat function kadang ktia ingin mengembalikan hasil eksekusi yang dilakukan di dalam function , atau kita sebut return value
// jika sebuah function ingin bisa mengembalikan value , kita bisa sebutkan tipe value yang dikembalikan saat penulisan function setelah kurung dengan tanda ->
// baris eksekusi terakhir di function dianggap sebagai kembalian valuenya
//  atau jika kita igin mengembalikan value sebelum baris eksekusi berakhir, kita bisa gunakan kata kunci return dan diikuti dengan value yang akan dikembalikan 

fn hasil_pertambahan(data: [i32; 10]) -> i32 {
    let mut hasil: i32 = 0;
    for angka in data {
        hasil += angka
    }
    return hasil
}

#[test]
fn returnvalue(){
    println!("{}", hasil_pertambahan([1,2,3,4,5,6,7,8,9,10]))
}

// Recursive function 
// recursion merupakan salah sat metode pemecahan masalah dimana sebuah solusi pada masalah tersebut bergantung pada solusi dari masalah yang lebih kecil yang merupakan bagian dari masalah tersebut
// rust mengimplementasikan recursion dengan memperbolehkan sebuah fungsi untuk memanggil dirinya sendiri(fungsi itu sendiri)
// rungsi yang memanggil fungsi itu sendiri biasanya disebut dengan recursive function


fn print_text_sebanyak(text: &str, times: u8){
    if times == 0 {
        return;
    } else {
        println!("{}", text);
    }

    print_text_sebanyak(text, times - 1);
}

#[test]
fn test_recursive(){
    print_text_sebanyak("Halo Gw Pais", 200);
}

// References
// references adalah pointer(penunjuk) yang bisa kita ikuti ke lokasi data aslinya di heap,
//  datanya sendiri dimiliki oleh variable lain, bukan si reference
// reference akan dijamin menunjuk value yang valid selama alur hidup reference tersebut
// jika alur hidup selesai, maka reference akan dihapus namun tidak dengan data yang ditunjuknya, karena data yang ditunjuknya mengikuti alur hidup variable owner nya
// untuk membuat reference di rust, kita bisa gunakan tanda &(and) sebelum tipe data nya, dan dalam satu waktu kita bisa membuat banyak reference
// sebelumnya kita tahu bahwa tipe data text st r selalu kita buat dalam bentuk &str, hal ini karena defaultnya adalah reference ke str

// Borrowing
// Ketika kita membuat reference, aksi itu kita sebut sebagai borrowing (meminjam)
// kalo diibaratkan di kehidupan, kita bisa meminjam barang, tapi jika sudah selesai menggunakan nya, kita harus mengambalikan ke owner (pemiliki) barang nya

#[test]
fn test_reference(){
    let mut nama_pertama:String =  String::from("Faiz");
    {
        let  nama_kedua: &String = &nama_pertama; //mengambil reference dari nama
        println!("{}", nama_kedua);
    }

    println!("{}", nama_pertama); // ini akan bisa karna nama kedua meminjam nya bukan menjadi owner nya
}

// Mutable References (Mutabble borrowing)
// pada kasus jika memang kita diperbolehkan memodifikasi value dari reference, maka kita bisa gunakan mutable references
// mutable reference adalah reference dengan tanda &mut, dimana artinya kita bisa memodifikasi value dari reference teserbut
// namun ada ketentuan jka bisa menggunakan mutable reference, variable owner juga harus mutable, jika variable owner adalah immutable maka mutable reference tidak bisa dilakukan 
// selain itu untuk menjamin keamanan, dalam satu waktu, hanya diperbolehkan membuat satu mutale reference, dan tidak ada reference lain nya

#[test]
fn test_mutable_reference(){
    let mut nama_pertama:String =  String::from("Faiz");
    {
        let mut nama_kedua: &mut String = &mut nama_pertama; //mengambil reference dari nama
        *nama_kedua = String::from("Guri Guri");
        println!("{}", nama_kedua);
    }

    {
        let nama_ketiga: &String = &nama_pertama;
        println!("nama ketiga hanya meminjam nama pertama yang sudah dipinjam nama kedua {}", nama_ketiga);
    }

    println!("{}", nama_pertama); // ini akan berubah menjadi guri karna nama kedua langsung mengubah isi data menggunakan referensi
}


// Ownership & Borrow
// seteah sekarang kita tahu function, kita akan bahas lagi tentang ownership di function parameter
// tipe data yang disimpan di heap, ketika kita kirim sebagai parameter di function, secara otomatis ownership nya akan berpindah ke parameter function yang dipanggil
// karena ownership nya berpindah ke function parameter nya, secara otomatis setelah function selesai di eksekusi maka owner dan value akan dihapus tidak bisa digunakan lagi
// oleh karena itu ada cara lain supaya parameter tidak menjadi owner dengan konsep borrowing
// sederhana nya parameter akan dipinjamkan saja data sebuah memori dari variable dan ketika scope function berakhir otomatis data akan kembali menjadi milik variable sebelumnya 
// jenis data yang berada di stack, hanya akan dikirim sebagai copy pada parameter
// NOTES: SANGAT PENTING Mungkin ada salah kata karna ini cukup sedikit rumit tolong simak ini baik baik

fn cetak_nama_ownership(nama:String){
   println!("{}", nama)
}

#[test]
fn memberikan_ownership_pada_param(){
    let nama: String = String::from("Eko Kurniawan");
    cetak_nama_ownership(nama);
    // println!("{}", nama /*ini akan berujung error karna ownership sudah dipindahkan */);
}

fn cetak_nama_borrow(nama: &String){
    println!("{}", *nama);
}

#[test]
fn memberikan_borrow_pada_param(){
    let nama: String = String::from("Eko Kurniawan");
    cetak_nama_borrow(&nama);
    println!("{}", nama) //ini akan tetap bisa karna data nama sudah dikembalikan
}

// Return value Ownership
// seperti yang sudah kita tahu, bahwa function bisa mengembalikan value
// value heap yang kita kembaikan di function, secara otomatis ownershipnya akan dimiliki oleh yang memanggil function tersebut
// sedangkan jika value stack, maka return value function akan di copy oleh yang memanggil function nya


fn full_name(first_name: String, last_name: String) -> String {
    return format!("{} {}", first_name, last_name);
}

#[test]
fn test_full_name(){
    let first_name: String = String::from("Faiz");
    let last_name: String = String::from("Hannan");
    let name = full_name(first_name, last_name);

    println!("{}", name);// <- ownership data dari firstname dan lastname sudah dimiliki name
    // println!("{}", first_name);// ini tak akan bisa
    // println!("{}", last_name); // ini juga tak akan bisa
}

// Dangling Pointer
// Dangling pointer adalah kondisi dimana ada reference (pointer) yang menunjuk ke value yang tidak ada di memory
// di rust hal ini tidak diperbolehkan, ketika kita mengambalikan reference dalam function, maka secara otomatis value akan dihapus karena sudah keluar dari scope function 
// pada kasus seperti ini, rust akan menganggap hal ini error, karena berpotensi terjadi dangling pointer 
// biasanya programmer golang sering kali membuat function yang mengembalikan pointer

// fn fungsi_return_pointer() -> &String /*lihat ada merah ini tidaklah diperbolehkan */ {
//     let nama: String = String::from("Faiz Hannan Hakim");
//     return &nama
// }

// maka dari itu solusi nya alih alih memberikan return dengan reference, di rust disarankan langsung mengembalikan value jangan sebuah pointer

fn fungsi_return_value() -> String{
    let nama: String = String::from("Nama");
    return nama;
}

#[test]
fn tes_return_pointer(){
    // let reference_nama: &String = fungsi_return_pointer();
    let value_nama: String = fungsi_return_value();
    println!("{}", value_nama);
}


// Slice
// Slice adalah reference ke sebagian elemen dari data collection (misal array)
// karena slice adalah reference, dia tidak memiliki ownership
// contoh misal kita punya array dengan tota 10 data, kita mau ambil 5 data terdepan, 
// maka kita bisa membuat slice sebagai reference data dari data ke-1 sampai ke-5

// Range
// walau kita sudah membahas range sebelumnya, namun range ini ada kaitan nya dengan slice maka supaya pemahaman lebih baik kita kaan membahas hubungan nya dengan slice 
// saat kita ingin mengambil sebagian data collection, kita butuh menentukan range untuk slice yang akan kita ambil
// rust sendiri memiliki banyak jenis range, sebelumnya kita sudah bahas tentang range exclusive dan range inclusive selain itu masih banyak yang lain 
// https://doc.rust-lang.org/std/ops/index.html#structs

// Range
// A (half-open) range bounded inclusively below and exclusively above (start..end).
// RangeFrom
// A range only bounded inclusively below (start..).
// RangeFull
// An unbounded range (..).
// RangeInclusive
// A range bounded inclusively below and above (start..=end).
// RangeTo
// A range only bounded exclusively above (..end).
// RangeToInclusive
// A range only bounded inclusively above (..=end).
// YeetExperimental
// Implement FromResidual<Yeet<T>> on your type to enable do yeet expr syntax in functions returning your type.

#[test]
fn slice_reference(){
    let array: [i32; 11] = [1,2,3,4,5,6,7,8,9,10,11];
    let slice1: &[i32] = &array[..7];
    println!("{:?}", slice1);
}

// String Slice 
// Sebelumnya saat kita menggunakan tipe data text kita selalu buat dalam bentuk &str (String slice), hal ini sebenernya berarti &str itu adalah reference ke sebagian atau seluruh data str 
// saat ita menggunakan tipe data String, kita juga mengambil sebagian karakter di String, hasil dari sebgian data itu adalah &str (String slice)
// Karena &str itu adalah slice, yang artinya adalah reference, maka sebenernya tidak memiliki ownership, dan oleh karena itu ketika kita assign ke variable lain atau ke function, yang di copy sebenernya adalah reference nya, datany tetap menggunakan data yang sama

#[test]
fn string_slice(){
    let mut nama_panjang: String = String::from("Faiz Hannan Hakim");
    let mut nama_depan: &str = &nama_panjang[..=4];
    let nama_tengah: &str = &nama_panjang[5..=10];
    let nama_belakang: &str = &nama_panjang[12..];
   
    println!("{}", nama_depan);
    println!("{}", nama_tengah);
    println!("{}", nama_belakang);
}

// Struct 
// Struct adalah tipe data mirip tuple yang bisa digunakan untuk menampung beberapa data dengan tipe yang berbeda
// yang membedakan dengan tuple, pada struct, kita bisa memberi nama untuk tiap datanya, atau dibilang field, sehingga lebih jelas dibanding tuple yang hanya menggunakan number 
// dengan menambahkan nama pada data di struct secara otomatis kita tidak perlu menentukan urutan posisi data yang selalu sama, bisa berubah ubah seiring pembuatan kode 
// untuk membuat struct kita bisa gunakan kata kunci struct 
// dan struct itu sama saja seperti tipe data lain bisa diperlakukan seperti tipe data lain

struct Person {
    nama: String,
    mbg: String,
    usia: u8,
    tinggi: f32,
    menikah: bool,
}
#[test]
fn test_struct() -> (){
    let mut Orang1: Person = Person { nama: String::from("Faiz"), mbg: String::from("Nadia"), usia: 24, tinggi: 176.2, menikah: true };
    println!("{} dan {}", Orang1.nama, Orang1.mbg)
}

struct InformasiLingkup{
    suhu: f32,
    kelembapan: u8,
    basa: u8,
    terik: bool,
}

#[test]
fn main_prog_iot() ->(){
    let mut dataIot: InformasiLingkup = InformasiLingkup { suhu: 0.0, kelembapan: 0, basa: 0, terik: false };

    {
        let mut data_suhu: &mut f32 = &mut dataIot.suhu;
        *data_suhu = 32.4;

    }

    {
        let mut data_kelembapan: &mut u8 = &mut dataIot.kelembapan; 
        *data_kelembapan = 6;
    }

    {
        let mut data_basa: &mut u8 = &mut dataIot.basa; 
        *data_basa = 10;
    }

    {
        let mut data_terik: &mut bool = &mut dataIot.terik; 
        *data_terik = true;
    }

    println!("{} {} {} {}", dataIot.suhu, dataIot.terik, dataIot.kelembapan, dataIot.basa)
}


//  Init Shorthand
// kadang ada kasus kita ingin membuat value untuk field di struct dari variable yang sudah ada
// jika misalnya nama variable sama dengan nama field, kita tidak perlu sebuthkan nama fieldnya secara eksplisit
// ini disebut initshorthand
#[derive(Clone, Debug)]
struct Personi{
    firstname: String,
    lastname:String,
}
#[test]
fn test_struct_person(){
    let firstname:String = String::from("Faiz");
    let lastname: String = String::from("Hakim");

    let person: Personi = Personi { firstname, lastname }; //<- kepemilikan first name dan last name sekarang sudah pindah pada person

    println!("{} {}", person.firstname, person.lastname);
}

// Method
// Method sebenernya sama seperti function, membuatnya menggunakan kata kunci fn, punya nama, bisa punya param, dan bisa punya return value
// yang membedakan dengan function adalah method itu tidak berdiri sendiri
// melainkan menempel pada struct, enum, atau trait, 
// pada methd, parameter pertama menggunakan self (objek itu sendiri)
// self adalah representasi dari instance / object dimana method tersebut menempel dan dipanggil
// untuk membuat method kita harus tentukan ingin meletakan di struct mana, caranya menggunakan kata kunci impl, lalu diikuti dengan nama structnya
// lalu didalamnya, kita bisa lakukan seperti kita membuat function
// untuk mengakses semua field yang ada di instance struct, kita bisa gunakan parameter self pertama di method
// biasanya parameter self dibuat dalam bentuk reference, agar ownershipnya tidak diambil oleh method yang dipanggil

struct Mobil{
    nama: String, 
    top_speed: u16,
}

impl Mobil {
    fn informasi_ke_orang(&self, orang: &Personi){
        println!("Halo {} {} ini adalah mobil {} dengan top speed {}", orang.firstname, orang.lastname, self.nama, self.top_speed )
    }
}

#[test]
fn test_method(){
    let orang: Personi = Personi { firstname: String::from("Faiz"), lastname: String::from("Anancuy")};
    let mobil_lambo: Mobil = Mobil { nama: String::from("Lambo"), top_speed: 300 };
    mobil_lambo.informasi_ke_orang(&orang);
    let orang2: Personi = orang.clone();
    println!("{} {}",orang.firstname, orang.lastname);
    println!("{} {}",orang2.firstname, orang2.lastname);
}

// Associated Functions
// Setiap functionyang dibuat dalamimpl kita sebut dengan Associated Functions, karena terkait dengan tipe data yang kita tentukan di impl
// Associated function yang memiliki parameter self artinya adalah method, dan dipanggil setelah kita membuat instance nya
// Namun kita juga bisa membuat function tanpa self parameter, yang artinya function tersebut tidak terhubung dengan instance nya
// untuk memanggil associated function yang bukan method, kita bisa langsung gunakan NamaType.nama_function()
// Biasanya Associated Functions bukan method, digunakan untuk membuat instance dari type nya
// kita bisa mengakses nya dengan menggunakan TipeData::associated_function_bukan_method

struct Makanan{
    nama: String,
    jenis: String,
}

impl Makanan{
    fn makanan_baru(nama: String, jenis: String)-> Makanan{
      Makanan { nama, jenis }
    }
}

#[test]
fn test_associated_function(){
    let makanan_barunya: Makanan = Makanan::makanan_baru(String::from("Durian"), String::from("Montong"));
    println!("{} {}", makanan_barunya.nama, makanan_barunya.jenis);
}

// Enum
// Enum atau Enumerations, merupakan tipe data yang dibuat untuk mengumpulkan beberapa kemungkinan value 
// untuk membuat enum, kita bisa gunakan kata kunci enum lalu diikuti dengan nama enum
// di dalam enum nya, kita tentukan kemungkinan value yang diperbolehkan
// untuk mengakses nya mirip mirip dengan associated function bukan method
// Nama_Enum::Jenis_Enum
enum Level {
    REGULER,
    PREMIUM,
    PLATINUM,
}

#[test]
fn test_enum(){
    let level: Level = Level::PLATINUM;
    // println!("{}", level); //Enum tak bisa langsung di print
}

// Enum Data
// Enum juga bisa digynakan untuk menympan data layaknya tuple
// tiap opsi / item di enum kita bisa tentukan bisa menyimpan data apa

enum Payment{
    CreditCard(String),

    BankTransfer(String, String),

    EWallet(String, String),
}

#[test]
fn test_payment(){
    let payment_bank: Payment = Payment::BankTransfer(String::from("BCA"), (String::from("8782374030")));
}

// Enum Method 
// Enum juga mirip seperti struct, kita bisa tambahkan method
// cara menambahkan method di enum caranya sama seperti menambahkan method di struct 

impl Payment {
    fn pay(&self, amount: u32) {
        println!("Paying Amount: {}", amount);
    }
}

#[test]
fn test_enum_method(){
    let pay_wallet: Payment = Payment::EWallet(String::from("Gopay"), String::from("33209302"));
   {
    let duit: u32 = 38974923;
     pay_wallet.pay(duit);
   }

   println!("Pembayaran sukses")
}

// Mengakses data di enum
// Secara default, kita tidak bisa mengakses data di enum
// untuk bisa mengakses data di dalam enum, kita harus belajar pattern matching

// Pattern Matching
// Selain menggunakan If, untuk percabangan di rust mendukung fitur bernama Pattern Matching menggunakan match
// Pattern Matching di rust sebenernya sangat kompleks, bisa digunakan untuk melakukan pengecekan value, variable, dan banyak hal

// Pattern Matching untuk enum
// Saat melakukan pattern matching di enum dengan match, kita wajib menentukan semua kondisi dari tiap opsi yang ada di enum, termasuk datanya
// jika kita tidak mencakup semua opsi dari enum data, maka match akan error

enum Hobi {
    TIDUR,
    MAKAN,
    NGODING,
    LARI
}

#[test]
fn test_pattern_matching_enum(){
    let hobi: Hobi = Hobi::NGODING;

    match hobi {
        Hobi::NGODING => {
            println!("NGODINGG");
        }

        Hobi::MAKAN => {
             println!("MAKAN");
        }

        // ini jika ingin yang lain
        Hobi::LARI => {
            println!("LARI")
        }

        Hobi::TIDUR => {
            println!("TIDUR")
        }
    }
}

// Destructuring Enum Data Patterns
// Sebelumnya kita pernah membuat enum dengan data
// pattern matching juga bisa digunakan untuk mengambil data yanv terdapat di Enum, atau kita sebut destructuring

enum Kendaraan{
    Sepeda(String),

    Motor(String, String, u8),

    Mobil(String, u16)
}

impl Kendaraan {
    
    fn informasi(&self){
        match self {
            Kendaraan::Sepeda(nama) => {
                println!("sepeda dengan nama: {}", nama);
            }

            Kendaraan::Motor(jenis, nama, top_speed) =>{
                println!("motor dengan jenis: {} nama: {} top speednye: {}", jenis, nama, top_speed)
            }

            Kendaraan::Mobil(nama, tahun_keluaran) => {
                println!("mobil dengan nama: {} tahun keluaran: {}", nama, tahun_keluaran)
            }
        }
    }
}

#[test]
fn test_destructuring_enum_data_patterns(){
    let kendaraan_gue: Kendaraan = Kendaraan::Motor(String::from("Vario"), String::from("Matic"), 200);

    kendaraan_gue.informasi();
}


// Pattern Matching Fallback
// kadang di kasus tertentu, seperti pattern mathcing pada value misal tipe string
// kita tak mungkin menuliskan semua kombinasi kata yang mungkin
// nah kasus kasus seperti ini kita bisa menggnakan 'other' pada pattern matching

#[test]
fn test_fallback_option(){
    let kata: &str ="Budis";

    match kata {
        "Budi" => {
            println!("ini si: {}", kata)
        }

        other => {
            println!("ini kata baru: {}", other)
        }
    }
}

// Multiple Patterns
// Pattern Matching bisa menggunakan beberapa kondisi menggunakan | (pipe)
// Misal jika kita buat kode: "Faiz" | "Hannan" | "Hakim"
// Artinya value boleh, Faiz | Hannan | Hakim

#[test]
fn test_multiple_patterns(){
    let name: &str = "Faiz";

    match name {
        "Faiz" | "Hannan" | "Hakim" =>{
            println!{"Ini nama lo"}
        }

        other => {
            println!("Ini bukan nama lu: {}", other)
        }
    }
}

// Range Patterns 
// Multiple patterns sangat cocok untuk match value lebih dari satu, tapi bagaimana jika kita butuh multiple value dalam bentuk range, misal 0 sampai 10
// Maka akan sangat meyulitkan jika harus dibuat dalam bentuk Multiple Patterns
// untungnya, pattern matching juga mendukung range patterns
// Jadi kita cukup gunakan tipe data Range (seperti yang pernah dibahas di materi Slice)
// Namun saat ini, range yang bisa digunakan adalah tipe data Inclusive range (sampai akhir)


#[test]
fn test_range_pattern(){
    let nilai: u8 = 73;

    match nilai {
        
        90..=100 => {
            println!("Gokil Lo")
        }

        80..=89 =>{
            println!("Mantap lo")
        }

        70..=79 => {
            println!("Oke dah Mayan")
        }

        60..=69 => {
            println!("peuy lo")
        }

        40..=59 => {
            println!("tolol lu ya")
        }

        0..=40 => {
            println!("goblok")
        }

        other =>{
            println!("heker lo")
        }


    }


}

// Destructuring Struct Patterns 
// Selain Enum, Pattern Matching juga bisa digunakan untuk melakukan destructuring terhadapa struct field 
// Namun untuk nama harus sama dengan nama field nya
// Kecuali untuk tipe tuple struct, kita bisa gunakan anama variable lain
// jika kita tidak buthu field nya untuk digunakan, kita bisa gunakan ..(titik sebanyak dua kali)
struct Coordinate{
    long: f32,
    lat: f32
}

impl Coordinate{
    fn new(long: f32, lat:f32) -> Coordinate {
        return Coordinate{long, lat}
    }
}
#[test]
fn test_struct_patterns(){
    let titik: Coordinate = Coordinate::new(13.42, 33.232);

    match titik{
        Coordinate{long, ..} =>{
            println!("longitude: {}", long)
        }

        // ini tak akan tereksekusi karna sudah di eksekusi oleh yang hanya long karna walaupun kita memebri ".." kosong, namun match sudah menganggap itu mengcover semua kemungkinan
        Coordinate{long, lat} =>{
            println!("longitude: {}, latitude: {}", long, lat)
        }
    }
}

// Ignoring
// Sebelemunya di struct jika kita tidak butuh fieldnya, kita bisa gunakan ..(titik 2)
// namun pada kasus tuple struct, enum kita tidak bisa melakukan hal itu karena posisi field nya sudah diatur sesuai dengan posisinya
// jika kita tidak butuh field tersebut, kita bisa ganti menjadi _ (underscore)
// atau jika tidak butuh data apapun, kita juga bisa gunakan _ (underscore) seluruhnya

enum JenisHewan{
    Mamalia(String, String),
    NonMamalia(String),
}

impl JenisHewan {
    fn new_mamalia(nama: String,umur: String) -> JenisHewan{
        return JenisHewan::Mamalia(nama, umur)
    }
}

#[test]
fn test_ignoring(){
    let kucing: JenisHewan = JenisHewan::new_mamalia(String::from("Mikel"), String::from("3"));

    match kucing {
        JenisHewan::Mamalia(nama, _) =>{
            println!("Hewan ini adalah mamalia, nama hewan ini adalah: {}", nama)
        }

        JenisHewan::NonMamalia(_) =>{
            println!("Ini hewan non mamalia")
        }

    }
}

// Match Expression
// Pattern Matching juga dianggap sebagai expression seperti if loop while dll, artinya bisa menghasilkan value 
struct Siswa{
    nama: String,
    umur: u8,
    lulus: bool,
}

impl Siswa {
    fn new_siswa(nama: String, umur: u8, lulus: bool) -> Siswa {
        return Siswa{nama: nama,umur: umur, lulus: lulus};
    }

    fn informasi(&self){
        println!("Siswa dengan nama: {} berumur: {} dinyatakan {}", self.nama, self.umur, if self.lulus {"Lulus"} else {"Tidak Lulus"})
    }
}

#[test]
fn test_match_expression(){
    let nilai: u8 = 100;

    let siswa1: Siswa = Siswa::new_siswa(String::from("Faiz"), 19, 
    match nilai {
        50 ..= 100 => {
            true
        } 

        _ => {
            false
        }
    }
);

   siswa1.informasi();
}

// Type Alias
// Kadang kita butuh membuat tipe data alias (nama lain) dari tipe data yang sudah ada
// ini biasa digunakan agar lebi mendeskripsikan tipe data dibandingkan yang aslinya

type ID = u32;

struct Customer{
    Id: ID,
    Nama: String,
    Umur: u8
}

struct Manager{
    Id: ID,
    nama: String,
    Umur: u8,
    Bagian: String
}

// Module
// Saat membuat aplikasi yang semakin komples, maka kode program kita akan semakin banyak
// agar kode yang kita buat semakin rapih kita bisa mengorganisir kode kita dalam bentuk module
// sehingg kode kode yang dalam fitur yang sama, bisa kita simpan dalam module yang sama agar lebih rapih
//  untuk membuat module kita bisa gunakan kata kunci mod lalu diikuti dengan nama module nya
// di dalam module tersebut, kita bisa simpan semua kode program yang ingin kita tempatkan
// best practice untuk membuat module nya seharusya di tempatkan di paling atas
// namun karna ini adalah vid belajar tak mengapa disni saja
mod model {
    pub struct Orang {
        firstname: String,
        lastname: String,
        username: String,
        email: String,
        age: u8,
    }

    impl Orang {
        pub fn orangBaru(firstname: String,
        lastname: String,
        username: String,
        email: String,
        age: u8,) -> Orang {
            return Orang{
                firstname, lastname, username, email, age
            }
        }
        
        pub fn sayHello(&self, name: &str){
            println!("hello {} my name is {}", name, self.firstname);
        }
    }
}

// Visibility
// Secara default, kode did dalam module seperti type, funciton method dll, itu hanya bisa diakses di dalam module yang sama
// jika kita ingin mengakses isi dar module tersebut di luar module nya, kita harus ubah akses dari private menjadi public 
// kita bisa menggunakan kata kunci pub diawal type ata function atau method atau struct dll
// Selanjutnya untuk mengakses type atau function atau struct dll, module yang sudah di pub
// adalah nama_module::nama_sesuatu_yang_dipanggil

#[test]
fn test_visibility(){
    let orang1: model::Orang = model::Orang::orangBaru(
        String::from("Budi"),
        String::from("Santoso"),
        String::from("budisantoso"),
        String::from("budi@email.com"),
        25,
    );

    orang1.sayHello("BudSkuy");
}

// Use Keyword
// Kadang agak terlalu sulit jika kita harus terus menerus menulis nama module ketika ingin menggunakan type atau funciton di sebuah module
// kita bisa menggunakan use untuk mengambil member dari sebuah module, masuk ke scope module yang melakukan use,
// Sehingg kita tidak perlu lagi menyebutkan nama module nya ketika menggunakan member tersebut
// jika kita melakukan use beberapa member di module yang berbeda, tapi ternyata member nya sama, maka kita bisa gunakna kata kunci as untuk membuat alias agar nama member tidak bentrok 

mod first {
    pub fn say_hello(){
        println!("Hello from first module")
    }
}

mod second {
    pub fn say_hello(){
        println!("Hello from second module")
    }
}

use first::say_hello;
use second::say_hello as say_hello_second;

#[test]
fn test_use(){
    say_hello();
    say_hello_second();
}

// Module Di File Terpisah
// walaupun kita sudah organisir kode program kita dalam module, namun jika disimpan di dalam satu file, lama lama kode program akan terlalu panjang dan sulit untuk di maintain
// kita bisa memisahkan module ke file terpisah, secara otomatis nama file akan menjadi nama module, jadi kita tidak wajib menambahkan kode mod lagi
// kecuali jika kita ingin menambahkan sub module, kita bisa tambahkan mod lagi di dalam file module yang sudah terpisah
// secara default, file module yang sudah dipisah tidak akan di include dalam program
// jika kita ingin menggunakan module file tersebut, kita harus include menggunakan mod namafile 

mod moddua;
mod modsatu;


#[test]
fn test_mod_terpisah_file(){
    moddua::say_hello();
    modsatu::say_hello();
}

// Penggunaan Use Lain Nya
// kadang ketika kode program kita sudah banyak, dan kita melakukan use banyak member di satu module, maka kode use kita akan sangat banyak
// ada beberapa cara lain untuk melakukan use, jika kita ingin menambil semua member di module, kita bisa gunakan tanda * (bintang)
// conoth: use module::* ( mengambil semua isi module)
// atau jika ingin mengambil beberapa saja, bisa sebutkan dalam tanda kurung kurawal {}
// contoh: use module::{A, B, C}

// Crate
// Crate adalah kode yang dijalankan oleh rust compiler untuk membuat aplikasi atau library
// Contoh di project yang sebelumnya kita buat, bentuknya adalah aplikasi, maka crate nya adalah src/main.rs
// Kita tidak bisa ubah itu, karena itu sudah jadi ketentuan jika ingin membuat aplikasi, maka kita perlu membuat main file yang berisi main function 
// di dalam module Crate, kita harus definisikan file file yang ingin kita gunakan sebagai Module
// dan ketika melakukan use di dile yang bukan main.rs kita harus gunakan "crate::" diawal untuk mengacu ke main.rs
// istilahnya crate adalah module utama
// syaratnya adalah harus di deklarasikan dulu sebagai module di main.rs baru bisa diambil di tempat lain
mod memanggil_crate;
// coba pergi ke memanggil_crate.rs

#[test]
fn memanggil_crate(){
    memanggil_crate::tes_memanggil_crate();
}

mod hewan_jenis {
    // Tambahkan pub agar bisa diakses dari luar modul
    #[derive(Debug)]
    pub enum JenisHewan {
        MAMALIA,
        NONMAMALIA,
    }

    pub struct Jenis {
        pub jenis: JenisHewan,
    }

    pub mod nama_hewan {
        // Perbaikan: &std::str diubah menjadi &str
        // Perbaikan: Menggunakan super untuk mengakses JenisHewan dari parent modul
        pub struct Kucing {
            pub nama: &'static str,
            pub jenis: super::JenisHewan,
        }
    }
}

#[test]
fn test_general() {
    let kucing: hewan_jenis::nama_hewan::Kucing = hewan_jenis::nama_hewan::Kucing {
        nama: "Jeki",
        jenis: hewan_jenis::JenisHewan::MAMALIA,
    };

    // Gunakan {:?} karena JenisHewan menggunakan #[derive(Debug)]
    println!("nama hewan: {} berjenis: {:?}", kucing.nama, kucing.jenis);
}

// Trait
// Trait adalah definisi fungsionalitas untuk tipe data lain 
// Biasanya Trait digunakan utnuk dasar dari implementasi beberapa tipe data
// di bahasa pemrograman lain seperti java atau golang, trait mirip sepeti interface
// trait berisi definisi method tanpa implementasi konkrit
// Untuk membuat Trait, kita bisa menggunakan kata kunci trait diikuti dengan nama trait nya



// Default Implementation
// Sebelumnya kita hanya membuat method di Trait tanpa implementasi konkritnya
// Trait sebenernya bisa juga digunakan untuk membuat Method dengan imp,ementasi konkrit, atau kita sebut dengan Default Implementation
// Secara Otomatis type yang nanti melakukan implementasi, akan mendapatkan default implementasi dari method tersebut
trait CanSayHello {
    // Default Implementation
    // Sebelumnya kita hanya membuat method di Trait tanpa implementasi konkritnya
    // Trait sebenernya bisa juga digunakan untuk membuat Method dengan imp,ementasi konkrit, atau kita sebut dengan Default Implementation
    // Secara Otomatis type yang nanti melakukan implementasi, akan mendapatkan default implementasi dari method tersebut
    fn hello(&self) -> &'static str {
        return "Hello"
    }

    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

// Implemebtsi trait
// trait bisa digunakan sebagai tipe data, namun tetap perlu ada implementasi konkritnya, misal menggunakan struct atau enum
// untuk implementasi trait kita bisa gunakan
// impl namaTrait for namaType {
// }

struct Manusia{
    nama: &'static str,
    umur: u16,
}

impl CanSayHello for Manusia {
    fn say_hello(&self) -> String {
        println!("nama saya {}", self.nama);
        return String::from(self.nama)
    }

    fn say_hello_to(&self, name: &str) -> String {
        println!("nama saya {} dan nama_kamu {}", self.nama, name);
        return String::from(self.nama).add("dan nama kamu adalah: ").add(name)
    }
}

// Menggunakan Trait
// Trait tidak bisa diubat instance nya
// Untuk membuat instance dengan tipe trait maka kita harus menggunakan implementasinya

#[test]
fn test_trait(){
    let manusia1: Manusia = Manusia {
        nama: "Adam",
        umur: 1000,
    };

    println!("{}", manusia1.hello());

    println!("{}", manusia1.say_hello_to("Hawa"))
}

// Trait Sebagai Parameter
// Salah satu keuntungan menggunakan Trait adalah ketika kita gunakan Trait sebagai parameter 
// Saat kita gunakan Trait sebagai parameter, maka kita bisa gunakan value apapun yang merupakan implementasi dari Trait tersebut sebagai value untuk parameternya
// Untuk menggunakan trait sebagai parameter, kita bisa gunakan kata kunci impl NamaTrait pada parameter nya
// Jika kita ingin tipe data reference bisa kita gunakan &impl NamaTrait

fn sayHelloBuatManusia(types: &impl CanSayHello){
    println!("{}", types.say_hello_to("Nadia"))
}

#[test]
fn test_trait_param(){
    let manusia2: Manusia = Manusia { nama: "Anancuy", umur: 19 };
    sayHelloBuatManusia(&manusia2);

    println!("{}", manusia2.say_hello())
}

// Multiple Trait 
// Type itu bisa mengimplementasikan lebih dari satu trait
// oleh karena itu, saat kita membuat parameter juga, kita bis abuat satu parameter untuk beberapa tipe trait
// kita bisa gunakan tanda + (plus) jika ingin membuat parameter dengan tipe Multiple Trait, misal (impl Trait1 + Trait2 +Trait3)

trait Skillnya {
    fn info_skill(&self){}
}

impl Skillnya for Manusia{
    fn info_skill(&self) {
        println!("gw jagooo mikir")
    }
}

fn skill_sama_say_hello(inst: &(impl CanSayHello + Skillnya)){
     inst.info_skill();
    inst.say_hello();
}

#[test]
fn test_multiple_trait(){
    let manusia3: Manusia = Manusia { nama: "Faiz", umur: 19 };
    manusia3.info_skill();
    manusia3.say_hello();
    skill_sama_say_hello(&manusia3);
}

// Return Trait
// Selain untuk parameter, trait juga bisa digunakan sebagai return value di function 
// namun seperti yang dijelaskan di awal, karena trait tidak bisa diubat instance nya secara langsung
// maka value yang kita kembalikan juga harus dalam bentuk implementasi type nya
// untuk membuat trait sebagai return value, kita perlu sebuthkan seperti parameter yaitu impl NamaTrait nya

struct SimplePerson {
    name: &'static str,
}

trait CanSayGoodbye{
    fn good_bye(&self) -> String;
}

impl CanSayGoodbye for SimplePerson{
    fn good_bye(&self) -> String {
        return String::from("dadah dari ").add(self.name);
    }
}

fn create_simple_person(name: &'static str) -> impl CanSayGoodbye{
    return SimplePerson{name}
}

#[test]
fn test_return_trait(){
    let simpleperson: Box<dyn CanSayGoodbye> = Box::new(create_simple_person("Jeki"));

    println!("{}", simpleperson.good_bye())
}

// Conflict Method Name 
// Salah Satu problem ketika menggunakan beberapa trait adalah, kadang nama method di trait itu bentrok atau konflik dengan method di trait lain nya
// atau bahkan bisa bentrok dengan method di type nya sendiri
// saat kita buat implementasi dari trait, rust tidak akan menjadikan itu sebagai error, namun masalahnya terjadi ketika kita memanggil method nya
// rust akan menjadikan itu error karena method nya ambigu, rust akan komplen karena ada beberapa method dengan nama yang sama
// cara agar kita bisa menentukan method yang ingin kita panggil, kita bisa sebutkan Type::nama_method(instance)

struct Makanann{
    nama: &'static str,
    jenis:  &'static str,
}

impl Makanann {
    fn new(nama: &'static str, jenis: &'static str) -> Makanann{
        return Makanann{nama, jenis}
    }
    fn makan(&self) -> String{
        return String::from("Ayo dimakan ").add(self.nama).add(" keburu abis")
    }
}

trait Hidangan {
    fn makan(&self, nama: &'static str) -> String;
}

impl Hidangan for Makanann{
    fn makan(&self, nama: &'static str) -> String {
         return String::from("Eh ").add(nama).add(" Ayo dimakan ").add(self.nama).add(" keburu abis")
    }
}


#[test]
fn test_conflict_trait_method(){
    let pisang_goreng: Makanann = Makanann::new("pisang goreng", "gorengan");
    println!("{}", pisang_goreng.makan())
    // println!("{}", pisang_goreng.makan("anan")) // lihat disini conflict dan malah method trait nya tak bisa 
    
}

// Super Trait 
// Trait bisa digabungkan dengan konsep mirip pewarisan, dimana satu trait bsa memiliki beberapa trait dibawahnya
// trait yang ada diatasnya bisa kita sebut super trait
// misal kita punya trait a, lalu kita buat trait b, dan trait c, trait a kita jadikan sebagai super trait dari trait b dan trait c
// artinya sekarang jika kita implementasi trait b atau trait c, secara otomatis kita harus implementasi juga trait A
// trait boleh memiliki lebih dari satu super trait, caranya kita bisa gunakan tanda + (plus)

trait CanSay: CanSayHello + CanSayGoodbye{
    fn say(&self);
    // sekarang jika kita implement CanSay
    // Kita harus implementasi CanSayHello dan CanSayGoodbye
}

// Generic
// Generic Merupakan fitur dimana kita bisa membuat function, struct, enum method dan trait, yang tipe datanya bisa diubah ketika digunakan
// fitur ini sangat berguna ketika memang kita ingin membuat kode yang generic/general untuk berbagai tipe data, sehingga kita tidak perlu tentukan dari awal tipe data yang ingin kita gunakan
// kita akan coba fitur generic ini di berbagai lokasi yang bisa dilakukan di rust

// Generic di struct
// ketika membuat generic di struct, kita bisa tambahkan tipe data generic setela nama struct menggunakan tanda <> (diamond), dimana di dalam tanda diamond tersebut, kita sebutkan nama nama tipe data generic nya
// tipe data generic bisa lebih dari satu, tinggal gunakan , (koma) sebagai pemisah tipe data generic nya 
// Biasanya nama tipe data generic hanya menggunakan satu huruf kapital

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}


#[test]
fn test_generic_struct(){
    let integer: Point<i32> = Point::<i32> {x: 5, y:10};

    println!("{:?}", integer )
}

// Generix Di enum
// sama seperti struct, di enum juga bisa menambahkan generic
// perbedaan nya kit aharus menulis tipe nya juga nama enumnya saat deklarasi dengan ::
// jika di struct tadi kita hanya perlu menulis tipenya saat deklarasi

enum Value<T> {
    NONE,
    VALUE(T)
}

#[test]
fn test_generic_enum(){
    let nilai: Value<u8> = Value::<u8>::VALUE(10);
    
}


// Batasan Tipe Generic
// saat embuat generic kita bisa memberikan batasan tipe yang diperbolehkan
// caranya kita bisa gunakan :(titik dua) diikuti dengan trait
// artinya generic type yang hanya diperbolehkan hanyalah implementasi dari trait
// jika ingin menggunakan multiple trait maka seperti biasa kita gunakan + (plus)

struct Hi<T: CanSayGoodbye>{
    value: T,
}
#[test]
fn test_batasan_generic(){
    let orang1: SimplePerson = SimplePerson { name: "Anjay" };
    let bisa_hai: Hi<SimplePerson> = Hi::<SimplePerson>{value: orang1};

    println!("{}", bisa_hai.value.good_bye())
}

// Partial Order
// Partial Order adalah implementasi/trait dari semua tipe data yang bisa dibandingkan 

// Generic Di Function
// kita juga bisa membuat generic di function, kita bisa tabahkan tipe data generic setelah nama function

fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 {
        return value1
    } else {
        return value2
    }
}

#[test]
fn test_generic_function(){
    let data_minimal: u8 = min::<u8>(2,3);

    println!("data minim: {}", data_minimal);
}

// Generic di Method
// kita juga bisa membuat generic di method, kita bisa tambahkan tipe data generic setelah kata kunci impl, yang secara otomatis bisa digunakan di semua method
// kita juga bisa membuat khusus untuk method tertentu ya caranya sama saja seperti generic dun function

impl <T> Point<T> {
    fn get_x(&self) -> &T {
        return &self.x
    }
    fn get_y(&self) -> &T {
        return &self.y
    }

    // kita juga bisa membuat khusus untuk method tertentu ya caranya sama saja seperti generic dun function
    fn pointBaru<E>(x: E, y:E)->Point<E>{
        return Point{x, y}
    }
}

// Generic di Trait
// Saat kita membuat Trait, kita juga bisa menambahkan generic type
// saat kita membuat generic type di trait secara otomatis kita akan memaksa implementasi trait tersebut menggunakan eneric type di implementasi
trait GetValue<T> {
    fn totalxy(&self) -> T;
}

// Where Clause sebelumnya saat menggunakan type bound(batasan jenis), kita akan menggunakan :(titik dua) diikuti dengan trait
// ada cara yang lain untuk menambahkan type bound(batasan jenis), caranya menggunakan kata kunci where
// ini akan lebih mudah dibaca ketika type bound sangat banyak

impl<T> GetValue<T> for Point<T>where
    T: Add<Output = T> + Copy, // Membatasi T agar bisa dijumlahkan dan disalin
{
    fn totalxy(&self) -> T {
        let x: T = self.x;
        let y: T = self.y;
        let hasil: T = x + y;
        
        hasil // Di Rust, ekspresi terakhir otomatis menjadi nilai kembalian (tanpa kata kunci return)
    }
}

// Overloadable Operators
// Sebelumnya kita pennah belajar tentang operator matematika untuk dara number
// apakah tipe data selain number mendkung operatorseperti +, -, dan yang lain nya?
// secara default tidak, namun rust memiliki fitur dimana kita bisa menimplementasikan operator dalam bentuk method, sehingga bisa menggunakan operatormatematika
// semua operator itu di representasikan dalam trait yang bisa kita implementasikan
// semua trait nya berada di module /Crate core::ops
// https://doc.rust-lang.org/core/ops/index.html

struct Apple {
    quantity: i32
}

impl Add for Apple {
    type Output = Apple;


    fn add(self, rhs: Self) -> Self::Output{
        return Apple{
            quantity: self.quantity + rhs.quantity
        }
    }

    
}

impl Apple{

    fn new(jumlah: i32) -> Apple {
        return Apple { quantity: jumlah }
    }

    fn addinRef(&self, rhs: Self) -> Apple{
        return Apple{
            quantity: self.clone().quantity + rhs.quantity
        }
    }
}

#[test]
fn test_overloadable_variable(){
    let mut apel: Apple = Apple::new(112);
    let mut apel2: Apple = Apple::new( 115);
    {
        let apel_minjem: &Apple = &apel;
       

        println!("{}", apel_minjem.quantity);
    }

    {
        let mut apel_pinjem: &mut Apple = &mut apel;
        let hasil:Apple = apel_pinjem.addinRef(Apple { quantity: 50 });
        println!("{}", hasil.quantity)
    }

    let hasil:Apple = apel + apel2;

    println!("{}", hasil.quantity)
}

// Null atau undefined
// rust tidak mengenal hal hal seperti null atau undefined
// lantas bagaimana jika kita ingin membuat variable yang memang datanya bisa ada atau tidak ada
// di rust ada satu tipe yakni Option
// Rust menyediakan Option enum yang merupakan representasi dari optional value
// sederhananya, Option enum menyediakan dua opsi None() untuk opsi nilai kosong dan Some() untuk opsi yang tidak kosong

fn tarik_data_api(keadaan: bool)-> Option<i32>{
    if keadaan{
        return Option::<i32>::Some(11);
    } else {
        return Option::None;
    }
}

#[test]
fn test_option(){
    let data: Option<i32> = tarik_data_api(false);

    if let Option::Some(nilai) = data{
        println!("inilah data yang didapat {}", nilai);
    } else if let Option::None = data {
       println!("ternyata gagal datanya kosong");
    }
}

// Comparing Operator,
// Selain Operator matematika, di rust juga bisa digunakan untuk pembuatan operator perbandingan menggunaka Module/Crate core:cmp
// Penggunaan nya sama, kita tinggal implementasi Tarit yang sesuai dengan operator yang ingin kita buat
// https://doc.rust-lang.org/core/cmp/index.html

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        return self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn test_comparing_operator(){
    let apel1: Apple = Apple::new(112);
    let apel2: Apple = Apple::new(112);
    println!("{}, {}", apel1 == apel2, apel1.eq(&apel2))
}

// String Manipulation
// Sebelumnya kita sudah beajar tipe data string
// kita bisa memnaipulasi data string detailnya:
// https://doc.rust-lang.org/std/primitive.str.html

#[test]
fn test_string_manipulation() {
    // ==========================================
    // 1. TRIMMING (Menghapus Whitespace)
    // ==========================================
    println!("--- 1. Trimming ---");
    let text = "   Halo Rust   ";
    println!("Asli: '{}'", text);
    println!("Trim (Kiri & Kanan): '{}'", text.trim());
    println!("Trim Start (Kiri): '{}'", text.trim_start());
    println!("Trim End (Kanan): '{}'", text.trim_end());
    println!();

    // ==========================================
    // 2. MENGUBAH HURUF (Case Conversion)
    // ==========================================
    println!("--- 2. Case Conversion ---");
    let kata = "Belajar Rust";
    println!("Asli: {}", kata);
    println!("Lowercase: {}", kata.to_lowercase());
    println!("Uppercase: {}", kata.to_uppercase());
    println!();

    // ==========================================
    // 3. PENGECEKAN & PENCARIAN
    // ==========================================
    println!("--- 3. Pengecekan & Pencarian ---");
    let kalimat = "Belajar Pemrograman Rust itu seru";
    println!("Kalimat: {}", kalimat);
    println!("Mengandung kata 'Rust'? {}", kalimat.contains("Rust"));
    println!("Diawali dengan 'Belajar'? {}", kalimat.starts_with("Belajar"));
    println!("Diakhiri dengan 'seru'? {}", kalimat.ends_with("seru"));
    
    // Pencarian posisi (index byte)
    if let Some(index) = kalimat.find("Rust") {
        println!("Kata 'Rust' ditemukan pada index byte ke: {}", index);
    }
    println!();

    // ==========================================
    // 4. REPLACEMENT (Penggantian Teks)
    // ==========================================
    println!("--- 4. Replacement ---");
    let buah = "apel pisang apel mangga apel";
    println!("Asli: {}", buah);
    
    // Mengganti semua kemunculan
    let ganti_semua = buah.replace("apel", "jeruk");
    println!("Ganti semua 'apel' ke 'jeruk': {}", ganti_semua);
    
    // Mengganti sejumlah N kemunculan saja
    let ganti_satu = buah.replacen("apel", "jeruk", 1);
    println!("Ganti 1 'apel' pertama: {}", ganti_satu);
    println!();

    // ==========================================
    // 5. SPLITTING (Pemisahan String)
    // ==========================================
    println!("--- 5. Splitting ---");
    let csv = "andi,budi,cahyono";
    println!("Data CSV: {}", csv);
    println!("Hasil pemisahan berdasarkan koma (,):");
    
    for item in csv.split(',') {
        println!("- {}", item);
    }
    println!();

    // ==========================================
    // 6. MUTASI (Khusus tipe data `String`)
    // ==========================================
    println!("--- 6. Mutasi String ---");
    let mut dinamis = String::from("Halo");
    println!("Awal: {}", dinamis);

    // Menambah string di akhir
    dinamis.push_str(" Dunia");
    println!("Setelah push_str(' Dunia'): {}", dinamis);

    // Menambah 1 karakter di akhir
    dinamis.push('!');
    println!("Setelah push('!'): {}", dinamis);

    // Menyisipkan string di index tertentu
    dinamis.insert_str(4, " Teman");
    println!("Setelah insert_str(4, ' Teman'): {}", dinamis);

    // Mengosongkan string
    dinamis.clear();
    println!("Setelah clear(), apakah kosong? {}", dinamis.is_empty());
    println!();

    // ==========================================
    // 7. PARSING (String ke Angka)
    // ==========================================
    println!("--- 7. Parsing String ---");
    let angka_str = "2026";
    // Mengubah &str ke tipe data i32
    let angka: i32 = angka_str.parse().unwrap();
    println!("String '{}' berhasil diubah menjadi integer: {}", angka_str, angka + 4);
}

// Formatting
// Sebelumnya kita sering menggunakan println!()
// println!() adalah macro, bukan function
// saat menggunakan macro println!() kita sering menambahkan parameter tambahan untuk menampilkan data
// secara default, data tidak bisa ditampilkan dalam macro println!, yang bisa ditampilkan hanyalah data yang sudah implementasi module core:fmt
// https://doc.rust-lang.org/core/fmt/index.html


impl Debug for SimplePerson{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SimplePerson")
        .field("nama", &self.name)
        .finish()
    }
}


#[test]
fn test_format() {
    let person: SimplePerson = SimplePerson{name: "Faiz"};

    println!("{:?}", person)
}


// Closure
// Closure adalah function tanpa nama yang biasanya disimpan di variable atau digunakan di parameter
// kita bisa membuat closure dan memanggil closure ektika membutuhkn nya
// untuk membua closure kita bisa menggunakan tipe data fn(paramType) -> returnType
// dan untuk memanggil closure, kita bisa panggil menggunakan nama variable atau parameter nya secara langsung

#[test]
fn test_closure(){
    let sum: fn(i32, i32) -> i32 = |value1: i32, value2:i32| -> i32{
        return value1+ value2
    };

    let result:i32 = sum(14,23);
    println!("{}", result)
}

// Cloure sebagai Parameter
// Closure juga bisa dijadikan parameter sebuah function

fn closure_param(nilai1:u8, nilai2:u8, closure: fn(u8, u8, )->u8) -> u8{
    return closure(nilai1, nilai2)
}

#[test]
fn test_closure_param(){
    let sum: fn(u8, u8) -> u8 = |value1: u8, value2:u8| -> u8{
        return value1+ value2
    };
    println!("{}", closure_param(112 as u8, 50 as u8, sum))
}


// Closure Scope
// saat kita membuat closure, perlu diperharikan bahwa kita bisa menangkap data di scope yang sama
// fitur ini kadang membingungkan, dan kurang disarankan, oleh karena itu harap bijak dalam menggunakan fitur ini

#[test]
fn test_closure_function() {
 let mut counter = 0;

 let mut increment = || {
    counter += 1
 };

 increment();
 increment();
 increment();

 println!("Counter: {}", counter)
}

// Collection 
// Sebelumnya kita sudah membahas tipe data array, namun array itu tidak bisa berkembang ukurannya
// untungnya rust menyediakan tipe data collection, jumlah data di collection bisa berkembang, selain itu, collection disimpan di heap, bukan di stack seperti array
// sebelumnya tipe data string juga termasuk collection, yaitu collection of characters
// tapi di materi ini kita akan bahas tipe collection selain string

// Secara garis besar, tipe data sequence di rust dibagi menjadi 3
// Sequences, yaitu tipe data Collection yang memiliki index
// Maps, yaitu tipe data collection berupa key-value
// Sets. yaitu tipe data collection unique yang tdak memiliki index


// Sequences
// Sequence adalah tipe data collectinon yang mirip seperti array, dia memili index
// List Sequence
// Vec (Vector)
// VeqDeque
// LinkedList

// Vector
// Vector adalah sequence yang urutn nya sesuai dengan yang kita inginkan
// menambah data ke vector dilakukan dibagian belakang
// cocok untuk implementasi tumpukan(stack, last in first out
// https://doc.rust-lang.org/std/struct/Vec.html

#[test]
fn test_vector() {
    let mut nama_nama: Vec<String> = Vec::new();
    nama_nama.push(String::from("Faiz"));
    nama_nama.push(String::from("Hannan"));

    // Perbaikan 2: Menggunakan for loop (borrowing agar vector tidak habis/moved)
    for name in &nama_nama {
        println!("{}", name);
    }

    // Perbaikan 2: Menggunakan for loop (borrowing agar vector tidak habis/moved)
    for name in &nama_nama {
        println!("{}", name);
    }
}

// VecDeque
//VecDeque sebenernya mirip dengan vector
// yang membedakan adalah dia memiliki kemampuan menambah data di depan(head) dan di belakang(end)
// Sehingga VecDeque juga cocok digunakan untuk implementasi Queue (Antrian), First In First Out ataupun Last In Last Out
// https://doc.rust-lang.org/std/collections/struct.VecDeque.html

#[test]
fn test_vec_deque() {
    // Membuat VecDeque baru
    let mut antrian: VecDeque<String> = VecDeque::new();

    // Menambah data di belakang (push_back), mirip seperti Vector biasa
    antrian.push_back(String::from("Faiz"));
    antrian.push_back(String::from("Hannan"));

    // Menambah data di depan (push_front)
    antrian.push_front(String::from("Budi"));

    // Urutan antrian saat ini: ["Budi", "Faiz", "Hannan"]

    // Iterasi menggunakan borrowing (&antrian) agar tidak moved
    for nama in &antrian {
        println!("{}", nama);
    }

    // Mengambil data dari depan (pop_front) - simulasi antrian (FIFO)
    let depan = antrian.pop_front();
    println!("Keluar dari depan: {:?}", depan); // Output: Some("Budi")

    // Mengambil data dari belakang (pop_back)
    let belakang = antrian.pop_back();
    println!("Keluar dari belakang: {:?}", belakang); // Output: Some("Hannan")
}

// LinkedList
// LinkedList merupakan implementasi Sequences menggunakan struktur data linked list.
// Struktur data linked list sangat efisien untuk penambahan dan pengurangan data, 
// oleh karena itu sangat cocok ketika kita butuh sequence yang tidak terprediksi ukurannya.
// Namun perlu diperhatikan, performa linked list tidak sebaik vector ketika mengakses 
// data menggunakan index, oleh karena itu linked list tidak memiliki fitur untuk mengakses data menggunakan index.
// https://doc.rust-lang.org/std/collections/struct.LinkedList.html

#[test]
fn test_linked_list() {
    // Membuat LinkedList baru
    let mut data: LinkedList<String> = LinkedList::new();

    // Menambah data di belakang (push_back)
    data.push_back(String::from("Faiz"));
    data.push_back(String::from("Hannan"));

    // Menambah data di depan (push_front)
    data.push_front(String::from("Budi"));

    // Urutan saat ini: ["Budi", "Faiz", "Hannan"]
    // Perhatikan: Kita tidak bisa mengakses data menggunakan index seperti `data[0]`

    // Iterasi menggunakan borrowing (&data) agar tidak moved
    for nama in &data {
        println!("{}", nama);
    }

    // Mengambil/menghapus data dari depan (pop_front)
    let depan = data.pop_front();
    println!("Keluar dari depan: {:?}", depan); // Output: Some("Budi")

    // Mengambil/menghapus data dari belakang (pop_back)
    let belakang = data.pop_back();
    println!("Keluar dari belakang: {:?}", belakang); // Output: Some("Hannan")
}

// Maps 
// Maps merupakan jenis collection yang berisi key-valye
// berbeda dengan sequence yang indec nya menggunakan number, pada map kita bebad menentukan tipe key
// Key itu unik, artinya jika kita memasukan data ke Map dengan key value yang sama, secara otomatis itu artinya operasi replace (mengganti)

// HashMap dan BTreeMap
// Rust memiliki dia implementasi map, yaitu HashMap dan BTreeMap
// perbedaan yang mencolok adalah pada BTreeMap, key akan diurutkan
// Dan karena HashMap tidak akan diurutkan, oleh karena itu operasi untuk memasukan data di HashMap lebih cepat dibanding BtreeMap,
// namun urutan key tidak bisa menjamin urutan nya sama sekali

#[test]
fn test_maps() {
    let mut hash_map1: HashMap<&'static str, &'static str> = HashMap::new();
    hash_map1.insert("112", "kjbdkbskbc");
    hash_map1.insert("882", "sikocak tuh");


    let mut btree_map1: BTreeMap<i32, String> = BTreeMap::new();
    btree_map1.insert(12, String::from("Fahri"));

    for (id, nama) in &hash_map1{
        println!("{}", id);
        println!("{}", nama);
    } // tak ada jaminan bahwa data akan berurutan

    for (id, nama) in &btree_map1 {
        println!("{}", id);
         println!("{}", nama);
    } // dijamin data akan selalu ber urutan

    let key: i32 = 12;
    let orang: &String = match btree_map1.get(&key){
        Option::None => {&String::from("Kosong")}

        Option::Some(val) => {&val}
    };


    println!("{}", orang)
}

// Sets merupakan tipe data collection dimana data di dalm set tidak boleh duplikat
// jika kita memasukan data ke dalam set dengan data yang sudah ada di dalam set secara otomatis data tidak akan diterima
// set tidak seperti sequence, data di set tidak bisa diakses menggunakan index
// Sama seperti Map, set memiliki dua implementasi di rust yaitu HashSet dan BTreeSet
// HashSet tidak menjamin urutan data, karena tujuan HashSet adalah memastikan tidak ada data duplikat secara tepat
// BTreeSet selain memastikan tidak ada data duplikat, juga mengurutkan data di dalam Set nya, oleh karena itu performanya lebih lambat dari HashSet karena perlu mengurutkan data setiap kita menambah data ke BTreeSet

use std::collections::BTreeSet;

#[test]
fn test_set() {
    // Membuat HashSet kosong untuk menampung data unik
    let mut hash_set1: HashSet<&'static str> = HashSet::new();
    
    // Memasukkan data ke HashSet
    hash_set1.insert("Fahri");
    hash_set1.insert("Budi");
    hash_set1.insert("Andi");
    hash_set1.insert("Andi");
    
    // Mencoba memasukkan data duplikat ("Fahri" sudah ada)
    // Data ini tidak akan ditambahkan, dan fungsi insert akan mengembalikan nilai false
    hash_set1.insert("Fahri"); 

    // Membuat BTreeSet untuk data yang unik sekaligus terurut
    let mut btree_set1: BTreeSet<i32> = BTreeSet::new();
    btree_set1.insert(50);
    btree_set1.insert(10);
    btree_set1.insert(30);
    btree_set1.insert(10); // Duplikat, akan diabaikan

    // Iterasi pada HashSet (urutan data acak / tidak dijamin)
    println!("=== HashSet Output (Tidak Berurutan) ===");
    for nama in &hash_set1 {
        println!("{}", nama);
    }

    // Iterasi pada BTreeSet (dijamin selalu berurutan dari kecil ke besar: 10, 30, 50)
    println!("=== BTreeSet Output (Dijamin Berurutan) ===");
    for angka in &btree_set1 {
        println!("{}", angka);
    }

    // Mengecek apakah sebuah nilai ada di dalam Set menggunakan method .contains()
    let target = "Budi";
    if hash_set1.contains(target) {
        println!("Ketemu! {} ada di dalam hash_set1", target);
    } else {
        println!("{} tidak ditemukan", target);
    }
}

// Iterator
// Rust memiliki module Iterator, yang digunakan sebagai mekanisme untuk melakukan operasi terhadap urutan dari data
// Semua tipe daya tang multiple seperti, Array, Slice dan Collection memiliki fitur iterator
// dengan menggunakan Iterarot, secara otomatis kita bisa melakukan iterasi data menggunakan For Loop terhadap value tersebut
// https://doc.rust-lang.org/std/iter/index.html

#[test]
fn test_iterator() {
    let mut mep: HashMap<i32, & 'static str> = HashMap::new();
    mep.insert(1, "Anda");
    mep.insert(2, "bayu");
    mep.insert(3, "Ikmal");

    for (absen, nama) in &mut mep.clone().into_iter() /* inilah implementasi yang sebenernya */{
        println!("absen: {} adalah: {}", absen, nama)
    } 

    {   
    let mut iterator:IntoIter<i32, &'static str> = mep.clone().into_iter(); 

    // Selama .next() masih mengembalikan Some(m), loop akan terus berjalan maju
    while let Some(m) = iterator.next() {
        println!("absen: {} adalah: {}", m.0, m.1);
    }
    }

}

// Error Handling
// Error merupaan hal yng sudah bisada dalam pembuatan aplikasi
// Hampir semua bahasa pemrograman memiliki fitur untuk Error Handling termasuk rust
// Rusttidak punya tipe data Exception, seperti di java, PHP, Javascript
// rust menggunakan pendekatan lain untuk Error Handling

// UNrecoverable Error
// Jika terdapat jenis error yang menrut kita tidak bisa dipulihkan maka kita bisa gunakan jenis Unrecoverable Error
// Rust menggunakan macro panic! untuk melakukan ini
// Beberapa hal di rust juga menggunakan Unrecoverable Error, contoh ketika mengakses index di array / vector diluar jangkauan indexnya, maka akan menjadi unrecoverable error

fn connecti_db(host: Option<String>) {
    match host {
        Some(data) => {
            println!("ada provider {}", data);
        }
        None => {
            println!("gaada");
            panic!("No database Founded");
        }
    }
}

#[test]
fn test_unrecoverable_error() {
    let nama_db: String = String::from("DB1");
    connecti_db(Option::Some(nama_db));
}

// Recoverable Error
// Seperti yang dijelaskan di awal, rust tidak memiliki tipe data Exception. Lantas bagaimana untuk jenis Recoveravle Error?, sama seperti Enum Option, Rust menyediakan Enum Result Untuk ini
// jadi ketika kita membuat function yang bisa mengembalikan sukses atau gagal, kita bisa buat function dengan return value enum result
// enum result hanya memiliki dua nilai, OK(T) dan Err(E)
// https://doc.rust-lang.org/std/result/enum.Result.html

fn recover_error(data: Result<String, String>){
    match data{
        Result::Ok(info) => {
            println!("Berhasil: {}", info)
        }

        Result::Err(err) => {
            println!("Gagal: {}", err)
        }
    }
}

fn test_connect(path: &'static str, skema: bool) -> Result<String, String>{
    if !skema {
        return Result::Err(String::from("Gagal terhubung ke db"))
    } 

    return Result::Ok(String::from("Berhasil terhubung ke db dari path").add(path))
}

#[test]
fn test_recover_error() {
    recover_error(test_connect("https:/sianjir", false));
}

// Lifetime
// Di materi ownership dan juga reference, kita sudah tau bahwa tiap data / reference memiliki lifetime yang sudah ditentukan
// secaradefault lifetime di rust suda ditemtukan mengikuti scope variavle
// sehingga aman dan rust juga melakukan borrow check pada saat melakukan kompilasi untuk memastikan tidak ada masalah yang bernama dangling reference(reference ke value yang sudah tidak ada di memory)

// #[test]
// fn test_dangling_reference() {
//     let r: &i32 ;
//     {
//         let x: i32 = 5;
//         // r = &x; //ini dangling pointer karna begitu scope x selesai, maka nilai x akan resmi di hapus, maka si r yang reference ke si x akan reference ke alamat yang memori nya sudah tidak ada/ tidak valid (dangling reference)
//     }

//     println!("r : {}", r)
// }


// Lifetime di function
// Salah satu hal yang membingungkan lain nya adalah ketika kita menggunakan reference sebagai parameter, sekalogus sebagai return value
// misal kita akan membuat function dengan dua parameter reference, lalu kita bandingkan dan mengemablikan salah satu parameter reference sebagai return value
// pada kasus ini, rust akan bingung karena harus melakukan borrow parameter pertama atau parameter kedua , karena kondisi nya bisa berbeda

// fn longest(value1: &str, value2: &str) -> &str /*disini akan terjadi error, karna rust akan bingung melakukan borrow param 1 atau 2 dst*/ {
//     if value1.len() > value2.len(){
//         return value1
//     }

//     return value2
// }

// Lifetime Annotation Syntax

// Pada kasus kifetime di parameter sebelumnya, rust menyediakan fitur bernama lifetime annotation, dimana kita bisa menyebutkan yang mana yang kemungkinan akan di borrow
// cara menambah lifetime annotation sama seperti generic, hanya saja type nya diawali dengan '(petik satu)
// selanjutnya pada variable yang kita ingin tandai lifetime annotation type, kita bisa tambahkan juga sebelum type aslinya
// intinya semua parameter yang "memiliki kemungkinan akan di return", harus ditambahkan lifetime annotation syntax

fn longest<'a>(val1: &'a str, val2: &'a str) -> &'a str /* disini aman rust tidak akan bingung untuk melakukan borrow param 1 atau 2 dst*/{
    if val1.len() > val2.len() {
        return val1
    } 
    return val2
}

#[test]
fn test_lifetime_annotation_syntx() {
    let hasil: &str = longest("Mantap", "Apa");
    println!("hasil: {}", hasil)
}

fn coba<'a, T: Copy>(data: &'a mut Vec<T>, ike: u8) -> Option<&'a mut T> {
    let mut count: u8 = 0;
    for d in data {
        count += 1;
        if count == ike {
            return Some(d)
        }
    };
    None
}


#[test]
fn testlife() {
    let mut data: Vec<f32> = vec![22.4, 3.142323232, 223.23, 323.22, 311.212];
   
    {
         let phi: &mut f32 = coba(&mut data, 2).unwrap();
         *phi = 100.200;
    }

   println!("ini yang dipilih: {}", coba(&mut data, 2).unwrap())
}

// INFOO!
// Lifetime annotation tidak akan mengubah waktu alur hidup, hanya penanda untuk membantu rust borrow checker
// oleh karena itu pada kasus jika ternyata alur hidup variable sudah selesai, maka bisa aja terjadi error seperti di awal, yaitu dangling reference

// Lifetime Annotation di Struct 
// Lifetime Annottion mirip seperti generic, kita bisa gunakan juga di struct dengan menggunakan Lidetime Annotation si struct, kita bisa menandai field dengan tipe reference
// dengan begitu, kita bisa menggunakan lifetime annotatin ketika nanti menggunakan struct tersebut

struct Student<'a> {
    name: &'a str,
}

fn longest_student_name<'a>(student1: &'a Student<'a>, student2: &'a Student<'a>) -> &'a Student< 'a> {
    if student1.name > student2.name {
        return student1
    }
    return student2
}

#[test]
fn test_struct_lifetime() {
    let mut data_student: Vec<Student> = Vec::new();
    let nama_nama: [&str; 10] = ["Faiz", "Cihuy", "Anjay", "Mabar", "GURINJAY", "Keren", "Asik", "Apalah", "OSNUISBSCBSISOSSASS", "Makan"];

    for nama in nama_nama {
        data_student.push(Student{name: nama});
    }

    let mut paling_panjanga_nama: &Student = &data_student[0];
    for student in &data_student{
        paling_panjanga_nama = longest_student_name(student, paling_panjanga_nama)
    };

    println!("nama terpanjang: {}", paling_panjanga_nama.name);
}

// Lifetime Annotation DI Method
// Lifetime Annotation juga bisa dilakukan di method

impl <'a> Student <'a> {
    
}

// Lifetime Annotation dan Generic Type
// Saat menggunakan Lifetime Annotation, kita bisa gabungkan bersama generic type
// Jadi tidak erlu khawatir, karena kita bisa menggunakan secara berbarengan

struct Teacher<'a, ID> where ID: Ord {
    id: ID,
    name: &'a str,
}

#[test]
fn test_lifetime_annotation_generic_struct(){
    let teacher: Teacher<i32> = Teacher {
        id: 10,
        name: "Faiz",
    };

    println!("{}", teacher.id);
    println!("{}", teacher.name);
}

// Attribute
// Attrebute merupakan cara menambah metadata (informasi tambahan) ke kode yang kita buat
// Syntax Attribute di rust mirip dengan di bahasa pemrograman C# menggunakan tanda #[NamaAttribute]
// Di bahasa pemrograman lain, ada juga yang menyebutnya sebagai Decorator di typescript atau Annotation di java / kotlin
// Attribte di rust sudah disediakan value value yang bisa digunakan, kita bisa opsi yang disediakan di
// https://doc.rust-lang.org/reference/attributes.html

#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
    name: String,
    location: String,
    website: String,
}

#[test]
fn test_attribute_debug(){
    let company = Company {
        name: "Rust".to_string(),
        location: "USA".to_string(),
        website: "https://www.rust-lang.org".to_string()
    };

    let company2 = Company {
        name: "Rust".to_string(),
        location: "USA".to_string(),
        website: "https://www.rust-lang.org".to_string()
    };

    println!("{:?}", company);
    println!("{}", company == company2);
}

// Tidak ada Magic di rust sebenernya ketika kita gunakan derive attribute
// ketika proses kompilasi rust membuat kode yang dibutuhkan sebelum dikompilasi
// untuk melihat hasil kdoe yang dibuat kita bisa gunakan plugin namanya cargo-expand
// dengan cara cargo install cargo-expand


// Smart Pointer
// Pointer adalah konsep yang umum dimana sebuah variable berisi alamat lokasi data di memory
// di rust, reference merupakan pointer
// smart pointer adalah tipe data pointer namun memiliki metadata(informasi tambahan) dan kemampuan lain selain sebagai penunjuk ke lokasi data 
// di rust yang menggunakan konsep ownership(pemilik) dan borrowing(peminjam), pada kebanyakan kasus, reference hanya meminjam data, sedangkan smart pointer merupakan pemilik dari data yang ditunjuk

// Box<T> untuk menunjuk data di heap
// Menggunakan Box<T>, mengizinkan kita membuat data di heap sedangkan pointer-nya disimpan di Stack
// https://doc.rust-lang.org/std/boxed/struct.Box.html
// ini bisa membuat lebih hemat
// contoh pada angka angka i32, u8 dll kan defaultnya dia akan disimpan di stack
// dimana akan terus membuat stack baru
// daripada seperti itu kenapa tidak buat 1 lokasi penyimpanan dan ketika diubah tak akan di clone / copy
// melainkan langsung mengubah saja ke heap

fn display_value(value: i32){
    println!("ini value {}", value)
}

fn display_reference_value(value: &i32){
    println!("ini value {}", value)
}
#[test]
fn test_smart_pointer() {
    let mut five: Box<i32> = Box::new(5);
    
    // Menggunakan * untuk mengubah nilai di dalam heap
    *five = 11; // data tidak akan di clone lalu diubah yang baru namun langsung yang asli di memori
    // mungkin hanya akan terasa kecil namun dalam proses besar atau berulang nantinya mungkin akan baru berasa hemat nya

    println!("{}", five);
    display_value(*five);
    display_reference_value(&five);
}   

// Recursive Data Type
// Single data dari box mungkin terlihat tidak begitu menarik, namun Box akan sangat berguna ketika kita menemui tipe data yang recursive
// Misal kita punya tipe data category, dimana di dalam nya bisa terdapat Category lagi, kita sering melihat jenis data seperti ini, contohnya di Toko Online

#[derive(Debug)]
enum Category {
    // Memperbaiki penulisan Box<Category>
    Of(String, Box<Category>),
    End,
}


#[test]
fn test_smart_pointer_recursive() {
    // Membuat data kategori secara rekursif (bersarang)
    let category = Category::Of(
        String::from("Electronics"),
        Box::new(Category::Of(
            String::from("Smartphone"),
            Box::new(Category::End),
        )),
    );

    println!("{:?}", category);
}

// Dereference
// Saat kita menggunakan Reference, kadang kita ingin melakukan manipulasi data langsung ke valuenya
// kita bisa melakukan dereference untuk mengakses langsung valuenya, bukan lagi reference nya
// Untuk melakukan Dereference, kita bisa menggunakan operator *(bintang)

#[test]
fn test_dereference() {
    let mut value1:Box<i32> = Box::new(10);
    let mut value2:Box<i32> = Box::new(100);

    *value1 += 200; //dengan dereference maka akan langsung mengubah nilai value aslinya

    println!("{}", *value1 + *value2);
}

// Dreference Trait
// Saat kita menggunakan Reference atau Box <T> kita bisa menggunakan * Operator untuk melakukan dereference
// bagaimana jika kita menggunakan tipe lain? misal struct yang ktia buat sendiri?
// namun jika kita ingin membuat struct yang kita buat memiliki kemampuan dereference, kita bisa menggunakan Deref Trait
// https://doc.rust-lang.org/std/ops/trait/Deref.html
// Khusus untuk mutable value kita juga bisa menggunakan DerefMut
// https://doc.rust-lang.org/std/ops/trait.DerefMut.html

struct MyValue<T> {
    value: T,
}

impl <T> Deref for MyValue<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn kali_sepuluh(angka: &mut i32){
    *angka = *angka * 10;
}

#[test]
fn test_deref() {
    let value: MyValue<i32> = MyValue {value: 10};
    let mut realValue: i32 = *value;
    kali_sepuluh(&mut realValue);
    println!("value: {}", realValue)
}

// Drop Trait
// Saat kita membuat vaue, ketika value tersebut keluar dari scope, secara otomatis value akan di drop (hapus) oleh rust
// drop trait merupakan trait yng bisa kita implementasikan, untuk membuat kode yang akan dieksekusi sebelum value di drop
// Misal menutup koneksi, resource dan lain lain

struct Book {
    tittle: String,
}

impl Book {
    fn new(nama: String)-> Book {
        return Book{tittle: nama}
    }
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Dropping Book: {}", self.tittle);
    }
}

#[test]
fn test_drop_book() {
    {
        let book1: Book = Book::new("Doraemon".to_string());

        println!("{}", book1.tittle)
    }

    println!("book1 sudah hilang");
}

// Multiple Ownership
// Pada umumnya, value biasanya hanya dimiliki oleh satu variable
// Namun, mungkin akan ada kasus dimana satu value dimiliki oleh beberapa variable, contoh misal pada struktur data Graph, dimana satu titik bisa berasal dari beberapa titik
// seperti yang kita tahu, bahwa defaultnya di rust satu value hanya bisa dimiliki oleh satu variable
// Jika kita ingin membuat satu value bisa dimiliki oleh beberapa variable, maka kita harus menggunakan type Rc<T> (Reference Counted)
// Rc<T> atau reference counted adalah tipe data smart pointer yang bisa digunakan utnuk lebih dari satu variable owner
// Penggunaan Rc<T> mirip dengan Box<T>
// https://doc.rust-lang.org/alloc/rc/index.html
// https://doc.rust-lang.org/alloc/rc/struct..Rc.html

enum Brand {
    Of(String, std::rc::Rc<Brand>),
    End,
}
#[test]
fn test_multiple_ownership() {
    let apple: std::rc::Rc<Brand> =  std::rc::Rc::new(Brand::Of("Apple".to_string(), std::rc::Rc::new(Brand::End)));
    let mut rnCount: usize = std::rc::Rc::strong_count(&apple);

    println!("reference apple saat ini: {}", rnCount);
    let laptop: Brand = Brand::Of("Laptop".to_string(), std::rc::Rc::clone(&apple));

    rnCount = std::rc::Rc::strong_count(&apple);

    {
        let laptop4: Brand = Brand::Of("Laptop".to_string(), std::rc::Rc::clone(&apple));

    rnCount = std::rc::Rc::strong_count(&apple);
    }
    println!("reference apple saat ini: {}", rnCount);

     rnCount = std::rc::Rc::strong_count(&apple);

      println!("reference apple saat ini: {}", rnCount);

      let angka_ubah_dipinjem: std::rc::Rc<u32> = std::rc::Rc::new(12);

      {
        let angka_ubah_minjem: std::rc::Rc<u32> = angka_ubah_dipinjem.clone();
        let angka_ubah_minjem: std::rc::Rc<u32> = angka_ubah_dipinjem.clone();

        println!("{}", std::rc::Rc::strong_count(&angka_ubah_dipinjem))
      }

      println!("{}", std::rc::Rc::strong_count(&angka_ubah_dipinjem))


}


// Interior Mutability
// Interior mutability adalah design pattern dalam rust yang memperbolehkan kita mengubah data walaupun ada reference immutable terhadap data tersebut
// sebelumnya kita tahu untuk membuat mutable reference, kita bisa gunakan &mut namun selain itu kita juga bisa menggunakan cara lain.
// Untuk melakukan hal ini kita bisa menggunakan type RefCell<T>

// RefCell<T>
// Tidak seperti Rc<T>, refcell mempresentasikan single ownership pada data yang ditunjuk, lantas apa bedanya dengan Box<t>?
// Pada materi borrowing kita tahu bahwa pada satu waktu, tidak diperbolehkan membuat mutable reference lebih dari satu, sehingga rust tidak akan bisa melakukan kompilasi kode program
// Dengan RefCell<T>, pengecekan jadi terjadi pada proses runtime (ketika program berjalan), bukan lagi ketika proses kompilasi
// https://doc.rust-lang.org/core/cell/struct.RefCell.html

// Aturan RefCell<T>
// banyak immutable borrow diperbolehkan
// Satu mutable borrow diperbolehkan
// Bnayak mutable borrow tidak diperbolehkan
// Sekaligus mutable dan immutable borrow tidak diperbolehkan

#[derive(Debug)]
struct Seller{
    name: std::cell::RefCell<String>,
    active: std::cell::RefCell<bool>,
}

#[derive(Debug)]
struct Guru{
    name: std::rc::Rc<Box<String>>,
    active: std::rc::Rc<bool>
}

impl Guru {
    fn new(nama: String, active: bool) -> Self {
        return Guru{name: std::rc::Rc::new(Box::new(nama)), 
            active: std::rc::Rc::new(active)}
    }
}

#[test]
fn test_ref_cell() {
    let seller  = Seller {
        name: std::cell::RefCell::new("Faiz".to_string()), //walau seller adalah let yang artinya dia adalah immutable namun akan tetap berubah karna RefCell akan diubah di runtime 
        active: std::cell::RefCell::new(true)
    };

    let guru: Guru = Guru::new("Faiizzz".to_string(), true);



   {
     let mut result = seller.name.borrow_mut();
    
    *result = "andi".to_string();
   }

    println!("{:?}", seller);
    println!("{:?}", guru)
}

// Static
// Static adalah variable seperti constant
// Cara pembuatan static variable mirip dengan COnstant, hanya menggunakan kata kunci static

static APPLICATION: &str = "My App";
#[test]
fn test_static() {
    println!("Application: {}", APPLICATION)
}

//Mutable Static
// Value di constant itu tidak bisa diubah lagi, sedangkan pada static, kita bisa buat mutable static, yang value nya bisa diubah lagi
// namun karena static itu bisa diakses oleh siapapun, jadi ada kemungkinan tidak aman, misal terjadi race condition
// Oleh karena itu untuk mengubah mutable static, kita wajib menggunakan unsafe block, atau unsafe function

static mut COUNTER: u32 = 0;
unsafe fn increment(){
    COUNTER += 1;
}

#[test]
fn test_static_mut() {
    // Menggunakan raw pointer untuk menghindari warning/error direct access
    unsafe {
        std::ptr::addr_of_mut!(COUNTER).write((*std::ptr::addr_of!(COUNTER)) + 1);
        println!("{}",  *std::ptr::addr_of_mut!(COUNTER));
    }
}

// Macro
// Sebelumnya, kita sudah sering menggunakan println!(), itu bukanlah function, melainkan macro
// macro adalah fitur di rust yang merupakan kode untuk membuat kode lain nya. di bahasa pemrograman lain, sering disebut dengan meta programming
// kelebihan macro adalah, kita bisa membuatode yang kita mau sebelum compile melakukan kompilasi
// kekurangan macro adalah, implementasi macro lebih kompleks dibanding implementasi function biasa

// Declarative Macro
// salah satu cara yang umum dilakukan untuk membuat macro adalah Declarative Macro menggunakan macro_rules!
// saat membuat macro menggunakan macro_rules, kita akan tentukan rule yang berisi (pattern) => {expansion}
// Pattern merupakan kondisi yang diinginkan, dan expansion merupakan kode yang akan dibuat oleh macro

// Patterns 
// Patterns yangterdapat di macro_rules sangat beragam, kita bisa cek di halaman dokumentasinya
// https://doc.rust-lang.org/reference/macros-by-example.html#metavariables


macro_rules! hi {
    () => {
        println!("Hi")
    };

    ($name: expr) => {
        println!("Hi {}!", $name)
    };
}

#[test]
fn test_macro() {
    hi!("Paes")
}

// Repetition
// Kadang saat membuat macro kita butuh param lebih dari satu, atau di macro disebut repetition
// ketika menggunakan macro, kita bisa gunakan $() diikuti koma, lalu repetition perator
// *, artinya boleh berapapun
// +, artinya boleh berapapun, tapi minimal satu
// ?, boleh satu atau kosong, sehingga tidak perlu pemisah koma

macro_rules! ulang{
    ($array:expr) => {
        for i in $array {
            println!("{}", i)
        }
    };

    ($($item:expr), +) => {
        $(
            println!("{}", $item);
        )*
    }
}

#[test]
fn test_repetition_macro() {
    let arey: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    ulang!(arey);
}