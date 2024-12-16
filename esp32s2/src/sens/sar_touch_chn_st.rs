#[doc = "Register `SAR_TOUCH_CHN_ST` reader"]
pub type R = crate::R<SAR_TOUCH_CHN_ST_SPEC>;
#[doc = "Register `SAR_TOUCH_CHN_ST` writer"]
pub type W = crate::W<SAR_TOUCH_CHN_ST_SPEC>;
#[doc = "Field `TOUCH_PAD_ACTIVE` reader - Touch active status"]
pub type TOUCH_PAD_ACTIVE_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_CHANNEL_CLR` writer - Clear touch channel"]
pub type TOUCH_CHANNEL_CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `TOUCH_MEAS_DONE` reader - Signal flag that indicates one touch pad is done."]
pub type TOUCH_MEAS_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:14 - Touch active status"]
    #[inline(always)]
    pub fn touch_pad_active(&self) -> TOUCH_PAD_ACTIVE_R {
        TOUCH_PAD_ACTIVE_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Signal flag that indicates one touch pad is done."]
    #[inline(always)]
    pub fn touch_meas_done(&self) -> TOUCH_MEAS_DONE_R {
        TOUCH_MEAS_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_CHN_ST")
            .field("touch_pad_active", &self.touch_pad_active())
            .field("touch_meas_done", &self.touch_meas_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 15:29 - Clear touch channel"]
    #[inline(always)]
    pub fn touch_channel_clr(&mut self) -> TOUCH_CHANNEL_CLR_W<SAR_TOUCH_CHN_ST_SPEC> {
        TOUCH_CHANNEL_CLR_W::new(self, 15)
    }
}
#[doc = "Touch channel status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_chn_st::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_chn_st::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_CHN_ST_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CHN_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_chn_st::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_CHN_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_chn_st::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_CHN_ST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_CHN_ST to value 0"]
impl crate::Resettable for SAR_TOUCH_CHN_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
