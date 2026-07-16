#[doc = "Register `DEBUG_RO` reader"]
pub type R = crate::R<DEBUG_RO_SPEC>;
#[doc = "Field `CPU_PWR_STATE` reader - need_des"]
pub type CPU_PWR_STATE_R = crate::FieldReader;
#[doc = "Field `IO_0P2A_OVDET` reader - need_des"]
pub type IO_0P2A_OVDET_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - need_des"]
    #[inline(always)]
    pub fn cpu_pwr_state(&self) -> CPU_PWR_STATE_R {
        CPU_PWR_STATE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn io_0p2a_ovdet(&self) -> IO_0P2A_OVDET_R {
        IO_0P2A_OVDET_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_RO")
            .field("cpu_pwr_state", &self.cpu_pwr_state())
            .field("io_0p2a_ovdet", &self.io_0p2a_ovdet())
            .finish()
    }
}
#[doc = "used for future potential eco, others don't care\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_ro::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_RO_SPEC;
impl crate::RegisterSpec for DEBUG_RO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_ro::R`](R) reader structure"]
impl crate::Readable for DEBUG_RO_SPEC {}
#[doc = "`reset()` method sets DEBUG_RO to value 0"]
impl crate::Resettable for DEBUG_RO_SPEC {}
