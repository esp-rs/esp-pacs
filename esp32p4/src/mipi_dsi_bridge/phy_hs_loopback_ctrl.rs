#[doc = "Register `PHY_HS_LOOPBACK_CTRL` reader"]
pub type R = crate::R<PHY_HS_LOOPBACK_CTRL_SPEC>;
#[doc = "Register `PHY_HS_LOOPBACK_CTRL` writer"]
pub type W = crate::W<PHY_HS_LOOPBACK_CTRL_SPEC>;
#[doc = "Field `PHY_HS_TXDATAHS_1` reader - txdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
pub type PHY_HS_TXDATAHS_1_R = crate::FieldReader;
#[doc = "Field `PHY_HS_TXDATAHS_1` writer - txdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
pub type PHY_HS_TXDATAHS_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_HS_TXREQUESTDATAHS_1` reader - txrequestdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
pub type PHY_HS_TXREQUESTDATAHS_1_R = crate::BitReader;
#[doc = "Field `PHY_HS_TXREQUESTDATAHS_1` writer - txrequestdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
pub type PHY_HS_TXREQUESTDATAHS_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_BASEDIR_1` reader - basedir_1 ctrl when enable dsi phy hs_loopback_test"]
pub type PHY_HS_BASEDIR_1_R = crate::BitReader;
#[doc = "Field `PHY_HS_BASEDIR_1` writer - basedir_1 ctrl when enable dsi phy hs_loopback_test"]
pub type PHY_HS_BASEDIR_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_TXDATAHS_0` reader - txdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
pub type PHY_HS_TXDATAHS_0_R = crate::FieldReader;
#[doc = "Field `PHY_HS_TXDATAHS_0` writer - txdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
pub type PHY_HS_TXDATAHS_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_HS_TXREQUESTDATAHS_0` reader - txrequestdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
pub type PHY_HS_TXREQUESTDATAHS_0_R = crate::BitReader;
#[doc = "Field `PHY_HS_TXREQUESTDATAHS_0` writer - txrequestdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
pub type PHY_HS_TXREQUESTDATAHS_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_BASEDIR_0` reader - basedir_0 ctrl when enable dsi phy hs_loopback_test"]
pub type PHY_HS_BASEDIR_0_R = crate::BitReader;
#[doc = "Field `PHY_HS_BASEDIR_0` writer - basedir_0 ctrl when enable dsi phy hs_loopback_test"]
pub type PHY_HS_BASEDIR_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_TXREQUESTHSCLK` reader - txrequesthsclk when enable dsi phy hs_loopback_test"]
pub type PHY_HS_TXREQUESTHSCLK_R = crate::BitReader;
#[doc = "Field `PHY_HS_TXREQUESTHSCLK` writer - txrequesthsclk when enable dsi phy hs_loopback_test"]
pub type PHY_HS_TXREQUESTHSCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_LOOPBACK_CHECK` writer - dsi phy hs_loopback test start check"]
pub type PHY_HS_LOOPBACK_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_LOOPBACK_CHECK_DONE` reader - dsi phy hs_loopback test check done"]
pub type PHY_HS_LOOPBACK_CHECK_DONE_R = crate::BitReader;
#[doc = "Field `PHY_HS_LOOPBACK_EN` reader - dsi phy hs_loopback ctrl en"]
pub type PHY_HS_LOOPBACK_EN_R = crate::BitReader;
#[doc = "Field `PHY_HS_LOOPBACK_EN` writer - dsi phy hs_loopback ctrl en"]
pub type PHY_HS_LOOPBACK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_HS_LOOPBACK_OK` reader - result of dsi phy hs_loopback test"]
pub type PHY_HS_LOOPBACK_OK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - txdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txdatahs_1(&self) -> PHY_HS_TXDATAHS_1_R {
        PHY_HS_TXDATAHS_1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - txrequestdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txrequestdatahs_1(&self) -> PHY_HS_TXREQUESTDATAHS_1_R {
        PHY_HS_TXREQUESTDATAHS_1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - basedir_1 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_basedir_1(&self) -> PHY_HS_BASEDIR_1_R {
        PHY_HS_BASEDIR_1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:23 - txdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txdatahs_0(&self) -> PHY_HS_TXDATAHS_0_R {
        PHY_HS_TXDATAHS_0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - txrequestdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txrequestdatahs_0(&self) -> PHY_HS_TXREQUESTDATAHS_0_R {
        PHY_HS_TXREQUESTDATAHS_0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - basedir_0 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_basedir_0(&self) -> PHY_HS_BASEDIR_0_R {
        PHY_HS_BASEDIR_0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - txrequesthsclk when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    pub fn phy_hs_txrequesthsclk(&self) -> PHY_HS_TXREQUESTHSCLK_R {
        PHY_HS_TXREQUESTHSCLK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - dsi phy hs_loopback test check done"]
    #[inline(always)]
    pub fn phy_hs_loopback_check_done(&self) -> PHY_HS_LOOPBACK_CHECK_DONE_R {
        PHY_HS_LOOPBACK_CHECK_DONE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - dsi phy hs_loopback ctrl en"]
    #[inline(always)]
    pub fn phy_hs_loopback_en(&self) -> PHY_HS_LOOPBACK_EN_R {
        PHY_HS_LOOPBACK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - result of dsi phy hs_loopback test"]
    #[inline(always)]
    pub fn phy_hs_loopback_ok(&self) -> PHY_HS_LOOPBACK_OK_R {
        PHY_HS_LOOPBACK_OK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_HS_LOOPBACK_CTRL")
            .field("phy_hs_txdatahs_1", &self.phy_hs_txdatahs_1().bits())
            .field(
                "phy_hs_txrequestdatahs_1",
                &self.phy_hs_txrequestdatahs_1().bit(),
            )
            .field("phy_hs_basedir_1", &self.phy_hs_basedir_1().bit())
            .field("phy_hs_txdatahs_0", &self.phy_hs_txdatahs_0().bits())
            .field(
                "phy_hs_txrequestdatahs_0",
                &self.phy_hs_txrequestdatahs_0().bit(),
            )
            .field("phy_hs_basedir_0", &self.phy_hs_basedir_0().bit())
            .field("phy_hs_txrequesthsclk", &self.phy_hs_txrequesthsclk().bit())
            .field(
                "phy_hs_loopback_check_done",
                &self.phy_hs_loopback_check_done().bit(),
            )
            .field("phy_hs_loopback_en", &self.phy_hs_loopback_en().bit())
            .field("phy_hs_loopback_ok", &self.phy_hs_loopback_ok().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PHY_HS_LOOPBACK_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - txdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_hs_txdatahs_1(&mut self) -> PHY_HS_TXDATAHS_1_W<PHY_HS_LOOPBACK_CTRL_SPEC> {
        PHY_HS_TXDATAHS_1_W::new(self, 0)
    }
    #[doc = "Bit 8 - txrequestdatahs_1 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_hs_txrequestdatahs_1(
        &mut self,
    ) -> PHY_HS_TXREQUESTDATAHS_1_W<PHY_HS_LOOPBACK_CTRL_SPEC> {
        PHY_HS_TXREQUESTDATAHS_1_W::new(self, 8)
    }
    #[doc = "Bit 9 - basedir_1 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_hs_basedir_1(&mut self) -> PHY_HS_BASEDIR_1_W<PHY_HS_LOOPBACK_CTRL_SPEC> {
        PHY_HS_BASEDIR_1_W::new(self, 9)
    }
    #[doc = "Bits 16:23 - txdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_hs_txdatahs_0(&mut self) -> PHY_HS_TXDATAHS_0_W<PHY_HS_LOOPBACK_CTRL_SPEC> {
        PHY_HS_TXDATAHS_0_W::new(self, 16)
    }
    #[doc = "Bit 24 - txrequestdatahs_0 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_hs_txrequestdatahs_0(
        &mut self,
    ) -> PHY_HS_TXREQUESTDATAHS_0_W<PHY_HS_LOOPBACK_CTRL_SPEC> {
        PHY_HS_TXREQUESTDATAHS_0_W::new(self, 24)
    }
    #[doc = "Bit 25 - basedir_0 ctrl when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_hs_basedir_0(&mut self) -> PHY_HS_BASEDIR_0_W<PHY_HS_LOOPBACK_CTRL_SPEC> {
        PHY_HS_BASEDIR_0_W::new(self, 25)
    }
    #[doc = "Bit 27 - txrequesthsclk when enable dsi phy hs_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_hs_txrequesthsclk(&mut self) -> PHY_HS_TXREQUESTHSCLK_W<PHY_HS_LOOPBACK_CTRL_SPEC> {
        PHY_HS_TXREQUESTHSCLK_W::new(self, 27)
    }
    #[doc = "Bit 28 - dsi phy hs_loopback test start check"]
    #[inline(always)]
    #[must_use]
    pub fn phy_hs_loopback_check(&mut self) -> PHY_HS_LOOPBACK_CHECK_W<PHY_HS_LOOPBACK_CTRL_SPEC> {
        PHY_HS_LOOPBACK_CHECK_W::new(self, 28)
    }
    #[doc = "Bit 30 - dsi phy hs_loopback ctrl en"]
    #[inline(always)]
    #[must_use]
    pub fn phy_hs_loopback_en(&mut self) -> PHY_HS_LOOPBACK_EN_W<PHY_HS_LOOPBACK_CTRL_SPEC> {
        PHY_HS_LOOPBACK_EN_W::new(self, 30)
    }
}
#[doc = "dsi phy hp_loopback test ctrl\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_hs_loopback_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_hs_loopback_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_HS_LOOPBACK_CTRL_SPEC;
impl crate::RegisterSpec for PHY_HS_LOOPBACK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_hs_loopback_ctrl::R`](R) reader structure"]
impl crate::Readable for PHY_HS_LOOPBACK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_hs_loopback_ctrl::W`](W) writer structure"]
impl crate::Writable for PHY_HS_LOOPBACK_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_HS_LOOPBACK_CTRL to value 0x0200"]
impl crate::Resettable for PHY_HS_LOOPBACK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
