#[doc = "Register `PHY_RSTZ` reader"]
pub type R = crate::R<PHY_RSTZ_SPEC>;
#[doc = "Register `PHY_RSTZ` writer"]
pub type W = crate::W<PHY_RSTZ_SPEC>;
#[doc = "Field `PHY_SHUTDOWNZ` reader - NA"]
pub type PHY_SHUTDOWNZ_R = crate::BitReader;
#[doc = "Field `PHY_SHUTDOWNZ` writer - NA"]
pub type PHY_SHUTDOWNZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_RSTZ` reader - NA"]
pub type PHY_RSTZ_R = crate::BitReader;
#[doc = "Field `PHY_RSTZ` writer - NA"]
pub type PHY_RSTZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_ENABLECLK` reader - NA"]
pub type PHY_ENABLECLK_R = crate::BitReader;
#[doc = "Field `PHY_ENABLECLK` writer - NA"]
pub type PHY_ENABLECLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_FORCEPLL` reader - NA"]
pub type PHY_FORCEPLL_R = crate::BitReader;
#[doc = "Field `PHY_FORCEPLL` writer - NA"]
pub type PHY_FORCEPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_shutdownz(&self) -> PHY_SHUTDOWNZ_R {
        PHY_SHUTDOWNZ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn phy_rstz(&self) -> PHY_RSTZ_R {
        PHY_RSTZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn phy_enableclk(&self) -> PHY_ENABLECLK_R {
        PHY_ENABLECLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn phy_forcepll(&self) -> PHY_FORCEPLL_R {
        PHY_FORCEPLL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_RSTZ")
            .field(
                "phy_shutdownz",
                &format_args!("{}", self.phy_shutdownz().bit()),
            )
            .field("phy_rstz", &format_args!("{}", self.phy_rstz().bit()))
            .field(
                "phy_enableclk",
                &format_args!("{}", self.phy_enableclk().bit()),
            )
            .field(
                "phy_forcepll",
                &format_args!("{}", self.phy_forcepll().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PHY_RSTZ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_shutdownz(&mut self) -> PHY_SHUTDOWNZ_W<PHY_RSTZ_SPEC> {
        PHY_SHUTDOWNZ_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_rstz(&mut self) -> PHY_RSTZ_W<PHY_RSTZ_SPEC> {
        PHY_RSTZ_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_enableclk(&mut self) -> PHY_ENABLECLK_W<PHY_RSTZ_SPEC> {
        PHY_ENABLECLK_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_forcepll(&mut self) -> PHY_FORCEPLL_W<PHY_RSTZ_SPEC> {
        PHY_FORCEPLL_W::new(self, 3)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_rstz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_rstz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_RSTZ_SPEC;
impl crate::RegisterSpec for PHY_RSTZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_rstz::R`](R) reader structure"]
impl crate::Readable for PHY_RSTZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_rstz::W`](W) writer structure"]
impl crate::Writable for PHY_RSTZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHY_RSTZ to value 0"]
impl crate::Resettable for PHY_RSTZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
