#[doc = "Register `RST_N` reader"]
pub type R = crate::R<RST_N_SPEC>;
#[doc = "Register `RST_N` writer"]
pub type W = crate::W<RST_N_SPEC>;
#[doc = "Field `CARD_RESET` reader - Hardware reset. 1: Active mode; 0: Reset. These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. SDHOST_RST_CARD_RESET\\[0\\] should be set to 1'b0 to reset card0, SDHOST_RST_CARD_RESET\\[1\\] should be set to 1'b0 to reset card1."]
pub type CARD_RESET_R = crate::FieldReader;
#[doc = "Field `CARD_RESET` writer - Hardware reset. 1: Active mode; 0: Reset. These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. SDHOST_RST_CARD_RESET\\[0\\] should be set to 1'b0 to reset card0, SDHOST_RST_CARD_RESET\\[1\\] should be set to 1'b0 to reset card1."]
pub type CARD_RESET_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Hardware reset. 1: Active mode; 0: Reset. These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. SDHOST_RST_CARD_RESET\\[0\\] should be set to 1'b0 to reset card0, SDHOST_RST_CARD_RESET\\[1\\] should be set to 1'b0 to reset card1."]
    #[inline(always)]
    pub fn card_reset(&self) -> CARD_RESET_R {
        CARD_RESET_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RST_N")
            .field("card_reset", &self.card_reset())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Hardware reset. 1: Active mode; 0: Reset. These bits cause the cards to enter pre-idle state, which requires them to be re-initialized. SDHOST_RST_CARD_RESET\\[0\\] should be set to 1'b0 to reset card0, SDHOST_RST_CARD_RESET\\[1\\] should be set to 1'b0 to reset card1."]
    #[inline(always)]
    pub fn card_reset(&mut self) -> CARD_RESET_W<RST_N_SPEC> {
        CARD_RESET_W::new(self, 0)
    }
}
#[doc = "Card reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_n::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_n::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST_N_SPEC;
impl crate::RegisterSpec for RST_N_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_n::R`](R) reader structure"]
impl crate::Readable for RST_N_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rst_n::W`](W) writer structure"]
impl crate::Writable for RST_N_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RST_N to value 0x01"]
impl crate::Resettable for RST_N_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
