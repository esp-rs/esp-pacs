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
            .field(
                "tx_arb_weigh_opt_dir_ch",
                &format_args!("{}", self.tx_arb_weigh_opt_dir_ch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_ARB_WEIGH_OPT_DIR_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tx_arb_weigh_opt_dir_ch(
        &mut self,
    ) -> TX_ARB_WEIGH_OPT_DIR_CH_W<TX_ARB_WEIGH_OPT_DIR_CH_SPEC> {
        TX_ARB_WEIGH_OPT_DIR_CH_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register is used to config off or on weigh optimization\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_arb_weigh_opt_dir_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_arb_weigh_opt_dir_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_ARB_WEIGH_OPT_DIR_CH_SPEC;
impl crate::RegisterSpec for TX_ARB_WEIGH_OPT_DIR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_arb_weigh_opt_dir_ch::R`](R) reader structure"]
impl crate::Readable for TX_ARB_WEIGH_OPT_DIR_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_arb_weigh_opt_dir_ch::W`](W) writer structure"]
impl crate::Writable for TX_ARB_WEIGH_OPT_DIR_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_ARB_WEIGH_OPT_DIR_CH%s to value 0"]
impl crate::Resettable for TX_ARB_WEIGH_OPT_DIR_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
