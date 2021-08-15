#[repr(C)]//让rust 按照C语言的内存布局排布
pub mod gdt_manager{
    let size:u16
    let base:u32
    pub  struct  gdt_seg_des{/*GDT描述符*/
        limit_low:u16,
        base_low:u16,
        base_med:u8,
        access:u8,
        flags_limit_high:u8,
        base_high:u8,
        seg_des:[u8;8]//逻辑上是8个连续的字节
    }
    let null_segment_descriptor=gdt_seg_des{};
    let unused_segment_descriptor=gdt_seg_des{};
    let code_segment_descriptor=gdt_seg_des{};
    let data_segment_descriptor=gdt_seg_des{};
    //TODO: 用rust语言解决它的地址偏移计算
    fn code_segment_selector()->u16{
        let mut address:u16;
        /*table表中它的偏移*/
        address=((u8*)&gdt_manager-(u8*)&code_segment_descriptor);
        return address;
    }
    fn data_segment_descriptor()->u16{
        let mut address:u16;
        address=((u8*)&gdt_manager-(u8*)&data_segment_descriptor);
        return address;
    }
    impl gdt_seg_des{
        /*base is 32bit*/
        fn get_base(&self)->&u32{//TODO:生命周期
                let mut base:u32=0;
                base |=self.seg_des[7];
                base <<=8;
                base |=self.seg_des[4];
                base <<=8;
                base |=self.seg_des[3];
                base <<8;
                base |=self.seg_des[2];
            return base;
        }
        /*limit is 20bit*/
        fn get_limit(&self)->&u32{
            let mut limit :u32=0;
            limit |= (self.seg_des[6] &0x0f)
            limit <<=4;
            limit | =self.seg_des[1];
            limit <<=8;
            limit|=self.seg_des[0];
            return limit;
        }
        /*flags is 4bit*/
        fn get_flags(&self)->&u8{
            let mut flags:u8=0;
            flags |=(self.seg_des[6]>>4);
            return flags;
        }
        /*access_type is 4bit*/
        fn get_access(&self)->&u8{
            let mut access_type:u8=0;
            access_type |=self.seg_des[5];
            return access_type;
        }

        /*初始化描述符*/
        fn init(&self){
            self.seg_des[0]=(u8)self.limit_low & 0x00ff;
            self.seg_des[1]=(u8)(self.limit_low>>16) & 0x00ff;
            self.seg_des[2]=(u8)self.base_low & 0x00ff;
            self.seg_des[3]=(self.base_low >>16) & 0x00ff;
            self.seg_des[4]=self.base_med;
            self.seg_des[5]=self.access;
            self.seg_des[6]=self.flags_limit_high;
            self.seg_des[7]=self.base_high;
        }
    }

}