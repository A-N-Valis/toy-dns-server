pub struct Header {
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
}
