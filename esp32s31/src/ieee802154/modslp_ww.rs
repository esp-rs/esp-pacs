#[doc = "Register `MODSLP_WW` reader"]
pub type R = crate::R<MODSLP_WW_SPEC>;
#[doc = "Register `MODSLP_WW` writer"]
pub type W = crate::W<MODSLP_WW_SPEC>;
#[doc = "Field `MODSLP_WW_INI` reader - "]
pub type MODSLP_WW_INI_R = crate::FieldReader<u16>;
#[doc = "Field `MODSLP_WW_INI` writer - "]
pub type MODSLP_WW_INI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MODSLP_WW_DELTA` reader - "]
pub type MODSLP_WW_DELTA_R = crate::FieldReader<u16>;
#[doc = "Field `MODSLP_WW_DELTA` writer - "]
pub type MODSLP_WW_DELTA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn modslp_ww_ini(&self) -> MODSLP_WW_INI_R {
        MODSLP_WW_INI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn modslp_ww_delta(&self) -> MODSLP_WW_DELTA_R {
        MODSLP_WW_DELTA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODSLP_WW")
            .field("modslp_ww_ini", &self.modslp_ww_ini())
            .field("modslp_ww_delta", &self.modslp_ww_delta())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn modslp_ww_ini(&mut self) -> MODSLP_WW_INI_W<'_, MODSLP_WW_SPEC> {
        MODSLP_WW_INI_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn modslp_ww_delta(&mut self) -> MODSLP_WW_DELTA_W<'_, MODSLP_WW_SPEC> {
        MODSLP_WW_DELTA_W::new(self, 16)
    }
}
#[doc = "MODSLP_WW\n\nYou can [`read`](crate::Reg::read) this register and get [`modslp_ww::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modslp_ww::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODSLP_WW_SPEC;
impl crate::RegisterSpec for MODSLP_WW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modslp_ww::R`](R) reader structure"]
impl crate::Readable for MODSLP_WW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modslp_ww::W`](W) writer structure"]
impl crate::Writable for MODSLP_WW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODSLP_WW to value 0"]
impl crate::Resettable for MODSLP_WW_SPEC {}
