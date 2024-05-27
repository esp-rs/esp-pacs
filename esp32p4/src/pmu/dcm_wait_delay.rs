///Register `DCM_WAIT_DELAY` reader
pub type R = crate::R<DCM_WAIT_DELAY_SPEC>;
///Register `DCM_WAIT_DELAY` writer
pub type W = crate::W<DCM_WAIT_DELAY_SPEC>;
///Field `DCDC_PRE_DELAY` reader - DCDC pre-on/post off delay
pub type DCDC_PRE_DELAY_R = crate::FieldReader;
///Field `DCDC_PRE_DELAY` writer - DCDC pre-on/post off delay
pub type DCDC_PRE_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DCDC_RES_OFF_DELAY` reader - DCDC fb res off delay
pub type DCDC_RES_OFF_DELAY_R = crate::FieldReader;
///Field `DCDC_RES_OFF_DELAY` writer - DCDC fb res off delay
pub type DCDC_RES_OFF_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DCDC_STABLE_DELAY` reader - DCDC stable delay
pub type DCDC_STABLE_DELAY_R = crate::FieldReader<u16>;
///Field `DCDC_STABLE_DELAY` writer - DCDC stable delay
pub type DCDC_STABLE_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:7 - DCDC pre-on/post off delay
    #[inline(always)]
    pub fn dcdc_pre_delay(&self) -> DCDC_PRE_DELAY_R {
        DCDC_PRE_DELAY_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DCDC fb res off delay
    #[inline(always)]
    pub fn dcdc_res_off_delay(&self) -> DCDC_RES_OFF_DELAY_R {
        DCDC_RES_OFF_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:25 - DCDC stable delay
    #[inline(always)]
    pub fn dcdc_stable_delay(&self) -> DCDC_STABLE_DELAY_R {
        DCDC_STABLE_DELAY_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCM_WAIT_DELAY")
            .field("dcdc_pre_delay", &self.dcdc_pre_delay())
            .field("dcdc_res_off_delay", &self.dcdc_res_off_delay())
            .field("dcdc_stable_delay", &self.dcdc_stable_delay())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DCDC pre-on/post off delay
    #[inline(always)]
    #[must_use]
    pub fn dcdc_pre_delay(&mut self) -> DCDC_PRE_DELAY_W<DCM_WAIT_DELAY_SPEC> {
        DCDC_PRE_DELAY_W::new(self, 0)
    }
    ///Bits 8:15 - DCDC fb res off delay
    #[inline(always)]
    #[must_use]
    pub fn dcdc_res_off_delay(&mut self) -> DCDC_RES_OFF_DELAY_W<DCM_WAIT_DELAY_SPEC> {
        DCDC_RES_OFF_DELAY_W::new(self, 8)
    }
    ///Bits 16:25 - DCDC stable delay
    #[inline(always)]
    #[must_use]
    pub fn dcdc_stable_delay(&mut self) -> DCDC_STABLE_DELAY_W<DCM_WAIT_DELAY_SPEC> {
        DCDC_STABLE_DELAY_W::new(self, 16)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`dcm_wait_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcm_wait_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DCM_WAIT_DELAY_SPEC;
impl crate::RegisterSpec for DCM_WAIT_DELAY_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dcm_wait_delay::R`](R) reader structure
impl crate::Readable for DCM_WAIT_DELAY_SPEC {}
///`write(|w| ..)` method takes [`dcm_wait_delay::W`](W) writer structure
impl crate::Writable for DCM_WAIT_DELAY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCM_WAIT_DELAY to value 0x004b_0205
impl crate::Resettable for DCM_WAIT_DELAY_SPEC {
    const RESET_VALUE: u32 = 0x004b_0205;
}
