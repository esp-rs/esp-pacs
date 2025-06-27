#[doc = "Register `PRO_ICACHE_REJECT_ST` reader"]
pub type R = crate::R<PRO_ICACHE_REJECT_ST_SPEC>;
#[doc = "Field `PRO_ICACHE_TAG_ATTR` reader - The bits are used to indicate the attribute of data from external memory when authentication fail. 0: invalidate, 1: execute-able, 2: read-able, 4: write-able."]
pub type PRO_ICACHE_TAG_ATTR_R = crate::FieldReader;
#[doc = "Field `PRO_ICACHE_CPU_ATTR` reader - The bits are used to indicate the attribute of CPU access icache when authentication fail. 0: invalidate, 1: execute-able, 2: read-able"]
pub type PRO_ICACHE_CPU_ATTR_R = crate::FieldReader;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_ICACHE_REJECT_ST")
            .field("pro_icache_tag_attr", &self.pro_icache_tag_attr())
            .field("pro_icache_cpu_attr", &self.pro_icache_cpu_attr())
            .finish()
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_reject_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_ICACHE_REJECT_ST_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_REJECT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_icache_reject_st::R`](R) reader structure"]
impl crate::Readable for PRO_ICACHE_REJECT_ST_SPEC {}
#[doc = "`reset()` method sets PRO_ICACHE_REJECT_ST to value 0"]
impl crate::Resettable for PRO_ICACHE_REJECT_ST_SPEC {}
