#[doc = "Register `DOEPDMAB` reader"]
pub type R = crate::R<DOEPDMAB_SPEC>;
#[doc = "Register `DOEPDMAB` writer"]
pub type W = crate::W<DOEPDMAB_SPEC>;
#[doc = "Field `DMABUFFERADDR` reader - "]
pub type DMABUFFERADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMABUFFERADDR` writer - "]
pub type DMABUFFERADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmabufferaddr(&self) -> DMABUFFERADDR_R {
        DMABUFFERADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMAB")
            .field("dmabufferaddr", &self.dmabufferaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmabufferaddr(&mut self) -> DMABUFFERADDR_W<DOEPDMAB_SPEC> {
        DMABUFFERADDR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMAB_SPEC;
impl crate::RegisterSpec for DOEPDMAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdmab::R`](R) reader structure"]
impl crate::Readable for DOEPDMAB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdmab::W`](W) writer structure"]
impl crate::Writable for DOEPDMAB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPDMAB to value 0"]
impl crate::Resettable for DOEPDMAB_SPEC {
    const RESET_VALUE: u32 = 0;
}
