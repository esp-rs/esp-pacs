#[doc = "Register `SAR_TOUCH_THRES4` reader"]
pub type R = crate::R<SAR_TOUCH_THRES4_SPEC>;
#[doc = "Register `SAR_TOUCH_THRES4` writer"]
pub type W = crate::W<SAR_TOUCH_THRES4_SPEC>;
#[doc = "Field `TOUCH_OUT_TH7` reader - the threshold for touch pad 7"]
pub type TOUCH_OUT_TH7_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_OUT_TH7` writer - the threshold for touch pad 7"]
pub type TOUCH_OUT_TH7_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_OUT_TH6` reader - the threshold for touch pad 6"]
pub type TOUCH_OUT_TH6_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_OUT_TH6` writer - the threshold for touch pad 6"]
pub type TOUCH_OUT_TH6_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the threshold for touch pad 7"]
    #[inline(always)]
    pub fn touch_out_th7(&self) -> TOUCH_OUT_TH7_R {
        TOUCH_OUT_TH7_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the threshold for touch pad 6"]
    #[inline(always)]
    pub fn touch_out_th6(&self) -> TOUCH_OUT_TH6_R {
        TOUCH_OUT_TH6_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES4")
            .field("touch_out_th7", &self.touch_out_th7())
            .field("touch_out_th6", &self.touch_out_th6())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - the threshold for touch pad 7"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th7(&mut self) -> TOUCH_OUT_TH7_W<SAR_TOUCH_THRES4_SPEC> {
        TOUCH_OUT_TH7_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the threshold for touch pad 6"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th6(&mut self) -> TOUCH_OUT_TH6_W<SAR_TOUCH_THRES4_SPEC> {
        TOUCH_OUT_TH6_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_THRES4_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_thres4::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_thres4::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES4 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES4_SPEC {
    const RESET_VALUE: u32 = 0;
}
