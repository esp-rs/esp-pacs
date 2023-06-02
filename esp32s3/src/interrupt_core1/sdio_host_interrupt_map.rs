#[doc = "Register `SDIO_HOST_INTERRUPT_MAP` reader"]
pub struct R(crate::R<SDIO_HOST_INTERRUPT_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_HOST_INTERRUPT_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_HOST_INTERRUPT_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_HOST_INTERRUPT_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_HOST_INTERRUPT_MAP` writer"]
pub struct W(crate::W<SDIO_HOST_INTERRUPT_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_HOST_INTERRUPT_MAP_SPEC>;
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
impl From<crate::W<SDIO_HOST_INTERRUPT_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_HOST_INTERRUPT_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_HOST_INTERRUPT_MAP` reader - this register used to map sdio_host interrupt to one of core1's external interrupt"]
pub type SDIO_HOST_INTERRUPT_MAP_R = crate::FieldReader;
#[doc = "Field `SDIO_HOST_INTERRUPT_MAP` writer - this register used to map sdio_host interrupt to one of core1's external interrupt"]
pub type SDIO_HOST_INTERRUPT_MAP_W<'a, const O: u8> =
    crate::FieldWriter<'a, SDIO_HOST_INTERRUPT_MAP_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - this register used to map sdio_host interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn sdio_host_interrupt_map(&self) -> SDIO_HOST_INTERRUPT_MAP_R {
        SDIO_HOST_INTERRUPT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_HOST_INTERRUPT_MAP")
            .field(
                "sdio_host_interrupt_map",
                &format_args!("{}", self.sdio_host_interrupt_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_HOST_INTERRUPT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map sdio_host interrupt to one of core1's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_host_interrupt_map(&mut self) -> SDIO_HOST_INTERRUPT_MAP_W<0> {
        SDIO_HOST_INTERRUPT_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdio_host interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_host_interrupt_map](index.html) module"]
pub struct SDIO_HOST_INTERRUPT_MAP_SPEC;
impl crate::RegisterSpec for SDIO_HOST_INTERRUPT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_host_interrupt_map::R](R) reader structure"]
impl crate::Readable for SDIO_HOST_INTERRUPT_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_host_interrupt_map::W](W) writer structure"]
impl crate::Writable for SDIO_HOST_INTERRUPT_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_HOST_INTERRUPT_MAP to value 0x10"]
impl crate::Resettable for SDIO_HOST_INTERRUPT_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
