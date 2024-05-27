///Register `TX_CH_ARB_WEIGH` reader
pub type R = crate::R<TX_CH_ARB_WEIGH_SPEC>;
///Register `TX_CH_ARB_WEIGH` writer
pub type W = crate::W<TX_CH_ARB_WEIGH_SPEC>;
///Field `TX_CH_ARB_WEIGH` reader - reserved
pub type TX_CH_ARB_WEIGH_R = crate::FieldReader;
///Field `TX_CH_ARB_WEIGH` writer - reserved
pub type TX_CH_ARB_WEIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - reserved
    #[inline(always)]
    pub fn tx_ch_arb_weigh(&self) -> TX_CH_ARB_WEIGH_R {
        TX_CH_ARB_WEIGH_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CH_ARB_WEIGH")
            .field("tx_ch_arb_weigh", &self.tx_ch_arb_weigh())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - reserved
    #[inline(always)]
    #[must_use]
    pub fn tx_ch_arb_weigh(&mut self) -> TX_CH_ARB_WEIGH_W<TX_CH_ARB_WEIGH_SPEC> {
        TX_CH_ARB_WEIGH_W::new(self, 0)
    }
}
/**This register is used to config ch0 arbiter weigh

You can [`read`](crate::generic::Reg::read) this register and get [`tx_ch_arb_weigh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ch_arb_weigh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TX_CH_ARB_WEIGH_SPEC;
impl crate::RegisterSpec for TX_CH_ARB_WEIGH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tx_ch_arb_weigh::R`](R) reader structure
impl crate::Readable for TX_CH_ARB_WEIGH_SPEC {}
///`write(|w| ..)` method takes [`tx_ch_arb_weigh::W`](W) writer structure
impl crate::Writable for TX_CH_ARB_WEIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_CH_ARB_WEIGH to value 0
impl crate::Resettable for TX_CH_ARB_WEIGH_SPEC {
    const RESET_VALUE: u32 = 0;
}
