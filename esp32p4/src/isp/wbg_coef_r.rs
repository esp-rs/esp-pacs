#[doc = "Register `WBG_COEF_R` reader"]
pub type R = crate::R<WBG_COEF_R_SPEC>;
#[doc = "Register `WBG_COEF_R` writer"]
pub type W = crate::W<WBG_COEF_R_SPEC>;
#[doc = "Field `WBG_R` reader - Configures the white balance red gain"]
pub type WBG_R_R = crate::FieldReader<u16>;
#[doc = "Field `WBG_R` writer - Configures the white balance red gain"]
pub type WBG_R_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Configures the white balance red gain"]
    #[inline(always)]
    pub fn wbg_r(&self) -> WBG_R_R {
        WBG_R_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WBG_COEF_R")
            .field("wbg_r", &self.wbg_r())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Configures the white balance red gain"]
    #[inline(always)]
    pub fn wbg_r(&mut self) -> WBG_R_W<'_, WBG_COEF_R_SPEC> {
        WBG_R_W::new(self, 0)
    }
}
#[doc = "white balance red gain register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wbg_coef_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wbg_coef_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WBG_COEF_R_SPEC;
impl crate::RegisterSpec for WBG_COEF_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wbg_coef_r::R`](R) reader structure"]
impl crate::Readable for WBG_COEF_R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wbg_coef_r::W`](W) writer structure"]
impl crate::Writable for WBG_COEF_R_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WBG_COEF_R to value 0x0100"]
impl crate::Resettable for WBG_COEF_R_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
