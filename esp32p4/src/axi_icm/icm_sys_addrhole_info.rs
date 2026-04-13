#[doc = "Register `ICM_SYS_ADDRHOLE_INFO` reader"]
pub type R = crate::R<ICM_SYS_ADDRHOLE_INFO_SPEC>;
#[doc = "Field `ICM_SYS_ADDRHOLE_ID` reader - "]
pub type ICM_SYS_ADDRHOLE_ID_R = crate::FieldReader;
#[doc = "Field `ICM_SYS_ADDRHOLE_WR` reader - "]
pub type ICM_SYS_ADDRHOLE_WR_R = crate::BitReader;
#[doc = "Field `ICM_SYS_ADDRHOLE_SECURE` reader - "]
pub type ICM_SYS_ADDRHOLE_SECURE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn icm_sys_addrhole_id(&self) -> ICM_SYS_ADDRHOLE_ID_R {
        ICM_SYS_ADDRHOLE_ID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn icm_sys_addrhole_wr(&self) -> ICM_SYS_ADDRHOLE_WR_R {
        ICM_SYS_ADDRHOLE_WR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn icm_sys_addrhole_secure(&self) -> ICM_SYS_ADDRHOLE_SECURE_R {
        ICM_SYS_ADDRHOLE_SECURE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_SYS_ADDRHOLE_INFO")
            .field("icm_sys_addrhole_id", &self.icm_sys_addrhole_id())
            .field("icm_sys_addrhole_wr", &self.icm_sys_addrhole_wr())
            .field("icm_sys_addrhole_secure", &self.icm_sys_addrhole_secure())
            .finish()
    }
}
#[doc = "SYS address hole info\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_sys_addrhole_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_SYS_ADDRHOLE_INFO_SPEC;
impl crate::RegisterSpec for ICM_SYS_ADDRHOLE_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_sys_addrhole_info::R`](R) reader structure"]
impl crate::Readable for ICM_SYS_ADDRHOLE_INFO_SPEC {}
#[doc = "`reset()` method sets ICM_SYS_ADDRHOLE_INFO to value 0"]
impl crate::Resettable for ICM_SYS_ADDRHOLE_INFO_SPEC {}
