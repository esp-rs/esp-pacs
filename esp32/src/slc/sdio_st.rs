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
pub type CMD_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUNC_ST` reader - "]
pub type FUNC_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDIO_WAKEUP` reader - "]
pub type SDIO_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `BUS_ST` reader - "]
pub type BUS_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUNC1_ACC_STATE` reader - "]
pub type FUNC1_ACC_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FUNC2_ACC_STATE` reader - "]
pub type FUNC2_ACC_STATE_R = crate::FieldReader<u8, u8>;
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
