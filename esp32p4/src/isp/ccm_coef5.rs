#[doc = "Register `CCM_COEF5` reader"]
pub type R = crate::R<CCM_COEF5_SPEC>;
#[doc = "Register `CCM_COEF5` writer"]
pub type W = crate::W<CCM_COEF5_SPEC>;
#[doc = "Field `CCM_BB` reader - this field configures the color correction matrix coefficient"]
pub type CCM_BB_R = crate::FieldReader<u16>;
#[doc = "Field `CCM_BB` writer - this field configures the color correction matrix coefficient"]
pub type CCM_BB_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    pub fn ccm_bb(&self) -> CCM_BB_R {
        CCM_BB_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_COEF5")
            .field("ccm_bb", &self.ccm_bb().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CCM_COEF5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:12 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn ccm_bb(&mut self) -> CCM_BB_W<CCM_COEF5_SPEC> {
        CCM_BB_W::new(self, 0)
    }
}
#[doc = "ccm coef register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccm_coef5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccm_coef5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCM_COEF5_SPEC;
impl crate::RegisterSpec for CCM_COEF5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccm_coef5::R`](R) reader structure"]
impl crate::Readable for CCM_COEF5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccm_coef5::W`](W) writer structure"]
impl crate::Writable for CCM_COEF5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCM_COEF5 to value 0x0740"]
impl crate::Resettable for CCM_COEF5_SPEC {
    const RESET_VALUE: u32 = 0x0740;
}
