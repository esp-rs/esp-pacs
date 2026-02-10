#[doc = "Register `RX_ARB_WEIGHT_OPT_DIS_CH%s` reader"]
pub type R = crate::R<RX_ARB_WEIGHT_OPT_DIS_CH_SPEC>;
#[doc = "Register `RX_ARB_WEIGHT_OPT_DIS_CH%s` writer"]
pub type W = crate::W<RX_ARB_WEIGHT_OPT_DIS_CH_SPEC>;
#[doc = "Field `RX_ARB_WEIGHT_OPT_DIS_CH` reader - reserved"]
pub type RX_ARB_WEIGHT_OPT_DIS_CH_R = crate::BitReader;
#[doc = "Field `RX_ARB_WEIGHT_OPT_DIS_CH` writer - reserved"]
pub type RX_ARB_WEIGHT_OPT_DIS_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn rx_arb_weight_opt_dis_ch(&self) -> RX_ARB_WEIGHT_OPT_DIS_CH_R {
        RX_ARB_WEIGHT_OPT_DIS_CH_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ARB_WEIGHT_OPT_DIS_CH")
            .field("rx_arb_weight_opt_dis_ch", &self.rx_arb_weight_opt_dis_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn rx_arb_weight_opt_dis_ch(
        &mut self,
    ) -> RX_ARB_WEIGHT_OPT_DIS_CH_W<'_, RX_ARB_WEIGHT_OPT_DIS_CH_SPEC> {
        RX_ARB_WEIGHT_OPT_DIS_CH_W::new(self, 0)
    }
}
#[doc = "RX channel %s weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_arb_weight_opt_dis_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_arb_weight_opt_dis_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_ARB_WEIGHT_OPT_DIS_CH_SPEC;
impl crate::RegisterSpec for RX_ARB_WEIGHT_OPT_DIS_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_arb_weight_opt_dis_ch::R`](R) reader structure"]
impl crate::Readable for RX_ARB_WEIGHT_OPT_DIS_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_arb_weight_opt_dis_ch::W`](W) writer structure"]
impl crate::Writable for RX_ARB_WEIGHT_OPT_DIS_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_ARB_WEIGHT_OPT_DIS_CH%s to value 0"]
impl crate::Resettable for RX_ARB_WEIGHT_OPT_DIS_CH_SPEC {}
