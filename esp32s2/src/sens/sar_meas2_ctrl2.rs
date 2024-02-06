#[doc = "Register `SAR_MEAS2_CTRL2` reader"]
pub type R = crate::R<SAR_MEAS2_CTRL2_SPEC>;
#[doc = "Register `SAR_MEAS2_CTRL2` writer"]
pub type W = crate::W<SAR_MEAS2_CTRL2_SPEC>;
#[doc = "Field `MEAS2_DATA_SAR` reader - SAR ADC2 data."]
pub type MEAS2_DATA_SAR_R = crate::FieldReader<u16>;
#[doc = "Field `MEAS2_DONE_SAR` reader - Indicate SAR ADC2 conversion is done."]
pub type MEAS2_DONE_SAR_R = crate::BitReader;
#[doc = "Field `MEAS2_START_SAR` reader - SAR ADC2 controller (in RTC) starts conversion, active only when SENS_MEAS2_START_FORCE = 1."]
pub type MEAS2_START_SAR_R = crate::BitReader;
#[doc = "Field `MEAS2_START_SAR` writer - SAR ADC2 controller (in RTC) starts conversion, active only when SENS_MEAS2_START_FORCE = 1."]
pub type MEAS2_START_SAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEAS2_START_FORCE` reader - 1: SAR ADC2 controller (in RTC) is started by software. 0: SAR ADC2 controller is started by ULP coprocessor."]
pub type MEAS2_START_FORCE_R = crate::BitReader;
#[doc = "Field `MEAS2_START_FORCE` writer - 1: SAR ADC2 controller (in RTC) is started by software. 0: SAR ADC2 controller is started by ULP coprocessor."]
pub type MEAS2_START_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_EN_PAD` reader - SAR ADC2 pad enable bitmap, active only whenSENS_SAR2_EN_PAD_FORCE = 1."]
pub type SAR2_EN_PAD_R = crate::FieldReader<u16>;
#[doc = "Field `SAR2_EN_PAD` writer - SAR ADC2 pad enable bitmap, active only whenSENS_SAR2_EN_PAD_FORCE = 1."]
pub type SAR2_EN_PAD_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SAR2_EN_PAD_FORCE` reader - 1: SAR ADC2 pad enable bitmap is controlled by software. 0: SAR ADC2 pad enable bitmap is controlled by ULP coprocessor."]
pub type SAR2_EN_PAD_FORCE_R = crate::BitReader;
#[doc = "Field `SAR2_EN_PAD_FORCE` writer - 1: SAR ADC2 pad enable bitmap is controlled by software. 0: SAR ADC2 pad enable bitmap is controlled by ULP coprocessor."]
pub type SAR2_EN_PAD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - SAR ADC2 data."]
    #[inline(always)]
    pub fn meas2_data_sar(&self) -> MEAS2_DATA_SAR_R {
        MEAS2_DATA_SAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Indicate SAR ADC2 conversion is done."]
    #[inline(always)]
    pub fn meas2_done_sar(&self) -> MEAS2_DONE_SAR_R {
        MEAS2_DONE_SAR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAR ADC2 controller (in RTC) starts conversion, active only when SENS_MEAS2_START_FORCE = 1."]
    #[inline(always)]
    pub fn meas2_start_sar(&self) -> MEAS2_START_SAR_R {
        MEAS2_START_SAR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: SAR ADC2 controller (in RTC) is started by software. 0: SAR ADC2 controller is started by ULP coprocessor."]
    #[inline(always)]
    pub fn meas2_start_force(&self) -> MEAS2_START_FORCE_R {
        MEAS2_START_FORCE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:30 - SAR ADC2 pad enable bitmap, active only whenSENS_SAR2_EN_PAD_FORCE = 1."]
    #[inline(always)]
    pub fn sar2_en_pad(&self) -> SAR2_EN_PAD_R {
        SAR2_EN_PAD_R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - 1: SAR ADC2 pad enable bitmap is controlled by software. 0: SAR ADC2 pad enable bitmap is controlled by ULP coprocessor."]
    #[inline(always)]
    pub fn sar2_en_pad_force(&self) -> SAR2_EN_PAD_FORCE_R {
        SAR2_EN_PAD_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS2_CTRL2")
            .field(
                "meas2_data_sar",
                &format_args!("{}", self.meas2_data_sar().bits()),
            )
            .field(
                "meas2_done_sar",
                &format_args!("{}", self.meas2_done_sar().bit()),
            )
            .field(
                "meas2_start_sar",
                &format_args!("{}", self.meas2_start_sar().bit()),
            )
            .field(
                "meas2_start_force",
                &format_args!("{}", self.meas2_start_force().bit()),
            )
            .field(
                "sar2_en_pad",
                &format_args!("{}", self.sar2_en_pad().bits()),
            )
            .field(
                "sar2_en_pad_force",
                &format_args!("{}", self.sar2_en_pad_force().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_MEAS2_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 17 - SAR ADC2 controller (in RTC) starts conversion, active only when SENS_MEAS2_START_FORCE = 1."]
    #[inline(always)]
    #[must_use]
    pub fn meas2_start_sar(&mut self) -> MEAS2_START_SAR_W<SAR_MEAS2_CTRL2_SPEC> {
        MEAS2_START_SAR_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1: SAR ADC2 controller (in RTC) is started by software. 0: SAR ADC2 controller is started by ULP coprocessor."]
    #[inline(always)]
    #[must_use]
    pub fn meas2_start_force(&mut self) -> MEAS2_START_FORCE_W<SAR_MEAS2_CTRL2_SPEC> {
        MEAS2_START_FORCE_W::new(self, 18)
    }
    #[doc = "Bits 19:30 - SAR ADC2 pad enable bitmap, active only whenSENS_SAR2_EN_PAD_FORCE = 1."]
    #[inline(always)]
    #[must_use]
    pub fn sar2_en_pad(&mut self) -> SAR2_EN_PAD_W<SAR_MEAS2_CTRL2_SPEC> {
        SAR2_EN_PAD_W::new(self, 19)
    }
    #[doc = "Bit 31 - 1: SAR ADC2 pad enable bitmap is controlled by software. 0: SAR ADC2 pad enable bitmap is controlled by ULP coprocessor."]
    #[inline(always)]
    #[must_use]
    pub fn sar2_en_pad_force(&mut self) -> SAR2_EN_PAD_FORCE_W<SAR_MEAS2_CTRL2_SPEC> {
        SAR2_EN_PAD_FORCE_W::new(self, 31)
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
#[doc = "Control RTC ADC2 conversion and status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_meas2_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas2_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_MEAS2_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_MEAS2_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_meas2_ctrl2::R`](R) reader structure"]
impl crate::Readable for SAR_MEAS2_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_meas2_ctrl2::W`](W) writer structure"]
impl crate::Writable for SAR_MEAS2_CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_MEAS2_CTRL2 to value 0"]
impl crate::Resettable for SAR_MEAS2_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0;
}
