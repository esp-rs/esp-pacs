#[doc = "Register `LPCPU_STS` reader"]
pub type R = crate::R<LPCPU_STS_SPEC>;
#[doc = "Field `LPCPU_CUR_ST` reader - current power state register for lpcpu"]
pub type LPCPU_CUR_ST_R = crate::FieldReader;
#[doc = "Field `LPCPU_IN_POWER_ON` reader - 1:lpcpu current in power on mode 0:lpcpu not in power on mode"]
pub type LPCPU_IN_POWER_ON_R = crate::BitReader;
#[doc = "Field `LPCPU_STALL_RDY` reader - 1:lpcpu stall not ready 0:lpcpu stall ready"]
pub type LPCPU_STALL_RDY_R = crate::BitReader;
#[doc = "Field `LPCPU_WAITI_RDY` reader - 1:lpcpu stall waiti ready 0:lpcpu stall waiti ready"]
pub type LPCPU_WAITI_RDY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - current power state register for lpcpu"]
    #[inline(always)]
    pub fn lpcpu_cur_st(&self) -> LPCPU_CUR_ST_R {
        LPCPU_CUR_ST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 1:lpcpu current in power on mode 0:lpcpu not in power on mode"]
    #[inline(always)]
    pub fn lpcpu_in_power_on(&self) -> LPCPU_IN_POWER_ON_R {
        LPCPU_IN_POWER_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1:lpcpu stall not ready 0:lpcpu stall ready"]
    #[inline(always)]
    pub fn lpcpu_stall_rdy(&self) -> LPCPU_STALL_RDY_R {
        LPCPU_STALL_RDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1:lpcpu stall waiti ready 0:lpcpu stall waiti ready"]
    #[inline(always)]
    pub fn lpcpu_waiti_rdy(&self) -> LPCPU_WAITI_RDY_R {
        LPCPU_WAITI_RDY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCPU_STS")
            .field("lpcpu_cur_st", &self.lpcpu_cur_st())
            .field("lpcpu_in_power_on", &self.lpcpu_in_power_on())
            .field("lpcpu_stall_rdy", &self.lpcpu_stall_rdy())
            .field("lpcpu_waiti_rdy", &self.lpcpu_waiti_rdy())
            .finish()
    }
}
#[doc = "status register for LPCPU PWR\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCPU_STS_SPEC;
impl crate::RegisterSpec for LPCPU_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcpu_sts::R`](R) reader structure"]
impl crate::Readable for LPCPU_STS_SPEC {}
#[doc = "`reset()` method sets LPCPU_STS to value 0x04"]
impl crate::Resettable for LPCPU_STS_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
