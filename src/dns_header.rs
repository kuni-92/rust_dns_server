#[derive(PartialEq, Debug)]
enum Qr {
    REQUEST,
    RESPONSE,
}

#[derive(PartialEq, Debug)]
enum Opcode {
    QUERY,
    IQUERY,
    STATUS,
    RESERVE,
}

#[derive(PartialEq, Debug)]
enum AA {
    HAVE,
    NONE,
}

#[derive(PartialEq, Debug)]
enum TC {
    FIT,
    TRUNCATED,
}

struct DnsHeader {
    field: [u16; 6],
}

impl DnsHeader {
    pub fn get_id(&self) -> u16 {
        return self.field[0];
    }

    pub fn get_qr(&self) -> Qr {
        match self.field[1] & 0b0000_0000_0000_0001 {
            0 => Qr::REQUEST,
            _ => Qr::RESPONSE,
        }
    }

    pub fn get_opcode(&self) -> Opcode {
        match (self.field[1] & 0b0000_0000_0000_1110) >> 1 {
            0 => Opcode::QUERY,
            1 => Opcode::IQUERY,
            2 => Opcode::STATUS,
            _ => Opcode::RESERVE,
        }
    }

    pub fn get_aa(&self) -> AA {
        match (self.field[1] & 0b0000_0000_0001_0000) >> 4 {
            0 => AA::NONE,
            1 => AA::HAVE,
            _ => unreachable!(),
        }
    }

    pub fn get_tc(&self) -> TC {
        match (self.field[1] & 0b0000_0000_0010_0000) >> 5 {
            0 => TC::FIT,
            1 => TC::TRUNCATED,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dns_header::{DnsHeader, Opcode, Qr, AA, TC};

    #[test]
    fn test_get_id() {
        let header_data: [u16; 6] = [1, 2, 3, 4, 5, 6];
        let id = DnsHeader { field: header_data }.get_id();
        dbg!(id);
        assert_eq!(1, id);
    }

    #[test]
    fn test_get_qr_response() {
        let header_data: [u16; 6] = [1, 0xFFFF, 3, 4, 5, 6];
        let qr = DnsHeader { field: header_data }.get_qr();
        dbg!(&qr);
        assert_eq!(Qr::RESPONSE, qr);
    }

    #[test]
    fn test_get_qr_request() {
        let header_data: [u16; 6] = [1, 0xFFFE, 3, 4, 5, 6];
        let qr = DnsHeader { field: header_data }.get_qr();
        dbg!(&qr);
        assert_eq!(Qr::REQUEST, qr);
    }

    #[test]
    fn test_get_opcode_query() {
        let header_data: [u16; 6] = [1, 0xFFF1, 3, 4, 5, 6];
        let opcode = DnsHeader { field: header_data }.get_opcode();
        dbg!(&opcode);
        assert_eq!(Opcode::QUERY, opcode);
    }

    #[test]
    fn test_get_opcode_iquery() {
        let header_data: [u16; 6] = [1, 0xFFF2, 3, 4, 5, 6];
        let opcode = DnsHeader { field: header_data }.get_opcode();
        dbg!(&opcode);
        assert_eq!(Opcode::IQUERY, opcode);
    }

    #[test]
    fn test_get_opcode_reserve_min() {
        let header_data: [u16; 6] = [1, 0xFFF3, 3, 4, 5, 6];
        let opcode = DnsHeader { field: header_data }.get_opcode();
        dbg!(&opcode);
        assert_eq!(Opcode::RESERVE, opcode);
    }

    #[test]
    fn test_get_opcode_reserve_max() {
        let header_data: [u16; 6] = [1, 0xFFFF, 3, 4, 5, 6];
        let opcode = DnsHeader { field: header_data }.get_opcode();
        dbg!(&opcode);
        assert_eq!(Opcode::RESERVE, opcode);
    }

    #[test]
    fn test_get_aa_have() {
        let header_data: [u16; 6] = [1, 0xFFFF, 3, 4, 5, 6];
        let aa = DnsHeader { field: header_data }.get_aa();
        dbg!(&aa);
        assert_eq!(AA::HAVE, aa);
    }

    #[test]
    fn test_get_aa_none() {
        let header_data: [u16; 6] = [1, 0xFFEF, 3, 4, 5, 6];
        let aa = DnsHeader { field: header_data }.get_aa();
        dbg!(&aa);
        assert_eq!(AA::NONE, aa);
    }

    #[test]
    fn test_get_tc_fit() {
        let header_data: [u16; 6] = [1, 0xFFDF, 3, 4, 5, 6];
        let tc = DnsHeader { field: header_data }.get_tc();
        dbg!(&tc);
        assert_eq!(TC::FIT, tc);
    }

    #[test]
    fn test_get_tc_truncate() {
        let header_data: [u16; 6] = [1, 0xFFFF, 3, 4, 5, 6];
        let tc = DnsHeader { field: header_data }.get_tc();
        dbg!(&tc);
        assert_eq!(TC::TRUNCATED, tc);
    }
}
