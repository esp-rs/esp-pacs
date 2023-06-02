#[doc = "Register `SPI_MEM_SRAM_CMD` reader"]
pub struct R(crate::R<SPI_MEM_SRAM_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_SRAM_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_SRAM_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_SRAM_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_SRAM_CMD` writer"]
pub struct W(crate::W<SPI_MEM_SRAM_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_SRAM_CMD_SPEC>;
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
impl From<crate::W<SPI_MEM_SRAM_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_SRAM_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_SCLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
pub type SPI_MEM_SCLK_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_SWB_MODE` reader - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
pub type SPI_MEM_SWB_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_SDIN_DUAL` reader - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SPI_MEM_SDIN_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SDOUT_DUAL` reader - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SPI_MEM_SDOUT_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SADDR_DUAL` reader - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
pub type SPI_MEM_SADDR_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SDIN_QUAD` reader - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SPI_MEM_SDIN_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SDOUT_QUAD` reader - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SPI_MEM_SDOUT_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SADDR_QUAD` reader - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SPI_MEM_SADDR_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SCMD_QUAD` reader - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
pub type SPI_MEM_SCMD_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SDIN_OCT` reader - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
pub type SPI_MEM_SDIN_OCT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SDOUT_OCT` reader - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
pub type SPI_MEM_SDOUT_OCT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SADDR_OCT` reader - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
pub type SPI_MEM_SADDR_OCT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SCMD_OCT` reader - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
pub type SPI_MEM_SCMD_OCT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SDUMMY_RIN` reader - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type SPI_MEM_SDUMMY_RIN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SDUMMY_RIN` writer - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type SPI_MEM_SDUMMY_RIN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_SRAM_CMD_SPEC, O>;
#[doc = "Field `SPI_MEM_SDUMMY_WOUT` reader - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
pub type SPI_MEM_SDUMMY_WOUT_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT` reader - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_DQS is output by the MSPI controller."]
pub type SPI_SMEM_WDUMMY_DQS_ALWAYS_OUT_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_WDUMMY_ALWAYS_OUT` reader - In the dummy phase of an MSPI write data transfer when accesses to external RAM, the level of SPI_IO\\[7:0\\] is output by the MSPI controller."]
pub type SPI_SMEM_WDUMMY_ALWAYS_OUT_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_DQS_IE_ALWAYS_ON` reader - When accesses to external RAM, 1: the IE signals of pads connected to SPI_DQS are always 1. 0: Others."]
pub type SPI_SMEM_DQS_IE_ALWAYS_ON_R = crate::BitReader;
#[doc = "Field `SPI_SMEM_DATA_IE_ALWAYS_ON` reader - When accesses to external RAM, 1: the IE signals of pads connected to SPI_IO\\[7:0\\] are always 1. 0: Others."]
pub type SPI_SMEM_DATA_IE_ALWAYS_ON_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is always on."]
    #[inline(always)]
    pub fn spi_mem_sclk_mode(&self) -> SPI_MEM_SCLK_MODE_R {
        SPI_MEM_SCLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:9 - Mode bits in the external RAM fast read mode it is combined with spi_mem_fastrd_mode bit."]
    #[inline(always)]
    pub fn spi_mem_swb_mode(&self) -> SPI_MEM_SWB_MODE_R {
        SPI_MEM_SWB_MODE_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bit 10 - For SPI0 external RAM , din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn spi_mem_sdin_dual(&self) -> SPI_MEM_SDIN_DUAL_R {
        SPI_MEM_SDIN_DUAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - For SPI0 external RAM , dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn spi_mem_sdout_dual(&self) -> SPI_MEM_SDOUT_DUAL_R {
        SPI_MEM_SDOUT_DUAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - For SPI0 external RAM , address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_dio."]
    #[inline(always)]
    pub fn spi_mem_saddr_dual(&self) -> SPI_MEM_SADDR_DUAL_R {
        SPI_MEM_SADDR_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - For SPI0 external RAM , din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn spi_mem_sdin_quad(&self) -> SPI_MEM_SDIN_QUAD_R {
        SPI_MEM_SDIN_QUAD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - For SPI0 external RAM , dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn spi_mem_sdout_quad(&self) -> SPI_MEM_SDOUT_QUAD_R {
        SPI_MEM_SDOUT_QUAD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn spi_mem_saddr_quad(&self) -> SPI_MEM_SADDR_QUAD_R {
        SPI_MEM_SADDR_QUAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - For SPI0 external RAM , cmd phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_usr_sram_qio."]
    #[inline(always)]
    pub fn spi_mem_scmd_quad(&self) -> SPI_MEM_SCMD_QUAD_R {
        SPI_MEM_SCMD_QUAD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - For SPI0 external RAM , din phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_sdin_oct(&self) -> SPI_MEM_SDIN_OCT_R {
        SPI_MEM_SDIN_OCT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - For SPI0 external RAM , dout phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_sdout_oct(&self) -> SPI_MEM_SDOUT_OCT_R {
        SPI_MEM_SDOUT_OCT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - For SPI0 external RAM , address phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_saddr_oct(&self) -> SPI_MEM_SADDR_OCT_R {
        SPI_MEM_SADDR_OCT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - For SPI0 external RAM , cmd phase apply 8 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_scmd_oct(&self) -> SPI_MEM_SCMD_OCT_R {
        SPI_MEM_SCMD_OCT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_mem_sdummy_rin(&self) -> SPI_MEM_SDUMMY_RIN_R {
        SPI_MEM_SDUMMY_RIN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - In the dummy phase of a MSPI write data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_mem_sdummy_wout(&self) -> SPI_MEM_SDUMMY_WOUT_R {
        SPI_MEM_SDUMMY_WOUT_R::new(((self.bits >> 23) & 1) != 0)
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
        f.debug_struct("SPI_MEM_SRAM_CMD")
            .field(
                "spi_mem_sclk_mode",
                &format_args!("{}", self.spi_mem_sclk_mode().bits()),
            )
            .field(
                "spi_mem_swb_mode",
                &format_args!("{}", self.spi_mem_swb_mode().bits()),
            )
            .field(
                "spi_mem_sdin_dual",
                &format_args!("{}", self.spi_mem_sdin_dual().bit()),
            )
            .field(
                "spi_mem_sdout_dual",
                &format_args!("{}", self.spi_mem_sdout_dual().bit()),
            )
            .field(
                "spi_mem_saddr_dual",
                &format_args!("{}", self.spi_mem_saddr_dual().bit()),
            )
            .field(
                "spi_mem_sdin_quad",
                &format_args!("{}", self.spi_mem_sdin_quad().bit()),
            )
            .field(
                "spi_mem_sdout_quad",
                &format_args!("{}", self.spi_mem_sdout_quad().bit()),
            )
            .field(
                "spi_mem_saddr_quad",
                &format_args!("{}", self.spi_mem_saddr_quad().bit()),
            )
            .field(
                "spi_mem_scmd_quad",
                &format_args!("{}", self.spi_mem_scmd_quad().bit()),
            )
            .field(
                "spi_mem_sdin_oct",
                &format_args!("{}", self.spi_mem_sdin_oct().bit()),
            )
            .field(
                "spi_mem_sdout_oct",
                &format_args!("{}", self.spi_mem_sdout_oct().bit()),
            )
            .field(
                "spi_mem_saddr_oct",
                &format_args!("{}", self.spi_mem_saddr_oct().bit()),
            )
            .field(
                "spi_mem_scmd_oct",
                &format_args!("{}", self.spi_mem_scmd_oct().bit()),
            )
            .field(
                "spi_mem_sdummy_rin",
                &format_args!("{}", self.spi_mem_sdummy_rin().bit()),
            )
            .field(
                "spi_mem_sdummy_wout",
                &format_args!("{}", self.spi_mem_sdummy_wout().bit()),
            )
            .field(
                "spi_smem_wdummy_dqs_always_out",
                &format_args!("{}", self.spi_smem_wdummy_dqs_always_out().bit()),
            )
            .field(
                "spi_smem_wdummy_always_out",
                &format_args!("{}", self.spi_smem_wdummy_always_out().bit()),
            )
            .field(
                "spi_smem_dqs_ie_always_on",
                &format_args!("{}", self.spi_smem_dqs_ie_always_on().bit()),
            )
            .field(
                "spi_smem_data_ie_always_on",
                &format_args!("{}", self.spi_smem_data_ie_always_on().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_SRAM_CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 22 - In the dummy phase of a MSPI read data transfer when accesses to external RAM, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_sdummy_rin(&mut self) -> SPI_MEM_SDUMMY_RIN_W<22> {
        SPI_MEM_SDUMMY_RIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 external RAM mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_sram_cmd](index.html) module"]
pub struct SPI_MEM_SRAM_CMD_SPEC;
impl crate::RegisterSpec for SPI_MEM_SRAM_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_sram_cmd::R](R) reader structure"]
impl crate::Readable for SPI_MEM_SRAM_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_sram_cmd::W](W) writer structure"]
impl crate::Writable for SPI_MEM_SRAM_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_SRAM_CMD to value 0xc040_0000"]
impl crate::Resettable for SPI_MEM_SRAM_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0xc040_0000;
}
