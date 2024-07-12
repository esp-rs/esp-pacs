#[doc = "Register `INT_FORCE_PHY` reader"]
pub type R = crate::R<INT_FORCE_PHY_SPEC>;
#[doc = "Register `INT_FORCE_PHY` writer"]
pub type W = crate::W<INT_FORCE_PHY_SPEC>;
#[doc = "Field `FORCE_PHY_ERRSOTHS_0` reader - NA"]
pub type FORCE_PHY_ERRSOTHS_0_R = crate::BitReader;
#[doc = "Field `FORCE_PHY_ERRSOTHS_0` writer - NA"]
pub type FORCE_PHY_ERRSOTHS_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_PHY_ERRSOTHS_1` reader - NA"]
pub type FORCE_PHY_ERRSOTHS_1_R = crate::BitReader;
#[doc = "Field `FORCE_PHY_ERRSOTHS_1` writer - NA"]
pub type FORCE_PHY_ERRSOTHS_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_PHY_ERRESC_0` reader - NA"]
pub type FORCE_PHY_ERRESC_0_R = crate::BitReader;
#[doc = "Field `FORCE_PHY_ERRESC_0` writer - NA"]
pub type FORCE_PHY_ERRESC_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_PHY_ERRESC_1` reader - NA"]
pub type FORCE_PHY_ERRESC_1_R = crate::BitReader;
#[doc = "Field `FORCE_PHY_ERRESC_1` writer - NA"]
pub type FORCE_PHY_ERRESC_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_phy_errsoths_0(&self) -> FORCE_PHY_ERRSOTHS_0_R {
        FORCE_PHY_ERRSOTHS_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_phy_errsoths_1(&self) -> FORCE_PHY_ERRSOTHS_1_R {
        FORCE_PHY_ERRSOTHS_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn force_phy_erresc_0(&self) -> FORCE_PHY_ERRESC_0_R {
        FORCE_PHY_ERRESC_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn force_phy_erresc_1(&self) -> FORCE_PHY_ERRESC_1_R {
        FORCE_PHY_ERRESC_1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_FORCE_PHY")
            .field("force_phy_errsoths_0", &self.force_phy_errsoths_0())
            .field("force_phy_errsoths_1", &self.force_phy_errsoths_1())
            .field("force_phy_erresc_0", &self.force_phy_erresc_0())
            .field("force_phy_erresc_1", &self.force_phy_erresc_1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_phy_errsoths_0(&mut self) -> FORCE_PHY_ERRSOTHS_0_W<INT_FORCE_PHY_SPEC> {
        FORCE_PHY_ERRSOTHS_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_phy_errsoths_1(&mut self) -> FORCE_PHY_ERRSOTHS_1_W<INT_FORCE_PHY_SPEC> {
        FORCE_PHY_ERRSOTHS_1_W::new(self, 1)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_phy_erresc_0(&mut self) -> FORCE_PHY_ERRESC_0_W<INT_FORCE_PHY_SPEC> {
        FORCE_PHY_ERRESC_0_W::new(self, 16)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_phy_erresc_1(&mut self) -> FORCE_PHY_ERRESC_1_W<INT_FORCE_PHY_SPEC> {
        FORCE_PHY_ERRESC_1_W::new(self, 17)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_force_phy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_force_phy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_FORCE_PHY_SPEC;
impl crate::RegisterSpec for INT_FORCE_PHY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_force_phy::R`](R) reader structure"]
impl crate::Readable for INT_FORCE_PHY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_force_phy::W`](W) writer structure"]
impl crate::Writable for INT_FORCE_PHY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_FORCE_PHY to value 0"]
impl crate::Resettable for INT_FORCE_PHY_SPEC {
    const RESET_VALUE: u32 = 0;
}
