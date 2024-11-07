#[doc = "Register `PHY_SHUTDOWNZ` reader"]
pub type R = crate::R<PHY_SHUTDOWNZ_SPEC>;
#[doc = "Register `PHY_SHUTDOWNZ` writer"]
pub type W = crate::W<PHY_SHUTDOWNZ_SPEC>;
#[doc = "Field `PHY_SHUTDOWNZ` reader - NA"]
pub type PHY_SHUTDOWNZ_R = crate::BitReader;
#[doc = "Field `PHY_SHUTDOWNZ` writer - NA"]
pub type PHY_SHUTDOWNZ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_shutdownz(&self) -> PHY_SHUTDOWNZ_R {
        PHY_SHUTDOWNZ_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_SHUTDOWNZ")
            .field("phy_shutdownz", &self.phy_shutdownz())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn phy_shutdownz(&mut self) -> PHY_SHUTDOWNZ_W<PHY_SHUTDOWNZ_SPEC> {
        PHY_SHUTDOWNZ_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`phy_shutdownz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phy_shutdownz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_SHUTDOWNZ_SPEC;
impl crate::RegisterSpec for PHY_SHUTDOWNZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_shutdownz::R`](R) reader structure"]
impl crate::Readable for PHY_SHUTDOWNZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_shutdownz::W`](W) writer structure"]
impl crate::Writable for PHY_SHUTDOWNZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_SHUTDOWNZ to value 0"]
impl crate::Resettable for PHY_SHUTDOWNZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
