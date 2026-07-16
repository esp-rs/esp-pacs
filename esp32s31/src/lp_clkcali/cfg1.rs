#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Field `DIV_NUMERATOR` reader - need_des"]
pub type DIV_NUMERATOR_R = crate::FieldReader<u16>;
#[doc = "Field `DIV_DENOMINATOR` reader - need_des"]
pub type DIV_DENOMINATOR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn div_numerator(&self) -> DIV_NUMERATOR_R {
        DIV_NUMERATOR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - need_des"]
    #[inline(always)]
    pub fn div_denominator(&self) -> DIV_DENOMINATOR_R {
        DIV_DENOMINATOR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("div_numerator", &self.div_numerator())
            .field("div_denominator", &self.div_denominator())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1_SPEC {}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {}
