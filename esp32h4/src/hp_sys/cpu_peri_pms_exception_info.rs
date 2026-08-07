#[doc = "Register `CPU_PERI_PMS_EXCEPTION_INFO` reader"]
pub type R = crate::R<CPU_PERI_PMS_EXCEPTION_INFO_SPEC>;
#[doc = "Field `CPU_PERI_PMS_EXCEPTION_DET` reader - Represents whether the cpu peripheral pms has been triggered.\\\\ 0: No triggered\\\\ 1: Has been triggered\\\\"]
pub type CPU_PERI_PMS_EXCEPTION_DET_R = crate::BitReader;
#[doc = "Field `CPU_PERI_PMS_EXCEPTION_ID` reader - Represents the master id when cpu peripheral pms has been triggered.\\\\"]
pub type CPU_PERI_PMS_EXCEPTION_ID_R = crate::FieldReader;
#[doc = "Field `CPU_PERI_PMS_EXCEPTION_MODE` reader - Represents the security mode when cpu peripheral pms has been triggered.\\\\"]
pub type CPU_PERI_PMS_EXCEPTION_MODE_R = crate::FieldReader;
#[doc = "Field `CPU_PERI_PMS_EXCEPTION_ADDR` reader - Represents the access address (bit23~bit0) when cpu peripheral pms has been triggered.\\\\"]
pub type CPU_PERI_PMS_EXCEPTION_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Represents whether the cpu peripheral pms has been triggered.\\\\ 0: No triggered\\\\ 1: Has been triggered\\\\"]
    #[inline(always)]
    pub fn cpu_peri_pms_exception_det(&self) -> CPU_PERI_PMS_EXCEPTION_DET_R {
        CPU_PERI_PMS_EXCEPTION_DET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Represents the master id when cpu peripheral pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn cpu_peri_pms_exception_id(&self) -> CPU_PERI_PMS_EXCEPTION_ID_R {
        CPU_PERI_PMS_EXCEPTION_ID_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - Represents the security mode when cpu peripheral pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn cpu_peri_pms_exception_mode(&self) -> CPU_PERI_PMS_EXCEPTION_MODE_R {
        CPU_PERI_PMS_EXCEPTION_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - Represents the access address (bit23~bit0) when cpu peripheral pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn cpu_peri_pms_exception_addr(&self) -> CPU_PERI_PMS_EXCEPTION_ADDR_R {
        CPU_PERI_PMS_EXCEPTION_ADDR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PERI_PMS_EXCEPTION_INFO")
            .field(
                "cpu_peri_pms_exception_det",
                &self.cpu_peri_pms_exception_det(),
            )
            .field(
                "cpu_peri_pms_exception_id",
                &self.cpu_peri_pms_exception_id(),
            )
            .field(
                "cpu_peri_pms_exception_mode",
                &self.cpu_peri_pms_exception_mode(),
            )
            .field(
                "cpu_peri_pms_exception_addr",
                &self.cpu_peri_pms_exception_addr(),
            )
            .finish()
    }
}
#[doc = "CPU Peripherals PMS exception info record register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_peri_pms_exception_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_PERI_PMS_EXCEPTION_INFO_SPEC;
impl crate::RegisterSpec for CPU_PERI_PMS_EXCEPTION_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_peri_pms_exception_info::R`](R) reader structure"]
impl crate::Readable for CPU_PERI_PMS_EXCEPTION_INFO_SPEC {}
#[doc = "`reset()` method sets CPU_PERI_PMS_EXCEPTION_INFO to value 0"]
impl crate::Resettable for CPU_PERI_PMS_EXCEPTION_INFO_SPEC {}
