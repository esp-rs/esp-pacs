#[doc = "Register `PHY_TX_TRIGGERS` reader"]
pub type R = crate::R<PHY_TX_TRIGGERS_SPEC>;
#[doc = "Register `PHY_TX_TRIGGERS` writer"]
pub type W = crate::W<PHY_TX_TRIGGERS_SPEC>;
#[doc = "Field `PHY_TX_TRIGGERS` reader - NA"]
pub type PHY_TX_TRIGGERS_R = crate::FieldReader;
#[doc = "Field `PHY_TX_TRIGGERS` writer - NA"]
pub type PHY_TX_TRIGGERS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    pub fn phy_tx_triggers(&self) -> PHY_TX_TRIGGERS_R {
        PHY_TX_TRIGGERS_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_TX_TRIGGERS")
            .field("phy_tx_triggers", &self.phy_tx_triggers())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    pub fn phy_tx_triggers(&mut self) -> PHY_TX_TRIGGERS_W<PHY_TX_TRIGGERS_SPEC> {
        PHY_TX_TRIGGERS_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_tx_triggers::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_tx_triggers::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_TX_TRIGGERS_SPEC;
impl crate::RegisterSpec for PHY_TX_TRIGGERS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_tx_triggers::R`](R) reader structure"]
impl crate::Readable for PHY_TX_TRIGGERS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_tx_triggers::W`](W) writer structure"]
impl crate::Writable for PHY_TX_TRIGGERS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHY_TX_TRIGGERS to value 0"]
impl crate::Resettable for PHY_TX_TRIGGERS_SPEC {}
