const CHSC6X_I2C_ID: u8 = 0x5c; //8bit 

const CHSC6X_MAX_POINTS_NUM: u8 = 1;
const CHSC6X_RES_MAX_X: u16 = 370;
const CHSC6X_RES_MAX_Y: u16 = 370;

/*MACRO SWITCH for driver update TP FW */
const CHSC6X_AUTO_UPGRADE: bool = false;

/*MACRO SWITCH for multi TP_VENDOR Compatible update TP FW */
const CHSC6X_MUL_VENDOR_UPGRADE: bool = false;

const MAX_IIC_WR_LEN: u8 = 8;
const MAX_IIC_RD_LEN: u8 = 16;

pub struct Chsc6x<B: I> {
    backend: B,
}


/* return: =read lenth succeed; <0 failed 
   read reg addr not need 
   just used for reading xy cord info*/
fn chsc6x_i2c_read(char id: u8, p_data: [u8], length: usize) -> i32 {
    return i2cRead(id, length, p_data);
}

/* RETURN:0->pass else->fail */
fn chsc6x_read_bytes_u16addr_sub(unsigned char id, unsigned short adr, unsigned char *rxbuf, unsigned short lenth)
{
    int ret = i2cReadPacket(id,adr,lenth,rxbuf);
    if(ret == lenth) {
        return 0;
    }else{
        return -1;
    }
}

/* RETURN:0->pass else->fail */
int chsc6x_write_bytes_u16addr_sub(unsigned char id, unsigned short adr, unsigned char *txbuf, unsigned short lenth)
{
    int ret = i2cSendPacket(id,adr,lenth,txbuf);
    if(ret == lenth) {
        return 0;
    }else{
        return -1;
    }
}

void chsc6x_msleep(int ms)
{
    tl_delay(32*ms);
}

fn reset() {
    pd7_out0();
    tl_delay(950);//30ms
    pd7_out1();
    tl_delay(950);//30ms
}

void chsc6x_tp_reset_active(void)
{
    pd7_out0();
    tl_delay(950);//30ms
    pd7_out1();
}

