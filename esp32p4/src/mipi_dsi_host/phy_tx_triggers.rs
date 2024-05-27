///Register `PHY_TX_TRIGGERS` reader
pub type R = crate::R<PHY_TX_TRIGGERS_SPEC>;
///Register `PHY_TX_TRIGGERS` writer
pub type W = crate::W<PHY_TX_TRIGGERS_SPEC>;
///Field `PHY_TX_TRIGGERS` reader - NA
pub type PHY_TX_TRIGGERS_R = crate::FieldReader;
///Field `PHY_TX_TRIGGERS` writer - NA
pub type PHY_TX_TRIGGERS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - NA
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
    ///Bits 0:3 - NA
    #[inline(always)]
    #[must_use]
    pub fn phy_tx_triggers(&mut self) -> PHY_TX_TRIGGERS_W<PHY_TX_TRIGGERS_SPEC> {
        PHY_TX_TRIGGERS_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`phy_tx_triggers::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tx_triggers::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PHY_TX_TRIGGERS_SPEC;
impl crate::RegisterSpec for PHY_TX_TRIGGERS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`phy_tx_triggers::R`](R) reader structure
impl crate::Readable for PHY_TX_TRIGGERS_SPEC {}
///`write(|w| ..)` method takes [`phy_tx_triggers::W`](W) writer structure
impl crate::Writable for PHY_TX_TRIGGERS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PHY_TX_TRIGGERS to value 0
impl crate::Resettable for PHY_TX_TRIGGERS_SPEC {
    const RESET_VALUE: u32 = 0;
}
