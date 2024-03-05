#[doc = "Register `CALI` reader"]
pub type R = crate::R<CALI_SPEC>;
#[doc = "Register `CALI` writer"]
pub type W = crate::W<CALI_SPEC>;
#[doc = "Field `APB_SARADC_CALI_CFG` reader - saradc cali factor"]
pub type APB_SARADC_CALI_CFG_R = crate::FieldReader<u32>;
#[doc = "Field `APB_SARADC_CALI_CFG` writer - saradc cali factor"]
pub type APB_SARADC_CALI_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - saradc cali factor"]
    #[inline(always)]
    pub fn apb_saradc_cali_cfg(&self) -> APB_SARADC_CALI_CFG_R {
        APB_SARADC_CALI_CFG_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALI")
            .field(
                "apb_saradc_cali_cfg",
                &format_args!("{}", self.apb_saradc_cali_cfg().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CALI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:16 - saradc cali factor"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_cali_cfg(&mut self) -> APB_SARADC_CALI_CFG_W<CALI_SPEC> {
        APB_SARADC_CALI_CFG_W::new(self, 0)
    }
}
#[doc = "digital saradc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cali::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cali::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALI_SPEC;
impl crate::RegisterSpec for CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cali::R`](R) reader structure"]
impl crate::Readable for CALI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cali::W`](W) writer structure"]
impl crate::Writable for CALI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALI to value 0x8000"]
impl crate::Resettable for CALI_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
