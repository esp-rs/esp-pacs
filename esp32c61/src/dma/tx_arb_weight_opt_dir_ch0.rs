#[doc = "Register `TX_ARB_WEIGHT_OPT_DIR_CH0` reader"]
pub type R = crate::R<TX_ARB_WEIGHT_OPT_DIR_CH0_SPEC>;
#[doc = "Register `TX_ARB_WEIGHT_OPT_DIR_CH0` writer"]
pub type W = crate::W<TX_ARB_WEIGHT_OPT_DIR_CH0_SPEC>;
#[doc = "Field `TX_ARB_WEIGHT_OPT_DIS_CH0` reader - reserved"]
pub type TX_ARB_WEIGHT_OPT_DIS_CH0_R = crate::BitReader;
#[doc = "Field `TX_ARB_WEIGHT_OPT_DIS_CH0` writer - reserved"]
pub type TX_ARB_WEIGHT_OPT_DIS_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn tx_arb_weight_opt_dis_ch0(&self) -> TX_ARB_WEIGHT_OPT_DIS_CH0_R {
        TX_ARB_WEIGHT_OPT_DIS_CH0_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_ARB_WEIGHT_OPT_DIR_CH0")
            .field(
                "tx_arb_weight_opt_dis_ch0",
                &self.tx_arb_weight_opt_dis_ch0(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn tx_arb_weight_opt_dis_ch0(
        &mut self,
    ) -> TX_ARB_WEIGHT_OPT_DIS_CH0_W<'_, TX_ARB_WEIGHT_OPT_DIR_CH0_SPEC> {
        TX_ARB_WEIGHT_OPT_DIS_CH0_W::new(self, 0)
    }
}
#[doc = "TX channel 0 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_arb_weight_opt_dir_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_arb_weight_opt_dir_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_ARB_WEIGHT_OPT_DIR_CH0_SPEC;
impl crate::RegisterSpec for TX_ARB_WEIGHT_OPT_DIR_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_arb_weight_opt_dir_ch0::R`](R) reader structure"]
impl crate::Readable for TX_ARB_WEIGHT_OPT_DIR_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_arb_weight_opt_dir_ch0::W`](W) writer structure"]
impl crate::Writable for TX_ARB_WEIGHT_OPT_DIR_CH0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_ARB_WEIGHT_OPT_DIR_CH0 to value 0"]
impl crate::Resettable for TX_ARB_WEIGHT_OPT_DIR_CH0_SPEC {}
