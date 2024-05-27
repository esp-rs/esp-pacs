#[doc = "Register `DSTS` reader"]
pub type R = crate::R<DSTS_SPEC>;
#[doc = "Field `SUSPSTS` reader - "]
pub type SUSPSTS_R = crate::BitReader;
#[doc = "Field `ENUMSPD` reader - "]
pub type ENUMSPD_R = crate::FieldReader;
#[doc = "Field `ERRTICERR` reader - "]
pub type ERRTICERR_R = crate::BitReader;
#[doc = "Field `SOFFN` reader - "]
pub type SOFFN_R = crate::FieldReader<u16>;
#[doc = "Field `DEVLNSTS` reader - "]
pub type DEVLNSTS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn errticerr(&self) -> ERRTICERR_R {
        ERRTICERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21"]
    #[inline(always)]
    pub fn soffn(&self) -> SOFFN_R {
        SOFFN_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn devlnsts(&self) -> DEVLNSTS_R {
        DEVLNSTS_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSTS")
            .field("suspsts", &self.suspsts())
            .field("enumspd", &self.enumspd())
            .field("errticerr", &self.errticerr())
            .field("soffn", &self.soffn())
            .field("devlnsts", &self.devlnsts())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTS_SPEC;
impl crate::RegisterSpec for DSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsts::R`](R) reader structure"]
impl crate::Readable for DSTS_SPEC {}
#[doc = "`reset()` method sets DSTS to value 0x02"]
impl crate::Resettable for DSTS_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
