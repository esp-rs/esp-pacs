#[doc = "Register `CCM_COEF1` reader"]
pub type R = crate::R<CCM_COEF1_SPEC>;
#[doc = "Register `CCM_COEF1` writer"]
pub type W = crate::W<CCM_COEF1_SPEC>;
#[doc = "Field `CCM_RB` reader - this field configures the color correction matrix coefficient"]
pub type CCM_RB_R = crate::FieldReader<u16>;
#[doc = "Field `CCM_RB` writer - this field configures the color correction matrix coefficient"]
pub type CCM_RB_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `CCM_GR` reader - this field configures the color correction matrix coefficient"]
pub type CCM_GR_R = crate::FieldReader<u16>;
#[doc = "Field `CCM_GR` writer - this field configures the color correction matrix coefficient"]
pub type CCM_GR_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    pub fn ccm_rb(&self) -> CCM_RB_R {
        CCM_RB_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:25 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    pub fn ccm_gr(&self) -> CCM_GR_R {
        CCM_GR_R::new(((self.bits >> 13) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_COEF1")
            .field("ccm_rb", &self.ccm_rb())
            .field("ccm_gr", &self.ccm_gr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    pub fn ccm_rb(&mut self) -> CCM_RB_W<CCM_COEF1_SPEC> {
        CCM_RB_W::new(self, 0)
    }
    #[doc = "Bits 13:25 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    pub fn ccm_gr(&mut self) -> CCM_GR_W<CCM_COEF1_SPEC> {
        CCM_GR_W::new(self, 13)
    }
}
#[doc = "ccm coef register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccm_coef1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccm_coef1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCM_COEF1_SPEC;
impl crate::RegisterSpec for CCM_COEF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccm_coef1::R`](R) reader structure"]
impl crate::Readable for CCM_COEF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccm_coef1::W`](W) writer structure"]
impl crate::Writable for CCM_COEF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCM_COEF1 to value 0x0228_10c0"]
impl crate::Resettable for CCM_COEF1_SPEC {
    const RESET_VALUE: u32 = 0x0228_10c0;
}
