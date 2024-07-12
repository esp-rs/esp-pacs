#[doc = "Register `CCM_COEF4` reader"]
pub type R = crate::R<CCM_COEF4_SPEC>;
#[doc = "Register `CCM_COEF4` writer"]
pub type W = crate::W<CCM_COEF4_SPEC>;
#[doc = "Field `CCM_BR` reader - this field configures the color correction matrix coefficient"]
pub type CCM_BR_R = crate::FieldReader<u16>;
#[doc = "Field `CCM_BR` writer - this field configures the color correction matrix coefficient"]
pub type CCM_BR_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `CCM_BG` reader - this field configures the color correction matrix coefficient"]
pub type CCM_BG_R = crate::FieldReader<u16>;
#[doc = "Field `CCM_BG` writer - this field configures the color correction matrix coefficient"]
pub type CCM_BG_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    pub fn ccm_br(&self) -> CCM_BR_R {
        CCM_BR_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:25 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    pub fn ccm_bg(&self) -> CCM_BG_R {
        CCM_BG_R::new(((self.bits >> 13) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_COEF4")
            .field("ccm_br", &self.ccm_br())
            .field("ccm_bg", &self.ccm_bg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn ccm_br(&mut self) -> CCM_BR_W<CCM_COEF4_SPEC> {
        CCM_BR_W::new(self, 0)
    }
    #[doc = "Bits 13:25 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn ccm_bg(&mut self) -> CCM_BG_W<CCM_COEF4_SPEC> {
        CCM_BG_W::new(self, 13)
    }
}
#[doc = "ccm coef register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm_coef4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm_coef4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCM_COEF4_SPEC;
impl crate::RegisterSpec for CCM_COEF4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccm_coef4::R`](R) reader structure"]
impl crate::Readable for CCM_COEF4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccm_coef4::W`](W) writer structure"]
impl crate::Writable for CCM_COEF4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCM_COEF4 to value 0x0258_1040"]
impl crate::Resettable for CCM_COEF4_SPEC {
    const RESET_VALUE: u32 = 0x0258_1040;
}
