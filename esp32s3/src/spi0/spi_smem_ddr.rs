///Register `SPI_SMEM_DDR` reader
pub type R = crate::R<SPI_SMEM_DDR_SPEC>;
///Register `SPI_SMEM_DDR` writer
pub type W = crate::W<SPI_SMEM_DDR_SPEC>;
///Field `EN` reader - 1: in ddr mode, 0 in sdr mode
pub type EN_R = crate::BitReader;
///Field `EN` writer - 1: in ddr mode, 0 in sdr mode
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_SMEM_VAR_DUMMY` reader - Set the bit to enable variable dummy cycle in spi ddr mode.
pub type SPI_SMEM_VAR_DUMMY_R = crate::BitReader;
///Field `SPI_SMEM_VAR_DUMMY` writer - Set the bit to enable variable dummy cycle in spi ddr mode.
pub type SPI_SMEM_VAR_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDAT_SWP` reader - Set the bit to reorder rx data of the word in spi ddr mode.
pub type RDAT_SWP_R = crate::BitReader;
///Field `RDAT_SWP` writer - Set the bit to reorder rx data of the word in spi ddr mode.
pub type RDAT_SWP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDAT_SWP` reader - Set the bit to reorder tx data of the word in spi ddr mode.
pub type WDAT_SWP_R = crate::BitReader;
///Field `WDAT_SWP` writer - Set the bit to reorder tx data of the word in spi ddr mode.
pub type WDAT_SWP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMD_DIS` reader - the bit is used to disable dual edge in CMD phase when ddr mode.
pub type CMD_DIS_R = crate::BitReader;
///Field `CMD_DIS` writer - the bit is used to disable dual edge in CMD phase when ddr mode.
pub type CMD_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_SMEM_OUTMINBYTELEN` reader - It is the minimum output data length in the ddr psram.
pub type SPI_SMEM_OUTMINBYTELEN_R = crate::FieldReader;
///Field `SPI_SMEM_OUTMINBYTELEN` writer - It is the minimum output data length in the ddr psram.
pub type SPI_SMEM_OUTMINBYTELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `SPI_SMEM_TX_DDR_MSK_EN` reader - Set this bit to mask the first or the last byte in MSPI ECC DDR write mode, when accesses to external RAM.
pub type SPI_SMEM_TX_DDR_MSK_EN_R = crate::BitReader;
///Field `SPI_SMEM_TX_DDR_MSK_EN` writer - Set this bit to mask the first or the last byte in MSPI ECC DDR write mode, when accesses to external RAM.
pub type SPI_SMEM_TX_DDR_MSK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_SMEM_RX_DDR_MSK_EN` reader - Set this bit to mask the first or the last byte in MSPI ECC DDR read mode, when accesses to external RAM.
pub type SPI_SMEM_RX_DDR_MSK_EN_R = crate::BitReader;
///Field `SPI_SMEM_RX_DDR_MSK_EN` writer - Set this bit to mask the first or the last byte in MSPI ECC DDR read mode, when accesses to external RAM.
pub type SPI_SMEM_RX_DDR_MSK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_SMEM_USR_DDR_DQS_THD` reader - The delay number of data strobe which from memory based on SPI_CLK.
pub type SPI_SMEM_USR_DDR_DQS_THD_R = crate::FieldReader;
///Field `SPI_SMEM_USR_DDR_DQS_THD` writer - The delay number of data strobe which from memory based on SPI_CLK.
pub type SPI_SMEM_USR_DDR_DQS_THD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `DQS_LOOP` reader - 1: Use internal signal as data strobe, the strobe can not be delayed by input timing module. 0: Use input SPI_DQS signal from PAD as data strobe, the strobe can be delayed by input timing module
pub type DQS_LOOP_R = crate::BitReader;
///Field `DQS_LOOP` writer - 1: Use internal signal as data strobe, the strobe can not be delayed by input timing module. 0: Use input SPI_DQS signal from PAD as data strobe, the strobe can be delayed by input timing module
pub type DQS_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DQS_LOOP_MODE` reader - When SPI_SMEM_DDR_DQS_LOOP and SPI_SMEM_DDR_EN are set, 1: Use internal SPI_CLK as data strobe. 0: Use internal ~SPI_CLK as data strobe. Otherwise this bit is not active.
pub type DQS_LOOP_MODE_R = crate::BitReader;
///Field `DQS_LOOP_MODE` writer - When SPI_SMEM_DDR_DQS_LOOP and SPI_SMEM_DDR_EN are set, 1: Use internal SPI_CLK as data strobe. 0: Use internal ~SPI_CLK as data strobe. Otherwise this bit is not active.
pub type DQS_LOOP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_SMEM_CLK_DIFF_EN` reader - Set this bit to enable the differential SPI_CLK#.
pub type SPI_SMEM_CLK_DIFF_EN_R = crate::BitReader;
///Field `SPI_SMEM_CLK_DIFF_EN` writer - Set this bit to enable the differential SPI_CLK#.
pub type SPI_SMEM_CLK_DIFF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_SMEM_HYPERBUS_MODE` reader - Set this bit to enable the SPI HyperBus mode.
pub type SPI_SMEM_HYPERBUS_MODE_R = crate::BitReader;
///Field `SPI_SMEM_HYPERBUS_MODE` writer - Set this bit to enable the SPI HyperBus mode.
pub type SPI_SMEM_HYPERBUS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_SMEM_DQS_CA_IN` reader - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
pub type SPI_SMEM_DQS_CA_IN_R = crate::BitReader;
///Field `SPI_SMEM_DQS_CA_IN` writer - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
pub type SPI_SMEM_DQS_CA_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_SMEM_HYPERBUS_DUMMY_2X` reader - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses to flash or SPI1 accesses flash or sram.
pub type SPI_SMEM_HYPERBUS_DUMMY_2X_R = crate::BitReader;
///Field `SPI_SMEM_HYPERBUS_DUMMY_2X` writer - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses to flash or SPI1 accesses flash or sram.
pub type SPI_SMEM_HYPERBUS_DUMMY_2X_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_SMEM_CLK_DIFF_INV` reader - Set this bit to invert SPI_DIFF when accesses to external RAM. .
pub type SPI_SMEM_CLK_DIFF_INV_R = crate::BitReader;
///Field `SPI_SMEM_CLK_DIFF_INV` writer - Set this bit to invert SPI_DIFF when accesses to external RAM. .
pub type SPI_SMEM_CLK_DIFF_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_SMEM_OCTA_RAM_ADDR` reader - Set this bit to enable octa_ram address out when accesses to external RAM, which means ADDR_OUT\[31:0\] = {spi_usr_addr_value\[25:4\], 6'd0, spi_usr_addr_value\[3:1\], 1'b0}.
pub type SPI_SMEM_OCTA_RAM_ADDR_R = crate::BitReader;
///Field `SPI_SMEM_OCTA_RAM_ADDR` writer - Set this bit to enable octa_ram address out when accesses to external RAM, which means ADDR_OUT\[31:0\] = {spi_usr_addr_value\[25:4\], 6'd0, spi_usr_addr_value\[3:1\], 1'b0}.
pub type SPI_SMEM_OCTA_RAM_ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI_SMEM_HYPERBUS_CA` reader - Set this bit to enable HyperRAM address out when accesses to external RAM, which means ADDR_OUT\[31:0\] = {spi_usr_addr_value\[19:4\], 13'd0, spi_usr_addr_value\[3:1\]}.
pub type SPI_SMEM_HYPERBUS_CA_R = crate::BitReader;
///Field `SPI_SMEM_HYPERBUS_CA` writer - Set this bit to enable HyperRAM address out when accesses to external RAM, which means ADDR_OUT\[31:0\] = {spi_usr_addr_value\[19:4\], 13'd0, spi_usr_addr_value\[3:1\]}.
pub type SPI_SMEM_HYPERBUS_CA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 1: in ddr mode, 0 in sdr mode
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set the bit to enable variable dummy cycle in spi ddr mode.
    #[inline(always)]
    pub fn spi_smem_var_dummy(&self) -> SPI_SMEM_VAR_DUMMY_R {
        SPI_SMEM_VAR_DUMMY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set the bit to reorder rx data of the word in spi ddr mode.
    #[inline(always)]
    pub fn rdat_swp(&self) -> RDAT_SWP_R {
        RDAT_SWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set the bit to reorder tx data of the word in spi ddr mode.
    #[inline(always)]
    pub fn wdat_swp(&self) -> WDAT_SWP_R {
        WDAT_SWP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - the bit is used to disable dual edge in CMD phase when ddr mode.
    #[inline(always)]
    pub fn cmd_dis(&self) -> CMD_DIS_R {
        CMD_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:11 - It is the minimum output data length in the ddr psram.
    #[inline(always)]
    pub fn spi_smem_outminbytelen(&self) -> SPI_SMEM_OUTMINBYTELEN_R {
        SPI_SMEM_OUTMINBYTELEN_R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    ///Bit 12 - Set this bit to mask the first or the last byte in MSPI ECC DDR write mode, when accesses to external RAM.
    #[inline(always)]
    pub fn spi_smem_tx_ddr_msk_en(&self) -> SPI_SMEM_TX_DDR_MSK_EN_R {
        SPI_SMEM_TX_DDR_MSK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Set this bit to mask the first or the last byte in MSPI ECC DDR read mode, when accesses to external RAM.
    #[inline(always)]
    pub fn spi_smem_rx_ddr_msk_en(&self) -> SPI_SMEM_RX_DDR_MSK_EN_R {
        SPI_SMEM_RX_DDR_MSK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:20 - The delay number of data strobe which from memory based on SPI_CLK.
    #[inline(always)]
    pub fn spi_smem_usr_ddr_dqs_thd(&self) -> SPI_SMEM_USR_DDR_DQS_THD_R {
        SPI_SMEM_USR_DDR_DQS_THD_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    ///Bit 21 - 1: Use internal signal as data strobe, the strobe can not be delayed by input timing module. 0: Use input SPI_DQS signal from PAD as data strobe, the strobe can be delayed by input timing module
    #[inline(always)]
    pub fn dqs_loop(&self) -> DQS_LOOP_R {
        DQS_LOOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - When SPI_SMEM_DDR_DQS_LOOP and SPI_SMEM_DDR_EN are set, 1: Use internal SPI_CLK as data strobe. 0: Use internal ~SPI_CLK as data strobe. Otherwise this bit is not active.
    #[inline(always)]
    pub fn dqs_loop_mode(&self) -> DQS_LOOP_MODE_R {
        DQS_LOOP_MODE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Set this bit to enable the differential SPI_CLK#.
    #[inline(always)]
    pub fn spi_smem_clk_diff_en(&self) -> SPI_SMEM_CLK_DIFF_EN_R {
        SPI_SMEM_CLK_DIFF_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Set this bit to enable the SPI HyperBus mode.
    #[inline(always)]
    pub fn spi_smem_hyperbus_mode(&self) -> SPI_SMEM_HYPERBUS_MODE_R {
        SPI_SMEM_HYPERBUS_MODE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
    #[inline(always)]
    pub fn spi_smem_dqs_ca_in(&self) -> SPI_SMEM_DQS_CA_IN_R {
        SPI_SMEM_DQS_CA_IN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses to flash or SPI1 accesses flash or sram.
    #[inline(always)]
    pub fn spi_smem_hyperbus_dummy_2x(&self) -> SPI_SMEM_HYPERBUS_DUMMY_2X_R {
        SPI_SMEM_HYPERBUS_DUMMY_2X_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Set this bit to invert SPI_DIFF when accesses to external RAM. .
    #[inline(always)]
    pub fn spi_smem_clk_diff_inv(&self) -> SPI_SMEM_CLK_DIFF_INV_R {
        SPI_SMEM_CLK_DIFF_INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Set this bit to enable octa_ram address out when accesses to external RAM, which means ADDR_OUT\[31:0\] = {spi_usr_addr_value\[25:4\], 6'd0, spi_usr_addr_value\[3:1\], 1'b0}.
    #[inline(always)]
    pub fn spi_smem_octa_ram_addr(&self) -> SPI_SMEM_OCTA_RAM_ADDR_R {
        SPI_SMEM_OCTA_RAM_ADDR_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Set this bit to enable HyperRAM address out when accesses to external RAM, which means ADDR_OUT\[31:0\] = {spi_usr_addr_value\[19:4\], 13'd0, spi_usr_addr_value\[3:1\]}.
    #[inline(always)]
    pub fn spi_smem_hyperbus_ca(&self) -> SPI_SMEM_HYPERBUS_CA_R {
        SPI_SMEM_HYPERBUS_CA_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_DDR")
            .field("en", &self.en())
            .field("spi_smem_var_dummy", &self.spi_smem_var_dummy())
            .field("rdat_swp", &self.rdat_swp())
            .field("wdat_swp", &self.wdat_swp())
            .field("cmd_dis", &self.cmd_dis())
            .field("spi_smem_outminbytelen", &self.spi_smem_outminbytelen())
            .field("spi_smem_tx_ddr_msk_en", &self.spi_smem_tx_ddr_msk_en())
            .field("spi_smem_rx_ddr_msk_en", &self.spi_smem_rx_ddr_msk_en())
            .field("spi_smem_usr_ddr_dqs_thd", &self.spi_smem_usr_ddr_dqs_thd())
            .field("dqs_loop", &self.dqs_loop())
            .field("dqs_loop_mode", &self.dqs_loop_mode())
            .field("spi_smem_clk_diff_en", &self.spi_smem_clk_diff_en())
            .field("spi_smem_hyperbus_mode", &self.spi_smem_hyperbus_mode())
            .field("spi_smem_dqs_ca_in", &self.spi_smem_dqs_ca_in())
            .field(
                "spi_smem_hyperbus_dummy_2x",
                &self.spi_smem_hyperbus_dummy_2x(),
            )
            .field("spi_smem_clk_diff_inv", &self.spi_smem_clk_diff_inv())
            .field("spi_smem_octa_ram_addr", &self.spi_smem_octa_ram_addr())
            .field("spi_smem_hyperbus_ca", &self.spi_smem_hyperbus_ca())
            .finish()
    }
}
impl W {
    ///Bit 0 - 1: in ddr mode, 0 in sdr mode
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<SPI_SMEM_DDR_SPEC> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Set the bit to enable variable dummy cycle in spi ddr mode.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_var_dummy(&mut self) -> SPI_SMEM_VAR_DUMMY_W<SPI_SMEM_DDR_SPEC> {
        SPI_SMEM_VAR_DUMMY_W::new(self, 1)
    }
    ///Bit 2 - Set the bit to reorder rx data of the word in spi ddr mode.
    #[inline(always)]
    #[must_use]
    pub fn rdat_swp(&mut self) -> RDAT_SWP_W<SPI_SMEM_DDR_SPEC> {
        RDAT_SWP_W::new(self, 2)
    }
    ///Bit 3 - Set the bit to reorder tx data of the word in spi ddr mode.
    #[inline(always)]
    #[must_use]
    pub fn wdat_swp(&mut self) -> WDAT_SWP_W<SPI_SMEM_DDR_SPEC> {
        WDAT_SWP_W::new(self, 3)
    }
    ///Bit 4 - the bit is used to disable dual edge in CMD phase when ddr mode.
    #[inline(always)]
    #[must_use]
    pub fn cmd_dis(&mut self) -> CMD_DIS_W<SPI_SMEM_DDR_SPEC> {
        CMD_DIS_W::new(self, 4)
    }
    ///Bits 5:11 - It is the minimum output data length in the ddr psram.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_outminbytelen(&mut self) -> SPI_SMEM_OUTMINBYTELEN_W<SPI_SMEM_DDR_SPEC> {
        SPI_SMEM_OUTMINBYTELEN_W::new(self, 5)
    }
    ///Bit 12 - Set this bit to mask the first or the last byte in MSPI ECC DDR write mode, when accesses to external RAM.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_tx_ddr_msk_en(&mut self) -> SPI_SMEM_TX_DDR_MSK_EN_W<SPI_SMEM_DDR_SPEC> {
        SPI_SMEM_TX_DDR_MSK_EN_W::new(self, 12)
    }
    ///Bit 13 - Set this bit to mask the first or the last byte in MSPI ECC DDR read mode, when accesses to external RAM.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_rx_ddr_msk_en(&mut self) -> SPI_SMEM_RX_DDR_MSK_EN_W<SPI_SMEM_DDR_SPEC> {
        SPI_SMEM_RX_DDR_MSK_EN_W::new(self, 13)
    }
    ///Bits 14:20 - The delay number of data strobe which from memory based on SPI_CLK.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_usr_ddr_dqs_thd(&mut self) -> SPI_SMEM_USR_DDR_DQS_THD_W<SPI_SMEM_DDR_SPEC> {
        SPI_SMEM_USR_DDR_DQS_THD_W::new(self, 14)
    }
    ///Bit 21 - 1: Use internal signal as data strobe, the strobe can not be delayed by input timing module. 0: Use input SPI_DQS signal from PAD as data strobe, the strobe can be delayed by input timing module
    #[inline(always)]
    #[must_use]
    pub fn dqs_loop(&mut self) -> DQS_LOOP_W<SPI_SMEM_DDR_SPEC> {
        DQS_LOOP_W::new(self, 21)
    }
    ///Bit 22 - When SPI_SMEM_DDR_DQS_LOOP and SPI_SMEM_DDR_EN are set, 1: Use internal SPI_CLK as data strobe. 0: Use internal ~SPI_CLK as data strobe. Otherwise this bit is not active.
    #[inline(always)]
    #[must_use]
    pub fn dqs_loop_mode(&mut self) -> DQS_LOOP_MODE_W<SPI_SMEM_DDR_SPEC> {
        DQS_LOOP_MODE_W::new(self, 22)
    }
    ///Bit 24 - Set this bit to enable the differential SPI_CLK#.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_clk_diff_en(&mut self) -> SPI_SMEM_CLK_DIFF_EN_W<SPI_SMEM_DDR_SPEC> {
        SPI_SMEM_CLK_DIFF_EN_W::new(self, 24)
    }
    ///Bit 25 - Set this bit to enable the SPI HyperBus mode.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_hyperbus_mode(&mut self) -> SPI_SMEM_HYPERBUS_MODE_W<SPI_SMEM_DDR_SPEC> {
        SPI_SMEM_HYPERBUS_MODE_W::new(self, 25)
    }
    ///Bit 26 - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_dqs_ca_in(&mut self) -> SPI_SMEM_DQS_CA_IN_W<SPI_SMEM_DDR_SPEC> {
        SPI_SMEM_DQS_CA_IN_W::new(self, 26)
    }
    ///Bit 27 - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses to flash or SPI1 accesses flash or sram.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_hyperbus_dummy_2x(
        &mut self,
    ) -> SPI_SMEM_HYPERBUS_DUMMY_2X_W<SPI_SMEM_DDR_SPEC> {
        SPI_SMEM_HYPERBUS_DUMMY_2X_W::new(self, 27)
    }
    ///Bit 28 - Set this bit to invert SPI_DIFF when accesses to external RAM. .
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_clk_diff_inv(&mut self) -> SPI_SMEM_CLK_DIFF_INV_W<SPI_SMEM_DDR_SPEC> {
        SPI_SMEM_CLK_DIFF_INV_W::new(self, 28)
    }
    ///Bit 29 - Set this bit to enable octa_ram address out when accesses to external RAM, which means ADDR_OUT\[31:0\] = {spi_usr_addr_value\[25:4\], 6'd0, spi_usr_addr_value\[3:1\], 1'b0}.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_octa_ram_addr(&mut self) -> SPI_SMEM_OCTA_RAM_ADDR_W<SPI_SMEM_DDR_SPEC> {
        SPI_SMEM_OCTA_RAM_ADDR_W::new(self, 29)
    }
    ///Bit 30 - Set this bit to enable HyperRAM address out when accesses to external RAM, which means ADDR_OUT\[31:0\] = {spi_usr_addr_value\[19:4\], 13'd0, spi_usr_addr_value\[3:1\]}.
    #[inline(always)]
    #[must_use]
    pub fn spi_smem_hyperbus_ca(&mut self) -> SPI_SMEM_HYPERBUS_CA_W<SPI_SMEM_DDR_SPEC> {
        SPI_SMEM_HYPERBUS_CA_W::new(self, 30)
    }
}
/**SPI0 external RAM DDR mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_ddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_smem_ddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SPI_SMEM_DDR_SPEC;
impl crate::RegisterSpec for SPI_SMEM_DDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`spi_smem_ddr::R`](R) reader structure
impl crate::Readable for SPI_SMEM_DDR_SPEC {}
///`write(|w| ..)` method takes [`spi_smem_ddr::W`](W) writer structure
impl crate::Writable for SPI_SMEM_DDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SPI_SMEM_DDR to value 0x3020
impl crate::Resettable for SPI_SMEM_DDR_SPEC {
    const RESET_VALUE: u32 = 0x3020;
}
