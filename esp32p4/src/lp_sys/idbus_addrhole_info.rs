#[doc = "Register `IDBUS_ADDRHOLE_INFO` reader"]
pub type R = crate::R<IDBUS_ADDRHOLE_INFO_SPEC>;
#[doc = "Field `IDBUS_ADDRHOLE_ID` reader - need_des"]
pub type IDBUS_ADDRHOLE_ID_R = crate::FieldReader;
#[doc = "Field `IDBUS_ADDRHOLE_WR` reader - need_des"]
pub type IDBUS_ADDRHOLE_WR_R = crate::BitReader;
#[doc = "Field `IDBUS_ADDRHOLE_SECURE` reader - need_des"]
pub type IDBUS_ADDRHOLE_SECURE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn idbus_addrhole_id(&self) -> IDBUS_ADDRHOLE_ID_R {
        IDBUS_ADDRHOLE_ID_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn idbus_addrhole_wr(&self) -> IDBUS_ADDRHOLE_WR_R {
        IDBUS_ADDRHOLE_WR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn idbus_addrhole_secure(&self) -> IDBUS_ADDRHOLE_SECURE_R {
        IDBUS_ADDRHOLE_SECURE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDBUS_ADDRHOLE_INFO")
            .field(
                "idbus_addrhole_id",
                &format_args!("{}", self.idbus_addrhole_id().bits()),
            )
            .field(
                "idbus_addrhole_wr",
                &format_args!("{}", self.idbus_addrhole_wr().bit()),
            )
            .field(
                "idbus_addrhole_secure",
                &format_args!("{}", self.idbus_addrhole_secure().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IDBUS_ADDRHOLE_INFO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idbus_addrhole_info::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDBUS_ADDRHOLE_INFO_SPEC;
impl crate::RegisterSpec for IDBUS_ADDRHOLE_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idbus_addrhole_info::R`](R) reader structure"]
impl crate::Readable for IDBUS_ADDRHOLE_INFO_SPEC {}
#[doc = "`reset()` method sets IDBUS_ADDRHOLE_INFO to value 0"]
impl crate::Resettable for IDBUS_ADDRHOLE_INFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
