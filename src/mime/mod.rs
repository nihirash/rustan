use mime_guess;

pub fn filename_to_mime(filename: String) -> String {
    let mime = mime_guess::from_path(filename.as_str());

    mime.first_or_octet_stream().to_string()
}
