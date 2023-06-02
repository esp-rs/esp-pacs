#[doc = "Register `OUT_LINK` reader"]
pub struct R(crate::R<OUT_LINK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_LINK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_LINK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_LINK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_LINK` writer"]
pub struct W(crate::W<OUT_LINK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_LINK_SPEC>;
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
impl From<crate::W<OUT_LINK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_LINK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTLINK_ADDR` reader - "]
pub type OUTLINK_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OUTLINK_ADDR` writer - "]
pub type OUTLINK_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, OUT_LINK_SPEC, 20, O, u32, u32>;
#[doc = "Field `OUTLINK_STOP` reader - "]
pub type OUTLINK_STOP_R = crate::BitReader;
#[doc = "Field `OUTLINK_STOP` writer - "]
pub type OUTLINK_STOP_W<'a, const O: u8> = crate::BitWriter<'a, OUT_LINK_SPEC, O>;
#[doc = "Field `OUTLINK_START` reader - "]
pub type OUTLINK_START_R = crate::BitReader;
#[doc = "Field `OUTLINK_START` writer - "]
pub type OUTLINK_START_W<'a, const O: u8> = crate::BitWriter<'a, OUT_LINK_SPEC, O>;
#[doc = "Field `OUTLINK_RESTART` reader - "]
pub type OUTLINK_RESTART_R = crate::BitReader;
#[doc = "Field `OUTLINK_RESTART` writer - "]
pub type OUTLINK_RESTART_W<'a, const O: u8> = crate::BitWriter<'a, OUT_LINK_SPEC, O>;
#[doc = "Field `OUTLINK_PARK` reader - "]
pub type OUTLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn outlink_addr(&self) -> OUTLINK_ADDR_R {
        OUTLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn outlink_stop(&self) -> OUTLINK_STOP_R {
        OUTLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn outlink_start(&self) -> OUTLINK_START_R {
        OUTLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn outlink_restart(&self) -> OUTLINK_RESTART_R {
        OUTLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn outlink_park(&self) -> OUTLINK_PARK_R {
        OUTLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK")
            .field(
                "outlink_addr",
                &format_args!("{}", self.outlink_addr().bits()),
            )
            .field(
                "outlink_stop",
                &format_args!("{}", self.outlink_stop().bit()),
            )
            .field(
                "outlink_start",
                &format_args!("{}", self.outlink_start().bit()),
            )
            .field(
                "outlink_restart",
                &format_args!("{}", self.outlink_restart().bit()),
            )
            .field(
                "outlink_park",
                &format_args!("{}", self.outlink_park().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_LINK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn outlink_addr(&mut self) -> OUTLINK_ADDR_W<0> {
        OUTLINK_ADDR_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn outlink_stop(&mut self) -> OUTLINK_STOP_W<28> {
        OUTLINK_STOP_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn outlink_start(&mut self) -> OUTLINK_START_W<29> {
        OUTLINK_START_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn outlink_restart(&mut self) -> OUTLINK_RESTART_W<30> {
        OUTLINK_RESTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_link](index.html) module"]
pub struct OUT_LINK_SPEC;
impl crate::RegisterSpec for OUT_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_link::R](R) reader structure"]
impl crate::Readable for OUT_LINK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_link::W](W) writer structure"]
impl crate::Writable for OUT_LINK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_LINK to value 0"]
impl crate::Resettable for OUT_LINK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
