#[doc = "Register `CLKSRC` reader"]
pub type R = crate::R<CLKSRC_SPEC>;
#[doc = "Register `CLKSRC` writer"]
pub type W = crate::W<CLKSRC_SPEC>;
#[doc = "Field `CLKSRC` reader - Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit\\[1:0\\] are assigned for card 0, bit\\[3:2\\] are assigned for card 1. Card 0 maps and internally routes clock divider\\[0:3\\] outputs to cclk_out\\[1:0\\] pins, depending on bit value. 00 : Clock divider 0; 01 : Clock divider 1; 10 : Clock divider 2; 11 : Clock divider 3."]
pub type CLKSRC_R = crate::FieldReader;
#[doc = "Field `CLKSRC` writer - Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit\\[1:0\\] are assigned for card 0, bit\\[3:2\\] are assigned for card 1. Card 0 maps and internally routes clock divider\\[0:3\\] outputs to cclk_out\\[1:0\\] pins, depending on bit value. 00 : Clock divider 0; 01 : Clock divider 1; 10 : Clock divider 2; 11 : Clock divider 3."]
pub type CLKSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit\\[1:0\\] are assigned for card 0, bit\\[3:2\\] are assigned for card 1. Card 0 maps and internally routes clock divider\\[0:3\\] outputs to cclk_out\\[1:0\\] pins, depending on bit value. 00 : Clock divider 0; 01 : Clock divider 1; 10 : Clock divider 2; 11 : Clock divider 3."]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKSRC")
            .field("clksrc", &self.clksrc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock divider source for two SD cards is supported. Each card has two bits assigned to it. For example, bit\\[1:0\\] are assigned for card 0, bit\\[3:2\\] are assigned for card 1. Card 0 maps and internally routes clock divider\\[0:3\\] outputs to cclk_out\\[1:0\\] pins, depending on bit value. 00 : Clock divider 0; 01 : Clock divider 1; 10 : Clock divider 2; 11 : Clock divider 3."]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W<CLKSRC_SPEC> {
        CLKSRC_W::new(self, 0)
    }
}
#[doc = "Clock source selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`clksrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKSRC_SPEC;
impl crate::RegisterSpec for CLKSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksrc::R`](R) reader structure"]
impl crate::Readable for CLKSRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clksrc::W`](W) writer structure"]
impl crate::Writable for CLKSRC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSRC to value 0"]
impl crate::Resettable for CLKSRC_SPEC {
    const RESET_VALUE: u32 = 0;
}
