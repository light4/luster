use std::io::{BufRead, BufReader, Read};

use failure::Error;

/// Takes an `R: BufRead` and:
///
/// - skips the leading UTF-8 BOM if there is one
/// - skips the unix shebang if there is one (if the first character is a '#', skips everything up
///   until but not including the first '\n')
///
/// This mimics the initial behavior of lua_loadfile[x].  In order to correctly detect and skip the
/// BOM and unix shebang, the internal buffer of the BufRead must be >= 3 bytes.
pub fn skip_prefix<R: BufRead>(r: &mut R) -> Result<(), Error> {
    if {
        let buf = r.fill_buf()?;
        buf.len() >= 3 && buf[0] == 0xef && buf[1] == 0xbb && buf[2] == 0xbf
    } {
        r.consume(3);
    }

    if {
        let buf = r.fill_buf()?;
        buf.len() >= 1 && buf[0] == b'#'
    } {
        r.consume(1);
        loop {
            let to_consume = {
                let buf = r.fill_buf()?;
                let mut i = 0;
                loop {
                    if i >= buf.len() || buf[i] == b'\n' {
                        break i;
                    }
                    i += 1;
                }
            };

            if to_consume == 0 {
                break;
            } else {
                r.consume(to_consume);
            }
        }
    }

    Ok(())
}

/// Reads a Lua script from a `R: Read` and wraps it in a BufReader
///
/// Also calls `skip_prefix` to skip any leading UTF-8 BOM or unix shebang.
pub fn buffered_read<R: Read>(r: R) -> Result<BufReader<R>, Error> {
    let mut r = BufReader::new(r);
    skip_prefix(&mut r)?;
    Ok(r)
}
