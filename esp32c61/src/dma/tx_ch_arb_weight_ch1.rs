#[doc = "Register `TX_CH_ARB_WEIGHT_CH1` reader"]
pub type R = crate::R<TX_CH_ARB_WEIGHT_CH1_SPEC>;
#[doc = "Register `TX_CH_ARB_WEIGHT_CH1` writer"]
pub type W = crate::W<TX_CH_ARB_WEIGHT_CH1_SPEC>;
#[doc = "Field `TX_ARB_WEIGHT_VALUE_CH1` reader - Configures the weight(i.e the number of tokens) of TX channel1"]
pub type TX_ARB_WEIGHT_VALUE_CH1_R = crate::FieldReader;
#[doc = "Field `TX_ARB_WEIGHT_VALUE_CH1` writer - Configures the weight(i.e the number of tokens) of TX channel1"]
pub type TX_ARB_WEIGHT_VALUE_CH1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configures the weight(i.e the number of tokens) of TX channel1"]
    #[inline(always)]
    pub fn tx_arb_weight_value_ch1(&self) -> TX_ARB_WEIGHT_VALUE_CH1_R {
        TX_ARB_WEIGHT_VALUE_CH1_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CH_ARB_WEIGHT_CH1")
            .field("tx_arb_weight_value_ch1", &self.tx_arb_weight_value_ch1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures the weight(i.e the number of tokens) of TX channel1"]
    #[inline(always)]
    pub fn tx_arb_weight_value_ch1(
        &mut self,
    ) -> TX_ARB_WEIGHT_VALUE_CH1_W<TX_CH_ARB_WEIGHT_CH1_SPEC> {
        TX_ARB_WEIGHT_VALUE_CH1_W::new(self, 0)
    }
}
#[doc = "TX channel 1 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ch_arb_weight_ch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch_arb_weight_ch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CH_ARB_WEIGHT_CH1_SPEC;
impl crate::RegisterSpec for TX_CH_ARB_WEIGHT_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_ch_arb_weight_ch1::R`](R) reader structure"]
impl crate::Readable for TX_CH_ARB_WEIGHT_CH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_ch_arb_weight_ch1::W`](W) writer structure"]
impl crate::Writable for TX_CH_ARB_WEIGHT_CH1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CH_ARB_WEIGHT_CH1 to value 0"]
impl crate::Resettable for TX_CH_ARB_WEIGHT_CH1_SPEC {}
