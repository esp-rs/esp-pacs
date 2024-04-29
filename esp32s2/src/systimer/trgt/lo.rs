#[doc = "Register `LO` reader"]
pub type R = crate::R<LO_SPEC>;
#[doc = "Register `LO` writer"]
pub type W = crate::W<LO_SPEC>;
#[doc = "Field `LO` reader - System timer target 0, low 32 bits."]
pub type LO_R = crate::FieldReader<u32>;
#[doc = "Field `LO` writer - System timer target 0, low 32 bits."]
pub type LO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:31 - System timer target 0, low 32 bits."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LO")
            .field("lo", &format_args!("{}", self.lo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - System timer target 0, low 32 bits."]
    #[inline(always)]
    #[must_use]
    pub fn lo(&mut self) -> LO_W<LO_SPEC> {
        LO_W::new(self, 0)
    }
}
#[doc = "System timer target 0, low 32 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LO_SPEC;
impl crate::RegisterSpec for LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lo::R`](R) reader structure"]
impl crate::Readable for LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lo::W`](W) writer structure"]
impl crate::Writable for LO_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LO to value 0"]
impl crate::Resettable for LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
