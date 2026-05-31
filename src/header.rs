pub struct Header {
    pub id: u16,
    pub qr: u8,
    pub opcode: u8,
    pub aa: u8,
    pub tc: u8,
    pub rd: u8,
    pub ra: u8,
    pub z: u8,
    pub rcode: u8,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl Header {
    pub fn new(
        id: u16,
        qr: u8,
        opcode: u8,
        aa: u8,
        tc: u8,
        rd: u8,
        ra: u8,
        z: u8,
        rcode: u8,
        qdcount: u16,
        ancount: u16,
        nscount: u16,
        arcount: u16,
    ) -> Self {
        Self {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }

    pub fn to_bytes(&self) -> [u8; 12] {
        let mut result = [0u8; 12];

        let id_bytes = self.id.to_be_bytes();
        result[0] = id_bytes[0];
        result[1] = id_bytes[1];

        let mut flags = [0u8; 2];
        flags[0] = self.qr << 7 | self.opcode << 3 | self.aa << 2 | self.tc << 1 | self.rd;
        flags[1] = self.ra << 7 | self.z << 4 | self.rcode;

        result[2] = flags[0];
        result[3] = flags[1];

        let qdcount_bytes = self.qdcount.to_be_bytes();
        result[4] = qdcount_bytes[0];
        result[5] = qdcount_bytes[1];

        let ancount_bytes = self.ancount.to_be_bytes();
        result[6] = ancount_bytes[0];
        result[7] = ancount_bytes[1];

        let nscount_bytes = self.nscount.to_be_bytes();
        result[8] = nscount_bytes[0];
        result[9] = nscount_bytes[1];

        let arcount_bytes = self.arcount.to_be_bytes();
        result[10] = arcount_bytes[0];
        result[11] = arcount_bytes[1];

        result
    }

    pub fn from_bytes(header_bytes: [u8; 12]) -> Self {
        let id = (header_bytes[0] as u16) << 8 | header_bytes[1] as u16;

        let flag1 = header_bytes[2];
        let qr = (flag1 >> 7) & 0b1;
        let opcode = (flag1 >> 3) & 0b1111;
        let aa = (flag1 >> 2) & 0b1;
        let tc = (flag1 >> 1) & 0b1;
        let rd = flag1 & 0b1;

        let flag2 = header_bytes[3];
        let ra = (flag2 >> 7) & 0b1;
        let z = (flag2 >> 4) & 0b111;
        let rcode = flag2 & 0b111;

        let qdcount = (header_bytes[4] as u16) << 8 | header_bytes[5] as u16;
        let ancount = (header_bytes[6] as u16) << 8 | header_bytes[7] as u16;
        let nscount = (header_bytes[8] as u16) << 8 | header_bytes[9] as u16;
        let arcount = (header_bytes[10] as u16) << 8 | header_bytes[11] as u16;

        Self {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }
}
