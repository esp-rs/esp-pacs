#[doc = "Register `RX_ARB_WEIGH_OPT_DIR` reader"]
pub type R = crate::R<RX_ARB_WEIGH_OPT_DIR_SPEC>;
#[doc = "Register `RX_ARB_WEIGH_OPT_DIR` writer"]
pub type W = crate::W<RX_ARB_WEIGH_OPT_DIR_SPEC>;
#[doc = "Field `RX_ARB_WEIGH_OPT_DIR` reader - reserved"]
pub type RX_ARB_WEIGH_OPT_DIR_R = crate::BitReader;
#[doc = "Field `RX_ARB_WEIGH_OPT_DIR` writer - reserved"]
pub type RX_ARB_WEIGH_OPT_DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn rx_arb_weigh_opt_dir(&self) -> RX_ARB_WEIGH_OPT_DIR_R {
        RX_ARB_WEIGH_OPT_DIR_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ARB_WEIGH_OPT_DIR")
            .field(
                "rx_arb_weigh_opt_dir",
                &format_args!("{}", self.rx_arb_weigh_opt_dir().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_ARB_WEIGH_OPT_DIR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rx_arb_weigh_opt_dir(&mut self) -> RX_ARB_WEIGH_OPT_DIR_W<RX_ARB_WEIGH_OPT_DIR_SPEC> {
        RX_ARB_WEIGH_OPT_DIR_W::new(self, 0)
    }
}
#[doc = "This register is used to config off or on weigh optimization\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_arb_weigh_opt_dir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_arb_weigh_opt_dir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_ARB_WEIGH_OPT_DIR_SPEC;
impl crate::RegisterSpec for RX_ARB_WEIGH_OPT_DIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_arb_weigh_opt_dir::R`](R) reader structure"]
impl crate::Readable for RX_ARB_WEIGH_OPT_DIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_arb_weigh_opt_dir::W`](W) writer structure"]
impl crate::Writable for RX_ARB_WEIGH_OPT_DIR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_ARB_WEIGH_OPT_DIR to value 0"]
impl crate::Resettable for RX_ARB_WEIGH_OPT_DIR_SPEC {
    const RESET_VALUE: u32 = 0;
}
