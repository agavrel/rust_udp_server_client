#[cfg(test)]
mod tests {
    #[test]
    fn file_extension_magic() {
        const WILD: u8 = 0xFC;

        let mut filename = "test.mp3";
        let mut result = crate::is_file_extension_matching_magic(filename, vec![0xAA, 0xFB]);
        assert_eq!(result, false);
        result = crate::is_file_extension_matching_magic(filename, vec![0xFF, 0xFB]);
        assert_eq!(result, true);
        filename = "test.webp";
        result = crate::is_file_extension_matching_magic(filename, vec![0x52, 0x49, 0x46, 0x46, WILD, WILD, WILD, WILD, 0x57, 0x45, 0x42, 0x50]);
        assert_eq!(result, true);
        result = crate::is_file_extension_matching_magic(filename, vec![0x52, 0x49, 0x46, 0x46, 0x42, WILD, WILD, WILD, 0x57, 0x45, 0x42, 0x50]);
        assert_eq!(result, true);
        result = crate::is_file_extension_matching_magic(filename, vec![0x25, 0x49, 0x46, 0x46, 0x42, WILD, WILD, WILD, 0x57, 0x45, 0x42, 0x50]);
        assert_eq!(result, false);
    }
}
