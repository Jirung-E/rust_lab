use tokio::{
    io::{self, AsyncWriteExt, AsyncReadExt, BufReader, BufWriter}, 
    net::{TcpListener, TcpStream},
};


async fn run_server(listener: TcpListener) {
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let (mut reader, mut writer) = io::split(socket);
            io::copy(&mut reader, &mut writer).await.unwrap();
        });
    }
}

async fn run_client_buf() {         // 675ms
    let stream = TcpStream::connect("localhost:7878").await.unwrap();
    let (reader, writer) = io::split(stream);
    let mut writer = BufWriter::new(writer);
    let mut reader = BufReader::new(reader);
    
    let mut packet = Vec::with_capacity(12);
    packet.extend_from_slice(&10u16.to_be_bytes());
    packet.extend_from_slice(b"Hello 1234");
    packet = packet.repeat(10);

    let start = tokio::time::Instant::now();
    
    for _ in 0..2000 {
        writer.write(&packet).await.unwrap();
        writer.flush().await.unwrap();

        for _ in 0..10 {
            let mut buf = vec![0; 2];
            reader.read_exact(&mut buf).await.unwrap();

            let size = u16::from_be_bytes([buf[0], buf[1]]) as usize;

            let mut data = vec![0; size];
            reader.read_exact(&mut data).await.unwrap();
        }

        // println!("{:?}", buf);
    }

    println!("1 - Elapsed: {:?}", start.elapsed());
}

async fn run_client() {             // 666ms
    let mut stream = TcpStream::connect("localhost:7878").await.unwrap();

    let mut packet = Vec::with_capacity(12);
    packet.extend_from_slice(&10u16.to_be_bytes());
    packet.extend_from_slice(b"Hello 1234");
    packet = packet.repeat(10);

    let start = tokio::time::Instant::now();
    
    for _ in 0..2000 {
        stream.write(&packet).await.unwrap();

        let mut buf = vec![0; 500];
        stream.read(&mut buf).await.unwrap();

        let mut s = 0;
        for _ in 0..10 {
            let size = u16::from_be_bytes([buf[s+0], buf[s+1]]) as usize;

            let mut data = vec![0; size];
            data.copy_from_slice(&buf[s+2..s+2+size]);

            s += size + 2;
        }

        // println!("{:?}", buf);
    }

    println!("2 - Elapsed: {:?}", start.elapsed());
}

#[tokio::main]
async fn main() {
    let addr = "localhost:7878";
    let server = TcpListener::bind(addr).await.unwrap();

    println!("Listening on: {}", addr);

    std::thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(run_server(server));
    });

    std::thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(run_client_buf());
    });

    std::thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(run_client());
    });

    loop {}
}