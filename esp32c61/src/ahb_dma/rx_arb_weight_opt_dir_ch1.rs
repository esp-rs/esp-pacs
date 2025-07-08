#[doc = "Register `RX_ARB_WEIGHT_OPT_DIR_CH1` reader"]
pub type R = crate::R<RX_ARB_WEIGHT_OPT_DIR_CH1_SPEC>;
#[doc = "Register `RX_ARB_WEIGHT_OPT_DIR_CH1` writer"]
pub type W = crate::W<RX_ARB_WEIGHT_OPT_DIR_CH1_SPEC>;
#[doc = "Field `RX_ARB_WEIGHT_OPT_DIS_CH1` reader - reserved"]
pub type RX_ARB_WEIGHT_OPT_DIS_CH1_R = crate::BitReader;
#[doc = "Field `RX_ARB_WEIGHT_OPT_DIS_CH1` writer - reserved"]
pub type RX_ARB_WEIGHT_OPT_DIS_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn rx_arb_weight_opt_dis_ch1(&self) -> RX_ARB_WEIGHT_OPT_DIS_CH1_R {
        RX_ARB_WEIGHT_OPT_DIS_CH1_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ARB_WEIGHT_OPT_DIR_CH1")
            .field(
                "rx_arb_weight_opt_dis_ch1",
                &self.rx_arb_weight_opt_dis_ch1(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn rx_arb_weight_opt_dis_ch1(
        &mut self,
    ) -> RX_ARB_WEIGHT_OPT_DIS_CH1_W<RX_ARB_WEIGHT_OPT_DIR_CH1_SPEC> {
        RX_ARB_WEIGHT_OPT_DIS_CH1_W::new(self, 0)
    }
}
#[doc = "RX channel 1 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_arb_weight_opt_dir_ch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_arb_weight_opt_dir_ch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_ARB_WEIGHT_OPT_DIR_CH1_SPEC;
impl crate::RegisterSpec for RX_ARB_WEIGHT_OPT_DIR_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_arb_weight_opt_dir_ch1::R`](R) reader structure"]
impl crate::Readable for RX_ARB_WEIGHT_OPT_DIR_CH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_arb_weight_opt_dir_ch1::W`](W) writer structure"]
impl crate::Writable for RX_ARB_WEIGHT_OPT_DIR_CH1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_ARB_WEIGHT_OPT_DIR_CH1 to value 0"]
impl crate::Resettable for RX_ARB_WEIGHT_OPT_DIR_CH1_SPEC {}
