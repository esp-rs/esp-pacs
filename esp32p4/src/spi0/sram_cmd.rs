#[doc = "Register `SRAM_CMD` reader"]
pub type R = crate::R<SRAM_CMD_SPEC>;
#[doc = "Register `SRAM_CMD` writer"]
pub type W = crate::W<SRAM_CMD_SPEC>;
#[doc = "Field `SCLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
pub type SCLK_MODE_R = crate::FieldReader;
#[doc = "Field `SCLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
pub type SCLK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWB_MODE` reader - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type SWB_MODE_R = crate::FieldReader;
#[doc = "Field `SWB_MODE` writer - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type SWB_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDIN_DUAL` reader - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SDIN_DUAL_R = crate::BitReader;
#[doc = "Field `SDIN_DUAL` writer - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SDIN_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDOUT_DUAL` reader - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SDOUT_DUAL_R = crate::BitReader;
#[doc = "Field `SDOUT_DUAL` writer - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SDOUT_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR_DUAL` reader - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SADDR_DUAL_R = crate::BitReader;
#[doc = "Field `SADDR_DUAL` writer - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SADDR_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIN_QUAD` reader - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SDIN_QUAD_R = crate::BitReader;
#[doc = "Field `SDIN_QUAD` writer - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SDIN_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDOUT_QUAD` reader - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SDOUT_QUAD_R = crate::BitReader;
#[doc = "Field `SDOUT_QUAD` writer - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SDOUT_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR_QUAD` reader - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SADDR_QUAD_R = crate::BitReader;
#[doc = "Field `SADDR_QUAD` writer - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SADDR_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMD_QUAD` reader - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SCMD_QUAD_R = crate::BitReader;
#[doc = "Field `SCMD_QUAD` writer - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SCMD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIN_OCT` reader - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
pub type SDIN_OCT_R = crate::BitReader;
#[doc = "Field `SDIN_OCT` writer - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
pub type SDIN_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDOUT_OCT` reader - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
pub type SDOUT_OCT_R = crate::BitReader;
#[doc = "Field `SDOUT_OCT` writer - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
pub type SDOUT_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR_OCT` reader - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
pub type SADDR_OCT_R = crate::BitReader;
#[doc = "Field `SADDR_OCT` writer - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
pub type SADDR_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCMD_OCT` reader - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
pub type SCMD_OCT_R = crate::BitReader;
#[doc = "Field `SCMD_OCT` writer - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
pub type SCMD_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDUMMY_RIN` reader - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type SDUMMY_RIN_R = crate::BitReader;
#[doc = "Field `SDUMMY_RIN` writer - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type SDUMMY_RIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDUMMY_WOUT` reader - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type SDUMMY_WOUT_R = crate::BitReader;
#[doc = "Field `SDUMMY_WOUT` writer - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type SDUMMY_WOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT` reader - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller."]
pub type SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT` writer - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller."]
pub type SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_WDUMMY_ALWAYS_OUT` reader - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
pub type SPI_SMEM_WDUMMY_ALWAYS_OUT_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_WDUMMY_ALWAYS_OUT` writer - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
pub type SPI_SMEM_WDUMMY_ALWAYS_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIN_HEX` reader - For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable."]
pub type SDIN_HEX_R = crate::BitReader;
#[doc = "Field `SDIN_HEX` writer - For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable."]
pub type SDIN_HEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDOUT_HEX` reader - For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable."]
pub type SDOUT_HEX_R = crate::BitReader;
#[doc = "Field `SDOUT_HEX` writer - For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable."]
pub type SDOUT_HEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_DQS_IE_ALWAYS_ON` reader - When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
pub type SPI_SMEM_DQS_IE_ALWAYS_ON_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_DQS_IE_ALWAYS_ON` writer - When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
pub type SPI_SMEM_DQS_IE_ALWAYS_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SMEM_DATA_IE_ALWAYS_ON` reader - When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
pub type SPI_SMEM_DATA_IE_ALWAYS_ON_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_DATA_IE_ALWAYS_ON` writer - When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
pub type SPI_SMEM_DATA_IE_ALWAYS_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
    #[inline(always)]
    pub fn sclk_mode(&self) -> SCLK_MODE_R {
        SCLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:9 - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn swb_mode(&self) -> SWB_MODE_R {
        SWB_MODE_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bit 10 - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn sdin_dual(&self) -> SDIN_DUAL_R {
        SDIN_DUAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn sdout_dual(&self) -> SDOUT_DUAL_R {
        SDOUT_DUAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn saddr_dual(&self) -> SADDR_DUAL_R {
        SADDR_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn sdin_quad(&self) -> SDIN_QUAD_R {
        SDIN_QUAD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn sdout_quad(&self) -> SDOUT_QUAD_R {
        SDOUT_QUAD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn saddr_quad(&self) -> SADDR_QUAD_R {
        SADDR_QUAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn scmd_quad(&self) -> SCMD_QUAD_R {
        SCMD_QUAD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdin_oct(&self) -> SDIN_OCT_R {
        SDIN_OCT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdout_oct(&self) -> SDOUT_OCT_R {
        SDOUT_OCT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn saddr_oct(&self) -> SADDR_OCT_R {
        SADDR_OCT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn scmd_oct(&self) -> SCMD_OCT_R {
        SCMD_OCT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn sdummy_rin(&self) -> SDUMMY_RIN_R {
        SDUMMY_RIN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn sdummy_wout(&self) -> SDUMMY_WOUT_R {
        SDUMMY_WOUT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_smem_wdummy_dqs_always_out(&self) -> SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_R {
        SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_smem_wdummy_always_out(&self) -> SPI_SMEM_WDUMMY_ALWAYS_OUT_R {
        SPI_SMEM_WDUMMY_ALWAYS_OUT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdin_hex(&self) -> SDIN_HEX_R {
        SDIN_HEX_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdout_hex(&self) -> SDOUT_HEX_R {
        SDOUT_HEX_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
    #[inline(always)]
    pub fn spi_smem_dqs_ie_always_on(&self) -> SPI_SMEM_DQS_IE_ALWAYS_ON_R {
        SPI_SMEM_DQS_IE_ALWAYS_ON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
    #[inline(always)]
    pub fn spi_smem_data_ie_always_on(&self) -> SPI_SMEM_DATA_IE_ALWAYS_ON_R {
        SPI_SMEM_DATA_IE_ALWAYS_ON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CMD")
            .field("sclk_mode", &self.sclk_mode())
            .field("swb_mode", &self.swb_mode())
            .field("sdin_dual", &self.sdin_dual())
            .field("sdout_dual", &self.sdout_dual())
            .field("saddr_dual", &self.saddr_dual())
            .field("sdin_quad", &self.sdin_quad())
            .field("sdout_quad", &self.sdout_quad())
            .field("saddr_quad", &self.saddr_quad())
            .field("scmd_quad", &self.scmd_quad())
            .field("sdin_oct", &self.sdin_oct())
            .field("sdout_oct", &self.sdout_oct())
            .field("saddr_oct", &self.saddr_oct())
            .field("scmd_oct", &self.scmd_oct())
            .field("sdummy_rin", &self.sdummy_rin())
            .field("sdummy_wout", &self.sdummy_wout())
            .field(
                "spi_smem_wdummy_dqs_always_out",
                &self.spi_smem_wdummy_dqs_always_out(),
            )
            .field(
                "spi_smem_wdummy_always_out",
                &self.spi_smem_wdummy_always_out(),
            )
            .field("sdin_hex", &self.sdin_hex())
            .field("sdout_hex", &self.sdout_hex())
            .field(
                "spi_smem_dqs_ie_always_on",
                &self.spi_smem_dqs_ie_always_on(),
            )
            .field(
                "spi_smem_data_ie_always_on",
                &self.spi_smem_data_ie_always_on(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
    #[inline(always)]
    pub fn sclk_mode(&mut self) -> SCLK_MODE_W<SRAM_CMD_SPEC> {
        SCLK_MODE_W::new(self, 0)
    }
    #[doc = "Bits 2:9 - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn swb_mode(&mut self) -> SWB_MODE_W<SRAM_CMD_SPEC> {
        SWB_MODE_W::new(self, 2)
    }
    #[doc = "Bit 10 - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn sdin_dual(&mut self) -> SDIN_DUAL_W<SRAM_CMD_SPEC> {
        SDIN_DUAL_W::new(self, 10)
    }
    #[doc = "Bit 11 - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn sdout_dual(&mut self) -> SDOUT_DUAL_W<SRAM_CMD_SPEC> {
        SDOUT_DUAL_W::new(self, 11)
    }
    #[doc = "Bit 12 - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn saddr_dual(&mut self) -> SADDR_DUAL_W<SRAM_CMD_SPEC> {
        SADDR_DUAL_W::new(self, 12)
    }
    #[doc = "Bit 14 - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn sdin_quad(&mut self) -> SDIN_QUAD_W<SRAM_CMD_SPEC> {
        SDIN_QUAD_W::new(self, 14)
    }
    #[doc = "Bit 15 - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn sdout_quad(&mut self) -> SDOUT_QUAD_W<SRAM_CMD_SPEC> {
        SDOUT_QUAD_W::new(self, 15)
    }
    #[doc = "Bit 16 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn saddr_quad(&mut self) -> SADDR_QUAD_W<SRAM_CMD_SPEC> {
        SADDR_QUAD_W::new(self, 16)
    }
    #[doc = "Bit 17 - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn scmd_quad(&mut self) -> SCMD_QUAD_W<SRAM_CMD_SPEC> {
        SCMD_QUAD_W::new(self, 17)
    }
    #[doc = "Bit 18 - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdin_oct(&mut self) -> SDIN_OCT_W<SRAM_CMD_SPEC> {
        SDIN_OCT_W::new(self, 18)
    }
    #[doc = "Bit 19 - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdout_oct(&mut self) -> SDOUT_OCT_W<SRAM_CMD_SPEC> {
        SDOUT_OCT_W::new(self, 19)
    }
    #[doc = "Bit 20 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn saddr_oct(&mut self) -> SADDR_OCT_W<SRAM_CMD_SPEC> {
        SADDR_OCT_W::new(self, 20)
    }
    #[doc = "Bit 21 - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn scmd_oct(&mut self) -> SCMD_OCT_W<SRAM_CMD_SPEC> {
        SCMD_OCT_W::new(self, 21)
    }
    #[doc = "Bit 22 - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn sdummy_rin(&mut self) -> SDUMMY_RIN_W<SRAM_CMD_SPEC> {
        SDUMMY_RIN_W::new(self, 22)
    }
    #[doc = "Bit 23 - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn sdummy_wout(&mut self) -> SDUMMY_WOUT_W<SRAM_CMD_SPEC> {
        SDUMMY_WOUT_W::new(self, 23)
    }
    #[doc = "Bit 24 - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_smem_wdummy_dqs_always_out(
        &mut self,
    ) -> SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_W<SRAM_CMD_SPEC> {
        SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_W::new(self, 24)
    }
    #[doc = "Bit 25 - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_smem_wdummy_always_out(&mut self) -> SPI_SMEM_WDUMMY_ALWAYS_OUT_W<SRAM_CMD_SPEC> {
        SPI_SMEM_WDUMMY_ALWAYS_OUT_W::new(self, 25)
    }
    #[doc = "Bit 26 - For SPI0 external RAM , din phase apply 16 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdin_hex(&mut self) -> SDIN_HEX_W<SRAM_CMD_SPEC> {
        SDIN_HEX_W::new(self, 26)
    }
    #[doc = "Bit 27 - For SPI0 external RAM , dout phase apply 16 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn sdout_hex(&mut self) -> SDOUT_HEX_W<SRAM_CMD_SPEC> {
        SDOUT_HEX_W::new(self, 27)
    }
    #[doc = "Bit 30 - When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
    #[inline(always)]
    pub fn spi_smem_dqs_ie_always_on(&mut self) -> SPI_SMEM_DQS_IE_ALWAYS_ON_W<SRAM_CMD_SPEC> {
        SPI_SMEM_DQS_IE_ALWAYS_ON_W::new(self, 30)
    }
    #[doc = "Bit 31 - When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
    #[inline(always)]
    pub fn spi_smem_data_ie_always_on(&mut self) -> SPI_SMEM_DATA_IE_ALWAYS_ON_W<SRAM_CMD_SPEC> {
        SPI_SMEM_DATA_IE_ALWAYS_ON_W::new(self, 31)
    }
}
#[doc = "SPI0 external RAM mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_CMD_SPEC;
impl crate::RegisterSpec for SRAM_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_cmd::R`](R) reader structure"]
impl crate::Readable for SRAM_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_cmd::W`](W) writer structure"]
impl crate::Writable for SRAM_CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM_CMD to value 0x80c0_0000"]
impl crate::Resettable for SRAM_CMD_SPEC {
    const RESET_VALUE: u32 = 0x80c0_0000;
}
