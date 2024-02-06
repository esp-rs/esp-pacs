#[doc = "Register `PHY_TST_CTRL0` reader"]
pub type R = crate::R<PHY_TST_CTRL0_SPEC>;
#[doc = "Register `PHY_TST_CTRL0` writer"]
pub type W = crate::W<PHY_TST_CTRL0_SPEC>;
#[doc = "Field `PHY_TESTCLR` reader - NA"]
pub type PHY_TESTCLR_R = crate::BitReader;
#[doc = "Field `PHY_TESTCLR` writer - NA"]
pub type PHY_TESTCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_TESTCLK` reader - NA"]
pub type PHY_TESTCLK_R = crate::BitReader;
#[doc = "Field `PHY_TESTCLK` writer - NA"]
pub type PHY_TESTCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_testclr(&self) -> PHY_TESTCLR_R {
        PHY_TESTCLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn phy_testclk(&self) -> PHY_TESTCLK_R {
        PHY_TESTCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_TST_CTRL0")
            .field("phy_testclr", &format_args!("{}", self.phy_testclr().bit()))
            .field("phy_testclk", &format_args!("{}", self.phy_testclk().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PHY_TST_CTRL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_testclr(&mut self) -> PHY_TESTCLR_W<PHY_TST_CTRL0_SPEC> {
        PHY_TESTCLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_testclk(&mut self) -> PHY_TESTCLK_W<PHY_TST_CTRL0_SPEC> {
        PHY_TESTCLK_W::new(self, 1)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_tst_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_tst_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_TST_CTRL0_SPEC;
impl crate::RegisterSpec for PHY_TST_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_tst_ctrl0::R`](R) reader structure"]
impl crate::Readable for PHY_TST_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_tst_ctrl0::W`](W) writer structure"]
impl crate::Writable for PHY_TST_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_TST_CTRL0 to value 0x01"]
impl crate::Resettable for PHY_TST_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
