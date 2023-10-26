#[doc = "Register `DOEPDMAB4` reader"]
pub type R = crate::R<DOEPDMAB4_SPEC>;
#[doc = "Register `DOEPDMAB4` writer"]
pub type W = crate::W<DOEPDMAB4_SPEC>;
#[doc = "Field `DMABUFFERADDR4` reader - "]
pub type DMABUFFERADDR4_R = crate::FieldReader<u32>;
#[doc = "Field `DMABUFFERADDR4` writer - "]
pub type DMABUFFERADDR4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmabufferaddr4(&self) -> DMABUFFERADDR4_R {
        DMABUFFERADDR4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMAB4")
            .field(
                "dmabufferaddr4",
                &format_args!("{}", self.dmabufferaddr4().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMAB4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmabufferaddr4(&mut self) -> DMABUFFERADDR4_W<DOEPDMAB4_SPEC, 0> {
        DMABUFFERADDR4_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepdmab4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepdmab4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMAB4_SPEC;
impl crate::RegisterSpec for DOEPDMAB4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdmab4::R`](R) reader structure"]
impl crate::Readable for DOEPDMAB4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepdmab4::W`](W) writer structure"]
impl crate::Writable for DOEPDMAB4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMAB4 to value 0"]
impl crate::Resettable for DOEPDMAB4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
