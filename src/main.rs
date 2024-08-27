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

async fn run_client_buf() {
    let stream = TcpStream::connect("localhost:7878").await.unwrap();
    let (reader, writer) = io::split(stream);
    let mut writer = BufWriter::new(writer);
    let mut reader = BufReader::new(reader);

    let start = tokio::time::Instant::now();
    
    for _ in 0..10000 {
        writer.write(b"12345678901234567890123456789012345678901234567890").await.unwrap();
        writer.flush().await.unwrap();

        let mut buf = vec![0; 500];
        reader.read(&mut buf).await.unwrap();
        // println!("{:?}", buf);
    }

    println!("1 - Elapsed: {:?}", start.elapsed());
}

async fn run_client() {
    let mut stream = TcpStream::connect("localhost:7878").await.unwrap();

    let start = tokio::time::Instant::now();
    
    for _ in 0..10000 {
        stream.write(b"12345678901234567890123456789012345678901234567890").await.unwrap();

        let mut buf = vec![0; 500];
        stream.read(&mut buf).await.unwrap();
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