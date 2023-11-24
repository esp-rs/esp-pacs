#[doc = "Register `SLC_AHB_TEST` reader"]
pub type R = crate::R<SLC_AHB_TEST_SPEC>;
#[doc = "Register `SLC_AHB_TEST` writer"]
pub type W = crate::W<SLC_AHB_TEST_SPEC>;
#[doc = "Field `SLC_AHB_TESTMODE` reader - "]
pub type SLC_AHB_TESTMODE_R = crate::FieldReader;
#[doc = "Field `SLC_AHB_TESTMODE` writer - "]
pub type SLC_AHB_TESTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SLC_AHB_TESTADDR` reader - "]
pub type SLC_AHB_TESTADDR_R = crate::FieldReader;
#[doc = "Field `SLC_AHB_TESTADDR` writer - "]
pub type SLC_AHB_TESTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn slc_ahb_testmode(&self) -> SLC_AHB_TESTMODE_R {
        SLC_AHB_TESTMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn slc_ahb_testaddr(&self) -> SLC_AHB_TESTADDR_R {
        SLC_AHB_TESTADDR_R::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_AHB_TEST")
            .field(
                "slc_ahb_testaddr",
                &format_args!("{}", self.slc_ahb_testaddr().bits()),
            )
            .field(
                "slc_ahb_testmode",
                &format_args!("{}", self.slc_ahb_testmode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_AHB_TEST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn slc_ahb_testmode(&mut self) -> SLC_AHB_TESTMODE_W<SLC_AHB_TEST_SPEC> {
        SLC_AHB_TESTMODE_W::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn slc_ahb_testaddr(&mut self) -> SLC_AHB_TESTADDR_W<SLC_AHB_TEST_SPEC> {
        SLC_AHB_TESTADDR_W::new(self, 4)
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
#[doc = "SLC_AHB_TEST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_ahb_test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_ahb_test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_AHB_TEST_SPEC;
impl crate::RegisterSpec for SLC_AHB_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_ahb_test::R`](R) reader structure"]
impl crate::Readable for SLC_AHB_TEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_ahb_test::W`](W) writer structure"]
impl crate::Writable for SLC_AHB_TEST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_AHB_TEST to value 0"]
impl crate::Resettable for SLC_AHB_TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
