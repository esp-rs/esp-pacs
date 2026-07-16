#[doc = "Register `TIMEOUT_INFO` reader"]
pub type R = crate::R<TIMEOUT_INFO_SPEC>;
#[doc = "Field `REG_TIMEOUT_SLAVE` reader - "]
pub type REG_TIMEOUT_SLAVE_R = crate::FieldReader;
#[doc = "Field `REG_TIMEOUT_ID` reader - "]
pub type REG_TIMEOUT_ID_R = crate::FieldReader;
#[doc = "Field `REG_TIMEOUT_WR` reader - "]
pub type REG_TIMEOUT_WR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn reg_timeout_slave(&self) -> REG_TIMEOUT_SLAVE_R {
        REG_TIMEOUT_SLAVE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:9"]
    #[inline(always)]
    pub fn reg_timeout_id(&self) -> REG_TIMEOUT_ID_R {
        REG_TIMEOUT_ID_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_timeout_wr(&self) -> REG_TIMEOUT_WR_R {
        REG_TIMEOUT_WR_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUT_INFO")
            .field("reg_timeout_slave", &self.reg_timeout_slave())
            .field("reg_timeout_id", &self.reg_timeout_id())
            .field("reg_timeout_wr", &self.reg_timeout_wr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEOUT_INFO_SPEC;
impl crate::RegisterSpec for TIMEOUT_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout_info::R`](R) reader structure"]
impl crate::Readable for TIMEOUT_INFO_SPEC {}
#[doc = "`reset()` method sets TIMEOUT_INFO to value 0"]
impl crate::Resettable for TIMEOUT_INFO_SPEC {}
