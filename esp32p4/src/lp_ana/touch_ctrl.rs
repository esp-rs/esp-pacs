#[doc = "Register `TOUCH_CTRL` reader"]
pub type R = crate::R<TOUCH_CTRL_SPEC>;
#[doc = "Register `TOUCH_CTRL` writer"]
pub type W = crate::W<TOUCH_CTRL_SPEC>;
#[doc = "Field `TOUCH_UPDATE_BASELINE_FREQ_SEL` reader - Configure the frequency point for software to update the baseline"]
pub type TOUCH_UPDATE_BASELINE_FREQ_SEL_R = crate::FieldReader;
#[doc = "Field `TOUCH_UPDATE_BASELINE_FREQ_SEL` writer - Configure the frequency point for software to update the baseline"]
pub type TOUCH_UPDATE_BASELINE_FREQ_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_UPDATE_BASELINE_PAD_SEL` reader - Configure the channel for software to update the baseline."]
pub type TOUCH_UPDATE_BASELINE_PAD_SEL_R = crate::FieldReader;
#[doc = "Field `TOUCH_UPDATE_BASELINE_PAD_SEL` writer - Configure the channel for software to update the baseline."]
pub type TOUCH_UPDATE_BASELINE_PAD_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FREQ_SCAN_CNT_RISE` reader - Configure the number of hit frequency points that need to be determined for touch in frequency hopping mode."]
pub type FREQ_SCAN_CNT_RISE_R = crate::FieldReader;
#[doc = "Field `FREQ_SCAN_CNT_RISE` writer - Configure the number of hit frequency points that need to be determined for touch in frequency hopping mode."]
pub type FREQ_SCAN_CNT_RISE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Configure the frequency point for software to update the baseline"]
    #[inline(always)]
    pub fn touch_update_baseline_freq_sel(&self) -> TOUCH_UPDATE_BASELINE_FREQ_SEL_R {
        TOUCH_UPDATE_BASELINE_FREQ_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Configure the channel for software to update the baseline."]
    #[inline(always)]
    pub fn touch_update_baseline_pad_sel(&self) -> TOUCH_UPDATE_BASELINE_PAD_SEL_R {
        TOUCH_UPDATE_BASELINE_PAD_SEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Configure the number of hit frequency points that need to be determined for touch in frequency hopping mode."]
    #[inline(always)]
    pub fn freq_scan_cnt_rise(&self) -> FREQ_SCAN_CNT_RISE_R {
        FREQ_SCAN_CNT_RISE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_CTRL")
            .field(
                "touch_update_baseline_freq_sel",
                &self.touch_update_baseline_freq_sel(),
            )
            .field(
                "touch_update_baseline_pad_sel",
                &self.touch_update_baseline_pad_sel(),
            )
            .field("freq_scan_cnt_rise", &self.freq_scan_cnt_rise())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure the frequency point for software to update the baseline"]
    #[inline(always)]
    pub fn touch_update_baseline_freq_sel(
        &mut self,
    ) -> TOUCH_UPDATE_BASELINE_FREQ_SEL_W<'_, TOUCH_CTRL_SPEC> {
        TOUCH_UPDATE_BASELINE_FREQ_SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:5 - Configure the channel for software to update the baseline."]
    #[inline(always)]
    pub fn touch_update_baseline_pad_sel(
        &mut self,
    ) -> TOUCH_UPDATE_BASELINE_PAD_SEL_W<'_, TOUCH_CTRL_SPEC> {
        TOUCH_UPDATE_BASELINE_PAD_SEL_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - Configure the number of hit frequency points that need to be determined for touch in frequency hopping mode."]
    #[inline(always)]
    pub fn freq_scan_cnt_rise(&mut self) -> FREQ_SCAN_CNT_RISE_W<'_, TOUCH_CTRL_SPEC> {
        FREQ_SCAN_CNT_RISE_W::new(self, 6)
    }
}
#[doc = "Touch Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_ctrl::R`](R) reader structure"]
impl crate::Readable for TOUCH_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_ctrl::W`](W) writer structure"]
impl crate::Writable for TOUCH_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_CTRL to value 0x40"]
impl crate::Resettable for TOUCH_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
