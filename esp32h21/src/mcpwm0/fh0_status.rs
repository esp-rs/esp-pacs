#[doc = "Register `FH0_STATUS` reader"]
pub type R = crate::R<FH0_STATUS_SPEC>;
#[doc = "Field `TZ0_CBC_ON` reader - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going"]
pub type TZ0_CBC_ON_R = crate::BitReader;
#[doc = "Field `TZ0_OST_ON` reader - Set and reset by hardware. If set, an one-shot mode action is on going"]
pub type TZ0_OST_ON_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going"]
    #[inline(always)]
    pub fn tz0_cbc_on(&self) -> TZ0_CBC_ON_R {
        TZ0_CBC_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set and reset by hardware. If set, an one-shot mode action is on going"]
    #[inline(always)]
    pub fn tz0_ost_on(&self) -> TZ0_OST_ON_R {
        TZ0_OST_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH0_STATUS")
            .field("tz0_cbc_on", &self.tz0_cbc_on())
            .field("tz0_ost_on", &self.tz0_ost_on())
            .finish()
    }
}
#[doc = "Status of fault events.\n\nYou can [`read`](crate::Reg::read) this register and get [`fh0_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FH0_STATUS_SPEC;
impl crate::RegisterSpec for FH0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh0_status::R`](R) reader structure"]
impl crate::Readable for FH0_STATUS_SPEC {}
#[doc = "`reset()` method sets FH0_STATUS to value 0"]
impl crate::Resettable for FH0_STATUS_SPEC {}
