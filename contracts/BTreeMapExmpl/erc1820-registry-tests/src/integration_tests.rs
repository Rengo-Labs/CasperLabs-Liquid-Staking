#[cfg(test)]
mod test_erc1820;

#[cfg(test)]
mod tests {
    use casper_types::{Key};
    use casper_types::bytesrepr::{Bytes, ToBytes};

    extern crate base64;
    use crate::test_erc1820::{Sender, TestERC1820};

    pub const HASH_ERC1820_SENDER: &str = "ERC777TokensSender";
    pub const HASH_ERC1820_RECIPIENT: &str = "ERC777TokensRecipient";

    #[test]
    fn should_install_and_set_i_impl() {
        let mut fixture = TestERC1820::install_contract();
        let ali = fixture.ali;
        let implementer = fixture.bob;
        let tag_sender = HASH_ERC1820_SENDER.to_string();

        fixture.set_interface_implementer(
            Key::from(ali),
            tag_sender.clone(),
            Key::from(implementer),
            Sender(ali)
        );

        //TODO Get BTreeMap
        let implementer_map = fixture.get_interface_implementer(Key::from(ali)).unwrap();

        let value = implementer_map.get_key_value(tag_sender.as_str()).unwrap();
        println!("{}", value.0);

        assert_eq!(
            Key::from(implementer),
            *value.1
        );
    }

    #[test]
    fn should_install_and_set_manager() {
        let mut fixture = TestERC1820::install_contract();
        let ali = fixture.ali;

        fixture.set_manager(
            Key::from(ali),
            Key::from(ali),
            Sender(ali)
        );

        let manager = fixture.get_manager(Key::from(ali));

    }


}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}