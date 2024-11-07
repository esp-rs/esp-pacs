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
            .field("phy_shutdownz", &self.phy_shutdownz())
            .field("phy_rstz", &self.phy_rstz())
            .field("phy_enableclk", &self.phy_enableclk())
            .field("phy_forcepll", &self.phy_forcepll())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_shutdownz(&mut self) -> PHY_SHUTDOWNZ_W<PHY_RSTZ_SPEC> {
        PHY_SHUTDOWNZ_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn phy_rstz(&mut self) -> PHY_RSTZ_W<PHY_RSTZ_SPEC> {
        PHY_RSTZ_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn phy_enableclk(&mut self) -> PHY_ENABLECLK_W<PHY_RSTZ_SPEC> {
        PHY_ENABLECLK_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn phy_forcepll(&mut self) -> PHY_FORCEPLL_W<PHY_RSTZ_SPEC> {
        PHY_FORCEPLL_W::new(self, 3)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_rstz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_rstz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_RSTZ_SPEC;
impl crate::RegisterSpec for PHY_RSTZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_rstz::R`](R) reader structure"]
impl crate::Readable for PHY_RSTZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_rstz::W`](W) writer structure"]
impl crate::Writable for PHY_RSTZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_RSTZ to value 0"]
impl crate::Resettable for PHY_RSTZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
