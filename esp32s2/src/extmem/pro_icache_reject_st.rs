#[doc = "Register `PRO_ICACHE_REJECT_ST` reader"]
pub struct R(crate::R<PRO_ICACHE_REJECT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_ICACHE_REJECT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_ICACHE_REJECT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_ICACHE_REJECT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_ICACHE_TAG_ATTR` reader - The bits are used to indicate the attribute of data from external memory when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
pub type PRO_ICACHE_TAG_ATTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRO_ICACHE_CPU_ATTR` reader - The bits are used to indicate the attribute of CPU access icache when authentication fail. 0: invalidate, 1: execute-able, 2: read-able"]
pub type PRO_ICACHE_CPU_ATTR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - The bits are used to indicate the attribute of data from external memory when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
    #[inline(always)]
    pub fn pro_icache_tag_attr(&self) -> PRO_ICACHE_TAG_ATTR_R {
        PRO_ICACHE_TAG_ATTR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The bits are used to indicate the attribute of CPU access icache when authentication fail. 0: invalidate, 1: execute-able, 2: read-able"]
    #[inline(always)]
    pub fn pro_icache_cpu_attr(&self) -> PRO_ICACHE_CPU_ATTR_R {
        PRO_ICACHE_CPU_ATTR_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_icache_reject_st](index.html) module"]
pub struct PRO_ICACHE_REJECT_ST_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_REJECT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_icache_reject_st::R](R) reader structure"]
impl crate::Readable for PRO_ICACHE_REJECT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_ICACHE_REJECT_ST to value 0"]
impl crate::Resettable for PRO_ICACHE_REJECT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
