#[doc = "Register `MISC` reader"]
pub type R = crate::R<MISC_SPEC>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MISC_SPEC>;
#[doc = "Field `CS_DIS(0-5)` reader - Set this bit to raise high SPI_CS%s pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS%s is in low level when SPI1 transfer starts"]
pub type CS_DIS_R = crate::BitReader;
#[doc = "Field `CS_DIS(0-5)` writer - Set this bit to raise high SPI_CS%s pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS%s is in low level when SPI1 transfer starts"]
pub type CS_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_DIS` reader - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub type CK_DIS_R = crate::BitReader;
#[doc = "Field `CK_DIS` writer - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub type CK_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_CS_POL` reader - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ SPI_MASTER_CS_POL. Can be configured in CONF state."]
pub type MASTER_CS_POL_R = crate::FieldReader;
#[doc = "Field `MASTER_CS_POL` writer - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ SPI_MASTER_CS_POL. Can be configured in CONF state."]
pub type MASTER_CS_POL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLK_DATA_DTR_EN` reader - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
pub type CLK_DATA_DTR_EN_R = crate::BitReader;
#[doc = "Field `CLK_DATA_DTR_EN` writer - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
pub type CLK_DATA_DTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_DTR_EN` reader - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
pub type DATA_DTR_EN_R = crate::BitReader;
#[doc = "Field `DATA_DTR_EN` writer - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
pub type DATA_DTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_DTR_EN` reader - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
pub type ADDR_DTR_EN_R = crate::BitReader;
#[doc = "Field `ADDR_DTR_EN` writer - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
pub type ADDR_DTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_DTR_EN` reader - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
pub type CMD_DTR_EN_R = crate::BitReader;
#[doc = "Field `CMD_DTR_EN` writer - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
pub type CMD_DTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD_DATA_SET` reader - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_DOUT or SPI_DIN state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub type CD_DATA_SET_R = crate::BitReader;
#[doc = "Field `CD_DATA_SET` writer - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_DOUT or SPI_DIN state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub type CD_DATA_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD_DUMMY_SET` reader - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_DUMMY state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub type CD_DUMMY_SET_R = crate::BitReader;
#[doc = "Field `CD_DUMMY_SET` writer - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_DUMMY state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub type CD_DUMMY_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD_ADDR_SET` reader - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_SEND_ADDR state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub type CD_ADDR_SET_R = crate::BitReader;
#[doc = "Field `CD_ADDR_SET` writer - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_SEND_ADDR state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub type CD_ADDR_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_CS_POL` reader - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub type SLAVE_CS_POL_R = crate::BitReader;
#[doc = "Field `SLAVE_CS_POL` writer - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub type SLAVE_CS_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQS_IDLE_EDGE` reader - The default value of spi_dqs. Can be configured in CONF state."]
pub type DQS_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `DQS_IDLE_EDGE` writer - The default value of spi_dqs. Can be configured in CONF state."]
pub type DQS_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD_CMD_SET` reader - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_SEND_CMD state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub type CD_CMD_SET_R = crate::BitReader;
#[doc = "Field `CD_CMD_SET` writer - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_SEND_CMD state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
pub type CD_CMD_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD_IDLE_EDGE` reader - The default value of spi_cd. Can be configured in CONF state."]
pub type CD_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CD_IDLE_EDGE` writer - The default value of spi_cd. Can be configured in CONF state."]
pub type CD_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub type CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub type CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUAD_DIN_PIN_SWAP` reader - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
pub type QUAD_DIN_PIN_SWAP_R = crate::BitReader;
#[doc = "Field `QUAD_DIN_PIN_SWAP` writer - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
pub type QUAD_DIN_PIN_SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Set this bit to raise high SPI_CS(0-5) pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS(0-5) is in low level when SPI1 transfer starts"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CS0_DIS` field.</div>"]
    #[inline(always)]
    pub fn cs_dis(&self, n: u8) -> CS_DIS_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        CS_DIS_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Set this bit to raise high SPI_CS(0-5) pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS(0-5) is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs_dis_iter(&self) -> impl Iterator<Item = CS_DIS_R> + '_ {
        (0..6).map(move |n| CS_DIS_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Set this bit to raise high SPI_CS0 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS0 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to raise high SPI_CS1 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS1 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to raise high SPI_CS2 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS2 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs2_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to raise high SPI_CS3 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS3 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs3_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to raise high SPI_CS4 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS4 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs4_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to raise high SPI_CS5 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS5 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    pub fn cs5_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_dis(&self) -> CK_DIS_R {
        CK_DIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ SPI_MASTER_CS_POL. Can be configured in CONF state."]
    #[inline(always)]
    pub fn master_cs_pol(&self) -> MASTER_CS_POL_R {
        MASTER_CS_POL_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
    #[inline(always)]
    pub fn clk_data_dtr_en(&self) -> CLK_DATA_DTR_EN_R {
        CLK_DATA_DTR_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn data_dtr_en(&self) -> DATA_DTR_EN_R {
        DATA_DTR_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn addr_dtr_en(&self) -> ADDR_DTR_EN_R {
        ADDR_DTR_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cmd_dtr_en(&self) -> CMD_DTR_EN_R {
        CMD_DTR_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_DOUT or SPI_DIN state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_data_set(&self) -> CD_DATA_SET_R {
        CD_DATA_SET_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_DUMMY state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_dummy_set(&self) -> CD_DUMMY_SET_R {
        CD_DUMMY_SET_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_SEND_ADDR state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_addr_set(&self) -> CD_ADDR_SET_R {
        CD_ADDR_SET_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    pub fn slave_cs_pol(&self) -> SLAVE_CS_POL_R {
        SLAVE_CS_POL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The default value of spi_dqs. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dqs_idle_edge(&self) -> DQS_IDLE_EDGE_R {
        DQS_IDLE_EDGE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_SEND_CMD state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_cmd_set(&self) -> CD_CMD_SET_R {
        CD_CMD_SET_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The default value of spi_cd. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cd_idle_edge(&self) -> CD_IDLE_EDGE_R {
        CD_IDLE_EDGE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn quad_din_pin_swap(&self) -> QUAD_DIN_PIN_SWAP_R {
        QUAD_DIN_PIN_SWAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("ck_dis", &self.ck_dis())
            .field("master_cs_pol", &self.master_cs_pol())
            .field("clk_data_dtr_en", &self.clk_data_dtr_en())
            .field("data_dtr_en", &self.data_dtr_en())
            .field("addr_dtr_en", &self.addr_dtr_en())
            .field("cmd_dtr_en", &self.cmd_dtr_en())
            .field("cd_data_set", &self.cd_data_set())
            .field("cd_dummy_set", &self.cd_dummy_set())
            .field("cd_addr_set", &self.cd_addr_set())
            .field("slave_cs_pol", &self.slave_cs_pol())
            .field("dqs_idle_edge", &self.dqs_idle_edge())
            .field("cd_cmd_set", &self.cd_cmd_set())
            .field("cd_idle_edge", &self.cd_idle_edge())
            .field("ck_idle_edge", &self.ck_idle_edge())
            .field("cs_keep_active", &self.cs_keep_active())
            .field("quad_din_pin_swap", &self.quad_din_pin_swap())
            .field("cs0_dis", &self.cs0_dis())
            .field("cs1_dis", &self.cs1_dis())
            .field("cs2_dis", &self.cs2_dis())
            .field("cs3_dis", &self.cs3_dis())
            .field("cs4_dis", &self.cs4_dis())
            .field("cs5_dis", &self.cs5_dis())
            .finish()
    }
}
impl W {
    #[doc = "Set this bit to raise high SPI_CS(0-5) pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS(0-5) is in low level when SPI1 transfer starts"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CS0_DIS` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn cs_dis(&mut self, n: u8) -> CS_DIS_W<MISC_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        CS_DIS_W::new(self, n)
    }
    #[doc = "Bit 0 - Set this bit to raise high SPI_CS0 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS0 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    #[must_use]
    pub fn cs0_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to raise high SPI_CS1 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS1 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    #[must_use]
    pub fn cs1_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to raise high SPI_CS2 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS2 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    #[must_use]
    pub fn cs2_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to raise high SPI_CS3 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS3 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    #[must_use]
    pub fn cs3_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to raise high SPI_CS4 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS4 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    #[must_use]
    pub fn cs4_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to raise high SPI_CS5 pin, which means that the SPI device(Ext_RAM(0)/flash(1)) connected to SPI_CS5 is in low level when SPI1 transfer starts"]
    #[inline(always)]
    #[must_use]
    pub fn cs5_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn ck_dis(&mut self) -> CK_DIS_W<MISC_SPEC> {
        CK_DIS_W::new(self, 6)
    }
    #[doc = "Bits 7:12 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ SPI_MASTER_CS_POL. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn master_cs_pol(&mut self) -> MASTER_CS_POL_W<MISC_SPEC> {
        MASTER_CS_POL_W::new(self, 7)
    }
    #[doc = "Bit 16 - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
    #[inline(always)]
    #[must_use]
    pub fn clk_data_dtr_en(&mut self) -> CLK_DATA_DTR_EN_W<MISC_SPEC> {
        CLK_DATA_DTR_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn data_dtr_en(&mut self) -> DATA_DTR_EN_W<MISC_SPEC> {
        DATA_DTR_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn addr_dtr_en(&mut self) -> ADDR_DTR_EN_W<MISC_SPEC> {
        ADDR_DTR_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_dtr_en(&mut self) -> CMD_DTR_EN_W<MISC_SPEC> {
        CMD_DTR_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_DOUT or SPI_DIN state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cd_data_set(&mut self) -> CD_DATA_SET_W<MISC_SPEC> {
        CD_DATA_SET_W::new(self, 20)
    }
    #[doc = "Bit 21 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_DUMMY state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cd_dummy_set(&mut self) -> CD_DUMMY_SET_W<MISC_SPEC> {
        CD_DUMMY_SET_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_SEND_ADDR state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cd_addr_set(&mut self) -> CD_ADDR_SET_W<MISC_SPEC> {
        CD_ADDR_SET_W::new(self, 22)
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn slave_cs_pol(&mut self) -> SLAVE_CS_POL_W<MISC_SPEC> {
        SLAVE_CS_POL_W::new(self, 23)
    }
    #[doc = "Bit 24 - The default value of spi_dqs. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn dqs_idle_edge(&mut self) -> DQS_IDLE_EDGE_W<MISC_SPEC> {
        DQS_IDLE_EDGE_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: spi_cd = !SPI_CD_IDLE_EDGE when SPI_ST\\[3:0\\] is in SPI_SEND_CMD state. 0: spi_cd = SPI_CD_IDLE_EDGE. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cd_cmd_set(&mut self) -> CD_CMD_SET_W<MISC_SPEC> {
        CD_CMD_SET_W::new(self, 25)
    }
    #[doc = "Bit 26 - The default value of spi_cd. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cd_idle_edge(&mut self) -> CD_IDLE_EDGE_W<MISC_SPEC> {
        CD_IDLE_EDGE_W::new(self, 26)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<MISC_SPEC> {
        CK_IDLE_EDGE_W::new(self, 29)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<MISC_SPEC> {
        CS_KEEP_ACTIVE_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: spi quad input swap enable 0: spi quad input swap disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn quad_din_pin_swap(&mut self) -> QUAD_DIN_PIN_SWAP_W<MISC_SPEC> {
        QUAD_DIN_PIN_SWAP_W::new(self, 31)
    }
}
#[doc = "SPI misc register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC to value 0x3e"]
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: u32 = 0x3e;
}
