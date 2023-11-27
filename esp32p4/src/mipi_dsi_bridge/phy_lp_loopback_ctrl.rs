#[doc = "Register `PHY_LP_LOOPBACK_CTRL` reader"]
pub type R = crate::R<PHY_LP_LOOPBACK_CTRL_SPEC>;
#[doc = "Register `PHY_LP_LOOPBACK_CTRL` writer"]
pub type W = crate::W<PHY_LP_LOOPBACK_CTRL_SPEC>;
#[doc = "Field `PHY_LP_TXDATAESC_1` reader - txdataesc_1 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXDATAESC_1_R = crate::FieldReader;
#[doc = "Field `PHY_LP_TXDATAESC_1` writer - txdataesc_1 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXDATAESC_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_LP_TXREQUESTESC_1` reader - txrequestesc_1 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXREQUESTESC_1_R = crate::BitReader;
#[doc = "Field `PHY_LP_TXREQUESTESC_1` writer - txrequestesc_1 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXREQUESTESC_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP_TXVALIDESC_1` reader - txvalidesc_1 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXVALIDESC_1_R = crate::BitReader;
#[doc = "Field `PHY_LP_TXVALIDESC_1` writer - txvalidesc_1 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXVALIDESC_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP_TXLPDTESC_1` reader - txlpdtesc_1 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXLPDTESC_1_R = crate::BitReader;
#[doc = "Field `PHY_LP_TXLPDTESC_1` writer - txlpdtesc_1 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXLPDTESC_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP_BASEDIR_1` reader - basedir_1 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_BASEDIR_1_R = crate::BitReader;
#[doc = "Field `PHY_LP_BASEDIR_1` writer - basedir_1 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_BASEDIR_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP_TXDATAESC_0` reader - txdataesc_0 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXDATAESC_0_R = crate::FieldReader;
#[doc = "Field `PHY_LP_TXDATAESC_0` writer - txdataesc_0 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXDATAESC_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_LP_TXREQUESTESC_0` reader - txrequestesc_0 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXREQUESTESC_0_R = crate::BitReader;
#[doc = "Field `PHY_LP_TXREQUESTESC_0` writer - txrequestesc_0 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXREQUESTESC_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP_TXVALIDESC_0` reader - txvalidesc_0 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXVALIDESC_0_R = crate::BitReader;
#[doc = "Field `PHY_LP_TXVALIDESC_0` writer - txvalidesc_0 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXVALIDESC_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP_TXLPDTESC_0` reader - txlpdtesc_0 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXLPDTESC_0_R = crate::BitReader;
#[doc = "Field `PHY_LP_TXLPDTESC_0` writer - txlpdtesc_0 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_TXLPDTESC_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP_BASEDIR_0` reader - basedir_0 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_BASEDIR_0_R = crate::BitReader;
#[doc = "Field `PHY_LP_BASEDIR_0` writer - basedir_0 ctrl when enable dsi phy lp_loopback_test"]
pub type PHY_LP_BASEDIR_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP_LOOPBACK_CHECK` writer - dsi phy lp_loopback test start check"]
pub type PHY_LP_LOOPBACK_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP_LOOPBACK_CHECK_DONE` reader - dsi phy lp_loopback test check done"]
pub type PHY_LP_LOOPBACK_CHECK_DONE_R = crate::BitReader;
#[doc = "Field `PHY_LP_LOOPBACK_EN` reader - dsi phy lp_loopback ctrl en"]
pub type PHY_LP_LOOPBACK_EN_R = crate::BitReader;
#[doc = "Field `PHY_LP_LOOPBACK_EN` writer - dsi phy lp_loopback ctrl en"]
pub type PHY_LP_LOOPBACK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP_LOOPBACK_OK` reader - result of dsi phy lp_loopback test"]
pub type PHY_LP_LOOPBACK_OK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - txdataesc_1 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    pub fn phy_lp_txdataesc_1(&self) -> PHY_LP_TXDATAESC_1_R {
        PHY_LP_TXDATAESC_1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - txrequestesc_1 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    pub fn phy_lp_txrequestesc_1(&self) -> PHY_LP_TXREQUESTESC_1_R {
        PHY_LP_TXREQUESTESC_1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - txvalidesc_1 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    pub fn phy_lp_txvalidesc_1(&self) -> PHY_LP_TXVALIDESC_1_R {
        PHY_LP_TXVALIDESC_1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - txlpdtesc_1 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    pub fn phy_lp_txlpdtesc_1(&self) -> PHY_LP_TXLPDTESC_1_R {
        PHY_LP_TXLPDTESC_1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - basedir_1 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    pub fn phy_lp_basedir_1(&self) -> PHY_LP_BASEDIR_1_R {
        PHY_LP_BASEDIR_1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - txdataesc_0 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    pub fn phy_lp_txdataesc_0(&self) -> PHY_LP_TXDATAESC_0_R {
        PHY_LP_TXDATAESC_0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - txrequestesc_0 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    pub fn phy_lp_txrequestesc_0(&self) -> PHY_LP_TXREQUESTESC_0_R {
        PHY_LP_TXREQUESTESC_0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - txvalidesc_0 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    pub fn phy_lp_txvalidesc_0(&self) -> PHY_LP_TXVALIDESC_0_R {
        PHY_LP_TXVALIDESC_0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - txlpdtesc_0 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    pub fn phy_lp_txlpdtesc_0(&self) -> PHY_LP_TXLPDTESC_0_R {
        PHY_LP_TXLPDTESC_0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - basedir_0 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    pub fn phy_lp_basedir_0(&self) -> PHY_LP_BASEDIR_0_R {
        PHY_LP_BASEDIR_0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - dsi phy lp_loopback test check done"]
    #[inline(always)]
    pub fn phy_lp_loopback_check_done(&self) -> PHY_LP_LOOPBACK_CHECK_DONE_R {
        PHY_LP_LOOPBACK_CHECK_DONE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - dsi phy lp_loopback ctrl en"]
    #[inline(always)]
    pub fn phy_lp_loopback_en(&self) -> PHY_LP_LOOPBACK_EN_R {
        PHY_LP_LOOPBACK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - result of dsi phy lp_loopback test"]
    #[inline(always)]
    pub fn phy_lp_loopback_ok(&self) -> PHY_LP_LOOPBACK_OK_R {
        PHY_LP_LOOPBACK_OK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_LP_LOOPBACK_CTRL")
            .field(
                "phy_lp_txdataesc_1",
                &format_args!("{}", self.phy_lp_txdataesc_1().bits()),
            )
            .field(
                "phy_lp_txrequestesc_1",
                &format_args!("{}", self.phy_lp_txrequestesc_1().bit()),
            )
            .field(
                "phy_lp_txvalidesc_1",
                &format_args!("{}", self.phy_lp_txvalidesc_1().bit()),
            )
            .field(
                "phy_lp_txlpdtesc_1",
                &format_args!("{}", self.phy_lp_txlpdtesc_1().bit()),
            )
            .field(
                "phy_lp_basedir_1",
                &format_args!("{}", self.phy_lp_basedir_1().bit()),
            )
            .field(
                "phy_lp_txdataesc_0",
                &format_args!("{}", self.phy_lp_txdataesc_0().bits()),
            )
            .field(
                "phy_lp_txrequestesc_0",
                &format_args!("{}", self.phy_lp_txrequestesc_0().bit()),
            )
            .field(
                "phy_lp_txvalidesc_0",
                &format_args!("{}", self.phy_lp_txvalidesc_0().bit()),
            )
            .field(
                "phy_lp_txlpdtesc_0",
                &format_args!("{}", self.phy_lp_txlpdtesc_0().bit()),
            )
            .field(
                "phy_lp_basedir_0",
                &format_args!("{}", self.phy_lp_basedir_0().bit()),
            )
            .field(
                "phy_lp_loopback_check_done",
                &format_args!("{}", self.phy_lp_loopback_check_done().bit()),
            )
            .field(
                "phy_lp_loopback_en",
                &format_args!("{}", self.phy_lp_loopback_en().bit()),
            )
            .field(
                "phy_lp_loopback_ok",
                &format_args!("{}", self.phy_lp_loopback_ok().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PHY_LP_LOOPBACK_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - txdataesc_1 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_txdataesc_1(&mut self) -> PHY_LP_TXDATAESC_1_W<PHY_LP_LOOPBACK_CTRL_SPEC> {
        PHY_LP_TXDATAESC_1_W::new(self, 0)
    }
    #[doc = "Bit 8 - txrequestesc_1 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_txrequestesc_1(&mut self) -> PHY_LP_TXREQUESTESC_1_W<PHY_LP_LOOPBACK_CTRL_SPEC> {
        PHY_LP_TXREQUESTESC_1_W::new(self, 8)
    }
    #[doc = "Bit 9 - txvalidesc_1 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_txvalidesc_1(&mut self) -> PHY_LP_TXVALIDESC_1_W<PHY_LP_LOOPBACK_CTRL_SPEC> {
        PHY_LP_TXVALIDESC_1_W::new(self, 9)
    }
    #[doc = "Bit 10 - txlpdtesc_1 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_txlpdtesc_1(&mut self) -> PHY_LP_TXLPDTESC_1_W<PHY_LP_LOOPBACK_CTRL_SPEC> {
        PHY_LP_TXLPDTESC_1_W::new(self, 10)
    }
    #[doc = "Bit 11 - basedir_1 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_basedir_1(&mut self) -> PHY_LP_BASEDIR_1_W<PHY_LP_LOOPBACK_CTRL_SPEC> {
        PHY_LP_BASEDIR_1_W::new(self, 11)
    }
    #[doc = "Bits 16:23 - txdataesc_0 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_txdataesc_0(&mut self) -> PHY_LP_TXDATAESC_0_W<PHY_LP_LOOPBACK_CTRL_SPEC> {
        PHY_LP_TXDATAESC_0_W::new(self, 16)
    }
    #[doc = "Bit 24 - txrequestesc_0 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_txrequestesc_0(&mut self) -> PHY_LP_TXREQUESTESC_0_W<PHY_LP_LOOPBACK_CTRL_SPEC> {
        PHY_LP_TXREQUESTESC_0_W::new(self, 24)
    }
    #[doc = "Bit 25 - txvalidesc_0 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_txvalidesc_0(&mut self) -> PHY_LP_TXVALIDESC_0_W<PHY_LP_LOOPBACK_CTRL_SPEC> {
        PHY_LP_TXVALIDESC_0_W::new(self, 25)
    }
    #[doc = "Bit 26 - txlpdtesc_0 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_txlpdtesc_0(&mut self) -> PHY_LP_TXLPDTESC_0_W<PHY_LP_LOOPBACK_CTRL_SPEC> {
        PHY_LP_TXLPDTESC_0_W::new(self, 26)
    }
    #[doc = "Bit 27 - basedir_0 ctrl when enable dsi phy lp_loopback_test"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_basedir_0(&mut self) -> PHY_LP_BASEDIR_0_W<PHY_LP_LOOPBACK_CTRL_SPEC> {
        PHY_LP_BASEDIR_0_W::new(self, 27)
    }
    #[doc = "Bit 28 - dsi phy lp_loopback test start check"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_loopback_check(&mut self) -> PHY_LP_LOOPBACK_CHECK_W<PHY_LP_LOOPBACK_CTRL_SPEC> {
        PHY_LP_LOOPBACK_CHECK_W::new(self, 28)
    }
    #[doc = "Bit 30 - dsi phy lp_loopback ctrl en"]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp_loopback_en(&mut self) -> PHY_LP_LOOPBACK_EN_W<PHY_LP_LOOPBACK_CTRL_SPEC> {
        PHY_LP_LOOPBACK_EN_W::new(self, 30)
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
#[doc = "dsi phy lp_loopback test ctrl\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_lp_loopback_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_lp_loopback_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_LP_LOOPBACK_CTRL_SPEC;
impl crate::RegisterSpec for PHY_LP_LOOPBACK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_lp_loopback_ctrl::R`](R) reader structure"]
impl crate::Readable for PHY_LP_LOOPBACK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_lp_loopback_ctrl::W`](W) writer structure"]
impl crate::Writable for PHY_LP_LOOPBACK_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHY_LP_LOOPBACK_CTRL to value 0"]
impl crate::Resettable for PHY_LP_LOOPBACK_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
