///Register `SAR_MEAS2_CTRL2` reader
pub type R = crate::R<SAR_MEAS2_CTRL2_SPEC>;
///Register `SAR_MEAS2_CTRL2` writer
pub type W = crate::W<SAR_MEAS2_CTRL2_SPEC>;
///Field `MEAS2_DATA_SAR` reader - SAR ADC2 data
pub type MEAS2_DATA_SAR_R = crate::FieldReader<u16>;
///Field `MEAS2_DONE_SAR` reader - SAR ADC2 conversion done indication
pub type MEAS2_DONE_SAR_R = crate::BitReader;
///Field `MEAS2_START_SAR` reader - SAR ADC2 controller (in RTC) starts conversion
pub type MEAS2_START_SAR_R = crate::BitReader;
///Field `MEAS2_START_SAR` writer - SAR ADC2 controller (in RTC) starts conversion
pub type MEAS2_START_SAR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MEAS2_START_FORCE` reader - 1: SAR ADC2 controller (in RTC) is started by SW
pub type MEAS2_START_FORCE_R = crate::BitReader;
///Field `MEAS2_START_FORCE` writer - 1: SAR ADC2 controller (in RTC) is started by SW
pub type MEAS2_START_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR2_EN_PAD` reader - SAR ADC2 pad enable bitmap
pub type SAR2_EN_PAD_R = crate::FieldReader<u16>;
///Field `SAR2_EN_PAD` writer - SAR ADC2 pad enable bitmap
pub type SAR2_EN_PAD_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `SAR2_EN_PAD_FORCE` reader - 1: SAR ADC2 pad enable bitmap is controlled by SW
pub type SAR2_EN_PAD_FORCE_R = crate::BitReader;
///Field `SAR2_EN_PAD_FORCE` writer - 1: SAR ADC2 pad enable bitmap is controlled by SW
pub type SAR2_EN_PAD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - SAR ADC2 data
    #[inline(always)]
    pub fn meas2_data_sar(&self) -> MEAS2_DATA_SAR_R {
        MEAS2_DATA_SAR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - SAR ADC2 conversion done indication
    #[inline(always)]
    pub fn meas2_done_sar(&self) -> MEAS2_DONE_SAR_R {
        MEAS2_DONE_SAR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SAR ADC2 controller (in RTC) starts conversion
    #[inline(always)]
    pub fn meas2_start_sar(&self) -> MEAS2_START_SAR_R {
        MEAS2_START_SAR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - 1: SAR ADC2 controller (in RTC) is started by SW
    #[inline(always)]
    pub fn meas2_start_force(&self) -> MEAS2_START_FORCE_R {
        MEAS2_START_FORCE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:30 - SAR ADC2 pad enable bitmap
    #[inline(always)]
    pub fn sar2_en_pad(&self) -> SAR2_EN_PAD_R {
        SAR2_EN_PAD_R::new(((self.bits >> 19) & 0x0fff) as u16)
    }
    ///Bit 31 - 1: SAR ADC2 pad enable bitmap is controlled by SW
    #[inline(always)]
    pub fn sar2_en_pad_force(&self) -> SAR2_EN_PAD_FORCE_R {
        SAR2_EN_PAD_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS2_CTRL2")
            .field("meas2_data_sar", &self.meas2_data_sar())
            .field("meas2_done_sar", &self.meas2_done_sar())
            .field("meas2_start_sar", &self.meas2_start_sar())
            .field("meas2_start_force", &self.meas2_start_force())
            .field("sar2_en_pad", &self.sar2_en_pad())
            .field("sar2_en_pad_force", &self.sar2_en_pad_force())
            .finish()
    }
}
impl W {
    ///Bit 17 - SAR ADC2 controller (in RTC) starts conversion
    #[inline(always)]
    #[must_use]
    pub fn meas2_start_sar(&mut self) -> MEAS2_START_SAR_W<SAR_MEAS2_CTRL2_SPEC> {
        MEAS2_START_SAR_W::new(self, 17)
    }
    ///Bit 18 - 1: SAR ADC2 controller (in RTC) is started by SW
    #[inline(always)]
    #[must_use]
    pub fn meas2_start_force(&mut self) -> MEAS2_START_FORCE_W<SAR_MEAS2_CTRL2_SPEC> {
        MEAS2_START_FORCE_W::new(self, 18)
    }
    ///Bits 19:30 - SAR ADC2 pad enable bitmap
    #[inline(always)]
    #[must_use]
    pub fn sar2_en_pad(&mut self) -> SAR2_EN_PAD_W<SAR_MEAS2_CTRL2_SPEC> {
        SAR2_EN_PAD_W::new(self, 19)
    }
    ///Bit 31 - 1: SAR ADC2 pad enable bitmap is controlled by SW
    #[inline(always)]
    #[must_use]
    pub fn sar2_en_pad_force(&mut self) -> SAR2_EN_PAD_FORCE_W<SAR_MEAS2_CTRL2_SPEC> {
        SAR2_EN_PAD_FORCE_W::new(self, 31)
    }
}
/**configure saradc2 controller

You can [`read`](crate::generic::Reg::read) this register and get [`sar_meas2_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas2_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_MEAS2_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_MEAS2_CTRL2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_meas2_ctrl2::R`](R) reader structure
impl crate::Readable for SAR_MEAS2_CTRL2_SPEC {}
///`write(|w| ..)` method takes [`sar_meas2_ctrl2::W`](W) writer structure
impl crate::Writable for SAR_MEAS2_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_MEAS2_CTRL2 to value 0
impl crate::Resettable for SAR_MEAS2_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0;
}
