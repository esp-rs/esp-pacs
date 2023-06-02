#[doc = "Register `CH2STATUS` reader"]
pub struct R(crate::R<CH2STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MEM_WADDR_EX` reader - reg_mem_waddr_ex_ch2."]
pub type MEM_WADDR_EX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `APB_MEM_RADDR` reader - reg_apb_mem_raddr_ch2."]
pub type APB_MEM_RADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STATE` reader - reg_state_ch2."]
pub type STATE_R = crate::FieldReader;
#[doc = "Field `MEM_OWNER_ERR` reader - reg_mem_owner_err_ch2."]
pub type MEM_OWNER_ERR_R = crate::BitReader;
#[doc = "Field `MEM_FULL` reader - reg_mem_full_ch2."]
pub type MEM_FULL_R = crate::BitReader;
#[doc = "Field `APB_MEM_RD_ERR` reader - reg_apb_mem_rd_err_ch2."]
pub type APB_MEM_RD_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - reg_mem_waddr_ex_ch2."]
    #[inline(always)]
    pub fn mem_waddr_ex(&self) -> MEM_WADDR_EX_R {
        MEM_WADDR_EX_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 12:20 - reg_apb_mem_raddr_ch2."]
    #[inline(always)]
    pub fn apb_mem_raddr(&self) -> APB_MEM_RADDR_R {
        APB_MEM_RADDR_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:24 - reg_state_ch2."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - reg_mem_owner_err_ch2."]
    #[inline(always)]
    pub fn mem_owner_err(&self) -> MEM_OWNER_ERR_R {
        MEM_OWNER_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reg_mem_full_ch2."]
    #[inline(always)]
    pub fn mem_full(&self) -> MEM_FULL_R {
        MEM_FULL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reg_apb_mem_rd_err_ch2."]
    #[inline(always)]
    pub fn apb_mem_rd_err(&self) -> APB_MEM_RD_ERR_R {
        APB_MEM_RD_ERR_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2STATUS")
            .field(
                "mem_waddr_ex",
                &format_args!("{}", self.mem_waddr_ex().bits()),
            )
            .field(
                "apb_mem_raddr",
                &format_args!("{}", self.apb_mem_raddr().bits()),
            )
            .field("state", &format_args!("{}", self.state().bits()))
            .field(
                "mem_owner_err",
                &format_args!("{}", self.mem_owner_err().bit()),
            )
            .field("mem_full", &format_args!("{}", self.mem_full().bit()))
            .field(
                "apb_mem_rd_err",
                &format_args!("{}", self.apb_mem_rd_err().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH2STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "RMT_CH2STATUS_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2status](index.html) module"]
pub struct CH2STATUS_SPEC;
impl crate::RegisterSpec for CH2STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2status::R](R) reader structure"]
impl crate::Readable for CH2STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH2STATUS to value 0"]
impl crate::Resettable for CH2STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
