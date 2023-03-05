#[cfg(test)]
mod tests {
    use arc::info;
    use arc::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn log() {
        debug!("debug");
        info!("info");
        warn!("warn");
        error!("error");
    }
}
