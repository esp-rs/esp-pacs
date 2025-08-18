#[doc = "Register `TX_ARB_WEIGH_OPT_DIR_CH%s` reader"]
pub type R = crate::R<TX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "Register `TX_ARB_WEIGH_OPT_DIR_CH%s` writer"]
pub type W = crate::W<TX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "Field `TX_ARB_WEIGH_OPT_DIR_CH` reader - reserved"]
pub type TX_ARB_WEIGH_OPT_DIR_CH_R = crate::BitReader;
#[doc = "Field `TX_ARB_WEIGH_OPT_DIR_CH` writer - reserved"]
pub type TX_ARB_WEIGH_OPT_DIR_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn tx_arb_weigh_opt_dir_ch(&self) -> TX_ARB_WEIGH_OPT_DIR_CH_R {
        TX_ARB_WEIGH_OPT_DIR_CH_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_ARB_WEIGH_OPT_DIR_CH")
            .field("tx_arb_weigh_opt_dir_ch", &self.tx_arb_weigh_opt_dir_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn tx_arb_weigh_opt_dir_ch(
        &mut self,
    ) -> TX_ARB_WEIGH_OPT_DIR_CH_W<'_, TX_ARB_WEIGH_OPT_DIR_CH_SPEC> {
        TX_ARB_WEIGH_OPT_DIR_CH_W::new(self, 0)
    }
}
#[doc = "TX channel %s weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_arb_weigh_opt_dir_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_arb_weigh_opt_dir_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_ARB_WEIGH_OPT_DIR_CH_SPEC;
impl crate::RegisterSpec for TX_ARB_WEIGH_OPT_DIR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_arb_weigh_opt_dir_ch::R`](R) reader structure"]
impl crate::Readable for TX_ARB_WEIGH_OPT_DIR_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_arb_weigh_opt_dir_ch::W`](W) writer structure"]
impl crate::Writable for TX_ARB_WEIGH_OPT_DIR_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_ARB_WEIGH_OPT_DIR_CH%s to value 0"]
impl crate::Resettable for TX_ARB_WEIGH_OPT_DIR_CH_SPEC {}
