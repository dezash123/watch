pub struct TouchEvent {
    x: u16, /*x coordinate */
    y: u16, /*y coordinate */
    flag: TouchFlag, /* touch event flag: 0 -- down; 1-- up; 2 -- contact */
    id: bool,   /*touch ID */
}

pub enum Error {
    
}

pub enum I2cMode {
    DMA,
}

#[repr(u8)]
pub enum TouchFlag {
    Down,
    Up,
    Contact,
}

pub struct FirmwareInfo {
    cfg_version: u16,  //customer read 
    boot_version: u16, //customer read 
    vendor_id: u16,    //customer read 
    project_id: u16,   //customer read 
    chip_id: u16,      //customer read 
    chip_type: u16,    //customer read 
    rpt_lcd_x: u16,    //customer read must after chsc6x_get_chip_info
    rpt_lcd_y: u16,    //customer read must after chsc6x_get_chip_info
    max_pt_num: u16,   //customer read must after chsc6x_get_chip_info
}


struct UpdateHeader {
    sig: u32,
    resv: u32,
    n_cfg: u32,
    n_match: u32,
    len_cfg: u32,
    len_boot: u32,
}

struct TestWrite {
    /* offset 0; */
    id: u8,    /* cmd_id; */
    idv: u8,    /* inverse of cmd_id */
    d0: u16,    /* data 0 */
    d1: u16,    /* data 1 */
    d2: u16,    /* data 2 */
    /* offset 8; */
    resv: u8,    /* offset 8 */
    tag: u8,    /* offset 9 */
    chk: u16,    /* 16 bit checksum */
    s2_pad0: u16,    /*  */
    s2_pad1: u16,    /*  */
}

struct TestRead {
    /* offset 0; */
    id: u8,    /* cmd_id; */
    cc: u8,    /* complete code */
    d0: u16,    /* data 0 */
    sn: u16,    /* session number */
    chk: u16,    /* 16 bit checksum */
}


use crate::consts::*;



/*MACRO SWITCH for driver update TP FW */
const CHSC6X_AUTO_UPGRADE: bool = false;

/*MACRO SWITCH for multi TP_VENDOR Compatible update TP FW */
const CHSC6X_MUL_VENDOR_UPGRADE: bool = false;

const MAX_IIC_WR_LEN: u8 = 8;
const MAX_IIC_RD_LEN: u8 = 16;


pub trait Chsc6xBackend {
    type AddressType;
    fn write<const N: usize>(&mut self, data: [u8; N]);
    fn write_address<const N: usize>(&mut self, addr: Self::AddressType, data: [u8; N]);
    fn read_address<const N: usize>(&mut self, addr: Self::AddressType) -> [u8; N];
    fn read_raw<const N: usize>(&mut self) -> [u8; N];
    fn set_reset_pin(&mut self, state: bool);
    fn delay_ms(&mut self, duration: u32);
}

pub struct Chsc6x<B: Chsc6xBackend> {
    backend: B,
}


impl Chsc6x {
    // touch info structure
    // Byte 0:
    // +---+---+---+---+---+
    // | 7 | 6 |5:4| 3 |2:1|
    // +---+---+---+---+---+
    // |YHb|XHb|FLG|ID |PTN|
    // +---+---+---+---+---+
    // Byte 1: XLB
    // Byte 2: YLB
    fn read_touch_info(&mut self) -> TouchEvent {
	let bytes = self.backend.read_raw::<3>(); 
        // let point_num: u8 = bytes[0] & 0x03;
	TouchEvent {
	    x: (((bytes[0] & 0x40) as u16) << 2) | (bytes[1] as u16),
            y: (((bytes[0] & 0x80) as u16) << 2) | (bytes[2] as u16),
            flag: (bytes[0] >> 4) & 0b11 as TouchFlag,
            id: (bytes[0] >> 2) & 0b1 as bool,
	}
    }

    fn reset(&mut self) {
	self.backend.set_reset_pin(false);
	self.backend.delay_ms(30);
	self.backend.set_reset_pin(false);
	self.backend.delay_ms(30);
    }

    fn reset_active(&mut self) {
	self.backend.set_reset_pin(false);
	self.backend.delay_ms(30);
	self.backend.set_reset_pin(false);
    }

    fn resume(&mut self) {
	self.reset();
    }

    fn suspend(&mut self) {
        self.reset();
	self.backend.write(SUSPEND.to_be_bytes());
    }
    
    fn db_check(&mut self)
    {
        self.reset();
	self.backend.write(DB_CHECK.to_be_bytes());
    }
    
    fn palm_check(&mut self) {
        self.reset();
	self.backend.write(PALM_CHECK.to_be_bytes());
    }

    fn init(&mut self) {
        for _ in 0..3 {
	    self.detect_ic();
            break;
        }
    }

    fn get_i2c_mode(&mut self) -> Result<I2cMode, Error> {

    }

    fn set_dma_mode(&mut self) -> Result<(), Error> {
        if self.get_i2c_mode()? == DMA {
            Ok(())
        }
        
        for _ in 0..5 {
            self.backend.delay_ms(20);
            self.backend.write()
            self.backend.delay_ms(30);
            
        }
        

        if (ret < 0) {
            return ret;
        }
        if (ret == DIRECTLY_MODE) {
            return 0;
        }

    
        while (retry++ < 5) {
            chsc6x_msleep(20);
            chsc6x_write_bytes_u16addr(g_i2c_addr, 0x42bd, cmd_2dma_42bd, 6);
            chsc6x_msleep(30);
            mod = chsc6x_get_i2cmode();
            if (mod == DIRECTLY_MODE) {
                break;
            }
        }
    
        if (mod == DIRECTLY_MODE) {
            return 0;
        } else {
            return -EPERM;
        }
    }

    fn set_mccode() -> u8 {

    }

    // FUNC In your systerm init process,Must call this interface function to detec if the TP IC is Chipsemi corp'. 
    // PARM pfw_infos: to get top 5 fw info in struct ts_fw_infos.
    // PARM update_ret_flag: point value=1 update succeed; point value=0 update failed, If opend CHSC6X_AUTO_UPGRADE macro.
    // RETURN 1:is chsc chip, 0:is not chsc chip
    //
    fn detect_ic() {
        let mut dwr: u8 = 0x05;
        for _ in 0..5 {
            if(set_dd_mode()) {
                self.reset_active();
                self.bulk_down_check(&dwr, 0x0602, 1);
                dwr = 0x00;
                self.bulk_down_check(&dwr, 0x0643, 1);
            } else {
                break;
            }
        }
        let mccode = self.get_mccode();
    }
    int chsc6x_tp_dect(struct ts_fw_infos *pfw_infos, unsigned char *update_ret_flag)
    {
        int ret = -1;
        int try_cnt;
        unsigned char dwr=0x05;
        unsigned short buf_tmpcfg[102];    
        
        g_mccode = 0xff;        /* default */
        g_i2c_addr = CHSC6X_I2C_ID;
        g_pfw_infos = pfw_infos;
    
        for(try_cnt=0; try_cnt<5; try_cnt++) {
            if (chsc6x_set_dd_mode()) {
                chsc6x_tp_reset_active();
                if (chsc6x_bulk_down_check(&dwr, 0x0602, 1) == 0) {    
                    dwr = 0x00;
                    chsc6x_bulk_down_check(&dwr, 0x0643, 1);    
                } else {
                    chsc6x_info("chsc6x: write 0x0602 failed \r\n");
                }
            } else {
                break;
            }
        }
        if(try_cnt >= 5) {
            chsc6x_info("chsc6x: chsc6x_set_dd_mode failed \r\n");
        }
        //chsc6x_info("chsc6x: chsc_boot size=%d \r\n", sizeof(chsc_boot));
    
        chsc6x_tp_mccode();    /* MUST: call this function there!!! */
        chsc6x_info("chsc6x: g_mccode is 0x%x \r\n",g_mccode);    
    
        if(g_mccode == 0xff) {
            chsc6x_err("chsc6x: get mccode fail! \r\n");
            return 0;
        }
    
        /*try to get running time tp-cfg. if fail : wrong boot? wrong rom-cfg?*/
        if (chsc6x_get_running_cfg(buf_tmpcfg, 0x9e00) == 0) {
            chsc6x_info("chsc6x: get_running_cfg pass !\r\n");
            g_chsc6x_cfg_ver = (unsigned int)buf_tmpcfg[1];
            g_chsc6x_cfg_ver = (g_chsc6x_cfg_ver<<16) + (unsigned int)buf_tmpcfg[0];
    
            g_pfw_infos->chsc6x_cfg_version = g_chsc6x_cfg_ver>>26;
            //g_pfw_infos->chsc6x_vendor_id = (g_chsc6x_cfg_ver>>9)&0x7F;
            //g_pfw_infos->chsc6x_project_id = g_chsc6x_cfg_ver&0x01FF;
            g_pfw_infos->chsc6x_vendor_id = ((g_chsc6x_cfg_ver>>9)&0x7F | ((g_chsc6x_cfg_ver>>22&0x03)<<7));
            g_pfw_infos->chsc6x_project_id = ((g_chsc6x_cfg_ver&0x01FF) | ((g_chsc6x_cfg_ver>>20&0x03)<<9));
            g_pfw_infos->chsc6x_rpt_lcd_x = buf_tmpcfg[38];
            g_pfw_infos->chsc6x_rpt_lcd_y = buf_tmpcfg[39];
            g_pfw_infos->chsc6x_chip_id = buf_tmpcfg[53]&0xff;
            g_pfw_infos->chsc6x_chip_type = (buf_tmpcfg[53]>>8)&0xf;
            chsc6x_info("chsc6x: vid=%d,pid=%d,boot_ver=0x%x,cfg_ver=%d,chip_id=0x%x\r\n", \
                g_pfw_infos->chsc6x_vendor_id,g_pfw_infos->chsc6x_project_id,g_pfw_infos->chsc6x_boot_version, \
                g_pfw_infos->chsc6x_cfg_version,g_pfw_infos->chsc6x_chip_id \
            );
        } else {
            if(0 == buf_tmpcfg[2] && 0 == buf_tmpcfg[3]) {
                g_chsc6x_boot_ver = 0;
             }
             chsc6x_find_ver();
        }
    
        if (0 == g_chsc6x_cfg_ver) {
            chsc6x_err("chsc6x: get tp-info fail! \r\n");
            return 0;
        }
      #if CHSC6X_AUTO_UPGRADE
        g_upgrade_flag = 0;
        ret = chsc6x_do_update_ifneed(0, 0);
        if(0 != ret) {
            *update_ret_flag = 0;
            chsc6x_err("chsc6x: do fw update failed !\r\n");
        } else {
            *update_ret_flag = 1;
            chsc6x_info("chsc6x: vid=%d,pid=%d,boot_ver=0x%x,cfg_ver=%d,chip_id=0x%x\r\n", \
                g_pfw_infos->chsc6x_vendor_id,g_pfw_infos->chsc6x_project_id, \
                g_pfw_infos->chsc6x_boot_version,g_pfw_infos->chsc6x_cfg_version,g_pfw_infos->chsc6x_chip_id \
            );
        }
      #endif  
        
    exit:
        chsc6x_tp_reset();
    
        return 1;
    }

    static int chsc6x_set_dd_mode(void)
    {
        int mod = -1;
        int retry = 0;
        int ret = 0;
        ret = chsc6x_get_i2cmode();
        if (ret < 0) {
            return ret;
        }
        if (ret == DIRECTLY_MODE) {
            return 0;
        }
    
        while (retry++ < 5) {
            chsc6x_msleep(20);
            chsc6x_write_bytes_u16addr(g_i2c_addr, 0x42bd, cmd_2dma_42bd, 6);
            chsc6x_msleep(30);
            mod = chsc6x_get_i2cmode();
            if (mod == DIRECTLY_MODE) {
                break;
            }
        }
    
        if (mod == DIRECTLY_MODE) {
            return 0;
        } else {
            return -EPERM;
        }
    }
}
    
