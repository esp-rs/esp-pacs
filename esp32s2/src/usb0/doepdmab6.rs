#[doc = "Register `DOEPDMAB6` reader"]
pub type R = crate::R<DOEPDMAB6_SPEC>;
#[doc = "Register `DOEPDMAB6` writer"]
pub type W = crate::W<DOEPDMAB6_SPEC>;
#[doc = "Field `DMABUFFERADDR6` reader - "]
pub type DMABUFFERADDR6_R = crate::FieldReader<u32>;
#[doc = "Field `DMABUFFERADDR6` writer - "]
pub type DMABUFFERADDR6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmabufferaddr6(&self) -> DMABUFFERADDR6_R {
        DMABUFFERADDR6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMAB6")
            .field(
                "dmabufferaddr6",
                &format_args!("{}", self.dmabufferaddr6().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMAB6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmabufferaddr6(&mut self) -> DMABUFFERADDR6_W<DOEPDMAB6_SPEC, 0> {
        DMABUFFERADDR6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMAB6_SPEC;
impl crate::RegisterSpec for DOEPDMAB6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdmab6::R`](R) reader structure"]
impl crate::Readable for DOEPDMAB6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdmab6::W`](W) writer structure"]
impl crate::Writable for DOEPDMAB6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMAB6 to value 0"]
impl crate::Resettable for DOEPDMAB6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
