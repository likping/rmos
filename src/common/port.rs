
pub mod Port{
    pub struct Bit8{
       pub port_number:i8,
    }
     struct Bit16{
        port_number:i8,
    }
     struct Bit32{
        port_number:i8,
    }
     struct Bit64{
        port_number:i8,
    }
    pub trait InputOutput{
        fn write(&self,data_:u8);
        fn read(&self)->u8;
    }
    impl InputOutput for Bit8{
        fn write(&self,data_:u8){
            use crate::printf;
            printf(b"write");
        }
        fn read(&self)->u8{
            use crate::printf;
            printf(b"read");
            return b'a';
        }
    }
    
}


