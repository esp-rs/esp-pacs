#[doc = "Register `TOUCH_STS` reader"]
pub type R = crate::R<TOUCH_STS_SPEC>;
#[doc = "Field `TOUCH_CUR_ST` reader - current power state register for touch"]
pub type TOUCH_CUR_ST_R = crate::FieldReader;
#[doc = "Field `TOUCH_IN_POWER_ON` reader - 1:touch current in power on mode 0:touch not in power on mode"]
pub type TOUCH_IN_POWER_ON_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - current power state register for touch"]
    #[inline(always)]
    pub fn touch_cur_st(&self) -> TOUCH_CUR_ST_R {
        TOUCH_CUR_ST_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 1:touch current in power on mode 0:touch not in power on mode"]
    #[inline(always)]
    pub fn touch_in_power_on(&self) -> TOUCH_IN_POWER_ON_R {
        TOUCH_IN_POWER_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_STS")
            .field("touch_cur_st", &self.touch_cur_st())
            .field("touch_in_power_on", &self.touch_in_power_on())
            .finish()
    }
}
#[doc = "status register for TOUCH PWR\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_STS_SPEC;
impl crate::RegisterSpec for TOUCH_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_sts::R`](R) reader structure"]
impl crate::Readable for TOUCH_STS_SPEC {}
#[doc = "`reset()` method sets TOUCH_STS to value 0x04"]
impl crate::Resettable for TOUCH_STS_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
