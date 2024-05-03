#[doc = "Register `STATUS3` reader"]
pub type R = crate::R<STATUS3_SPEC>;
#[doc = "Field `YO` reader - component y transferred from rgb input"]
pub type YO_R = crate::FieldReader<u16>;
#[doc = "Field `Y_READY` reader - component y valid signal, high active"]
pub type Y_READY_R = crate::BitReader;
#[doc = "Field `CBO` reader - component cb transferred from rgb input"]
pub type CBO_R = crate::FieldReader<u16>;
#[doc = "Field `CB_READY` reader - component cb valid signal, high active"]
pub type CB_READY_R = crate::BitReader;
#[doc = "Field `CRO` reader - component cr transferred from rgb input"]
pub type CRO_R = crate::FieldReader<u16>;
#[doc = "Field `CR_READY` reader - component cr valid signal, high active"]
pub type CR_READY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - component y transferred from rgb input"]
    #[inline(always)]
    pub fn yo(&self) -> YO_R {
        YO_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - component y valid signal, high active"]
    #[inline(always)]
    pub fn y_ready(&self) -> Y_READY_R {
        Y_READY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:18 - component cb transferred from rgb input"]
    #[inline(always)]
    pub fn cbo(&self) -> CBO_R {
        CBO_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bit 19 - component cb valid signal, high active"]
    #[inline(always)]
    pub fn cb_ready(&self) -> CB_READY_R {
        CB_READY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:28 - component cr transferred from rgb input"]
    #[inline(always)]
    pub fn cro(&self) -> CRO_R {
        CRO_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - component cr valid signal, high active"]
    #[inline(always)]
    pub fn cr_ready(&self) -> CR_READY_R {
        CR_READY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS3")
            .field("yo", &self.yo().bits())
            .field("y_ready", &self.y_ready().bit())
            .field("cbo", &self.cbo().bits())
            .field("cb_ready", &self.cb_ready().bit())
            .field("cro", &self.cro().bits())
            .field("cr_ready", &self.cr_ready().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS3_SPEC;
impl crate::RegisterSpec for STATUS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status3::R`](R) reader structure"]
impl crate::Readable for STATUS3_SPEC {}
#[doc = "`reset()` method sets STATUS3 to value 0"]
impl crate::Resettable for STATUS3_SPEC {
    const RESET_VALUE: u32 = 0;
}
