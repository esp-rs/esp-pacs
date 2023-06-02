#[doc = "Register `SRAM_CMD` reader"]
pub struct R(crate::R<SRAM_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAM_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAM_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAM_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAM_CMD` writer"]
pub struct W(crate::W<SRAM_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAM_CMD_SPEC>;
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
impl From<crate::W<SRAM_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAM_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLK_MODE` reader - SPI_CLK mode bits when SPI0 accesses to Ext_RAM. 0: SPI_CLK is off when CS inactive 1: SPI_CLK is delayed one cycle after CS inactive 2: SPI_CLK is delayed two cycles after CS inactive 3: SPI_CLK is always on."]
pub type SCLK_MODE_R = crate::FieldReader;
#[doc = "Field `SCLK_MODE` writer - SPI_CLK mode bits when SPI0 accesses to Ext_RAM. 0: SPI_CLK is off when CS inactive 1: SPI_CLK is delayed one cycle after CS inactive 2: SPI_CLK is delayed two cycles after CS inactive 3: SPI_CLK is always on."]
pub type SCLK_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, SRAM_CMD_SPEC, 2, O>;
#[doc = "Field `SWB_MODE` reader - Mode bits when SPI0 accesses to Ext_RAM."]
pub type SWB_MODE_R = crate::FieldReader;
#[doc = "Field `SWB_MODE` writer - Mode bits when SPI0 accesses to Ext_RAM."]
pub type SWB_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, SRAM_CMD_SPEC, 8, O>;
#[doc = "Field `SDIN_DUAL` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DIN phase."]
pub type SDIN_DUAL_R = crate::BitReader;
#[doc = "Field `SDIN_DUAL` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DIN phase."]
pub type SDIN_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SDOUT_DUAL` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DOUT phase."]
pub type SDOUT_DUAL_R = crate::BitReader;
#[doc = "Field `SDOUT_DUAL` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DOUT phase."]
pub type SDOUT_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SADDR_DUAL` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in ADDR phase."]
pub type SADDR_DUAL_R = crate::BitReader;
#[doc = "Field `SADDR_DUAL` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in ADDR phase."]
pub type SADDR_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SCMD_DUAL` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in CMD phase."]
pub type SCMD_DUAL_R = crate::BitReader;
#[doc = "Field `SCMD_DUAL` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in CMD phase."]
pub type SCMD_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SDIN_QUAD` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DIN phase."]
pub type SDIN_QUAD_R = crate::BitReader;
#[doc = "Field `SDIN_QUAD` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DIN phase."]
pub type SDIN_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SDOUT_QUAD` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DOUT phase."]
pub type SDOUT_QUAD_R = crate::BitReader;
#[doc = "Field `SDOUT_QUAD` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DOUT phase."]
pub type SDOUT_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SADDR_QUAD` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in ADDR phase."]
pub type SADDR_QUAD_R = crate::BitReader;
#[doc = "Field `SADDR_QUAD` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in ADDR phase."]
pub type SADDR_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SCMD_QUAD` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in CMD phase."]
pub type SCMD_QUAD_R = crate::BitReader;
#[doc = "Field `SCMD_QUAD` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in CMD phase."]
pub type SCMD_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SDIN_OCT` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DIN phase."]
pub type SDIN_OCT_R = crate::BitReader;
#[doc = "Field `SDIN_OCT` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DIN phase."]
pub type SDIN_OCT_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SDOUT_OCT` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DOUT phase."]
pub type SDOUT_OCT_R = crate::BitReader;
#[doc = "Field `SDOUT_OCT` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DOUT phase."]
pub type SDOUT_OCT_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SADDR_OCT` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in ADDR phase."]
pub type SADDR_OCT_R = crate::BitReader;
#[doc = "Field `SADDR_OCT` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in ADDR phase."]
pub type SADDR_OCT_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SCMD_OCT` reader - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in CMD phase."]
pub type SCMD_OCT_R = crate::BitReader;
#[doc = "Field `SCMD_OCT` writer - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in CMD phase."]
pub type SCMD_OCT_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
#[doc = "Field `SDUMMY_OUT` reader - When SPI0 accesses to Ext_RAM, in the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
pub type SDUMMY_OUT_R = crate::BitReader;
#[doc = "Field `SDUMMY_OUT` writer - When SPI0 accesses to Ext_RAM, in the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
pub type SDUMMY_OUT_W<'a, const O: u8> = crate::BitWriter<'a, SRAM_CMD_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - SPI_CLK mode bits when SPI0 accesses to Ext_RAM. 0: SPI_CLK is off when CS inactive 1: SPI_CLK is delayed one cycle after CS inactive 2: SPI_CLK is delayed two cycles after CS inactive 3: SPI_CLK is always on."]
    #[inline(always)]
    pub fn sclk_mode(&self) -> SCLK_MODE_R {
        SCLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:9 - Mode bits when SPI0 accesses to Ext_RAM."]
    #[inline(always)]
    pub fn swb_mode(&self) -> SWB_MODE_R {
        SWB_MODE_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bit 10 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DIN phase."]
    #[inline(always)]
    pub fn sdin_dual(&self) -> SDIN_DUAL_R {
        SDIN_DUAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DOUT phase."]
    #[inline(always)]
    pub fn sdout_dual(&self) -> SDOUT_DUAL_R {
        SDOUT_DUAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in ADDR phase."]
    #[inline(always)]
    pub fn saddr_dual(&self) -> SADDR_DUAL_R {
        SADDR_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in CMD phase."]
    #[inline(always)]
    pub fn scmd_dual(&self) -> SCMD_DUAL_R {
        SCMD_DUAL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DIN phase."]
    #[inline(always)]
    pub fn sdin_quad(&self) -> SDIN_QUAD_R {
        SDIN_QUAD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DOUT phase."]
    #[inline(always)]
    pub fn sdout_quad(&self) -> SDOUT_QUAD_R {
        SDOUT_QUAD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in ADDR phase."]
    #[inline(always)]
    pub fn saddr_quad(&self) -> SADDR_QUAD_R {
        SADDR_QUAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in CMD phase."]
    #[inline(always)]
    pub fn scmd_quad(&self) -> SCMD_QUAD_R {
        SCMD_QUAD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DIN phase."]
    #[inline(always)]
    pub fn sdin_oct(&self) -> SDIN_OCT_R {
        SDIN_OCT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DOUT phase."]
    #[inline(always)]
    pub fn sdout_oct(&self) -> SDOUT_OCT_R {
        SDOUT_OCT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in ADDR phase."]
    #[inline(always)]
    pub fn saddr_oct(&self) -> SADDR_OCT_R {
        SADDR_OCT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in CMD phase."]
    #[inline(always)]
    pub fn scmd_oct(&self) -> SCMD_OCT_R {
        SCMD_OCT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When SPI0 accesses to Ext_RAM, in the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
    #[inline(always)]
    pub fn sdummy_out(&self) -> SDUMMY_OUT_R {
        SDUMMY_OUT_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CMD")
            .field("sclk_mode", &format_args!("{}", self.sclk_mode().bits()))
            .field("swb_mode", &format_args!("{}", self.swb_mode().bits()))
            .field("sdin_dual", &format_args!("{}", self.sdin_dual().bit()))
            .field("sdout_dual", &format_args!("{}", self.sdout_dual().bit()))
            .field("saddr_dual", &format_args!("{}", self.saddr_dual().bit()))
            .field("scmd_dual", &format_args!("{}", self.scmd_dual().bit()))
            .field("sdin_quad", &format_args!("{}", self.sdin_quad().bit()))
            .field("sdout_quad", &format_args!("{}", self.sdout_quad().bit()))
            .field("saddr_quad", &format_args!("{}", self.saddr_quad().bit()))
            .field("scmd_quad", &format_args!("{}", self.scmd_quad().bit()))
            .field("sdin_oct", &format_args!("{}", self.sdin_oct().bit()))
            .field("sdout_oct", &format_args!("{}", self.sdout_oct().bit()))
            .field("saddr_oct", &format_args!("{}", self.saddr_oct().bit()))
            .field("scmd_oct", &format_args!("{}", self.scmd_oct().bit()))
            .field("sdummy_out", &format_args!("{}", self.sdummy_out().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI_CLK mode bits when SPI0 accesses to Ext_RAM. 0: SPI_CLK is off when CS inactive 1: SPI_CLK is delayed one cycle after CS inactive 2: SPI_CLK is delayed two cycles after CS inactive 3: SPI_CLK is always on."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_mode(&mut self) -> SCLK_MODE_W<0> {
        SCLK_MODE_W::new(self)
    }
    #[doc = "Bits 2:9 - Mode bits when SPI0 accesses to Ext_RAM."]
    #[inline(always)]
    #[must_use]
    pub fn swb_mode(&mut self) -> SWB_MODE_W<2> {
        SWB_MODE_W::new(self)
    }
    #[doc = "Bit 10 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DIN phase."]
    #[inline(always)]
    #[must_use]
    pub fn sdin_dual(&mut self) -> SDIN_DUAL_W<10> {
        SDIN_DUAL_W::new(self)
    }
    #[doc = "Bit 11 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in DOUT phase."]
    #[inline(always)]
    #[must_use]
    pub fn sdout_dual(&mut self) -> SDOUT_DUAL_W<11> {
        SDOUT_DUAL_W::new(self)
    }
    #[doc = "Bit 12 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in ADDR phase."]
    #[inline(always)]
    #[must_use]
    pub fn saddr_dual(&mut self) -> SADDR_DUAL_W<12> {
        SADDR_DUAL_W::new(self)
    }
    #[doc = "Bit 13 - When SPI0 accesses to Ext_RAM, set this bit to enable 2-bm in CMD phase."]
    #[inline(always)]
    #[must_use]
    pub fn scmd_dual(&mut self) -> SCMD_DUAL_W<13> {
        SCMD_DUAL_W::new(self)
    }
    #[doc = "Bit 14 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DIN phase."]
    #[inline(always)]
    #[must_use]
    pub fn sdin_quad(&mut self) -> SDIN_QUAD_W<14> {
        SDIN_QUAD_W::new(self)
    }
    #[doc = "Bit 15 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in DOUT phase."]
    #[inline(always)]
    #[must_use]
    pub fn sdout_quad(&mut self) -> SDOUT_QUAD_W<15> {
        SDOUT_QUAD_W::new(self)
    }
    #[doc = "Bit 16 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in ADDR phase."]
    #[inline(always)]
    #[must_use]
    pub fn saddr_quad(&mut self) -> SADDR_QUAD_W<16> {
        SADDR_QUAD_W::new(self)
    }
    #[doc = "Bit 17 - When SPI0 accesses to Ext_RAM, set this bit to enable 4-bm in CMD phase."]
    #[inline(always)]
    #[must_use]
    pub fn scmd_quad(&mut self) -> SCMD_QUAD_W<17> {
        SCMD_QUAD_W::new(self)
    }
    #[doc = "Bit 18 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DIN phase."]
    #[inline(always)]
    #[must_use]
    pub fn sdin_oct(&mut self) -> SDIN_OCT_W<18> {
        SDIN_OCT_W::new(self)
    }
    #[doc = "Bit 19 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in DOUT phase."]
    #[inline(always)]
    #[must_use]
    pub fn sdout_oct(&mut self) -> SDOUT_OCT_W<19> {
        SDOUT_OCT_W::new(self)
    }
    #[doc = "Bit 20 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in ADDR phase."]
    #[inline(always)]
    #[must_use]
    pub fn saddr_oct(&mut self) -> SADDR_OCT_W<20> {
        SADDR_OCT_W::new(self)
    }
    #[doc = "Bit 21 - When SPI0 accesses to Ext_RAM, set this bit to enable 8-bm in CMD phase."]
    #[inline(always)]
    #[must_use]
    pub fn scmd_oct(&mut self) -> SCMD_OCT_W<21> {
        SCMD_OCT_W::new(self)
    }
    #[doc = "Bit 22 - When SPI0 accesses to Ext_RAM, in the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
    #[inline(always)]
    #[must_use]
    pub fn sdummy_out(&mut self) -> SDUMMY_OUT_W<22> {
        SDUMMY_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 external RAM mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sram_cmd](index.html) module"]
pub struct SRAM_CMD_SPEC;
impl crate::RegisterSpec for SRAM_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sram_cmd::R](R) reader structure"]
impl crate::Readable for SRAM_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sram_cmd::W](W) writer structure"]
impl crate::Writable for SRAM_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_CMD to value 0"]
impl crate::Resettable for SRAM_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
