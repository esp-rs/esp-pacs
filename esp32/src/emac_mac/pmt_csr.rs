#[doc = "Register `PMT_CSR` reader"]
pub type R = crate::R<PMT_CSR_SPEC>;
#[doc = "Field `PWRDWN` reader - When set the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame.This bit must only be set when MGKPKTEN GLBLUCAST or RWKPKTEN bit is set high."]
pub type PWRDWN_R = crate::BitReader;
#[doc = "Field `MGKPKTEN` reader - When set enables generation of a power management event because of magic packet reception."]
pub type MGKPKTEN_R = crate::BitReader;
#[doc = "Field `RWKPKTEN` reader - When set enables generation of a power management event because of remote wake-up frame reception"]
pub type RWKPKTEN_R = crate::BitReader;
#[doc = "Field `MGKPRCVD` reader - When set this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by a Read into this register."]
pub type MGKPRCVD_R = crate::BitReader;
#[doc = "Field `RWKPRCVD` reader - When set this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
pub type RWKPRCVD_R = crate::BitReader;
#[doc = "Field `GLBLUCAST` reader - When set enables any unicast packet filtered by the MAC (DAFilter) address recognition to be a remote wake-up frame."]
pub type GLBLUCAST_R = crate::BitReader;
#[doc = "Field `RWKPTR` reader - The maximum value of the pointer is 7 the detail information please refer to PMT_RWUFFR."]
pub type RWKPTR_R = crate::FieldReader;
#[doc = "Field `RWKFILTRST` reader - When this bit is set it resets the RWKPTR register to 3’b000."]
pub type RWKFILTRST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When set the MAC receiver drops all received frames until it receives the expected magic packet or remote wake-up frame.This bit must only be set when MGKPKTEN GLBLUCAST or RWKPKTEN bit is set high."]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set enables generation of a power management event because of magic packet reception."]
    #[inline(always)]
    pub fn mgkpkten(&self) -> MGKPKTEN_R {
        MGKPKTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set enables generation of a power management event because of remote wake-up frame reception"]
    #[inline(always)]
    pub fn rwkpkten(&self) -> RWKPKTEN_R {
        RWKPKTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - When set this bit indicates that the power management event is generated because of the reception of a magic packet. This bit is cleared by a Read into this register."]
    #[inline(always)]
    pub fn mgkprcvd(&self) -> MGKPRCVD_R {
        MGKPRCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set this bit indicates the power management event is generated because of the reception of a remote wake-up frame. This bit is cleared by a Read into this register."]
    #[inline(always)]
    pub fn rwkprcvd(&self) -> RWKPRCVD_R {
        RWKPRCVD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - When set enables any unicast packet filtered by the MAC (DAFilter) address recognition to be a remote wake-up frame."]
    #[inline(always)]
    pub fn glblucast(&self) -> GLBLUCAST_R {
        GLBLUCAST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 24:28 - The maximum value of the pointer is 7 the detail information please refer to PMT_RWUFFR."]
    #[inline(always)]
    pub fn rwkptr(&self) -> RWKPTR_R {
        RWKPTR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - When this bit is set it resets the RWKPTR register to 3’b000."]
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> RWKFILTRST_R {
        RWKFILTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMT_CSR")
            .field("pwrdwn", &format_args!("{}", self.pwrdwn().bit()))
            .field("mgkpkten", &format_args!("{}", self.mgkpkten().bit()))
            .field("rwkpkten", &format_args!("{}", self.rwkpkten().bit()))
            .field("mgkprcvd", &format_args!("{}", self.mgkprcvd().bit()))
            .field("rwkprcvd", &format_args!("{}", self.rwkprcvd().bit()))
            .field("glblucast", &format_args!("{}", self.glblucast().bit()))
            .field("rwkptr", &format_args!("{}", self.rwkptr().bits()))
            .field("rwkfiltrst", &format_args!("{}", self.rwkfiltrst().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PMT_CSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "PMT Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmt_csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMT_CSR_SPEC;
impl crate::RegisterSpec for PMT_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmt_csr::R`](R) reader structure"]
impl crate::Readable for PMT_CSR_SPEC {}
#[doc = "`reset()` method sets PMT_CSR to value 0"]
impl crate::Resettable for PMT_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
