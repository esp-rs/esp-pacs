#[doc = "Register `PHY_TEST_CTRL1` reader"]
pub type R = crate::R<PHY_TEST_CTRL1_SPEC>;
#[doc = "Register `PHY_TEST_CTRL1` writer"]
pub type W = crate::W<PHY_TEST_CTRL1_SPEC>;
#[doc = "Field `PHY_TESTDIN` reader - NA"]
pub type PHY_TESTDIN_R = crate::FieldReader;
#[doc = "Field `PHY_TESTDIN` writer - NA"]
pub type PHY_TESTDIN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_TESTDOUT` reader - NA"]
pub type PHY_TESTDOUT_R = crate::FieldReader;
#[doc = "Field `PHY_TESTEN` reader - NA"]
pub type PHY_TESTEN_R = crate::BitReader;
#[doc = "Field `PHY_TESTEN` writer - NA"]
pub type PHY_TESTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn phy_testdin(&self) -> PHY_TESTDIN_R {
        PHY_TESTDIN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn phy_testdout(&self) -> PHY_TESTDOUT_R {
        PHY_TESTDOUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn phy_testen(&self) -> PHY_TESTEN_R {
        PHY_TESTEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PHY_TEST_CTRL1")
            .field(
                "phy_testdin",
                &format_args!("{}", self.phy_testdin().bits()),
            )
            .field(
                "phy_testdout",
                &format_args!("{}", self.phy_testdout().bits()),
            )
            .field("phy_testen", &format_args!("{}", self.phy_testen().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PHY_TEST_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_testdin(&mut self) -> PHY_TESTDIN_W<PHY_TEST_CTRL1_SPEC> {
        PHY_TESTDIN_W::new(self, 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn phy_testen(&mut self) -> PHY_TESTEN_W<PHY_TEST_CTRL1_SPEC> {
        PHY_TESTEN_W::new(self, 16)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_test_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_test_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_TEST_CTRL1_SPEC;
impl crate::RegisterSpec for PHY_TEST_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_test_ctrl1::R`](R) reader structure"]
impl crate::Readable for PHY_TEST_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_test_ctrl1::W`](W) writer structure"]
impl crate::Writable for PHY_TEST_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHY_TEST_CTRL1 to value 0"]
impl crate::Resettable for PHY_TEST_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
