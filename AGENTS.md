# EigenGraph Agent Guidelines & Architectural Rules

## 🔒 ATURAN UTAMA: SIDEBAR TERKUNCI & WORKSPACE BEBAS MODEL SLIDE VERTIKAL

### 1. 🛡️ Layout Sidebar Terkunci Mutlak (Hingga Ada Perintah Eksplisit):
Layout Sidebar Kiri (`src/app.rs`) dan macOS Window Chrome **DIKUNCI PENUH DAN TIDAK BOLEH DIUBAH SAMA SEKALI** sampai pengguna secara eksplisit memberikan perintah tertulis (misal: `UBAH_SIDEBAR:` atau `UBAH_UI:`).

**Aturan Wajib Sidebar (macOS App Store Style):**
- **Full-Height Geometry**: Sidebar membentang tanpa jeda dari ujung atas (`y = 0`) ke ujung bawah layar.
- **Window Controls di Dalam Sidebar**: Lampu lalu lintas macOS (`🔴  🟡  🟢`) berada di pojok atas sidebar.
- **Pill Search Bar (Tanpa Logo)**: Kolom `🔍 Search` berada langsung di bawah traffic lights tanpa logo merk/header.
- **Dilarang Menambahkan Divider Header / Badge**: Tidak boleh ada section header seperti `LIBRARY`, `1D..10D`, atau pill badge.
- **Daftar Baku 13 Navigation Pillars**:
  1. `📁   My Library`
  2. `💡   Research & Concept`
  3. `📐   Drafting & Drawings`
  4. `🏗️   Modeling & Geometry`
  5. `⏱️   Time & Scheduling`
  6. `💰   Cost & Budgeting`
  7. `🌱   Sustainability & Energy`
  8. `🏢   Facility Management`
  9. `🛡️   Safety/Health & Safety`
  10. `♻️   Lean Construction`
  11. `🤖   Industrialized & Automation`
  12. `🔮   Prediction`
  13. `📊   Results`
- **Bagian Bawah Tersemat**: Profil Pengguna (`👤 Harids Irawan`) dengan gear `⚙️` (System Preferences ⌘,). Tombol Update dipindahkan ke dalam menu Settings Preferences, dan tombol AI Copilot di sidebar dihilangkan agar antarmuka bersih minimalis.

---

### 2. 🚀 Kebebasan Modifikasi Lembar Kerja (Workspace Canvas):
Untuk **Lembar Kerja (Area Canvas Utama)** pada seluruh pilar navigasi:
- **Bebas Diubah & Ditingkatkan**: Agen memiliki izin penuh untuk memodifikasi, memperkaya komponen grafis, menambah algoritma, panel kontrol, dan fitur analitik baru sesuai kebutuhan riset dan pengembangan fitur.
- **Wajib Model Slide Vertikal (Gaya My Library)**:
  - Tampilan lembar kerja **harus menggunakan scrolling vertikal yang mengalir mulus** (`egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| { ... })`).
  - Struktur konten disusun vertikal bertingkat (*stacked cards*) bergaya macOS modern:
    1. *Header Pilar Minimalis*: Judul bersih dengan pemisah garis halus (tanpa strip-bar ganda / breadcrumbs).
    2. *Hero/Featured Master Card*: Ringkasan proyek utama, status ARI, dan quick action.
    3. *Data & Parameter Control Cards*: Pengaturan parameter numerik, simulasi, dan matriks kekakuan.
    4. *Visualizer / Blueprint / Interactive Canvas*: Area gambar kerja 2D/3D atau grafik konvergensi.
    5. *Tabel Audit & Data Detil*: Rekapitulasi BoQ, emisi karbon, atau spektrum respon.
- **Minimalis & Bersih**: Hindari toolbar pita horizontal (*strip bar*) yang memotong ruang visual canvas.

---

### 3. Window & Asset Icon:
- Window icon di-load dari `assets/icon.png` via `eframe::icon_data::from_png_bytes`.
- macOS application bundle icon adalah `assets/AppIcon.icns`.

---

### 4. ⚡ Protokol Efisiensi Token & Eksekusi Lokal (Local-First Development):
- **Local-First Secara Default**:
  - Semua pengerjaan fitur baru, modifikasi antarmuka, perbaikan bug, dan pengujian unit dijalankan secara **100% lokal di dalam workspace komputer pengguna**.
  - Agen **DILARANG** melakukan pemindaian remote GitHub, querying API (`curl api.github.com`), pengecekan workflow GitHub Actions, atau pembacaan git remote secara proaktif saat menangani instruksi coding atau penambahan fitur biasa.
- **Pengecualian (Hanya Berdasarkan Perintah Eksplisit)**:
  - Operasi yang melibatkan jaringan / remote GitHub (seperti `git push`, memantau CI/CD build, atau mempublikasikan rilis) **HANYA** boleh dijalankan jika pengguna secara eksplisit memberikan perintah tertulis (misalnya: *"push ke github"*, *"buatkan rilis"*, *"cek status build"*, atau sejenisnya).
- **Prinsip Hemat Token**:
  - Agen wajib langsung berfokus ke file target yang bersangkutan tanpa melakukan pembacaan direktori yang tidak perlu atau membuang token konteks untuk hal di luar ruang lingkup tugas.

