#[doc = "Register `SAR_MEAS1_CTRL2` reader"]
pub type R = crate::R<SAR_MEAS1_CTRL2_SPEC>;
#[doc = "Register `SAR_MEAS1_CTRL2` writer"]
pub type W = crate::W<SAR_MEAS1_CTRL2_SPEC>;
#[doc = "Field `MEAS1_DATA_SAR` reader - SAR ADC1 data"]
pub type MEAS1_DATA_SAR_R = crate::FieldReader<u16>;
#[doc = "Field `MEAS1_DONE_SAR` reader - Indicate SAR ADC1 conversion is done."]
pub type MEAS1_DONE_SAR_R = crate::BitReader;
#[doc = "Field `MEAS1_START_SAR` reader - SAR ADC1 controller (in RTC) starts conversion, active only when SENS_MEAS1_START_FORCE = 1."]
pub type MEAS1_START_SAR_R = crate::BitReader;
#[doc = "Field `MEAS1_START_SAR` writer - SAR ADC1 controller (in RTC) starts conversion, active only when SENS_MEAS1_START_FORCE = 1."]
pub type MEAS1_START_SAR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MEAS1_START_FORCE` reader - 1: SAR ADC1 controller (in RTC) is started by software. 0: SAR ADC1 controller is started by ULP coprocessor."]
pub type MEAS1_START_FORCE_R = crate::BitReader;
#[doc = "Field `MEAS1_START_FORCE` writer - 1: SAR ADC1 controller (in RTC) is started by software. 0: SAR ADC1 controller is started by ULP coprocessor."]
pub type MEAS1_START_FORCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAR1_EN_PAD` reader - SAR ADC1 pad enable bitmap, active only when SENS_SAR1_EN_PAD_FORCE = 1."]
pub type SAR1_EN_PAD_R = crate::FieldReader<u16>;
#[doc = "Field `SAR1_EN_PAD` writer - SAR ADC1 pad enable bitmap, active only when SENS_SAR1_EN_PAD_FORCE = 1."]
pub type SAR1_EN_PAD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `SAR1_EN_PAD_FORCE` reader - 1: SAR ADC1 pad enable bitmap is controlled by software. 0: SAR ADC1 pad enable bitmap is controlled by ULP coprocessor."]
pub type SAR1_EN_PAD_FORCE_R = crate::BitReader;
#[doc = "Field `SAR1_EN_PAD_FORCE` writer - 1: SAR ADC1 pad enable bitmap is controlled by software. 0: SAR ADC1 pad enable bitmap is controlled by ULP coprocessor."]
pub type SAR1_EN_PAD_FORCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - SAR ADC1 data"]
    #[inline(always)]
    pub fn meas1_data_sar(&self) -> MEAS1_DATA_SAR_R {
        MEAS1_DATA_SAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Indicate SAR ADC1 conversion is done."]
    #[inline(always)]
    pub fn meas1_done_sar(&self) -> MEAS1_DONE_SAR_R {
        MEAS1_DONE_SAR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SAR ADC1 controller (in RTC) starts conversion, active only when SENS_MEAS1_START_FORCE = 1."]
    #[inline(always)]
    pub fn meas1_start_sar(&self) -> MEAS1_START_SAR_R {
        MEAS1_START_SAR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: SAR ADC1 controller (in RTC) is started by software. 0: SAR ADC1 controller is started by ULP coprocessor."]
    #[inline(always)]
    pub fn meas1_start_force(&self) -> MEAS1_START_FORCE_R {
        MEAS1_START_FORCE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:30 - SAR ADC1 pad enable bitmap, active only when SENS_SAR1_EN_PAD_FORCE = 1."]
    #[inline(always)]
    pub fn sar1_en_pad(&self) -> SAR1_EN_PAD_R {
        SAR1_EN_PAD_R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - 1: SAR ADC1 pad enable bitmap is controlled by software. 0: SAR ADC1 pad enable bitmap is controlled by ULP coprocessor."]
    #[inline(always)]
    pub fn sar1_en_pad_force(&self) -> SAR1_EN_PAD_FORCE_R {
        SAR1_EN_PAD_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS1_CTRL2")
            .field(
                "meas1_data_sar",
                &format_args!("{}", self.meas1_data_sar().bits()),
            )
            .field(
                "meas1_done_sar",
                &format_args!("{}", self.meas1_done_sar().bit()),
            )
            .field(
                "meas1_start_sar",
                &format_args!("{}", self.meas1_start_sar().bit()),
            )
            .field(
                "meas1_start_force",
                &format_args!("{}", self.meas1_start_force().bit()),
            )
            .field(
                "sar1_en_pad",
                &format_args!("{}", self.sar1_en_pad().bits()),
            )
            .field(
                "sar1_en_pad_force",
                &format_args!("{}", self.sar1_en_pad_force().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_MEAS1_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 17 - SAR ADC1 controller (in RTC) starts conversion, active only when SENS_MEAS1_START_FORCE = 1."]
    #[inline(always)]
    #[must_use]
    pub fn meas1_start_sar(&mut self) -> MEAS1_START_SAR_W<SAR_MEAS1_CTRL2_SPEC, 17> {
        MEAS1_START_SAR_W::new(self)
    }
    #[doc = "Bit 18 - 1: SAR ADC1 controller (in RTC) is started by software. 0: SAR ADC1 controller is started by ULP coprocessor."]
    #[inline(always)]
    #[must_use]
    pub fn meas1_start_force(&mut self) -> MEAS1_START_FORCE_W<SAR_MEAS1_CTRL2_SPEC, 18> {
        MEAS1_START_FORCE_W::new(self)
    }
    #[doc = "Bits 19:30 - SAR ADC1 pad enable bitmap, active only when SENS_SAR1_EN_PAD_FORCE = 1."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_en_pad(&mut self) -> SAR1_EN_PAD_W<SAR_MEAS1_CTRL2_SPEC, 19> {
        SAR1_EN_PAD_W::new(self)
    }
    #[doc = "Bit 31 - 1: SAR ADC1 pad enable bitmap is controlled by software. 0: SAR ADC1 pad enable bitmap is controlled by ULP coprocessor."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_en_pad_force(&mut self) -> SAR1_EN_PAD_FORCE_W<SAR_MEAS1_CTRL2_SPEC, 31> {
        SAR1_EN_PAD_FORCE_W::new(self)
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
#[doc = "Control RTC ADC1 conversion and status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_meas1_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas1_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_MEAS1_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_MEAS1_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_meas1_ctrl2::R`](R) reader structure"]
impl crate::Readable for SAR_MEAS1_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_meas1_ctrl2::W`](W) writer structure"]
impl crate::Writable for SAR_MEAS1_CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_MEAS1_CTRL2 to value 0"]
impl crate::Resettable for SAR_MEAS1_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
