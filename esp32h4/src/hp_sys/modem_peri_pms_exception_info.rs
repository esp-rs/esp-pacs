#[doc = "Register `MODEM_PERI_PMS_EXCEPTION_INFO` reader"]
pub type R = crate::R<MODEM_PERI_PMS_EXCEPTION_INFO_SPEC>;
#[doc = "Field `MODEM_PERI_PMS_EXCEPTION_DET` reader - Represents whether the modem peripheral pms has been triggered.\\\\ 0: No triggered\\\\ 1: Has been triggered\\\\"]
pub type MODEM_PERI_PMS_EXCEPTION_DET_R = crate::BitReader;
#[doc = "Field `MODEM_PERI_PMS_EXCEPTION_ID` reader - Represents the master id when modem peripheral pms has been triggered.\\\\"]
pub type MODEM_PERI_PMS_EXCEPTION_ID_R = crate::FieldReader;
#[doc = "Field `MODEM_PERI_PMS_EXCEPTION_MODE` reader - Represents the security mode when modem peripheral pms has been triggered.\\\\"]
pub type MODEM_PERI_PMS_EXCEPTION_MODE_R = crate::FieldReader;
#[doc = "Field `MODEM_PERI_PMS_EXCEPTION_ADDR` reader - Represents the access address (bit23~bit0) when modem peripheral pms has been triggered.\\\\"]
pub type MODEM_PERI_PMS_EXCEPTION_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Represents whether the modem peripheral pms has been triggered.\\\\ 0: No triggered\\\\ 1: Has been triggered\\\\"]
    #[inline(always)]
    pub fn modem_peri_pms_exception_det(&self) -> MODEM_PERI_PMS_EXCEPTION_DET_R {
        MODEM_PERI_PMS_EXCEPTION_DET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Represents the master id when modem peripheral pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn modem_peri_pms_exception_id(&self) -> MODEM_PERI_PMS_EXCEPTION_ID_R {
        MODEM_PERI_PMS_EXCEPTION_ID_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - Represents the security mode when modem peripheral pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn modem_peri_pms_exception_mode(&self) -> MODEM_PERI_PMS_EXCEPTION_MODE_R {
        MODEM_PERI_PMS_EXCEPTION_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - Represents the access address (bit23~bit0) when modem peripheral pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn modem_peri_pms_exception_addr(&self) -> MODEM_PERI_PMS_EXCEPTION_ADDR_R {
        MODEM_PERI_PMS_EXCEPTION_ADDR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_PERI_PMS_EXCEPTION_INFO")
            .field(
                "modem_peri_pms_exception_det",
                &self.modem_peri_pms_exception_det(),
            )
            .field(
                "modem_peri_pms_exception_id",
                &self.modem_peri_pms_exception_id(),
            )
            .field(
                "modem_peri_pms_exception_mode",
                &self.modem_peri_pms_exception_mode(),
            )
            .field(
                "modem_peri_pms_exception_addr",
                &self.modem_peri_pms_exception_addr(),
            )
            .finish()
    }
}
#[doc = "MODEM Peripherals PMS exception info record register\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_peri_pms_exception_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_PERI_PMS_EXCEPTION_INFO_SPEC;
impl crate::RegisterSpec for MODEM_PERI_PMS_EXCEPTION_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_peri_pms_exception_info::R`](R) reader structure"]
impl crate::Readable for MODEM_PERI_PMS_EXCEPTION_INFO_SPEC {}
#[doc = "`reset()` method sets MODEM_PERI_PMS_EXCEPTION_INFO to value 0"]
impl crate::Resettable for MODEM_PERI_PMS_EXCEPTION_INFO_SPEC {}
