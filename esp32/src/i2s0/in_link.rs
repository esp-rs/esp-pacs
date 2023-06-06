#[doc = "Register `IN_LINK` reader"]
pub struct R(crate::R<IN_LINK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_LINK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_LINK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_LINK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_LINK` writer"]
pub struct W(crate::W<IN_LINK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_LINK_SPEC>;
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
impl From<crate::W<IN_LINK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_LINK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INLINK_ADDR` reader - "]
pub type INLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `INLINK_ADDR` writer - "]
pub type INLINK_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, IN_LINK_SPEC, 20, O, u32>;
#[doc = "Field `INLINK_STOP` reader - "]
pub type INLINK_STOP_R = crate::BitReader;
#[doc = "Field `INLINK_STOP` writer - "]
pub type INLINK_STOP_W<'a, const O: u8> = crate::BitWriter<'a, IN_LINK_SPEC, O>;
#[doc = "Field `INLINK_START` reader - "]
pub type INLINK_START_R = crate::BitReader;
#[doc = "Field `INLINK_START` writer - "]
pub type INLINK_START_W<'a, const O: u8> = crate::BitWriter<'a, IN_LINK_SPEC, O>;
#[doc = "Field `INLINK_RESTART` reader - "]
pub type INLINK_RESTART_R = crate::BitReader;
#[doc = "Field `INLINK_RESTART` writer - "]
pub type INLINK_RESTART_W<'a, const O: u8> = crate::BitWriter<'a, IN_LINK_SPEC, O>;
#[doc = "Field `INLINK_PARK` reader - "]
pub type INLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn inlink_addr(&self) -> INLINK_ADDR_R {
        INLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn inlink_stop(&self) -> INLINK_STOP_R {
        INLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn inlink_start(&self) -> INLINK_START_R {
        INLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn inlink_restart(&self) -> INLINK_RESTART_R {
        INLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn inlink_park(&self) -> INLINK_PARK_R {
        INLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_LINK")
            .field(
                "inlink_addr",
                &format_args!("{}", self.inlink_addr().bits()),
            )
            .field("inlink_stop", &format_args!("{}", self.inlink_stop().bit()))
            .field(
                "inlink_start",
                &format_args!("{}", self.inlink_start().bit()),
            )
            .field(
                "inlink_restart",
                &format_args!("{}", self.inlink_restart().bit()),
            )
            .field("inlink_park", &format_args!("{}", self.inlink_park().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_LINK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn inlink_addr(&mut self) -> INLINK_ADDR_W<0> {
        INLINK_ADDR_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn inlink_stop(&mut self) -> INLINK_STOP_W<28> {
        INLINK_STOP_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn inlink_start(&mut self) -> INLINK_START_W<29> {
        INLINK_START_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn inlink_restart(&mut self) -> INLINK_RESTART_W<30> {
        INLINK_RESTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_link](index.html) module"]
pub struct IN_LINK_SPEC;
impl crate::RegisterSpec for IN_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_link::R](R) reader structure"]
impl crate::Readable for IN_LINK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_link::W](W) writer structure"]
impl crate::Writable for IN_LINK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_LINK to value 0"]
impl crate::Resettable for IN_LINK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
