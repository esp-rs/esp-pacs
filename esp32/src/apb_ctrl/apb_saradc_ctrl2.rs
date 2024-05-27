///Register `APB_SARADC_CTRL2` reader
pub type R = crate::R<APB_SARADC_CTRL2_SPEC>;
///Register `APB_SARADC_CTRL2` writer
pub type W = crate::W<APB_SARADC_CTRL2_SPEC>;
///Field `SARADC_MEAS_NUM_LIMIT` reader -
pub type SARADC_MEAS_NUM_LIMIT_R = crate::BitReader;
///Field `SARADC_MEAS_NUM_LIMIT` writer -
pub type SARADC_MEAS_NUM_LIMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SARADC_MAX_MEAS_NUM` reader - max conversion number
pub type SARADC_MAX_MEAS_NUM_R = crate::FieldReader;
///Field `SARADC_MAX_MEAS_NUM` writer - max conversion number
pub type SARADC_MAX_MEAS_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SARADC_SAR1_INV` reader - 1: data to DIG ADC1 CTRL is inverted otherwise not
pub type SARADC_SAR1_INV_R = crate::BitReader;
///Field `SARADC_SAR1_INV` writer - 1: data to DIG ADC1 CTRL is inverted otherwise not
pub type SARADC_SAR1_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SARADC_SAR2_INV` reader - 1: data to DIG ADC2 CTRL is inverted otherwise not
pub type SARADC_SAR2_INV_R = crate::BitReader;
///Field `SARADC_SAR2_INV` writer - 1: data to DIG ADC2 CTRL is inverted otherwise not
pub type SARADC_SAR2_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn saradc_meas_num_limit(&self) -> SARADC_MEAS_NUM_LIMIT_R {
        SARADC_MEAS_NUM_LIMIT_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:8 - max conversion number
    #[inline(always)]
    pub fn saradc_max_meas_num(&self) -> SARADC_MAX_MEAS_NUM_R {
        SARADC_MAX_MEAS_NUM_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    ///Bit 9 - 1: data to DIG ADC1 CTRL is inverted otherwise not
    #[inline(always)]
    pub fn saradc_sar1_inv(&self) -> SARADC_SAR1_INV_R {
        SARADC_SAR1_INV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - 1: data to DIG ADC2 CTRL is inverted otherwise not
    #[inline(always)]
    pub fn saradc_sar2_inv(&self) -> SARADC_SAR2_INV_R {
        SARADC_SAR2_INV_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SARADC_CTRL2")
            .field("saradc_meas_num_limit", &self.saradc_meas_num_limit())
            .field("saradc_max_meas_num", &self.saradc_max_meas_num())
            .field("saradc_sar1_inv", &self.saradc_sar1_inv())
            .field("saradc_sar2_inv", &self.saradc_sar2_inv())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn saradc_meas_num_limit(&mut self) -> SARADC_MEAS_NUM_LIMIT_W<APB_SARADC_CTRL2_SPEC> {
        SARADC_MEAS_NUM_LIMIT_W::new(self, 0)
    }
    ///Bits 1:8 - max conversion number
    #[inline(always)]
    #[must_use]
    pub fn saradc_max_meas_num(&mut self) -> SARADC_MAX_MEAS_NUM_W<APB_SARADC_CTRL2_SPEC> {
        SARADC_MAX_MEAS_NUM_W::new(self, 1)
    }
    ///Bit 9 - 1: data to DIG ADC1 CTRL is inverted otherwise not
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar1_inv(&mut self) -> SARADC_SAR1_INV_W<APB_SARADC_CTRL2_SPEC> {
        SARADC_SAR1_INV_W::new(self, 9)
    }
    ///Bit 10 - 1: data to DIG ADC2 CTRL is inverted otherwise not
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar2_inv(&mut self) -> SARADC_SAR2_INV_W<APB_SARADC_CTRL2_SPEC> {
        SARADC_SAR2_INV_W::new(self, 10)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_saradc_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APB_SARADC_CTRL2_SPEC;
impl crate::RegisterSpec for APB_SARADC_CTRL2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`apb_saradc_ctrl2::R`](R) reader structure
impl crate::Readable for APB_SARADC_CTRL2_SPEC {}
///`write(|w| ..)` method takes [`apb_saradc_ctrl2::W`](W) writer structure
impl crate::Writable for APB_SARADC_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APB_SARADC_CTRL2 to value 0x01fe
impl crate::Resettable for APB_SARADC_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x01fe;
}
