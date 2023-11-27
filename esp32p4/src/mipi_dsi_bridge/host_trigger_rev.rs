#[doc = "Register `HOST_TRIGGER_REV` reader"]
pub type R = crate::R<HOST_TRIGGER_REV_SPEC>;
#[doc = "Register `HOST_TRIGGER_REV` writer"]
pub type W = crate::W<HOST_TRIGGER_REV_SPEC>;
#[doc = "Field `TX_TRIGGER_REV_EN` reader - tx_trigger reverse. 0: disable, 1: enable"]
pub type TX_TRIGGER_REV_EN_R = crate::BitReader;
#[doc = "Field `TX_TRIGGER_REV_EN` writer - tx_trigger reverse. 0: disable, 1: enable"]
pub type TX_TRIGGER_REV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TRIGGER_REV_EN` reader - rx_trigger reverse. 0: disable, 1: enable"]
pub type RX_TRIGGER_REV_EN_R = crate::BitReader;
#[doc = "Field `RX_TRIGGER_REV_EN` writer - rx_trigger reverse. 0: disable, 1: enable"]
pub type RX_TRIGGER_REV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - tx_trigger reverse. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tx_trigger_rev_en(&self) -> TX_TRIGGER_REV_EN_R {
        TX_TRIGGER_REV_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rx_trigger reverse. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn rx_trigger_rev_en(&self) -> RX_TRIGGER_REV_EN_R {
        RX_TRIGGER_REV_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_TRIGGER_REV")
            .field(
                "tx_trigger_rev_en",
                &format_args!("{}", self.tx_trigger_rev_en().bit()),
            )
            .field(
                "rx_trigger_rev_en",
                &format_args!("{}", self.rx_trigger_rev_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_TRIGGER_REV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - tx_trigger reverse. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_trigger_rev_en(&mut self) -> TX_TRIGGER_REV_EN_W<HOST_TRIGGER_REV_SPEC> {
        TX_TRIGGER_REV_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - rx_trigger reverse. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_trigger_rev_en(&mut self) -> RX_TRIGGER_REV_EN_W<HOST_TRIGGER_REV_SPEC> {
        RX_TRIGGER_REV_EN_W::new(self, 1)
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
#[doc = "dsi_bridge host trigger reverse control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_trigger_rev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_trigger_rev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_TRIGGER_REV_SPEC;
impl crate::RegisterSpec for HOST_TRIGGER_REV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_trigger_rev::R`](R) reader structure"]
impl crate::Readable for HOST_TRIGGER_REV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_trigger_rev::W`](W) writer structure"]
impl crate::Writable for HOST_TRIGGER_REV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_TRIGGER_REV to value 0"]
impl crate::Resettable for HOST_TRIGGER_REV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
