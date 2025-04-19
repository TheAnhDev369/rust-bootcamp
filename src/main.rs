fn main() {
    // In ra terminal Hello, World! sau khi chạy cargo run
    println!("Hello, world!");
    println!("Tạo project:                     cargo new project");
    println!("Di chuyển vào dự án vừa tạo:     cd name_project");
    println!("Kiểm tra project:                cargo check");
    println!("Chạy project:                    cargo run");
    println!("Biên dịch project:               cargo build");
    println!("Hàm main là hàm thực hiện đầu tiên");

    // Ghi chú trong Rust
    println!("Ghi chú (comment trong Rust)");
    println!("Các loại ghi chú:");
    println!("// là ghi chú dòng");
    println!("/*...*/ là ghi chú nhiều dòng (block)");

    // println!("Hello, world! lần 2");
    println!("Hello, world! lần 2");

    /*
        In ra Hello
        In ra World!
    */
    println!("Hello");
    println!("World!");

    // Biến trong Rust
    println!("Biến trong Rust");
    println!("Biến là đại diện cho giá trị (value)");
    println!("Biến mặc định là bất biến (immutable)");
    println!("Để tạo biến có thể thay đổi, sử dụng từ khóa 'mut'");

    println!("Ví dụ 1:");
    let x = 5;
    println!("Biến x có giá trị là: {}", x);

    let mut y = 25;
    y = y + 125;
    println!("{}", y);

    // In biến trong Rust
    println!("In biến trong Rust:");
    println!("Ví dụ 1:");
    let year = 2025;
    println!("In ra năm: {}", year);

    // In ra thông tin + biến trong Rust
    println!("In ra thông tin + biến:");
    let year1 = 2025;
    println!("Năm: {}", year1);

    // In ra nhiều biến trong Rust
    println!("In ra nhiều biến:");
    let year2 = 2025;
    let name = "Rust Bootcamp";
    println!("{} in {}", name, year2);

    // In nhiều biến không theo thứ tự (Placeholders) trong Rust
    println!("In nhiều biến không theo thứ tự (Placeholders)");
    let year3 = 2025;
    let name1 = "Rust Bootcamp";
    println!("{1} in {0}", name1, year3);

    println!("Mặc định sẽ dùng {{}} để 'nhét giá trị vào chuỗi, mặc định nó in theo thứ tự truyền tham số:'");
    println!("Trong Rust, println! dùng cú pháp gọi là 'positional arguments (đối số theo vị trí)' để thực hiện định dạng chuỗi");
    println!("Ở đây, {{1}} nghĩa là in ra đối số thứ hai (chỉ số bắt đầu từ 0), còn {{0}} sẽ là đối số thứ nhất");
    println!("{{0}} có nghĩa là: in ra biến đầu tiên ta truyền vào macro println!");
    println!("{{1}} có nghĩa là: in ra biến thứ hai ta truyền vào macro println!");
    println!(".....");
    println!("{{5}} có nghĩa là: in ra biến thứ sáu ta truyền vào macro println!");
    println!(".....");

    // In ra nhiều biến theo tên biến
    let year4 = 2025;
    let name2 = "Rust Bootcamp";
    println!("{name2} in {year4}");
    println!("Đây là cú pháp 'String Interpolation' trong Rust, siêu gọn và tiện");
    println!(
        "Trong đó: name2 và year4 là tên biến, ta không cần ghi theo thứ tự như {{0}}, {{1}} nữa"
    );
    println!("Mà Rust sẽ tự 'nội suy giá trị từ tên biến' vào trong chuỗi");

    // Cách sử dụng và ưu điểm
    println!("Khi nào thì nên dùng cái này?");
    println!(
        "Sử dụng {{}} + thứ tự {{0}}, {{1}} -> Ưu điểm là khi ta cần tái sử dụng thứ tự nhiều lần"
    );
    println!("{{name}} -> Ưu điểm: Rõ ràng, dễ đọc, ít lỗi nhầm thứ tự");

    // Kiểu dữ liệu trong Rust
    println!("Kiểu dữ liệu trong Rust");
    println!("Có 2 kiểu dữ liệu trong ngôn ngữ Rust:");
    println!("Scalar: lưu trữ đơn dữ liệu như integer, float, char, boolean, string");
    println!("Compound: lưu trữ đa giá trị như Array, Tuple");

    // Ví dụ Scalar
    println!("Ví dụ 1:");
    println!("Scalar:");
    let bootcamp = "Rust Bootcamp";
    println!("&str: kiểu chuỗi bất biến (string literal).");
    println!("{}", bootcamp);

    let year5 = 2025;
    println!("i32: kiểu số nguyên 32-bit (mặc định)");
    println!("{}", year5);

    let free = true; // Kiểu boolean (true hoặc false)
    println!("Khoá học này miễn phí đúng không ? {}", free);

    println!("Rust có kiểu suy đoán, ta không nhất thiết phải ghi rõ kiểu nếu Rust suy luận được.");

    // Constants: Hằng số
    println!("Constants: Hằng số");
    println!("const thường dùng để khai báo 'hằng số, giá trị không thể thay đổi'");
    println!("Phải chỉ định kiểu rõ ràng. Ví dụ: 'f32' ở đây là float 32-bit.");
    println!("Tên hằng số thường viết HOA.");
    const PI: f32 = 3.14;
    println!("1 PI bằng 7 tỷ hay {}", PI);

    // Array: định nghĩa mảng:
    println!("Array: định nghĩa mảng:");
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Mảng 'arr' chứa 5 phần tử, mỗi phần tử là kiểu i32.");
    println!("[i32; 5]: nghĩa là mảng kiểu i32, dài 5 phần tử");

    println!("Truy cập phần tử:");
    println!("arr[0]: truy cập phần tử đầu tiên (giống các ngôn ngữ như C, Python, Java,...)");
    let first = arr[0];
    println!("{}", first);
    println!("Mảng trong Rust có số phần tử cố định, và tất cả các phần tử PHẢI CÙNG KIỂU.");
    println!("Đặc điểm: cố định kích thước, cùng kiểu dữ liệu.");

    println!("Part 1");
    println!("");
}
