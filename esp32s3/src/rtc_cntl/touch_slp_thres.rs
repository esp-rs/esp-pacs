#[doc = "Register `TOUCH_SLP_THRES` reader"]
pub type R = crate::R<TOUCH_SLP_THRES_SPEC>;
#[doc = "Register `TOUCH_SLP_THRES` writer"]
pub type W = crate::W<TOUCH_SLP_THRES_SPEC>;
#[doc = "Field `TOUCH_SLP_TH` reader - the threshold for sleep touch pad"]
pub type TOUCH_SLP_TH_R = crate::FieldReader<u32>;
#[doc = "Field `TOUCH_SLP_TH` writer - the threshold for sleep touch pad"]
pub type TOUCH_SLP_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `TOUCH_SLP_APPROACH_EN` reader - sleep pad approach function enable"]
pub type TOUCH_SLP_APPROACH_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_SLP_APPROACH_EN` writer - sleep pad approach function enable"]
pub type TOUCH_SLP_APPROACH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SLP_PAD` reader - configure which pad as slp pad"]
pub type TOUCH_SLP_PAD_R = crate::FieldReader;
#[doc = "Field `TOUCH_SLP_PAD` writer - configure which pad as slp pad"]
pub type TOUCH_SLP_PAD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:21 - the threshold for sleep touch pad"]
    #[inline(always)]
    pub fn touch_slp_th(&self) -> TOUCH_SLP_TH_R {
        TOUCH_SLP_TH_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 26 - sleep pad approach function enable"]
    #[inline(always)]
    pub fn touch_slp_approach_en(&self) -> TOUCH_SLP_APPROACH_EN_R {
        TOUCH_SLP_APPROACH_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - configure which pad as slp pad"]
    #[inline(always)]
    pub fn touch_slp_pad(&self) -> TOUCH_SLP_PAD_R {
        TOUCH_SLP_PAD_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_SLP_THRES")
            .field("touch_slp_th", &self.touch_slp_th())
            .field("touch_slp_approach_en", &self.touch_slp_approach_en())
            .field("touch_slp_pad", &self.touch_slp_pad())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - the threshold for sleep touch pad"]
    #[inline(always)]
    pub fn touch_slp_th(&mut self) -> TOUCH_SLP_TH_W<TOUCH_SLP_THRES_SPEC> {
        TOUCH_SLP_TH_W::new(self, 0)
    }
    #[doc = "Bit 26 - sleep pad approach function enable"]
    #[inline(always)]
    pub fn touch_slp_approach_en(&mut self) -> TOUCH_SLP_APPROACH_EN_W<TOUCH_SLP_THRES_SPEC> {
        TOUCH_SLP_APPROACH_EN_W::new(self, 26)
    }
    #[doc = "Bits 27:31 - configure which pad as slp pad"]
    #[inline(always)]
    pub fn touch_slp_pad(&mut self) -> TOUCH_SLP_PAD_W<TOUCH_SLP_THRES_SPEC> {
        TOUCH_SLP_PAD_W::new(self, 27)
    }
}
#[doc = "configure touch controller\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_slp_thres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_slp_thres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_SLP_THRES_SPEC;
impl crate::RegisterSpec for TOUCH_SLP_THRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_slp_thres::R`](R) reader structure"]
impl crate::Readable for TOUCH_SLP_THRES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_slp_thres::W`](W) writer structure"]
impl crate::Writable for TOUCH_SLP_THRES_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_SLP_THRES to value 0x7800_0000"]
impl crate::Resettable for TOUCH_SLP_THRES_SPEC {
    const RESET_VALUE: u32 = 0x7800_0000;
}
