#[doc = "Register `DDR` reader"]
pub struct R(crate::R<DDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDR` writer"]
pub struct W(crate::W<DDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_FMEM_DDR_EN` reader - 1: in DDR mode, 0: in SDR mode."]
pub type SPI_FMEM_DDR_EN_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_DDR_EN` writer - 1: in DDR mode, 0: in SDR mode."]
pub type SPI_FMEM_DDR_EN_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
#[doc = "Field `SPI_FMEM_VAR_DUMMY` reader - Set the bit to enable variable dummy cycle in DDRmode."]
pub type SPI_FMEM_VAR_DUMMY_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_VAR_DUMMY` writer - Set the bit to enable variable dummy cycle in DDRmode."]
pub type SPI_FMEM_VAR_DUMMY_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
#[doc = "Field `SPI_FMEM_DDR_RDAT_SWP` reader - Set the bit to reorder RX data of the word in DDR mode."]
pub type SPI_FMEM_DDR_RDAT_SWP_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_DDR_RDAT_SWP` writer - Set the bit to reorder RX data of the word in DDR mode."]
pub type SPI_FMEM_DDR_RDAT_SWP_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
#[doc = "Field `SPI_FMEM_DDR_WDAT_SWP` reader - Set the bit to reorder TX data of the word in DDR mode."]
pub type SPI_FMEM_DDR_WDAT_SWP_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_DDR_WDAT_SWP` writer - Set the bit to reorder TX data of the word in DDR mode."]
pub type SPI_FMEM_DDR_WDAT_SWP_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
#[doc = "Field `SPI_FMEM_DDR_CMD_DIS` reader - the bit is used to disable dual edge in command phase when DDR mode."]
pub type SPI_FMEM_DDR_CMD_DIS_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_DDR_CMD_DIS` writer - the bit is used to disable dual edge in command phase when DDR mode."]
pub type SPI_FMEM_DDR_CMD_DIS_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
#[doc = "Field `SPI_FMEM_OUTMINBYTELEN` reader - It is the minimum output data length in the panda device."]
pub type SPI_FMEM_OUTMINBYTELEN_R = crate::FieldReader;
#[doc = "Field `SPI_FMEM_OUTMINBYTELEN` writer - It is the minimum output data length in the panda device."]
pub type SPI_FMEM_OUTMINBYTELEN_W<'a, const O: u8> = crate::FieldWriter<'a, DDR_SPEC, 7, O>;
#[doc = "Field `SPI_FMEM_USR_DDR_DQS_THD` reader - The delay number of data strobe which from memory based on SPI_CLK."]
pub type SPI_FMEM_USR_DDR_DQS_THD_R = crate::FieldReader;
#[doc = "Field `SPI_FMEM_USR_DDR_DQS_THD` writer - The delay number of data strobe which from memory based on SPI_CLK."]
pub type SPI_FMEM_USR_DDR_DQS_THD_W<'a, const O: u8> = crate::FieldWriter<'a, DDR_SPEC, 7, O>;
#[doc = "Field `SPI_FMEM_DDR_DQS_LOOP` reader - 1: Use internal signal as data strobe, the strobe can not be delayed by input timing module. 0: Use input SPI_DQS signal from PAD as data strobe, the strobe can be delayed by input timing module"]
pub type SPI_FMEM_DDR_DQS_LOOP_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_DDR_DQS_LOOP` writer - 1: Use internal signal as data strobe, the strobe can not be delayed by input timing module. 0: Use input SPI_DQS signal from PAD as data strobe, the strobe can be delayed by input timing module"]
pub type SPI_FMEM_DDR_DQS_LOOP_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
#[doc = "Field `SPI_FMEM_DDR_DQS_LOOP_MODE` reader - When SPI_FMEM_DDR_DQS_LOOP and SPI_FMEM_DDR_EN are set, 1: Use internal SPI_CLK as data strobe. 0: Use internal ~SPI_CLK as data strobe. Otherwise this bit is not active."]
pub type SPI_FMEM_DDR_DQS_LOOP_MODE_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_DDR_DQS_LOOP_MODE` writer - When SPI_FMEM_DDR_DQS_LOOP and SPI_FMEM_DDR_EN are set, 1: Use internal SPI_CLK as data strobe. 0: Use internal ~SPI_CLK as data strobe. Otherwise this bit is not active."]
pub type SPI_FMEM_DDR_DQS_LOOP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
#[doc = "Field `SPI_FMEM_CLK_DIFF_EN` reader - Set this bit to enable the differential SPI_CLK#."]
pub type SPI_FMEM_CLK_DIFF_EN_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_CLK_DIFF_EN` writer - Set this bit to enable the differential SPI_CLK#."]
pub type SPI_FMEM_CLK_DIFF_EN_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
#[doc = "Field `SPI_FMEM_HYPERBUS_MODE` reader - Set this bit to enable the SPI HyperBus mode."]
pub type SPI_FMEM_HYPERBUS_MODE_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_HYPERBUS_MODE` writer - Set this bit to enable the SPI HyperBus mode."]
pub type SPI_FMEM_HYPERBUS_MODE_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
#[doc = "Field `SPI_FMEM_DQS_CA_IN` reader - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR."]
pub type SPI_FMEM_DQS_CA_IN_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_DQS_CA_IN` writer - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR."]
pub type SPI_FMEM_DQS_CA_IN_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
#[doc = "Field `SPI_FMEM_HYPERBUS_DUMMY_2X` reader - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram."]
pub type SPI_FMEM_HYPERBUS_DUMMY_2X_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_HYPERBUS_DUMMY_2X` writer - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram."]
pub type SPI_FMEM_HYPERBUS_DUMMY_2X_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
#[doc = "Field `SPI_FMEM_CLK_DIFF_INV` reader - Set this bit to invert SPI_DIFF when accesses to flash. ."]
pub type SPI_FMEM_CLK_DIFF_INV_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_CLK_DIFF_INV` writer - Set this bit to invert SPI_DIFF when accesses to flash. ."]
pub type SPI_FMEM_CLK_DIFF_INV_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
#[doc = "Field `SPI_FMEM_OCTA_RAM_ADDR` reader - Set this bit to enable octa_ram address out when accesses to flash, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[25:4\\], 6'd0, spi_usr_addr_value\\[3:1\\], 1'b0}."]
pub type SPI_FMEM_OCTA_RAM_ADDR_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_OCTA_RAM_ADDR` writer - Set this bit to enable octa_ram address out when accesses to flash, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[25:4\\], 6'd0, spi_usr_addr_value\\[3:1\\], 1'b0}."]
pub type SPI_FMEM_OCTA_RAM_ADDR_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
#[doc = "Field `SPI_FMEM_HYPERBUS_CA` reader - Set this bit to enable HyperRAM address out when accesses to flash, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[19:4\\], 13'd0, spi_usr_addr_value\\[3:1\\]}."]
pub type SPI_FMEM_HYPERBUS_CA_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_HYPERBUS_CA` writer - Set this bit to enable HyperRAM address out when accesses to flash, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[19:4\\], 13'd0, spi_usr_addr_value\\[3:1\\]}."]
pub type SPI_FMEM_HYPERBUS_CA_W<'a, const O: u8> = crate::BitWriter<'a, DDR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - 1: in DDR mode, 0: in SDR mode."]
    #[inline(always)]
    pub fn spi_fmem_ddr_en(&self) -> SPI_FMEM_DDR_EN_R {
        SPI_FMEM_DDR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set the bit to enable variable dummy cycle in DDRmode."]
    #[inline(always)]
    pub fn spi_fmem_var_dummy(&self) -> SPI_FMEM_VAR_DUMMY_R {
        SPI_FMEM_VAR_DUMMY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set the bit to reorder RX data of the word in DDR mode."]
    #[inline(always)]
    pub fn spi_fmem_ddr_rdat_swp(&self) -> SPI_FMEM_DDR_RDAT_SWP_R {
        SPI_FMEM_DDR_RDAT_SWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set the bit to reorder TX data of the word in DDR mode."]
    #[inline(always)]
    pub fn spi_fmem_ddr_wdat_swp(&self) -> SPI_FMEM_DDR_WDAT_SWP_R {
        SPI_FMEM_DDR_WDAT_SWP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - the bit is used to disable dual edge in command phase when DDR mode."]
    #[inline(always)]
    pub fn spi_fmem_ddr_cmd_dis(&self) -> SPI_FMEM_DDR_CMD_DIS_R {
        SPI_FMEM_DDR_CMD_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:11 - It is the minimum output data length in the panda device."]
    #[inline(always)]
    pub fn spi_fmem_outminbytelen(&self) -> SPI_FMEM_OUTMINBYTELEN_R {
        SPI_FMEM_OUTMINBYTELEN_R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bits 14:20 - The delay number of data strobe which from memory based on SPI_CLK."]
    #[inline(always)]
    pub fn spi_fmem_usr_ddr_dqs_thd(&self) -> SPI_FMEM_USR_DDR_DQS_THD_R {
        SPI_FMEM_USR_DDR_DQS_THD_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bit 21 - 1: Use internal signal as data strobe, the strobe can not be delayed by input timing module. 0: Use input SPI_DQS signal from PAD as data strobe, the strobe can be delayed by input timing module"]
    #[inline(always)]
    pub fn spi_fmem_ddr_dqs_loop(&self) -> SPI_FMEM_DDR_DQS_LOOP_R {
        SPI_FMEM_DDR_DQS_LOOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When SPI_FMEM_DDR_DQS_LOOP and SPI_FMEM_DDR_EN are set, 1: Use internal SPI_CLK as data strobe. 0: Use internal ~SPI_CLK as data strobe. Otherwise this bit is not active."]
    #[inline(always)]
    pub fn spi_fmem_ddr_dqs_loop_mode(&self) -> SPI_FMEM_DDR_DQS_LOOP_MODE_R {
        SPI_FMEM_DDR_DQS_LOOP_MODE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to enable the differential SPI_CLK#."]
    #[inline(always)]
    pub fn spi_fmem_clk_diff_en(&self) -> SPI_FMEM_CLK_DIFF_EN_R {
        SPI_FMEM_CLK_DIFF_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set this bit to enable the SPI HyperBus mode."]
    #[inline(always)]
    pub fn spi_fmem_hyperbus_mode(&self) -> SPI_FMEM_HYPERBUS_MODE_R {
        SPI_FMEM_HYPERBUS_MODE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR."]
    #[inline(always)]
    pub fn spi_fmem_dqs_ca_in(&self) -> SPI_FMEM_DQS_CA_IN_R {
        SPI_FMEM_DQS_CA_IN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram."]
    #[inline(always)]
    pub fn spi_fmem_hyperbus_dummy_2x(&self) -> SPI_FMEM_HYPERBUS_DUMMY_2X_R {
        SPI_FMEM_HYPERBUS_DUMMY_2X_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to invert SPI_DIFF when accesses to flash. ."]
    #[inline(always)]
    pub fn spi_fmem_clk_diff_inv(&self) -> SPI_FMEM_CLK_DIFF_INV_R {
        SPI_FMEM_CLK_DIFF_INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to enable octa_ram address out when accesses to flash, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[25:4\\], 6'd0, spi_usr_addr_value\\[3:1\\], 1'b0}."]
    #[inline(always)]
    pub fn spi_fmem_octa_ram_addr(&self) -> SPI_FMEM_OCTA_RAM_ADDR_R {
        SPI_FMEM_OCTA_RAM_ADDR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to enable HyperRAM address out when accesses to flash, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[19:4\\], 13'd0, spi_usr_addr_value\\[3:1\\]}."]
    #[inline(always)]
    pub fn spi_fmem_hyperbus_ca(&self) -> SPI_FMEM_HYPERBUS_CA_R {
        SPI_FMEM_HYPERBUS_CA_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDR")
            .field(
                "spi_fmem_ddr_en",
                &format_args!("{}", self.spi_fmem_ddr_en().bit()),
            )
            .field(
                "spi_fmem_var_dummy",
                &format_args!("{}", self.spi_fmem_var_dummy().bit()),
            )
            .field(
                "spi_fmem_ddr_rdat_swp",
                &format_args!("{}", self.spi_fmem_ddr_rdat_swp().bit()),
            )
            .field(
                "spi_fmem_ddr_wdat_swp",
                &format_args!("{}", self.spi_fmem_ddr_wdat_swp().bit()),
            )
            .field(
                "spi_fmem_ddr_cmd_dis",
                &format_args!("{}", self.spi_fmem_ddr_cmd_dis().bit()),
            )
            .field(
                "spi_fmem_outminbytelen",
                &format_args!("{}", self.spi_fmem_outminbytelen().bits()),
            )
            .field(
                "spi_fmem_usr_ddr_dqs_thd",
                &format_args!("{}", self.spi_fmem_usr_ddr_dqs_thd().bits()),
            )
            .field(
                "spi_fmem_ddr_dqs_loop",
                &format_args!("{}", self.spi_fmem_ddr_dqs_loop().bit()),
            )
            .field(
                "spi_fmem_ddr_dqs_loop_mode",
                &format_args!("{}", self.spi_fmem_ddr_dqs_loop_mode().bit()),
            )
            .field(
                "spi_fmem_clk_diff_en",
                &format_args!("{}", self.spi_fmem_clk_diff_en().bit()),
            )
            .field(
                "spi_fmem_hyperbus_mode",
                &format_args!("{}", self.spi_fmem_hyperbus_mode().bit()),
            )
            .field(
                "spi_fmem_dqs_ca_in",
                &format_args!("{}", self.spi_fmem_dqs_ca_in().bit()),
            )
            .field(
                "spi_fmem_hyperbus_dummy_2x",
                &format_args!("{}", self.spi_fmem_hyperbus_dummy_2x().bit()),
            )
            .field(
                "spi_fmem_clk_diff_inv",
                &format_args!("{}", self.spi_fmem_clk_diff_inv().bit()),
            )
            .field(
                "spi_fmem_octa_ram_addr",
                &format_args!("{}", self.spi_fmem_octa_ram_addr().bit()),
            )
            .field(
                "spi_fmem_hyperbus_ca",
                &format_args!("{}", self.spi_fmem_hyperbus_ca().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 1: in DDR mode, 0: in SDR mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_ddr_en(&mut self) -> SPI_FMEM_DDR_EN_W<0> {
        SPI_FMEM_DDR_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set the bit to enable variable dummy cycle in DDRmode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_var_dummy(&mut self) -> SPI_FMEM_VAR_DUMMY_W<1> {
        SPI_FMEM_VAR_DUMMY_W::new(self)
    }
    #[doc = "Bit 2 - Set the bit to reorder RX data of the word in DDR mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_ddr_rdat_swp(&mut self) -> SPI_FMEM_DDR_RDAT_SWP_W<2> {
        SPI_FMEM_DDR_RDAT_SWP_W::new(self)
    }
    #[doc = "Bit 3 - Set the bit to reorder TX data of the word in DDR mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_ddr_wdat_swp(&mut self) -> SPI_FMEM_DDR_WDAT_SWP_W<3> {
        SPI_FMEM_DDR_WDAT_SWP_W::new(self)
    }
    #[doc = "Bit 4 - the bit is used to disable dual edge in command phase when DDR mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_ddr_cmd_dis(&mut self) -> SPI_FMEM_DDR_CMD_DIS_W<4> {
        SPI_FMEM_DDR_CMD_DIS_W::new(self)
    }
    #[doc = "Bits 5:11 - It is the minimum output data length in the panda device."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_outminbytelen(&mut self) -> SPI_FMEM_OUTMINBYTELEN_W<5> {
        SPI_FMEM_OUTMINBYTELEN_W::new(self)
    }
    #[doc = "Bits 14:20 - The delay number of data strobe which from memory based on SPI_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_usr_ddr_dqs_thd(&mut self) -> SPI_FMEM_USR_DDR_DQS_THD_W<14> {
        SPI_FMEM_USR_DDR_DQS_THD_W::new(self)
    }
    #[doc = "Bit 21 - 1: Use internal signal as data strobe, the strobe can not be delayed by input timing module. 0: Use input SPI_DQS signal from PAD as data strobe, the strobe can be delayed by input timing module"]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_ddr_dqs_loop(&mut self) -> SPI_FMEM_DDR_DQS_LOOP_W<21> {
        SPI_FMEM_DDR_DQS_LOOP_W::new(self)
    }
    #[doc = "Bit 22 - When SPI_FMEM_DDR_DQS_LOOP and SPI_FMEM_DDR_EN are set, 1: Use internal SPI_CLK as data strobe. 0: Use internal ~SPI_CLK as data strobe. Otherwise this bit is not active."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_ddr_dqs_loop_mode(&mut self) -> SPI_FMEM_DDR_DQS_LOOP_MODE_W<22> {
        SPI_FMEM_DDR_DQS_LOOP_MODE_W::new(self)
    }
    #[doc = "Bit 24 - Set this bit to enable the differential SPI_CLK#."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_clk_diff_en(&mut self) -> SPI_FMEM_CLK_DIFF_EN_W<24> {
        SPI_FMEM_CLK_DIFF_EN_W::new(self)
    }
    #[doc = "Bit 25 - Set this bit to enable the SPI HyperBus mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_hyperbus_mode(&mut self) -> SPI_FMEM_HYPERBUS_MODE_W<25> {
        SPI_FMEM_HYPERBUS_MODE_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to enable the input of SPI_DQS signal in SPI phases of CMD and ADDR."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_dqs_ca_in(&mut self) -> SPI_FMEM_DQS_CA_IN_W<26> {
        SPI_FMEM_DQS_CA_IN_W::new(self)
    }
    #[doc = "Bit 27 - Set this bit to enable the vary dummy function in SPI HyperBus mode, when SPI0 accesses flash or SPI1 accesses flash or sram."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_hyperbus_dummy_2x(&mut self) -> SPI_FMEM_HYPERBUS_DUMMY_2X_W<27> {
        SPI_FMEM_HYPERBUS_DUMMY_2X_W::new(self)
    }
    #[doc = "Bit 28 - Set this bit to invert SPI_DIFF when accesses to flash. ."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_clk_diff_inv(&mut self) -> SPI_FMEM_CLK_DIFF_INV_W<28> {
        SPI_FMEM_CLK_DIFF_INV_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to enable octa_ram address out when accesses to flash, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[25:4\\], 6'd0, spi_usr_addr_value\\[3:1\\], 1'b0}."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_octa_ram_addr(&mut self) -> SPI_FMEM_OCTA_RAM_ADDR_W<29> {
        SPI_FMEM_OCTA_RAM_ADDR_W::new(self)
    }
    #[doc = "Bit 30 - Set this bit to enable HyperRAM address out when accesses to flash, which means ADDR_OUT\\[31:0\\] = {spi_usr_addr_value\\[19:4\\], 13'd0, spi_usr_addr_value\\[3:1\\]}."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fmem_hyperbus_ca(&mut self) -> SPI_FMEM_HYPERBUS_CA_W<30> {
        SPI_FMEM_HYPERBUS_CA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 DDR control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddr](index.html) module"]
pub struct DDR_SPEC;
impl crate::RegisterSpec for DDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddr::R](R) reader structure"]
impl crate::Readable for DDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddr::W](W) writer structure"]
impl crate::Writable for DDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDR to value 0x20"]
impl crate::Resettable for DDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
