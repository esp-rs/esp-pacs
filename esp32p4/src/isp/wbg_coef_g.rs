#[doc = "Register `WBG_COEF_G` reader"]
pub type R = crate::R<WBG_COEF_G_SPEC>;
#[doc = "Register `WBG_COEF_G` writer"]
pub type W = crate::W<WBG_COEF_G_SPEC>;
#[doc = "Field `WBG_G` reader - Configures the white balance green gain"]
pub type WBG_G_R = crate::FieldReader<u16>;
#[doc = "Field `WBG_G` writer - Configures the white balance green gain"]
pub type WBG_G_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Configures the white balance green gain"]
    #[inline(always)]
    pub fn wbg_g(&self) -> WBG_G_R {
        WBG_G_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WBG_COEF_G")
            .field("wbg_g", &self.wbg_g())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Configures the white balance green gain"]
    #[inline(always)]
    pub fn wbg_g(&mut self) -> WBG_G_W<'_, WBG_COEF_G_SPEC> {
        WBG_G_W::new(self, 0)
    }
}
#[doc = "white balance green gain register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wbg_coef_g::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wbg_coef_g::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WBG_COEF_G_SPEC;
impl crate::RegisterSpec for WBG_COEF_G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wbg_coef_g::R`](R) reader structure"]
impl crate::Readable for WBG_COEF_G_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wbg_coef_g::W`](W) writer structure"]
impl crate::Writable for WBG_COEF_G_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WBG_COEF_G to value 0x0100"]
impl crate::Resettable for WBG_COEF_G_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
