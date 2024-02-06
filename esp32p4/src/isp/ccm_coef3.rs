#[doc = "Register `CCM_COEF3` reader"]
pub type R = crate::R<CCM_COEF3_SPEC>;
#[doc = "Register `CCM_COEF3` writer"]
pub type W = crate::W<CCM_COEF3_SPEC>;
#[doc = "Field `CCM_GG` reader - this field configures the color correction matrix coefficient"]
pub type CCM_GG_R = crate::FieldReader<u16>;
#[doc = "Field `CCM_GG` writer - this field configures the color correction matrix coefficient"]
pub type CCM_GG_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `CCM_GB` reader - this field configures the color correction matrix coefficient"]
pub type CCM_GB_R = crate::FieldReader<u16>;
#[doc = "Field `CCM_GB` writer - this field configures the color correction matrix coefficient"]
pub type CCM_GB_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    pub fn ccm_gg(&self) -> CCM_GG_R {
        CCM_GG_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:25 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    pub fn ccm_gb(&self) -> CCM_GB_R {
        CCM_GB_R::new(((self.bits >> 13) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCM_COEF3")
            .field("ccm_gg", &format_args!("{}", self.ccm_gg().bits()))
            .field("ccm_gb", &format_args!("{}", self.ccm_gb().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CCM_COEF3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:12 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn ccm_gg(&mut self) -> CCM_GG_W<CCM_COEF3_SPEC> {
        CCM_GG_W::new(self, 0)
    }
    #[doc = "Bits 13:25 - this field configures the color correction matrix coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn ccm_gb(&mut self) -> CCM_GB_W<CCM_COEF3_SPEC> {
        CCM_GB_W::new(self, 13)
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
#[doc = "ccm coef register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccm_coef3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccm_coef3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCM_COEF3_SPEC;
impl crate::RegisterSpec for CCM_COEF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccm_coef3::R`](R) reader structure"]
impl crate::Readable for CCM_COEF3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccm_coef3::W`](W) writer structure"]
impl crate::Writable for CCM_COEF3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCM_COEF3 to value 0x0220_0680"]
impl crate::Resettable for CCM_COEF3_SPEC {
    const RESET_VALUE: u32 = 0x0220_0680;
}
