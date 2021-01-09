use pkcs11_uri::Pkcs11Uri;

fn main() {
    // let level = log::LevelFilter::Debug;
    let level = log::LevelFilter::Info;
    let _ = simplelog::SimpleLogger::init(level, simplelog::Config::default());
    if let Err(err) = try_main() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
fn try_main() -> anyhow::Result<()> {
    let uri_str = r"pkcs11:
        type=private;
        slot-id=327;
        serial=d2dcb3ad5e30674d;
        token=lpc55-2ac0c213b4903b76;
        slot-manufacturer=SoftHSM%20project;
        object=lpc55-2ac0c213b4903b76%20@%202021-01-08T20:41:24
            ?pin-value=1234
            &module-path=/usr/lib/libsofthsm2.so";
    let uri = Pkcs11Uri::try_parse(uri_str)?;
    let (context, session, object) = uri.identify_object().unwrap();

    //  CKM_SHA256_RSA_PKCS
    let mechanism = pkcs11::types::CK_MECHANISM {
        // mechanism: pkcs11::types::CKM_SHA256_RSA_PKCS,
        mechanism: pkcs11::types::CKM_RSA_PKCS,
        pParameter: std::ptr::null_mut(),
        ulParameterLen: 0,
    };

    // now do a signature, assuming this is an RSA key
    context.sign_init(session, &mechanism, object).unwrap();
    let data = String::from("PKCS #11 is pretty horrible").into_bytes();
    let signature = context.sign(session, &data).unwrap();

    println!("signature: {:x?}", signature.as_slice());
    Ok(())
}
