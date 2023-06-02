#[doc = "Register `SLC_SDIO_ST` reader"]
pub struct R(crate::R<SLC_SDIO_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_SDIO_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_SDIO_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_SDIO_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_SDIO_ST` writer"]
pub struct W(crate::W<SLC_SDIO_ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_SDIO_ST_SPEC>;
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
impl From<crate::W<SLC_SDIO_ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_SDIO_ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_CMD_ST` reader - "]
pub type SLC_CMD_ST_R = crate::FieldReader;
#[doc = "Field `SLC_CMD_ST` writer - "]
pub type SLC_CMD_ST_W<'a, const O: u8> = crate::FieldWriter<'a, SLC_SDIO_ST_SPEC, 3, O>;
#[doc = "Field `SLC_FUNC_ST` reader - "]
pub type SLC_FUNC_ST_R = crate::FieldReader;
#[doc = "Field `SLC_FUNC_ST` writer - "]
pub type SLC_FUNC_ST_W<'a, const O: u8> = crate::FieldWriter<'a, SLC_SDIO_ST_SPEC, 4, O>;
#[doc = "Field `SLC_SDIO_WAKEUP` reader - "]
pub type SLC_SDIO_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLC_SDIO_WAKEUP` writer - "]
pub type SLC_SDIO_WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, SLC_SDIO_ST_SPEC, O>;
#[doc = "Field `SLC_BUS_ST` reader - "]
pub type SLC_BUS_ST_R = crate::FieldReader;
#[doc = "Field `SLC_BUS_ST` writer - "]
pub type SLC_BUS_ST_W<'a, const O: u8> = crate::FieldWriter<'a, SLC_SDIO_ST_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn slc_cmd_st(&self) -> SLC_CMD_ST_R {
        SLC_CMD_ST_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn slc_func_st(&self) -> SLC_FUNC_ST_R {
        SLC_FUNC_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_sdio_wakeup(&self) -> SLC_SDIO_WAKEUP_R {
        SLC_SDIO_WAKEUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn slc_bus_st(&self) -> SLC_BUS_ST_R {
        SLC_BUS_ST_R::new(((self.bits >> 12) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_SDIO_ST")
            .field("slc_bus_st", &format_args!("{}", self.slc_bus_st().bits()))
            .field(
                "slc_sdio_wakeup",
                &format_args!("{}", self.slc_sdio_wakeup().bit()),
            )
            .field(
                "slc_func_st",
                &format_args!("{}", self.slc_func_st().bits()),
            )
            .field("slc_cmd_st", &format_args!("{}", self.slc_cmd_st().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_SDIO_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn slc_cmd_st(&mut self) -> SLC_CMD_ST_W<0> {
        SLC_CMD_ST_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn slc_func_st(&mut self) -> SLC_FUNC_ST_W<4> {
        SLC_FUNC_ST_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn slc_sdio_wakeup(&mut self) -> SLC_SDIO_WAKEUP_W<8> {
        SLC_SDIO_WAKEUP_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn slc_bus_st(&mut self) -> SLC_BUS_ST_W<12> {
        SLC_BUS_ST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLC_SDIO_ST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_sdio_st](index.html) module"]
pub struct SLC_SDIO_ST_SPEC;
impl crate::RegisterSpec for SLC_SDIO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_sdio_st::R](R) reader structure"]
impl crate::Readable for SLC_SDIO_ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_sdio_st::W](W) writer structure"]
impl crate::Writable for SLC_SDIO_ST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_SDIO_ST to value 0"]
impl crate::Resettable for SLC_SDIO_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
