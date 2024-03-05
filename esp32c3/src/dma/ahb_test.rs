#[doc = "Register `AHB_TEST` reader"]
pub type R = crate::R<AHB_TEST_SPEC>;
#[doc = "Register `AHB_TEST` writer"]
pub type W = crate::W<AHB_TEST_SPEC>;
#[doc = "Field `AHB_TESTMODE` reader - reserved"]
pub type AHB_TESTMODE_R = crate::FieldReader;
#[doc = "Field `AHB_TESTMODE` writer - reserved"]
pub type AHB_TESTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AHB_TESTADDR` reader - reserved"]
pub type AHB_TESTADDR_R = crate::FieldReader;
#[doc = "Field `AHB_TESTADDR` writer - reserved"]
pub type AHB_TESTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - reserved"]
    #[inline(always)]
    pub fn ahb_testmode(&self) -> AHB_TESTMODE_R {
        AHB_TESTMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - reserved"]
    #[inline(always)]
    pub fn ahb_testaddr(&self) -> AHB_TESTADDR_R {
        AHB_TESTADDR_R::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_TEST")
            .field(
                "ahb_testmode",
                &format_args!("{}", self.ahb_testmode().bits()),
            )
            .field(
                "ahb_testaddr",
                &format_args!("{}", self.ahb_testaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AHB_TEST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_testmode(&mut self) -> AHB_TESTMODE_W<AHB_TEST_SPEC> {
        AHB_TESTMODE_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_testaddr(&mut self) -> AHB_TESTADDR_W<AHB_TEST_SPEC> {
        AHB_TESTADDR_W::new(self, 4)
    }
}
#[doc = "DMA_AHB_TEST_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_TEST_SPEC;
impl crate::RegisterSpec for AHB_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_test::R`](R) reader structure"]
impl crate::Readable for AHB_TEST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_test::W`](W) writer structure"]
impl crate::Writable for AHB_TEST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB_TEST to value 0"]
impl crate::Resettable for AHB_TEST_SPEC {
    const RESET_VALUE: u32 = 0;
}
