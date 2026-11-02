#[doc = "Register `SMEM_DDR` reader"]
pub type R = crate::R<SMEM_DDR_SPEC>;
#[doc = "Register `SMEM_DDR` writer"]
pub type W = crate::W<SMEM_DDR_SPEC>;
#[doc = "Field `EN` reader - 1: in DDR mode, 0 in SDR mode"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - 1: in DDR mode, 0 in SDR mode"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_VAR_DUMMY` reader - Set the bit to enable variable dummy cycle in spi DDR mode."]
pub type SMEM_VAR_DUMMY_R = crate::BitReader;
#[doc = "Field `SMEM_VAR_DUMMY` writer - Set the bit to enable variable dummy cycle in spi DDR mode."]
pub type SMEM_VAR_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDAT_SWP` reader - Set the bit to reorder rx data of the word in spi DDR mode."]
pub type RDAT_SWP_R = crate::BitReader;
#[doc = "Field `RDAT_SWP` writer - Set the bit to reorder rx data of the word in spi DDR mode."]
pub type RDAT_SWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDAT_SWP` reader - Set the bit to reorder tx data of the word in spi DDR mode."]
pub type WDAT_SWP_R = crate::BitReader;
#[doc = "Field `WDAT_SWP` writer - Set the bit to reorder tx data of the word in spi DDR mode."]
pub type WDAT_SWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_DIS` reader - the bit is used to disable dual edge in command phase when DDR mode."]
pub type CMD_DIS_R = crate::BitReader;
#[doc = "Field `CMD_DIS` writer - the bit is used to disable dual edge in command phase when DDR mode."]
pub type CMD_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_OUTMINBYTELEN` reader - It is the minimum output data length in the DDR psram."]
pub type SMEM_OUTMINBYTELEN_R = crate::FieldReader;
#[doc = "Field `SMEM_OUTMINBYTELEN` writer - It is the minimum output data length in the DDR psram."]
pub type SMEM_OUTMINBYTELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SMEM_TX_DDR_MSK_EN` reader - Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to external RAM."]
pub type SMEM_TX_DDR_MSK_EN_R = crate::BitReader;
#[doc = "Field `SMEM_TX_DDR_MSK_EN` writer - Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to external RAM."]
pub type SMEM_TX_DDR_MSK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_RX_DDR_MSK_EN` reader - Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to external RAM."]
pub type SMEM_RX_DDR_MSK_EN_R = crate::BitReader;
#[doc = "Field `SMEM_RX_DDR_MSK_EN` writer - Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to external RAM."]
pub type SMEM_RX_DDR_MSK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_USR_DDR_DQS_THD` reader - The delay number of data strobe which from memory based on SPI clock."]
pub type SMEM_USR_DDR_DQS_THD_R = crate::FieldReader;
#[doc = "Field `SMEM_USR_DDR_DQS_THD` writer - The delay number of data strobe which from memory based on SPI clock."]
pub type SMEM_USR_DDR_DQS_THD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DQS_LOOP` reader - 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS."]
pub type DQS_LOOP_R = crate::BitReader;
#[doc = "Field `DQS_LOOP` writer - 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS."]
pub type DQS_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_CLK_DIFF_EN` reader - Set this bit to enable the differential SPI_CLK#."]
pub type SMEM_CLK_DIFF_EN_R = crate::BitReader;
#[doc = "Field `SMEM_CLK_DIFF_EN` writer - Set this bit to enable the differential SPI_CLK#."]
pub type SMEM_CLK_DIFF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_DQS_CA_IN` reader - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR."]
pub type SMEM_DQS_CA_IN_R = crate::BitReader;
#[doc = "Field `SMEM_DQS_CA_IN` writer - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR."]
pub type SMEM_DQS_CA_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_HYPERBUS_DUMMY_2X` reader - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram."]
pub type SMEM_HYPERBUS_DUMMY_2X_R = crate::BitReader;
#[doc = "Field `SMEM_HYPERBUS_DUMMY_2X` writer - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram."]
pub type SMEM_HYPERBUS_DUMMY_2X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_CLK_DIFF_INV` reader - Set this bit to invert SPI_DIFF when accesses to external RAM. ."]
pub type SMEM_CLK_DIFF_INV_R = crate::BitReader;
#[doc = "Field `SMEM_CLK_DIFF_INV` writer - Set this bit to invert SPI_DIFF when accesses to external RAM. ."]
pub type SMEM_CLK_DIFF_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_OCTA_RAM_ADDR` reader - Set this bit to enable octa_ram address out when accesses to external RAM, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[25:4\\], 6'd0, spi_usr_addr_value\\[3:1\\], 1'b0}."]
pub type SMEM_OCTA_RAM_ADDR_R = crate::BitReader;
#[doc = "Field `SMEM_OCTA_RAM_ADDR` writer - Set this bit to enable octa_ram address out when accesses to external RAM, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[25:4\\], 6'd0, spi_usr_addr_value\\[3:1\\], 1'b0}."]
pub type SMEM_OCTA_RAM_ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMEM_HYPERBUS_CA` reader - Set this bit to enable HyperRAM address out when accesses to external RAM, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[19:4\\], 13'd0, spi_usr_addr_value\\[3:1\\]}."]
pub type SMEM_HYPERBUS_CA_R = crate::BitReader;
#[doc = "Field `SMEM_HYPERBUS_CA` writer - Set this bit to enable HyperRAM address out when accesses to external RAM, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[19:4\\], 13'd0, spi_usr_addr_value\\[3:1\\]}."]
pub type SMEM_HYPERBUS_CA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: in DDR mode, 0 in SDR mode"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set the bit to enable variable dummy cycle in spi DDR mode."]
    #[inline(always)]
    pub fn smem_var_dummy(&self) -> SMEM_VAR_DUMMY_R {
        SMEM_VAR_DUMMY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set the bit to reorder rx data of the word in spi DDR mode."]
    #[inline(always)]
    pub fn rdat_swp(&self) -> RDAT_SWP_R {
        RDAT_SWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set the bit to reorder tx data of the word in spi DDR mode."]
    #[inline(always)]
    pub fn wdat_swp(&self) -> WDAT_SWP_R {
        WDAT_SWP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the bit is used to disable dual edge in command phase when DDR mode."]
    #[inline(always)]
    pub fn cmd_dis(&self) -> CMD_DIS_R {
        CMD_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:11 - It is the minimum output data length in the DDR psram."]
    #[inline(always)]
    pub fn smem_outminbytelen(&self) -> SMEM_OUTMINBYTELEN_R {
        SMEM_OUTMINBYTELEN_R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bit 12 - Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to external RAM."]
    #[inline(always)]
    pub fn smem_tx_ddr_msk_en(&self) -> SMEM_TX_DDR_MSK_EN_R {
        SMEM_TX_DDR_MSK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to external RAM."]
    #[inline(always)]
    pub fn smem_rx_ddr_msk_en(&self) -> SMEM_RX_DDR_MSK_EN_R {
        SMEM_RX_DDR_MSK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:20 - The delay number of data strobe which from memory based on SPI clock."]
    #[inline(always)]
    pub fn smem_usr_ddr_dqs_thd(&self) -> SMEM_USR_DDR_DQS_THD_R {
        SMEM_USR_DDR_DQS_THD_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bit 21 - 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS."]
    #[inline(always)]
    pub fn dqs_loop(&self) -> DQS_LOOP_R {
        DQS_LOOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to enable the differential SPI_CLK#."]
    #[inline(always)]
    pub fn smem_clk_diff_en(&self) -> SMEM_CLK_DIFF_EN_R {
        SMEM_CLK_DIFF_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR."]
    #[inline(always)]
    pub fn smem_dqs_ca_in(&self) -> SMEM_DQS_CA_IN_R {
        SMEM_DQS_CA_IN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram."]
    #[inline(always)]
    pub fn smem_hyperbus_dummy_2x(&self) -> SMEM_HYPERBUS_DUMMY_2X_R {
        SMEM_HYPERBUS_DUMMY_2X_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to invert SPI_DIFF when accesses to external RAM. ."]
    #[inline(always)]
    pub fn smem_clk_diff_inv(&self) -> SMEM_CLK_DIFF_INV_R {
        SMEM_CLK_DIFF_INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to enable octa_ram address out when accesses to external RAM, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[25:4\\], 6'd0, spi_usr_addr_value\\[3:1\\], 1'b0}."]
    #[inline(always)]
    pub fn smem_octa_ram_addr(&self) -> SMEM_OCTA_RAM_ADDR_R {
        SMEM_OCTA_RAM_ADDR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to enable HyperRAM address out when accesses to external RAM, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[19:4\\], 13'd0, spi_usr_addr_value\\[3:1\\]}."]
    #[inline(always)]
    pub fn smem_hyperbus_ca(&self) -> SMEM_HYPERBUS_CA_R {
        SMEM_HYPERBUS_CA_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_DDR")
            .field("en", &self.en())
            .field("smem_var_dummy", &self.smem_var_dummy())
            .field("rdat_swp", &self.rdat_swp())
            .field("wdat_swp", &self.wdat_swp())
            .field("cmd_dis", &self.cmd_dis())
            .field("smem_outminbytelen", &self.smem_outminbytelen())
            .field("smem_tx_ddr_msk_en", &self.smem_tx_ddr_msk_en())
            .field("smem_rx_ddr_msk_en", &self.smem_rx_ddr_msk_en())
            .field("smem_usr_ddr_dqs_thd", &self.smem_usr_ddr_dqs_thd())
            .field("dqs_loop", &self.dqs_loop())
            .field("smem_clk_diff_en", &self.smem_clk_diff_en())
            .field("smem_dqs_ca_in", &self.smem_dqs_ca_in())
            .field("smem_hyperbus_dummy_2x", &self.smem_hyperbus_dummy_2x())
            .field("smem_clk_diff_inv", &self.smem_clk_diff_inv())
            .field("smem_octa_ram_addr", &self.smem_octa_ram_addr())
            .field("smem_hyperbus_ca", &self.smem_hyperbus_ca())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1: in DDR mode, 0 in SDR mode"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, SMEM_DDR_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set the bit to enable variable dummy cycle in spi DDR mode."]
    #[inline(always)]
    pub fn smem_var_dummy(&mut self) -> SMEM_VAR_DUMMY_W<'_, SMEM_DDR_SPEC> {
        SMEM_VAR_DUMMY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set the bit to reorder rx data of the word in spi DDR mode."]
    #[inline(always)]
    pub fn rdat_swp(&mut self) -> RDAT_SWP_W<'_, SMEM_DDR_SPEC> {
        RDAT_SWP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set the bit to reorder tx data of the word in spi DDR mode."]
    #[inline(always)]
    pub fn wdat_swp(&mut self) -> WDAT_SWP_W<'_, SMEM_DDR_SPEC> {
        WDAT_SWP_W::new(self, 3)
    }
    #[doc = "Bit 4 - the bit is used to disable dual edge in command phase when DDR mode."]
    #[inline(always)]
    pub fn cmd_dis(&mut self) -> CMD_DIS_W<'_, SMEM_DDR_SPEC> {
        CMD_DIS_W::new(self, 4)
    }
    #[doc = "Bits 5:11 - It is the minimum output data length in the DDR psram."]
    #[inline(always)]
    pub fn smem_outminbytelen(&mut self) -> SMEM_OUTMINBYTELEN_W<'_, SMEM_DDR_SPEC> {
        SMEM_OUTMINBYTELEN_W::new(self, 5)
    }
    #[doc = "Bit 12 - Set this bit to mask the first or the last byte in SPI0 ECC DDR write mode, when accesses to external RAM."]
    #[inline(always)]
    pub fn smem_tx_ddr_msk_en(&mut self) -> SMEM_TX_DDR_MSK_EN_W<'_, SMEM_DDR_SPEC> {
        SMEM_TX_DDR_MSK_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to mask the first or the last byte in SPI0 ECC DDR read mode, when accesses to external RAM."]
    #[inline(always)]
    pub fn smem_rx_ddr_msk_en(&mut self) -> SMEM_RX_DDR_MSK_EN_W<'_, SMEM_DDR_SPEC> {
        SMEM_RX_DDR_MSK_EN_W::new(self, 13)
    }
    #[doc = "Bits 14:20 - The delay number of data strobe which from memory based on SPI clock."]
    #[inline(always)]
    pub fn smem_usr_ddr_dqs_thd(&mut self) -> SMEM_USR_DDR_DQS_THD_W<'_, SMEM_DDR_SPEC> {
        SMEM_USR_DDR_DQS_THD_W::new(self, 14)
    }
    #[doc = "Bit 21 - 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS."]
    #[inline(always)]
    pub fn dqs_loop(&mut self) -> DQS_LOOP_W<'_, SMEM_DDR_SPEC> {
        DQS_LOOP_W::new(self, 21)
    }
    #[doc = "Bit 24 - Set this bit to enable the differential SPI_CLK#."]
    #[inline(always)]
    pub fn smem_clk_diff_en(&mut self) -> SMEM_CLK_DIFF_EN_W<'_, SMEM_DDR_SPEC> {
        SMEM_CLK_DIFF_EN_W::new(self, 24)
    }
    #[doc = "Bit 26 - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR."]
    #[inline(always)]
    pub fn smem_dqs_ca_in(&mut self) -> SMEM_DQS_CA_IN_W<'_, SMEM_DDR_SPEC> {
        SMEM_DQS_CA_IN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram."]
    #[inline(always)]
    pub fn smem_hyperbus_dummy_2x(&mut self) -> SMEM_HYPERBUS_DUMMY_2X_W<'_, SMEM_DDR_SPEC> {
        SMEM_HYPERBUS_DUMMY_2X_W::new(self, 27)
    }
    #[doc = "Bit 28 - Set this bit to invert SPI_DIFF when accesses to external RAM. ."]
    #[inline(always)]
    pub fn smem_clk_diff_inv(&mut self) -> SMEM_CLK_DIFF_INV_W<'_, SMEM_DDR_SPEC> {
        SMEM_CLK_DIFF_INV_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set this bit to enable octa_ram address out when accesses to external RAM, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[25:4\\], 6'd0, spi_usr_addr_value\\[3:1\\], 1'b0}."]
    #[inline(always)]
    pub fn smem_octa_ram_addr(&mut self) -> SMEM_OCTA_RAM_ADDR_W<'_, SMEM_DDR_SPEC> {
        SMEM_OCTA_RAM_ADDR_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set this bit to enable HyperRAM address out when accesses to external RAM, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[19:4\\], 13'd0, spi_usr_addr_value\\[3:1\\]}."]
    #[inline(always)]
    pub fn smem_hyperbus_ca(&mut self) -> SMEM_HYPERBUS_CA_W<'_, SMEM_DDR_SPEC> {
        SMEM_HYPERBUS_CA_W::new(self, 30)
    }
}
#[doc = "SPI0 external RAM DDR mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_ddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_ddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_DDR_SPEC;
impl crate::RegisterSpec for SMEM_DDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_ddr::R`](R) reader structure"]
impl crate::Readable for SMEM_DDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_ddr::W`](W) writer structure"]
impl crate::Writable for SMEM_DDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_DDR to value 0x3020"]
impl crate::Resettable for SMEM_DDR_SPEC {
    const RESET_VALUE: u32 = 0x3020;
}
