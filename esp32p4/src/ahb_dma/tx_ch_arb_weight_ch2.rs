#[doc = "Register `TX_CH_ARB_WEIGHT_CH2` reader"]
pub type R = crate::R<TX_CH_ARB_WEIGHT_CH2_SPEC>;
#[doc = "Register `TX_CH_ARB_WEIGHT_CH2` writer"]
pub type W = crate::W<TX_CH_ARB_WEIGHT_CH2_SPEC>;
#[doc = "Field `TX_ARB_WEIGHT_VALUE_CH2` reader - Configures the weight(i.e the number of tokens) of TX channel2"]
pub type TX_ARB_WEIGHT_VALUE_CH2_R = crate::FieldReader;
#[doc = "Field `TX_ARB_WEIGHT_VALUE_CH2` writer - Configures the weight(i.e the number of tokens) of TX channel2"]
pub type TX_ARB_WEIGHT_VALUE_CH2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configures the weight(i.e the number of tokens) of TX channel2"]
    #[inline(always)]
    pub fn tx_arb_weight_value_ch2(&self) -> TX_ARB_WEIGHT_VALUE_CH2_R {
        TX_ARB_WEIGHT_VALUE_CH2_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CH_ARB_WEIGHT_CH2")
            .field("tx_arb_weight_value_ch2", &self.tx_arb_weight_value_ch2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures the weight(i.e the number of tokens) of TX channel2"]
    #[inline(always)]
    pub fn tx_arb_weight_value_ch2(
        &mut self,
    ) -> TX_ARB_WEIGHT_VALUE_CH2_W<'_, TX_CH_ARB_WEIGHT_CH2_SPEC> {
        TX_ARB_WEIGHT_VALUE_CH2_W::new(self, 0)
    }
}
#[doc = "TX channel 2 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ch_arb_weight_ch2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch_arb_weight_ch2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CH_ARB_WEIGHT_CH2_SPEC;
impl crate::RegisterSpec for TX_CH_ARB_WEIGHT_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_ch_arb_weight_ch2::R`](R) reader structure"]
impl crate::Readable for TX_CH_ARB_WEIGHT_CH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_ch_arb_weight_ch2::W`](W) writer structure"]
impl crate::Writable for TX_CH_ARB_WEIGHT_CH2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CH_ARB_WEIGHT_CH2 to value 0"]
impl crate::Resettable for TX_CH_ARB_WEIGHT_CH2_SPEC {}
