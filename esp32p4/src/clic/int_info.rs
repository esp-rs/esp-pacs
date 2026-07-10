#[doc = "Register `INT_INFO` reader"]
pub type R = crate::R<INT_INFO_SPEC>;
#[doc = "Field `NUM_INT` reader - The number of interrupt sources."]
pub type NUM_INT_R = crate::FieldReader<u16>;
#[doc = "Field `VERSION` reader - The lower 4 bits are the modified version of the hardware implementation; the upper 4 bits are the CLIC architecture version information."]
pub type VERSION_R = crate::FieldReader;
#[doc = "Field `CTLBITS` reader - The effective bits of priority in the CLICINTCTL register."]
pub type CTLBITS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:12 - The number of interrupt sources."]
    #[inline(always)]
    pub fn num_int(&self) -> NUM_INT_R {
        NUM_INT_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:20 - The lower 4 bits are the modified version of the hardware implementation; the upper 4 bits are the CLIC architecture version information."]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:28 - The effective bits of priority in the CLICINTCTL register."]
    #[inline(always)]
    pub fn ctlbits(&self) -> CTLBITS_R {
        CTLBITS_R::new(((self.bits >> 21) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_INFO")
            .field("ctlbits", &self.ctlbits())
            .field("version", &self.version())
            .field("num_int", &self.num_int())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`int_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_INFO_SPEC;
impl crate::RegisterSpec for INT_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_info::R`](R) reader structure"]
impl crate::Readable for INT_INFO_SPEC {}
#[doc = "`reset()` method sets INT_INFO to value 0"]
impl crate::Resettable for INT_INFO_SPEC {}
