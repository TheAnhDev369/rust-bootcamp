fn main() {
    //  In ra terminal Hello, World! sau khi chay cargo run
    println!("Hello, world!");
    println!("Tao project:                     cargo new project");
    println!("Di chuyen vao du an vua tao:     cd name_project");
    println!("Check project:                   cargo check");
    println!("Chay Project:                    cargo run");
    println!("Build Project:                   cargo build");
    println!("Ham main la gham thuc hien dau tien");

    println!("Ghi chu (comment trong Rust)");
    println!("Cac loai ghi chu :");
    println!("// la ghi chu dong");
    println!("/*...*/ la ghi chu nhieu dong(block)");

    // println!("Hello, world! lan 2");
    println!("Hello, world! lan 2");

    /*
        In ra Hello
        In ra World!
    */
    println!("Hello");
    println!("World!");

    //  Bien trong Rust
    println!("Bien la dai dien cho gia tri (value)");
    println!("Bien mac dinh la bat bien (immutable)");
    println!("De tao bien co the thay doi, su dung tu khoa 'mut' ");
    println!("Vi du nhu sau");
    println!("Dinh nghia bien x co gia tri ban dau la 5");
    let x = 5;
    println!("Gia tri cua bien x la {}", x);
    // x = x + 1;

    // Doi voi ngon ngu khac thi cong duoc, nhung voi ngon ngu Rust thi bao do:
    // cannot mutate immutable variable 'x'
    //
    // Ta ket luan rang bien x nay la bat bien
    // Neu ta muon tang hoac thay doi gia tri cua bien x thi ta phai them tu khoa mut,
    // de bien nay khong con la bat bien nua, va trinh compile cung se biet ta them tu khoa mut vao de nham muc dich thay doi gia tri
    println!("Neu muon tang gia tri cua bien x len 1, ta lam nhu sau voi bien y de minh hoa: ");
    let mut y = 255;
    y = y + 15;
    println!("{}", y);

    //  In bien trong Rust
    println!("De in ra 1 bien, ta la nhu sau:");
    println!("Vi du 1:");
    let year = 2025;
    println!("In ra nam: {}", year);

    //  In ra thong tin + bien trong Rust
    println!("In ra thong tin + bien: ");
    let year1 = 2025;
    println!("Year: {}", year1);

    //  In ra nhieu bien trong Rust
    println!("In ra nhieu bien: ");
    let year2 = 2025;
    let name = "Rust Bootcamp";
    println!("{} in {}", name, year2);

    //  In nhieu bien khong theo thu tu (Placeholders) trong Rust
    println!("In nhieu bien khong theo thu tu (Placeholders) ");
    let year3 = 2025;
    let name1 = "Rust Bottcamp";
    println!("{1} in {0}", name1, year3);

    println!("Mac dinh se dung {{}} de 'nhet gia tri vao chuoi, mac dinh no in theo thu tu truyen tham so: '");
    println!("Trong Rust, println! dung cu phap goi la 'poisitional arguments(doi so theo vi tri)' de thuc hien dinh dang chuoi");
    println!("O day, {{1}} nghia la in ra doi so thu hai (chi so bat dau tu 0), con {{0}} se la doi so thu nhat");
    println!("{{0}} co nghia la : in ra bien dau tien ta truyen vao macro println!");
    println!("{{1}} co nghia la : in ra bien thu hai ta truyen vao macro println!");
    println!(".....");
    println!("{{5}} co nghia la : in ra bien thu sau ta truyen vao macro println!");
    println!(".....");

    //  In ra nhieu bien theo bien ten
    let year4 = 2025;
    let name2 = "Rust Bootcamp";
    println!(" {name2} in {year4}");
    println!("Day la cu phap 'String Interpolation' trong Rust, sieu gon va tien ");
    println!(
        "Trong do: name2 va year4 la ten bien, ta khong can ghi theo thu tu nhu {{0}}, {{1}} nua"
    );
    println!("Ma Rust se tu 'noi suy gia tri tu ten bien' vao trong chuoi");

    //  Cach su dung va uu diem
    println!("Khi nao thi ne dung cai nay ?");
    println!(
        "Su dung {{}} + thu tu {{0}}, {{1}} -> Uu diem la khi ta can tai su dung thu tu nhieu lan"
    );
    println!("{{name}} -> Uu diem: Ro rang, de doc, it loi nham thu tu");

    //  Kieu du lieu trong Rust
    println!("Kieu du lieu trong Rust");
    println!("Co 2 kieu du lieu trong ngon ngu Rust:");
    println!("Scalar: luu tru don du lieu nhu integer, float, char, boolean, string");
    println!("Compund: luu tru da gia tri nhu Array, Tuple");
    println!("Vi du 1:");
    println!("Scalar:");
    let bootcamp = "Rust Bootcamp";
    println!("Khoa hoc nay ten la gi ? {}", bootcamp);
    println!("&str: kieu chuoi bat bien (string litetal).");
    let year5 = 2025;
    println!("Nam nay la nam {}", year5);
    println!("i32: kieu so nguyen 32 bit (mac dinh)");
    let free = true; //   Kieu boolean(True and False)
    println!("Day la 1 khoa hoc mien phi dung khong {}", free);
    println!("Rust co kieu suy doan, ta khong nhat thiet phai ghi ro kieu neu Rust doan duoc");

    println!("Constants: Hang so");
    println!("const thuong dung de khai bao 'hang so, gia tri khong the thay doi'");
    println!("Phai chi dinh kieu ro rang 'f32 o day la folat 32-bit");
    println!("Ten hang so thuong viet HOA");
    const PI: f32 = 3.14;
    println!("1 PI bang 7 ty hay {}", PI);

    println!("Array: dinh nghia mang: ");
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Mang arr chua 5 phan tu, moi phan tu la i32.");
    println!("[i32; 5]: nghia la mang kieu i32, dai 5 phan tu");

    println!("Truy cap phan tu");
    println!("arr[0]: truy cap phan tu dau tien(giong cac ngon ngu nhu C, PyThon, Java,...)");
    let first = arr[0];
    println!(" phan tu dau tien cua mang la {}", first);
    println!("Mang trong Rust co phan tu co dinh, va tat ca cac phan tu PHAI CUNG KIEU");
    println!("Dac diem: co dinh kich thuoc, cung kieu du lieu");
}
