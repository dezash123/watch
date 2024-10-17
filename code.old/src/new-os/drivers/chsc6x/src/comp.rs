// FUNC In your systerm init process,Must call this interface function to detec if the TP IC is Chipsemi corp'. 
// PARM pfw_infos: to get top 5 fw info in struct ts_fw_infos.
// PARM update_ret_flag: point value=1 update succeed; point value=0 update failed, If opend CHSC6X_AUTO_UPGRADE macro.
// RETURN 1:is chsc chip, 0:is not chsc chip

// extern int chsc6x_tp_dect(struct ts_fw_infos *pfw_infos, unsigned char *update_ret_flag);


// FUNC You can call this interfacce function to realize upgrade TP Firmware by OTA. 
// PARM pfw_infos: to get top 6 fw infos in struct ts_fw_infos, after ota upgrade.
// PARM p_fw_upd: array address of the upgrade firmware array 
// PARM fw_len: total size of the upgrade firmware array 
// RETURN NULL
//
// extern void chsc6x_ota_upgrade_tp_fw(struct ts_fw_infos *pfw_infos, unsigned char* p_fw_upd, unsigned int fw_len);


// FUNC: get fw info in struct ts_fw_infos you can call this func anytime.
// PARM pfw_infos: can get all fw infos in struct ts_fw_infos, after call this interface.
// RETURN NULL
// 
// extern void chsc6x_get_chip_info(struct ts_fw_infos *infos);

#define TXRX_ADDR       (0x9000)
#define CMD_ADDR        (0x9f00)
#define RSP_ADDR        (0x9f40)
//#define MAX_CHIP_ID     (10)
//unsigned char chsc6x_chip_name[MAX_CHIP_ID][20];
static unsigned int g_chsc6x_cfg_ver = 0; 
static unsigned int g_chsc6x_boot_ver = 0;

static unsigned int g_mccode;    /* 1:3536 */
static unsigned int g_upgrade_flag = 0; /* 0:driver init upgrade, 1:OTA upgrade */
static unsigned char g_i2c_addr;
struct ts_fw_infos *g_pfw_infos;

struct UpdateHeader {
    sig: u32,
    resv: u32,
    n_cfg: u32,
    n_match: u32,
    len_cfg: u32,
    len_boot: u32,
}

struct ctp_tst_wr_t {
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
} ;

struct ctp_tst_rd_t {
    /* offset 0; */
    id: u8,    /* cmd_id; */
    cc: u8,    /* complete code */
    d0: u16,    /* data 0 */
    sn: u16,    /* session number */
    chk: u16,    /* 16 bit checksum */
} ;

#define EPERM           (1)
#define DIRECTLY_MODE   (0x0)
#define DEDICATE_MODE   (0x1)
#define LEN_CMD_CHK_TX  (10)
#define LEN_CMD_PKG_TX  (16)
#define LEN_RSP_CHK_RX  (8)
#define MAX_BULK_SIZE   (1024)

// to direct memory access mode
static cmd_2dma_42bd: [u8; 6] = [ 0x28, 0x35, 0xc1, 0x00, 0x35, 0xae ];


/* RETURN:0->pass else->fail */
static int chsc6x_read_bytes_u16addr(unsigned char id, unsigned short adr, unsigned char *rxbuf, unsigned short lenth)
{
    int ret = 0;
    int retry;
    unsigned short ofs_adr;
    int len = lenth;
    int rd_len = 0;
	int offset = 0;

	while (len > 0) {
        ofs_adr = adr + offset;
		if (len > MAX_IIC_RD_LEN) {
			rd_len = MAX_IIC_RD_LEN;
			len -= MAX_IIC_RD_LEN;
		} else {
			rd_len = len;
			len = 0;
		}

		retry = 0;
		while (chsc6x_read_bytes_u16addr_sub(id, ofs_adr, &rxbuf[offset], rd_len) != 0) {
			if (retry++ == 3) {
				ret = -1;
				break;
			}
		}
		offset += MAX_IIC_RD_LEN;
		if (ret < 0) {
			break;
		}
	}

	return ret;
}

