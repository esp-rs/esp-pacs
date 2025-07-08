#[doc = "Register `TX_CH_ARB_WEIGH_CH%s` reader"]
pub type R = crate::R<TX_CH_ARB_WEIGH_CH_SPEC>;
#[doc = "Register `TX_CH_ARB_WEIGH_CH%s` writer"]
pub type W = crate::W<TX_CH_ARB_WEIGH_CH_SPEC>;
#[doc = "Field `TX_CH_ARB_WEIGH_CH` reader - Configures the weight(i.e the number of tokens) of TX channel%s"]
pub type TX_CH_ARB_WEIGH_CH_R = crate::FieldReader;
#[doc = "Field `TX_CH_ARB_WEIGH_CH` writer - Configures the weight(i.e the number of tokens) of TX channel%s"]
pub type TX_CH_ARB_WEIGH_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configures the weight(i.e the number of tokens) of TX channel%s"]
    #[inline(always)]
    pub fn tx_ch_arb_weigh_ch(&self) -> TX_CH_ARB_WEIGH_CH_R {
        TX_CH_ARB_WEIGH_CH_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CH_ARB_WEIGH_CH")
            .field("tx_ch_arb_weigh_ch", &self.tx_ch_arb_weigh_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures the weight(i.e the number of tokens) of TX channel%s"]
    #[inline(always)]
    pub fn tx_ch_arb_weigh_ch(&mut self) -> TX_CH_ARB_WEIGH_CH_W<TX_CH_ARB_WEIGH_CH_SPEC> {
        TX_CH_ARB_WEIGH_CH_W::new(self, 0)
    }
}
#[doc = "TX channel %s arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ch_arb_weigh_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch_arb_weigh_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CH_ARB_WEIGH_CH_SPEC;
impl crate::RegisterSpec for TX_CH_ARB_WEIGH_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_ch_arb_weigh_ch::R`](R) reader structure"]
impl crate::Readable for TX_CH_ARB_WEIGH_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_ch_arb_weigh_ch::W`](W) writer structure"]
impl crate::Writable for TX_CH_ARB_WEIGH_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CH_ARB_WEIGH_CH%s to value 0"]
impl crate::Resettable for TX_CH_ARB_WEIGH_CH_SPEC {}
