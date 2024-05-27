///Register `SAR_TOUCH_CTRL1` reader
pub type R = crate::R<SAR_TOUCH_CTRL1_SPEC>;
///Register `SAR_TOUCH_CTRL1` writer
pub type W = crate::W<SAR_TOUCH_CTRL1_SPEC>;
///Field `TOUCH_MEAS_DELAY` reader - the meas length (in 8MHz)
pub type TOUCH_MEAS_DELAY_R = crate::FieldReader<u16>;
///Field `TOUCH_MEAS_DELAY` writer - the meas length (in 8MHz)
pub type TOUCH_MEAS_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TOUCH_XPD_WAIT` reader - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD
pub type TOUCH_XPD_WAIT_R = crate::FieldReader;
///Field `TOUCH_XPD_WAIT` writer - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD
pub type TOUCH_XPD_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TOUCH_OUT_SEL` reader - 1: when the counter is greater then the threshold the touch pad is considered as "touched" 0: when the counter is less than the threshold the touch pad is considered as "touched"
pub type TOUCH_OUT_SEL_R = crate::BitReader;
///Field `TOUCH_OUT_SEL` writer - 1: when the counter is greater then the threshold the touch pad is considered as "touched" 0: when the counter is less than the threshold the touch pad is considered as "touched"
pub type TOUCH_OUT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH_OUT_1EN` reader - 1: wakeup interrupt is generated if SET1 is "touched" 0: wakeup interrupt is generated only if SET1 &amp; SET2 is both "touched"
pub type TOUCH_OUT_1EN_R = crate::BitReader;
///Field `TOUCH_OUT_1EN` writer - 1: wakeup interrupt is generated if SET1 is "touched" 0: wakeup interrupt is generated only if SET1 &amp; SET2 is both "touched"
pub type TOUCH_OUT_1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XPD_HALL_FORCE` reader - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor
pub type XPD_HALL_FORCE_R = crate::BitReader;
///Field `XPD_HALL_FORCE` writer - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor
pub type XPD_HALL_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HALL_PHASE_FORCE` reader - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor
pub type HALL_PHASE_FORCE_R = crate::BitReader;
///Field `HALL_PHASE_FORCE` writer - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor
pub type HALL_PHASE_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - the meas length (in 8MHz)
    #[inline(always)]
    pub fn touch_meas_delay(&self) -> TOUCH_MEAS_DELAY_R {
        TOUCH_MEAS_DELAY_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD
    #[inline(always)]
    pub fn touch_xpd_wait(&self) -> TOUCH_XPD_WAIT_R {
        TOUCH_XPD_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - 1: when the counter is greater then the threshold the touch pad is considered as "touched" 0: when the counter is less than the threshold the touch pad is considered as "touched"
    #[inline(always)]
    pub fn touch_out_sel(&self) -> TOUCH_OUT_SEL_R {
        TOUCH_OUT_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - 1: wakeup interrupt is generated if SET1 is "touched" 0: wakeup interrupt is generated only if SET1 &amp; SET2 is both "touched"
    #[inline(always)]
    pub fn touch_out_1en(&self) -> TOUCH_OUT_1EN_R {
        TOUCH_OUT_1EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor
    #[inline(always)]
    pub fn xpd_hall_force(&self) -> XPD_HALL_FORCE_R {
        XPD_HALL_FORCE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor
    #[inline(always)]
    pub fn hall_phase_force(&self) -> HALL_PHASE_FORCE_R {
        HALL_PHASE_FORCE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_CTRL1")
            .field("touch_meas_delay", &self.touch_meas_delay())
            .field("touch_xpd_wait", &self.touch_xpd_wait())
            .field("touch_out_sel", &self.touch_out_sel())
            .field("touch_out_1en", &self.touch_out_1en())
            .field("xpd_hall_force", &self.xpd_hall_force())
            .field("hall_phase_force", &self.hall_phase_force())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - the meas length (in 8MHz)
    #[inline(always)]
    #[must_use]
    pub fn touch_meas_delay(&mut self) -> TOUCH_MEAS_DELAY_W<SAR_TOUCH_CTRL1_SPEC> {
        TOUCH_MEAS_DELAY_W::new(self, 0)
    }
    ///Bits 16:23 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD
    #[inline(always)]
    #[must_use]
    pub fn touch_xpd_wait(&mut self) -> TOUCH_XPD_WAIT_W<SAR_TOUCH_CTRL1_SPEC> {
        TOUCH_XPD_WAIT_W::new(self, 16)
    }
    ///Bit 24 - 1: when the counter is greater then the threshold the touch pad is considered as "touched" 0: when the counter is less than the threshold the touch pad is considered as "touched"
    #[inline(always)]
    #[must_use]
    pub fn touch_out_sel(&mut self) -> TOUCH_OUT_SEL_W<SAR_TOUCH_CTRL1_SPEC> {
        TOUCH_OUT_SEL_W::new(self, 24)
    }
    ///Bit 25 - 1: wakeup interrupt is generated if SET1 is "touched" 0: wakeup interrupt is generated only if SET1 &amp; SET2 is both "touched"
    #[inline(always)]
    #[must_use]
    pub fn touch_out_1en(&mut self) -> TOUCH_OUT_1EN_W<SAR_TOUCH_CTRL1_SPEC> {
        TOUCH_OUT_1EN_W::new(self, 25)
    }
    ///Bit 26 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor
    #[inline(always)]
    #[must_use]
    pub fn xpd_hall_force(&mut self) -> XPD_HALL_FORCE_W<SAR_TOUCH_CTRL1_SPEC> {
        XPD_HALL_FORCE_W::new(self, 26)
    }
    ///Bit 27 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor
    #[inline(always)]
    #[must_use]
    pub fn hall_phase_force(&mut self) -> HALL_PHASE_FORCE_W<SAR_TOUCH_CTRL1_SPEC> {
        HALL_PHASE_FORCE_W::new(self, 27)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_TOUCH_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CTRL1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_touch_ctrl1::R`](R) reader structure
impl crate::Readable for SAR_TOUCH_CTRL1_SPEC {}
///`write(|w| ..)` method takes [`sar_touch_ctrl1::W`](W) writer structure
impl crate::Writable for SAR_TOUCH_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_TOUCH_CTRL1 to value 0x0204_1000
impl crate::Resettable for SAR_TOUCH_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x0204_1000;
}
