#[doc = "Register `REDUNDANCY_SIG1` reader"]
pub type R = crate::R<REDUNDANCY_SIG1_SPEC>;
#[doc = "Register `REDUNDANCY_SIG1` writer"]
pub type W = crate::W<REDUNDANCY_SIG1_SPEC>;
#[doc = "Field `REDCY_SIG1` reader - Those bits are prepared for ECO."]
pub type REDCY_SIG1_R = crate::FieldReader<u32>;
#[doc = "Field `REDCY_SIG1` writer - Those bits are prepared for ECO."]
pub type REDCY_SIG1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are prepared for ECO."]
    #[inline(always)]
    pub fn redcy_sig1(&self) -> REDCY_SIG1_R {
        REDCY_SIG1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REDUNDANCY_SIG1")
            .field("redcy_sig1", &format_args!("{}", self.redcy_sig1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REDUNDANCY_SIG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are prepared for ECO."]
    #[inline(always)]
    #[must_use]
    pub fn redcy_sig1(&mut self) -> REDCY_SIG1_W<REDUNDANCY_SIG1_SPEC> {
        REDCY_SIG1_W::new(self, 0)
    }
}
#[doc = "Cache redundancy signal 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`redundancy_sig1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redundancy_sig1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REDUNDANCY_SIG1_SPEC;
impl crate::RegisterSpec for REDUNDANCY_SIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`redundancy_sig1::R`](R) reader structure"]
impl crate::Readable for REDUNDANCY_SIG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`redundancy_sig1::W`](W) writer structure"]
impl crate::Writable for REDUNDANCY_SIG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REDUNDANCY_SIG1 to value 0"]
impl crate::Resettable for REDUNDANCY_SIG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
