#[doc = "Register `OUT_WIGHT` reader"]
pub type R = crate::R<OUT_WIGHT_SPEC>;
#[doc = "Register `OUT_WIGHT` writer"]
pub type W = crate::W<OUT_WIGHT_SPEC>;
#[doc = "Field `TX_WEIGHT` reader - The weight of Tx channel 0."]
pub type TX_WEIGHT_R = crate::FieldReader;
#[doc = "Field `TX_WEIGHT` writer - The weight of Tx channel 0."]
pub type TX_WEIGHT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 8:11 - The weight of Tx channel 0."]
    #[inline(always)]
    pub fn tx_weight(&self) -> TX_WEIGHT_R {
        TX_WEIGHT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_WIGHT")
            .field("tx_weight", &self.tx_weight())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:11 - The weight of Tx channel 0."]
    #[inline(always)]
    pub fn tx_weight(&mut self) -> TX_WEIGHT_W<OUT_WIGHT_SPEC> {
        TX_WEIGHT_W::new(self, 8)
    }
}
#[doc = "Weight register of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_wight::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_wight::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_WIGHT_SPEC;
impl crate::RegisterSpec for OUT_WIGHT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_wight::R`](R) reader structure"]
impl crate::Readable for OUT_WIGHT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_wight::W`](W) writer structure"]
impl crate::Writable for OUT_WIGHT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_WIGHT to value 0x0f00"]
impl crate::Resettable for OUT_WIGHT_SPEC {
    const RESET_VALUE: u32 = 0x0f00;
}
