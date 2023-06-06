#[doc = "Register `SLC_APBWIN_CONF` reader"]
pub struct R(crate::R<SLC_APBWIN_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_APBWIN_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_APBWIN_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_APBWIN_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_APBWIN_CONF` writer"]
pub struct W(crate::W<SLC_APBWIN_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_APBWIN_CONF_SPEC>;
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
impl From<crate::W<SLC_APBWIN_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_APBWIN_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_APBWIN_ADDR` reader - *******Description***********"]
pub type SLC_APBWIN_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SLC_APBWIN_ADDR` writer - *******Description***********"]
pub type SLC_APBWIN_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, SLC_APBWIN_CONF_SPEC, 28, O, u32>;
#[doc = "Field `SLC_APBWIN_WR` reader - *******Description***********"]
pub type SLC_APBWIN_WR_R = crate::BitReader;
#[doc = "Field `SLC_APBWIN_WR` writer - *******Description***********"]
pub type SLC_APBWIN_WR_W<'a, const O: u8> = crate::BitWriter<'a, SLC_APBWIN_CONF_SPEC, O>;
#[doc = "Field `SLC_APBWIN_START` reader - *******Description***********"]
pub type SLC_APBWIN_START_R = crate::BitReader;
#[doc = "Field `SLC_APBWIN_START` writer - *******Description***********"]
pub type SLC_APBWIN_START_W<'a, const O: u8> = crate::BitWriter<'a, SLC_APBWIN_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:27 - *******Description***********"]
    #[inline(always)]
    pub fn slc_apbwin_addr(&self) -> SLC_APBWIN_ADDR_R {
        SLC_APBWIN_ADDR_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28 - *******Description***********"]
    #[inline(always)]
    pub fn slc_apbwin_wr(&self) -> SLC_APBWIN_WR_R {
        SLC_APBWIN_WR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - *******Description***********"]
    #[inline(always)]
    pub fn slc_apbwin_start(&self) -> SLC_APBWIN_START_R {
        SLC_APBWIN_START_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_APBWIN_CONF")
            .field(
                "slc_apbwin_addr",
                &format_args!("{}", self.slc_apbwin_addr().bits()),
            )
            .field(
                "slc_apbwin_wr",
                &format_args!("{}", self.slc_apbwin_wr().bit()),
            )
            .field(
                "slc_apbwin_start",
                &format_args!("{}", self.slc_apbwin_start().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_APBWIN_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:27 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc_apbwin_addr(&mut self) -> SLC_APBWIN_ADDR_W<0> {
        SLC_APBWIN_ADDR_W::new(self)
    }
    #[doc = "Bit 28 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc_apbwin_wr(&mut self) -> SLC_APBWIN_WR_W<28> {
        SLC_APBWIN_WR_W::new(self)
    }
    #[doc = "Bit 29 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc_apbwin_start(&mut self) -> SLC_APBWIN_START_W<29> {
        SLC_APBWIN_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_apbwin_conf](index.html) module"]
pub struct SLC_APBWIN_CONF_SPEC;
impl crate::RegisterSpec for SLC_APBWIN_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_apbwin_conf::R](R) reader structure"]
impl crate::Readable for SLC_APBWIN_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_apbwin_conf::W](W) writer structure"]
impl crate::Writable for SLC_APBWIN_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_APBWIN_CONF to value 0"]
impl crate::Resettable for SLC_APBWIN_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
