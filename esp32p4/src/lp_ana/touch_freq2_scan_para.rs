#[doc = "Register `TOUCH_FREQ2_SCAN_PARA` reader"]
pub type R = crate::R<TOUCH_FREQ2_SCAN_PARA_SPEC>;
#[doc = "Register `TOUCH_FREQ2_SCAN_PARA` writer"]
pub type W = crate::W<TOUCH_FREQ2_SCAN_PARA_SPEC>;
#[doc = "Field `TOUCH_FREQ2_DCAP_LPF` reader - need_des"]
pub type TOUCH_FREQ2_DCAP_LPF_R = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ2_DCAP_LPF` writer - need_des"]
pub type TOUCH_FREQ2_DCAP_LPF_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `TOUCH_FREQ2_DRES_LPF` reader - need_des"]
pub type TOUCH_FREQ2_DRES_LPF_R = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ2_DRES_LPF` writer - need_des"]
pub type TOUCH_FREQ2_DRES_LPF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_FREQ2_DRV_LS` reader - need_des"]
pub type TOUCH_FREQ2_DRV_LS_R = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ2_DRV_LS` writer - need_des"]
pub type TOUCH_FREQ2_DRV_LS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_FREQ2_DRV_HS` reader - need_des"]
pub type TOUCH_FREQ2_DRV_HS_R = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ2_DRV_HS` writer - need_des"]
pub type TOUCH_FREQ2_DRV_HS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TOUCH_FREQ2_DBIAS` reader - need_des"]
pub type TOUCH_FREQ2_DBIAS_R = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ2_DBIAS` writer - need_des"]
pub type TOUCH_FREQ2_DBIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_dcap_lpf(&self) -> TOUCH_FREQ2_DCAP_LPF_R {
        TOUCH_FREQ2_DCAP_LPF_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_dres_lpf(&self) -> TOUCH_FREQ2_DRES_LPF_R {
        TOUCH_FREQ2_DRES_LPF_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_drv_ls(&self) -> TOUCH_FREQ2_DRV_LS_R {
        TOUCH_FREQ2_DRV_LS_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:17 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_drv_hs(&self) -> TOUCH_FREQ2_DRV_HS_R {
        TOUCH_FREQ2_DRV_HS_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn touch_freq2_dbias(&self) -> TOUCH_FREQ2_DBIAS_R {
        TOUCH_FREQ2_DBIAS_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_FREQ2_SCAN_PARA")
            .field("touch_freq2_dcap_lpf", &self.touch_freq2_dcap_lpf().bits())
            .field("touch_freq2_dres_lpf", &self.touch_freq2_dres_lpf().bits())
            .field("touch_freq2_drv_ls", &self.touch_freq2_drv_ls().bits())
            .field("touch_freq2_drv_hs", &self.touch_freq2_drv_hs().bits())
            .field("touch_freq2_dbias", &self.touch_freq2_dbias().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_FREQ2_SCAN_PARA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_freq2_dcap_lpf(&mut self) -> TOUCH_FREQ2_DCAP_LPF_W<TOUCH_FREQ2_SCAN_PARA_SPEC> {
        TOUCH_FREQ2_DCAP_LPF_W::new(self, 0)
    }
    #[doc = "Bits 7:8 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_freq2_dres_lpf(&mut self) -> TOUCH_FREQ2_DRES_LPF_W<TOUCH_FREQ2_SCAN_PARA_SPEC> {
        TOUCH_FREQ2_DRES_LPF_W::new(self, 7)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_freq2_drv_ls(&mut self) -> TOUCH_FREQ2_DRV_LS_W<TOUCH_FREQ2_SCAN_PARA_SPEC> {
        TOUCH_FREQ2_DRV_LS_W::new(self, 9)
    }
    #[doc = "Bits 13:17 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_freq2_drv_hs(&mut self) -> TOUCH_FREQ2_DRV_HS_W<TOUCH_FREQ2_SCAN_PARA_SPEC> {
        TOUCH_FREQ2_DRV_HS_W::new(self, 13)
    }
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_freq2_dbias(&mut self) -> TOUCH_FREQ2_DBIAS_W<TOUCH_FREQ2_SCAN_PARA_SPEC> {
        TOUCH_FREQ2_DBIAS_W::new(self, 18)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_freq2_scan_para::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_freq2_scan_para::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_FREQ2_SCAN_PARA_SPEC;
impl crate::RegisterSpec for TOUCH_FREQ2_SCAN_PARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_freq2_scan_para::R`](R) reader structure"]
impl crate::Readable for TOUCH_FREQ2_SCAN_PARA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_freq2_scan_para::W`](W) writer structure"]
impl crate::Writable for TOUCH_FREQ2_SCAN_PARA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_FREQ2_SCAN_PARA to value 0"]
impl crate::Resettable for TOUCH_FREQ2_SCAN_PARA_SPEC {
    const RESET_VALUE: u32 = 0;
}
