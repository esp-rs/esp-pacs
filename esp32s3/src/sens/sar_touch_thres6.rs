///Register `SAR_TOUCH_THRES6` reader
pub type R = crate::R<SAR_TOUCH_THRES6_SPEC>;
///Register `SAR_TOUCH_THRES6` writer
pub type W = crate::W<SAR_TOUCH_THRES6_SPEC>;
///Field `SAR_TOUCH_OUT_TH6` reader - Finger threshold for touch pad 6
pub type SAR_TOUCH_OUT_TH6_R = crate::FieldReader<u32>;
///Field `SAR_TOUCH_OUT_TH6` writer - Finger threshold for touch pad 6
pub type SAR_TOUCH_OUT_TH6_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:21 - Finger threshold for touch pad 6
    #[inline(always)]
    pub fn sar_touch_out_th6(&self) -> SAR_TOUCH_OUT_TH6_R {
        SAR_TOUCH_OUT_TH6_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES6")
            .field("sar_touch_out_th6", &self.sar_touch_out_th6())
            .finish()
    }
}
impl W {
    ///Bits 0:21 - Finger threshold for touch pad 6
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_out_th6(&mut self) -> SAR_TOUCH_OUT_TH6_W<SAR_TOUCH_THRES6_SPEC> {
        SAR_TOUCH_OUT_TH6_W::new(self, 0)
    }
}
/**configure touch thres of touch pad

You can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_TOUCH_THRES6_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_touch_thres6::R`](R) reader structure
impl crate::Readable for SAR_TOUCH_THRES6_SPEC {}
///`write(|w| ..)` method takes [`sar_touch_thres6::W`](W) writer structure
impl crate::Writable for SAR_TOUCH_THRES6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_TOUCH_THRES6 to value 0
impl crate::Resettable for SAR_TOUCH_THRES6_SPEC {
    const RESET_VALUE: u32 = 0;
}
