#[doc = "Register `EXT_WAKEUP0` reader"]
pub type R = crate::R<EXT_WAKEUP0_SPEC>;
#[doc = "Register `EXT_WAKEUP0` writer"]
pub type W = crate::W<EXT_WAKEUP0_SPEC>;
#[doc = "Field `SEL` reader - GPIO\\[0-17\\] can be used to wake up the chip when the chip is in the sleep mode. This register prompts the pad source to wake up the chip when the latter is indeep/light sleep mode. 0: select GPIO0; 1: select GPIO2, etc"]
pub type SEL_R = crate::FieldReader;
#[doc = "Field `SEL` writer - GPIO\\[0-17\\] can be used to wake up the chip when the chip is in the sleep mode. This register prompts the pad source to wake up the chip when the latter is indeep/light sleep mode. 0: select GPIO0; 1: select GPIO2, etc"]
pub type SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 27:31 - GPIO\\[0-17\\] can be used to wake up the chip when the chip is in the sleep mode. This register prompts the pad source to wake up the chip when the latter is indeep/light sleep mode. 0: select GPIO0; 1: select GPIO2, etc"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP0")
            .field("sel", &self.sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 27:31 - GPIO\\[0-17\\] can be used to wake up the chip when the chip is in the sleep mode. This register prompts the pad source to wake up the chip when the latter is indeep/light sleep mode. 0: select GPIO0; 1: select GPIO2, etc"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<'_, EXT_WAKEUP0_SPEC> {
        SEL_W::new(self, 27)
    }
}
#[doc = "External wake up configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP0_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup0::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup0::W`](W) writer structure"]
impl crate::Writable for EXT_WAKEUP0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_WAKEUP0 to value 0"]
impl crate::Resettable for EXT_WAKEUP0_SPEC {}
