#[doc = "Register `ICM_INT_ST` reader"]
pub type R = crate::R<ICM_INT_ST_SPEC>;
#[doc = "Field `DLOCK_INT_ST` reader - "]
pub type DLOCK_INT_ST_R = crate::BitReader;
#[doc = "Field `ICM_SYS_ADDRHOLE_INT_ST` reader - "]
pub type ICM_SYS_ADDRHOLE_INT_ST_R = crate::BitReader;
#[doc = "Field `ICM_CPU_ADDRHOLE_INT_ST` reader - "]
pub type ICM_CPU_ADDRHOLE_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dlock_int_st(&self) -> DLOCK_INT_ST_R {
        DLOCK_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icm_sys_addrhole_int_st(&self) -> ICM_SYS_ADDRHOLE_INT_ST_R {
        ICM_SYS_ADDRHOLE_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn icm_cpu_addrhole_int_st(&self) -> ICM_CPU_ADDRHOLE_INT_ST_R {
        ICM_CPU_ADDRHOLE_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_INT_ST")
            .field("dlock_int_st", &self.dlock_int_st())
            .field("icm_sys_addrhole_int_st", &self.icm_sys_addrhole_int_st())
            .field("icm_cpu_addrhole_int_st", &self.icm_cpu_addrhole_int_st())
            .finish()
    }
}
#[doc = "ICM interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_INT_ST_SPEC;
impl crate::RegisterSpec for ICM_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_int_st::R`](R) reader structure"]
impl crate::Readable for ICM_INT_ST_SPEC {}
#[doc = "`reset()` method sets ICM_INT_ST to value 0"]
impl crate::Resettable for ICM_INT_ST_SPEC {}
