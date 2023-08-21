#[doc = "Register `DOEPDMAB2` reader"]
pub type R = crate::R<DOEPDMAB2_SPEC>;
#[doc = "Register `DOEPDMAB2` writer"]
pub type W = crate::W<DOEPDMAB2_SPEC>;
#[doc = "Field `DMABUFFERADDR2` reader - "]
pub type DMABUFFERADDR2_R = crate::FieldReader<u32>;
#[doc = "Field `DMABUFFERADDR2` writer - "]
pub type DMABUFFERADDR2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmabufferaddr2(&self) -> DMABUFFERADDR2_R {
        DMABUFFERADDR2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMAB2")
            .field(
                "dmabufferaddr2",
                &format_args!("{}", self.dmabufferaddr2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMAB2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmabufferaddr2(&mut self) -> DMABUFFERADDR2_W<DOEPDMAB2_SPEC, 0> {
        DMABUFFERADDR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMAB2_SPEC;
impl crate::RegisterSpec for DOEPDMAB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdmab2::R`](R) reader structure"]
impl crate::Readable for DOEPDMAB2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdmab2::W`](W) writer structure"]
impl crate::Writable for DOEPDMAB2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMAB2 to value 0"]
impl crate::Resettable for DOEPDMAB2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
