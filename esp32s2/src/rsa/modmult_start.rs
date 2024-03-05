#[doc = "Register `MODMULT_START` writer"]
pub type W = crate::W<MODMULT_START_SPEC>;
#[doc = "Field `MODMULT_START` writer - Set this bit to 1 to start the modular multiplication."]
pub type MODMULT_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODMULT_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to start the modular multiplication."]
    #[inline(always)]
    #[must_use]
    pub fn modmult_start(&mut self) -> MODMULT_START_W<MODMULT_START_SPEC> {
        MODMULT_START_W::new(self, 0)
    }
}
#[doc = "Modular multiplication starting bit\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modmult_start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODMULT_START_SPEC;
impl crate::RegisterSpec for MODMULT_START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`modmult_start::W`](W) writer structure"]
impl crate::Writable for MODMULT_START_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODMULT_START to value 0"]
impl crate::Resettable for MODMULT_START_SPEC {
    const RESET_VALUE: u32 = 0;
}
