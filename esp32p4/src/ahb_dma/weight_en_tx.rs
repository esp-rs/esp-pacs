#[doc = "Register `WEIGHT_EN_TX` reader"]
pub type R = crate::R<WEIGHT_EN_TX_SPEC>;
#[doc = "Register `WEIGHT_EN_TX` writer"]
pub type W = crate::W<WEIGHT_EN_TX_SPEC>;
#[doc = "Field `WEIGHT_EN_TX` reader - This register is used to config arbiter weight function off/on"]
pub type WEIGHT_EN_TX_R = crate::BitReader;
#[doc = "Field `WEIGHT_EN_TX` writer - This register is used to config arbiter weight function off/on"]
pub type WEIGHT_EN_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is used to config arbiter weight function off/on"]
    #[inline(always)]
    pub fn weight_en_tx(&self) -> WEIGHT_EN_TX_R {
        WEIGHT_EN_TX_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WEIGHT_EN_TX")
            .field("weight_en_tx", &self.weight_en_tx())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to config arbiter weight function off/on"]
    #[inline(always)]
    pub fn weight_en_tx(&mut self) -> WEIGHT_EN_TX_W<'_, WEIGHT_EN_TX_SPEC> {
        WEIGHT_EN_TX_W::new(self, 0)
    }
}
#[doc = "This register is used to config arbiter weigh function to on or off for tx dir\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WEIGHT_EN_TX_SPEC;
impl crate::RegisterSpec for WEIGHT_EN_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`weight_en_tx::R`](R) reader structure"]
impl crate::Readable for WEIGHT_EN_TX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`weight_en_tx::W`](W) writer structure"]
impl crate::Writable for WEIGHT_EN_TX_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WEIGHT_EN_TX to value 0"]
impl crate::Resettable for WEIGHT_EN_TX_SPEC {}
