#[doc = "Register `DEBNCE` reader"]
pub type R = crate::R<DEBNCE_SPEC>;
#[doc = "Register `DEBNCE` writer"]
pub type W = crate::W<DEBNCE_SPEC>;
#[doc = "Field `DEBOUNCE_COUNT` reader - Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \\verb+~+ 25 ms to prevent the card instability when the card is inserted or removed."]
pub type DEBOUNCE_COUNT_R = crate::FieldReader<u32>;
#[doc = "Field `DEBOUNCE_COUNT` writer - Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \\verb+~+ 25 ms to prevent the card instability when the card is inserted or removed."]
pub type DEBOUNCE_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \\verb+~+ 25 ms to prevent the card instability when the card is inserted or removed."]
    #[inline(always)]
    pub fn debounce_count(&self) -> DEBOUNCE_COUNT_R {
        DEBOUNCE_COUNT_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBNCE")
            .field("debounce_count", &self.debounce_count())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - Number of host clocks (clk) used by debounce filter logic. The typical debounce time is 5 \\verb+~+ 25 ms to prevent the card instability when the card is inserted or removed."]
    #[inline(always)]
    #[must_use]
    pub fn debounce_count(&mut self) -> DEBOUNCE_COUNT_W<DEBNCE_SPEC> {
        DEBOUNCE_COUNT_W::new(self, 0)
    }
}
#[doc = "Debounce filter time configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debnce::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debnce::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBNCE_SPEC;
impl crate::RegisterSpec for DEBNCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debnce::R`](R) reader structure"]
impl crate::Readable for DEBNCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debnce::W`](W) writer structure"]
impl crate::Writable for DEBNCE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBNCE to value 0"]
impl crate::Resettable for DEBNCE_SPEC {
    const RESET_VALUE: u32 = 0;
}
