#[doc = "Register `OUT_PRI` reader"]
pub type R = crate::R<OUT_PRI_SPEC>;
#[doc = "Register `OUT_PRI` writer"]
pub type W = crate::W<OUT_PRI_SPEC>;
#[doc = "Field `TX_PRI` reader - The priority of Tx channel0. The larger of the value the higher of the priority."]
pub type TX_PRI_R = crate::FieldReader;
#[doc = "Field `TX_PRI` writer - The priority of Tx channel0. The larger of the value the higher of the priority."]
pub type TX_PRI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_CH_ARB_WEIGH` reader - The weight of Tx channel0"]
pub type TX_CH_ARB_WEIGH_R = crate::FieldReader;
#[doc = "Field `TX_CH_ARB_WEIGH` writer - The weight of Tx channel0"]
pub type TX_CH_ARB_WEIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_ARB_WEIGH_OPT_DIR` reader - 0: mean not optimazation weight function ,1: mean optimazation"]
pub type TX_ARB_WEIGH_OPT_DIR_R = crate::BitReader;
#[doc = "Field `TX_ARB_WEIGH_OPT_DIR` writer - 0: mean not optimazation weight function ,1: mean optimazation"]
pub type TX_ARB_WEIGH_OPT_DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - The priority of Tx channel0. The larger of the value the higher of the priority."]
    #[inline(always)]
    pub fn tx_pri(&self) -> TX_PRI_R {
        TX_PRI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The weight of Tx channel0"]
    #[inline(always)]
    pub fn tx_ch_arb_weigh(&self) -> TX_CH_ARB_WEIGH_R {
        TX_CH_ARB_WEIGH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 0: mean not optimazation weight function ,1: mean optimazation"]
    #[inline(always)]
    pub fn tx_arb_weigh_opt_dir(&self) -> TX_ARB_WEIGH_OPT_DIR_R {
        TX_ARB_WEIGH_OPT_DIR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_PRI")
            .field("tx_pri", &self.tx_pri().bits())
            .field("tx_ch_arb_weigh", &self.tx_ch_arb_weigh().bits())
            .field("tx_arb_weigh_opt_dir", &self.tx_arb_weigh_opt_dir().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_PRI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - The priority of Tx channel0. The larger of the value the higher of the priority."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pri(&mut self) -> TX_PRI_W<OUT_PRI_SPEC> {
        TX_PRI_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - The weight of Tx channel0"]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch_arb_weigh(&mut self) -> TX_CH_ARB_WEIGH_W<OUT_PRI_SPEC> {
        TX_CH_ARB_WEIGH_W::new(self, 4)
    }
    #[doc = "Bit 8 - 0: mean not optimazation weight function ,1: mean optimazation"]
    #[inline(always)]
    #[must_use]
    pub fn tx_arb_weigh_opt_dir(&mut self) -> TX_ARB_WEIGH_OPT_DIR_W<OUT_PRI_SPEC> {
        TX_ARB_WEIGH_OPT_DIR_W::new(self, 8)
    }
}
#[doc = "Priority register of Tx channel0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_pri::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_pri::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_PRI_SPEC;
impl crate::RegisterSpec for OUT_PRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_pri::R`](R) reader structure"]
impl crate::Readable for OUT_PRI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_pri::W`](W) writer structure"]
impl crate::Writable for OUT_PRI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_PRI to value 0"]
impl crate::Resettable for OUT_PRI_SPEC {
    const RESET_VALUE: u32 = 0;
}
