#[doc = "Register `DOEPDMAB1` reader"]
pub type R = crate::R<DOEPDMAB1_SPEC>;
#[doc = "Register `DOEPDMAB1` writer"]
pub type W = crate::W<DOEPDMAB1_SPEC>;
#[doc = "Field `DMABUFFERADDR1` reader - "]
pub type DMABUFFERADDR1_R = crate::FieldReader<u32>;
#[doc = "Field `DMABUFFERADDR1` writer - "]
pub type DMABUFFERADDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmabufferaddr1(&self) -> DMABUFFERADDR1_R {
        DMABUFFERADDR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMAB1")
            .field(
                "dmabufferaddr1",
                &format_args!("{}", self.dmabufferaddr1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMAB1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmabufferaddr1(&mut self) -> DMABUFFERADDR1_W<DOEPDMAB1_SPEC> {
        DMABUFFERADDR1_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMAB1_SPEC;
impl crate::RegisterSpec for DOEPDMAB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdmab1::R`](R) reader structure"]
impl crate::Readable for DOEPDMAB1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdmab1::W`](W) writer structure"]
impl crate::Writable for DOEPDMAB1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPDMAB1 to value 0"]
impl crate::Resettable for DOEPDMAB1_SPEC {
    const RESET_VALUE: u32 = 0;
}
