#include "chsc6x_platform.h"


/* return: =read lenth succeed; <0 failed 
   read reg addr not need 
   just used for reading xy cord info*/
int chsc6x_i2c_read(unsigned char id, unsigned char *p_data, unsigned short lenth)
{    
    return i2cRead(id, lenth, p_data);
}

/* RETURN:0->pass else->fail */
int chsc6x_read_bytes_u16addr_sub(unsigned char id, unsigned short adr, unsigned char *rxbuf, unsigned short lenth)
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

void chsc6x_tp_reset(void)
{
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

