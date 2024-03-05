#[doc = "Register `IN_PRI_CH%s` reader"]
pub type R = crate::R<IN_PRI_CH_SPEC>;
#[doc = "Register `IN_PRI_CH%s` writer"]
pub type W = crate::W<IN_PRI_CH_SPEC>;
#[doc = "Field `RX_PRI_CH` reader - The priority of Rx channel 0. The larger of the value the higher of the priority."]
pub type RX_PRI_CH_R = crate::FieldReader;
#[doc = "Field `RX_PRI_CH` writer - The priority of Rx channel 0. The larger of the value the higher of the priority."]
pub type RX_PRI_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_CH_ARB_WEIGH_CH` reader - The weight of Rx channel 0"]
pub type RX_CH_ARB_WEIGH_CH_R = crate::FieldReader;
#[doc = "Field `RX_CH_ARB_WEIGH_CH` writer - The weight of Rx channel 0"]
pub type RX_CH_ARB_WEIGH_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_ARB_WEIGH_OPT_DIR_CH` reader - 0: mean not optimazation weight function ,1: mean optimazation"]
pub type RX_ARB_WEIGH_OPT_DIR_CH_R = crate::BitReader;
#[doc = "Field `RX_ARB_WEIGH_OPT_DIR_CH` writer - 0: mean not optimazation weight function ,1: mean optimazation"]
pub type RX_ARB_WEIGH_OPT_DIR_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - The priority of Rx channel 0. The larger of the value the higher of the priority."]
    #[inline(always)]
    pub fn rx_pri_ch(&self) -> RX_PRI_CH_R {
        RX_PRI_CH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The weight of Rx channel 0"]
    #[inline(always)]
    pub fn rx_ch_arb_weigh_ch(&self) -> RX_CH_ARB_WEIGH_CH_R {
        RX_CH_ARB_WEIGH_CH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 0: mean not optimazation weight function ,1: mean optimazation"]
    #[inline(always)]
    pub fn rx_arb_weigh_opt_dir_ch(&self) -> RX_ARB_WEIGH_OPT_DIR_CH_R {
        RX_ARB_WEIGH_OPT_DIR_CH_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_PRI_CH")
            .field("rx_pri_ch", &format_args!("{}", self.rx_pri_ch().bits()))
            .field(
                "rx_ch_arb_weigh_ch",
                &format_args!("{}", self.rx_ch_arb_weigh_ch().bits()),
            )
            .field(
                "rx_arb_weigh_opt_dir_ch",
                &format_args!("{}", self.rx_arb_weigh_opt_dir_ch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_PRI_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - The priority of Rx channel 0. The larger of the value the higher of the priority."]
    #[inline(always)]
    #[must_use]
    pub fn rx_pri_ch(&mut self) -> RX_PRI_CH_W<IN_PRI_CH_SPEC> {
        RX_PRI_CH_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - The weight of Rx channel 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch_arb_weigh_ch(&mut self) -> RX_CH_ARB_WEIGH_CH_W<IN_PRI_CH_SPEC> {
        RX_CH_ARB_WEIGH_CH_W::new(self, 4)
    }
    #[doc = "Bit 8 - 0: mean not optimazation weight function ,1: mean optimazation"]
    #[inline(always)]
    #[must_use]
    pub fn rx_arb_weigh_opt_dir_ch(&mut self) -> RX_ARB_WEIGH_OPT_DIR_CH_W<IN_PRI_CH_SPEC> {
        RX_ARB_WEIGH_OPT_DIR_CH_W::new(self, 8)
    }
}
#[doc = "Priority register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pri_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pri_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_PRI_CH_SPEC;
impl crate::RegisterSpec for IN_PRI_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_pri_ch::R`](R) reader structure"]
impl crate::Readable for IN_PRI_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_pri_ch::W`](W) writer structure"]
impl crate::Writable for IN_PRI_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_PRI_CH%s to value 0"]
impl crate::Resettable for IN_PRI_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
