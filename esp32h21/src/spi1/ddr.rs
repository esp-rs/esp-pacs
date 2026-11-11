#[doc = "Register `DDR` reader"]
pub type R = crate::R<DDR_SPEC>;
#[doc = "Field `FMEM_DDR_EN` reader - 1: in ddr mode, 0 in sdr mode"]
pub type FMEM_DDR_EN_R = crate::BitReader;
#[doc = "Field `FMEM_VAR_DUMMY` reader - Set the bit to enable variable dummy cycle in spi ddr mode."]
pub type FMEM_VAR_DUMMY_R = crate::BitReader;
#[doc = "Field `FMEM_DDR_RDAT_SWP` reader - Set the bit to reorder rx data of the word in spi ddr mode."]
pub type FMEM_DDR_RDAT_SWP_R = crate::BitReader;
#[doc = "Field `FMEM_DDR_WDAT_SWP` reader - Set the bit to reorder tx data of the word in spi ddr mode."]
pub type FMEM_DDR_WDAT_SWP_R = crate::BitReader;
#[doc = "Field `FMEM_DDR_CMD_DIS` reader - the bit is used to disable dual edge in command phase when ddr mode."]
pub type FMEM_DDR_CMD_DIS_R = crate::BitReader;
#[doc = "Field `FMEM_OUTMINBYTELEN` reader - It is the minimum output data length in the panda device."]
pub type FMEM_OUTMINBYTELEN_R = crate::FieldReader;
#[doc = "Field `FMEM_USR_DDR_DQS_THD` reader - The delay number of data strobe which from memory based on SPI clock."]
pub type FMEM_USR_DDR_DQS_THD_R = crate::FieldReader;
#[doc = "Field `FMEM_DDR_DQS_LOOP` reader - 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS."]
pub type FMEM_DDR_DQS_LOOP_R = crate::BitReader;
#[doc = "Field `FMEM_CLK_DIFF_EN` reader - Set this bit to enable the differential SPI_CLK#."]
pub type FMEM_CLK_DIFF_EN_R = crate::BitReader;
#[doc = "Field `FMEM_DQS_CA_IN` reader - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR."]
pub type FMEM_DQS_CA_IN_R = crate::BitReader;
#[doc = "Field `FMEM_HYPERBUS_DUMMY_2X` reader - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram."]
pub type FMEM_HYPERBUS_DUMMY_2X_R = crate::BitReader;
#[doc = "Field `FMEM_CLK_DIFF_INV` reader - Set this bit to invert SPI_DIFF when accesses to flash. ."]
pub type FMEM_CLK_DIFF_INV_R = crate::BitReader;
#[doc = "Field `FMEM_OCTA_RAM_ADDR` reader - Set this bit to enable octa_ram address out when accesses to flash, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[25:4\\], 6'd0, spi_usr_addr_value\\[3:1\\], 1'b0}."]
pub type FMEM_OCTA_RAM_ADDR_R = crate::BitReader;
#[doc = "Field `FMEM_HYPERBUS_CA` reader - Set this bit to enable HyperRAM address out when accesses to flash, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[19:4\\], 13'd0, spi_usr_addr_value\\[3:1\\]}."]
pub type FMEM_HYPERBUS_CA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: in ddr mode, 0 in sdr mode"]
    #[inline(always)]
    pub fn fmem_ddr_en(&self) -> FMEM_DDR_EN_R {
        FMEM_DDR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set the bit to enable variable dummy cycle in spi ddr mode."]
    #[inline(always)]
    pub fn fmem_var_dummy(&self) -> FMEM_VAR_DUMMY_R {
        FMEM_VAR_DUMMY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set the bit to reorder rx data of the word in spi ddr mode."]
    #[inline(always)]
    pub fn fmem_ddr_rdat_swp(&self) -> FMEM_DDR_RDAT_SWP_R {
        FMEM_DDR_RDAT_SWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set the bit to reorder tx data of the word in spi ddr mode."]
    #[inline(always)]
    pub fn fmem_ddr_wdat_swp(&self) -> FMEM_DDR_WDAT_SWP_R {
        FMEM_DDR_WDAT_SWP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the bit is used to disable dual edge in command phase when ddr mode."]
    #[inline(always)]
    pub fn fmem_ddr_cmd_dis(&self) -> FMEM_DDR_CMD_DIS_R {
        FMEM_DDR_CMD_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:11 - It is the minimum output data length in the panda device."]
    #[inline(always)]
    pub fn fmem_outminbytelen(&self) -> FMEM_OUTMINBYTELEN_R {
        FMEM_OUTMINBYTELEN_R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bits 14:20 - The delay number of data strobe which from memory based on SPI clock."]
    #[inline(always)]
    pub fn fmem_usr_ddr_dqs_thd(&self) -> FMEM_USR_DDR_DQS_THD_R {
        FMEM_USR_DDR_DQS_THD_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bit 21 - 1: Do not need the input of SPI_DQS signal, SPI0 starts to receive data when spi0_slv_st is in SPI_MEM_DIN state. It is used when there is no SPI_DQS signal or SPI_DQS signal is not stable. 0: SPI0 starts to store data at the positive and negative edge of SPI_DQS."]
    #[inline(always)]
    pub fn fmem_ddr_dqs_loop(&self) -> FMEM_DDR_DQS_LOOP_R {
        FMEM_DDR_DQS_LOOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to enable the differential SPI_CLK#."]
    #[inline(always)]
    pub fn fmem_clk_diff_en(&self) -> FMEM_CLK_DIFF_EN_R {
        FMEM_CLK_DIFF_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR."]
    #[inline(always)]
    pub fn fmem_dqs_ca_in(&self) -> FMEM_DQS_CA_IN_R {
        FMEM_DQS_CA_IN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram."]
    #[inline(always)]
    pub fn fmem_hyperbus_dummy_2x(&self) -> FMEM_HYPERBUS_DUMMY_2X_R {
        FMEM_HYPERBUS_DUMMY_2X_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to invert SPI_DIFF when accesses to flash. ."]
    #[inline(always)]
    pub fn fmem_clk_diff_inv(&self) -> FMEM_CLK_DIFF_INV_R {
        FMEM_CLK_DIFF_INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to enable octa_ram address out when accesses to flash, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[25:4\\], 6'd0, spi_usr_addr_value\\[3:1\\], 1'b0}."]
    #[inline(always)]
    pub fn fmem_octa_ram_addr(&self) -> FMEM_OCTA_RAM_ADDR_R {
        FMEM_OCTA_RAM_ADDR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to enable HyperRAM address out when accesses to flash, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[19:4\\], 13'd0, spi_usr_addr_value\\[3:1\\]}."]
    #[inline(always)]
    pub fn fmem_hyperbus_ca(&self) -> FMEM_HYPERBUS_CA_R {
        FMEM_HYPERBUS_CA_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDR")
            .field("fmem_ddr_en", &self.fmem_ddr_en())
            .field("fmem_var_dummy", &self.fmem_var_dummy())
            .field("fmem_ddr_rdat_swp", &self.fmem_ddr_rdat_swp())
            .field("fmem_ddr_wdat_swp", &self.fmem_ddr_wdat_swp())
            .field("fmem_ddr_cmd_dis", &self.fmem_ddr_cmd_dis())
            .field("fmem_outminbytelen", &self.fmem_outminbytelen())
            .field("fmem_usr_ddr_dqs_thd", &self.fmem_usr_ddr_dqs_thd())
            .field("fmem_ddr_dqs_loop", &self.fmem_ddr_dqs_loop())
            .field("fmem_clk_diff_en", &self.fmem_clk_diff_en())
            .field("fmem_dqs_ca_in", &self.fmem_dqs_ca_in())
            .field("fmem_hyperbus_dummy_2x", &self.fmem_hyperbus_dummy_2x())
            .field("fmem_clk_diff_inv", &self.fmem_clk_diff_inv())
            .field("fmem_octa_ram_addr", &self.fmem_octa_ram_addr())
            .field("fmem_hyperbus_ca", &self.fmem_hyperbus_ca())
            .finish()
    }
}
#[doc = "SPI1 DDR control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDR_SPEC;
impl crate::RegisterSpec for DDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr::R`](R) reader structure"]
impl crate::Readable for DDR_SPEC {}
#[doc = "`reset()` method sets DDR to value 0x20"]
impl crate::Resettable for DDR_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
