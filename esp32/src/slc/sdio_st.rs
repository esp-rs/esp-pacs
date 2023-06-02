#[doc = "Register `SDIO_ST` reader"]
pub struct R(crate::R<SDIO_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMD_ST` reader - "]
pub type CMD_ST_R = crate::FieldReader;
#[doc = "Field `FUNC_ST` reader - "]
pub type FUNC_ST_R = crate::FieldReader;
#[doc = "Field `SDIO_WAKEUP` reader - "]
pub type SDIO_WAKEUP_R = crate::BitReader;
#[doc = "Field `BUS_ST` reader - "]
pub type BUS_ST_R = crate::FieldReader;
#[doc = "Field `FUNC1_ACC_STATE` reader - "]
pub type FUNC1_ACC_STATE_R = crate::FieldReader;
#[doc = "Field `FUNC2_ACC_STATE` reader - "]
pub type FUNC2_ACC_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cmd_st(&self) -> CMD_ST_R {
        CMD_ST_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn func_st(&self) -> FUNC_ST_R {
        FUNC_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sdio_wakeup(&self) -> SDIO_WAKEUP_R {
        SDIO_WAKEUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn bus_st(&self) -> BUS_ST_R {
        BUS_ST_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn func1_acc_state(&self) -> FUNC1_ACC_STATE_R {
        FUNC1_ACC_STATE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn func2_acc_state(&self) -> FUNC2_ACC_STATE_R {
        FUNC2_ACC_STATE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_ST")
            .field("cmd_st", &format_args!("{}", self.cmd_st().bits()))
            .field("func_st", &format_args!("{}", self.func_st().bits()))
            .field("sdio_wakeup", &format_args!("{}", self.sdio_wakeup().bit()))
            .field("bus_st", &format_args!("{}", self.bus_st().bits()))
            .field(
                "func1_acc_state",
                &format_args!("{}", self.func1_acc_state().bits()),
            )
            .field(
                "func2_acc_state",
                &format_args!("{}", self.func2_acc_state().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_st](index.html) module"]
pub struct SDIO_ST_SPEC;
impl crate::RegisterSpec for SDIO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_st::R](R) reader structure"]
impl crate::Readable for SDIO_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDIO_ST to value 0"]
impl crate::Resettable for SDIO_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
