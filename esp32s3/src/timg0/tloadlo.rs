#[doc = "Register `T%sLOADLO` reader"]
pub type R = crate::R<TLOADLO_SPEC>;
#[doc = "Register `T%sLOADLO` writer"]
pub type W = crate::W<TLOADLO_SPEC>;
#[doc = "Field `LOAD_LO` reader - Low 32 bits of the value that a reload will load onto timer %s time-base Counter."]
pub type LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `LOAD_LO` writer - Low 32 bits of the value that a reload will load onto timer %s time-base Counter."]
pub type LOAD_LO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Low 32 bits of the value that a reload will load onto timer %s time-base Counter."]
    #[inline(always)]
    pub fn load_lo(&self) -> LOAD_LO_R {
        LOAD_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TLOADLO")
            .field("load_lo", &format_args!("{}", self.load_lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TLOADLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Low 32 bits of the value that a reload will load onto timer %s time-base Counter."]
    #[inline(always)]
    #[must_use]
    pub fn load_lo(&mut self) -> LOAD_LO_W<TLOADLO_SPEC, 0> {
        LOAD_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer %s reload value, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tloadlo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tloadlo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TLOADLO_SPEC;
impl crate::RegisterSpec for TLOADLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tloadlo::R`](R) reader structure"]
impl crate::Readable for TLOADLO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tloadlo::W`](W) writer structure"]
impl crate::Writable for TLOADLO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T%sLOADLO to value 0"]
impl crate::Resettable for TLOADLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
