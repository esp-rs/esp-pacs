#[doc = "Register `IN_PRI` reader"]
pub type R = crate::R<IN_PRI_SPEC>;
#[doc = "Register `IN_PRI` writer"]
pub type W = crate::W<IN_PRI_SPEC>;
#[doc = "Field `RX_PRI` reader - The priority of Rx channel 0. The larger of the value the higher of the priority."]
pub type RX_PRI_R = crate::FieldReader;
#[doc = "Field `RX_PRI` writer - The priority of Rx channel 0. The larger of the value the higher of the priority."]
pub type RX_PRI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_CH_ARB_WEIGH` reader - The weight of Rx channel 0"]
pub type RX_CH_ARB_WEIGH_R = crate::FieldReader;
#[doc = "Field `RX_CH_ARB_WEIGH` writer - The weight of Rx channel 0"]
pub type RX_CH_ARB_WEIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_ARB_WEIGH_OPT_DIR` reader - 0: mean not optimazation weight function ,1: mean optimazation"]
pub type RX_ARB_WEIGH_OPT_DIR_R = crate::BitReader;
#[doc = "Field `RX_ARB_WEIGH_OPT_DIR` writer - 0: mean not optimazation weight function ,1: mean optimazation"]
pub type RX_ARB_WEIGH_OPT_DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - The priority of Rx channel 0. The larger of the value the higher of the priority."]
    #[inline(always)]
    pub fn rx_pri(&self) -> RX_PRI_R {
        RX_PRI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The weight of Rx channel 0"]
    #[inline(always)]
    pub fn rx_ch_arb_weigh(&self) -> RX_CH_ARB_WEIGH_R {
        RX_CH_ARB_WEIGH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 0: mean not optimazation weight function ,1: mean optimazation"]
    #[inline(always)]
    pub fn rx_arb_weigh_opt_dir(&self) -> RX_ARB_WEIGH_OPT_DIR_R {
        RX_ARB_WEIGH_OPT_DIR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_PRI")
            .field("rx_pri", &self.rx_pri())
            .field("rx_ch_arb_weigh", &self.rx_ch_arb_weigh())
            .field("rx_arb_weigh_opt_dir", &self.rx_arb_weigh_opt_dir())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - The priority of Rx channel 0. The larger of the value the higher of the priority."]
    #[inline(always)]
    pub fn rx_pri(&mut self) -> RX_PRI_W<IN_PRI_SPEC> {
        RX_PRI_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - The weight of Rx channel 0"]
    #[inline(always)]
    pub fn rx_ch_arb_weigh(&mut self) -> RX_CH_ARB_WEIGH_W<IN_PRI_SPEC> {
        RX_CH_ARB_WEIGH_W::new(self, 4)
    }
    #[doc = "Bit 8 - 0: mean not optimazation weight function ,1: mean optimazation"]
    #[inline(always)]
    pub fn rx_arb_weigh_opt_dir(&mut self) -> RX_ARB_WEIGH_OPT_DIR_W<IN_PRI_SPEC> {
        RX_ARB_WEIGH_OPT_DIR_W::new(self, 8)
    }
}
#[doc = "Priority register of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_pri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_pri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_PRI_SPEC;
impl crate::RegisterSpec for IN_PRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_pri::R`](R) reader structure"]
impl crate::Readable for IN_PRI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_pri::W`](W) writer structure"]
impl crate::Writable for IN_PRI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_PRI to value 0"]
impl crate::Resettable for IN_PRI_SPEC {
    const RESET_VALUE: u32 = 0;
}
