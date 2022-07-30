// use tokio::fs::File;
// use tokio::io::{self, AsyncReadExt};

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut f = File::open("foo.txt").await?;
//     let mut buffer = [0; 10];

//     // read up to 10 bytes
//     let n = f.read(&mut buffer[..]).await?;

//     println!("The bytes: {:?}", &buffer[..n]);
//     Ok(())
// }

// use tokio::io::{self, AsyncWriteExt};
// use tokio::fs::File;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut file = File::create("foo.txt").await?;

//     // Writes some prefix of the byte string, but not necessarily all of it.
//     let n = file.write(b"some bytes").await?;

//     println!("Wrote the first {} bytes of 'some bytes'.", n);
//     Ok(())
// }

use tokio::fs::File;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut reader: &[u8] = b"hello world copy file";
    let mut file = File::create("foo.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    Ok(())
}
