#[doc = "Register `IC_PRELOAD_CNT` reader"]
pub type R = crate::R<IC_PRELOAD_CNT_SPEC>;
#[doc = "Field `IC_PRELOAD_CNT` reader - The bits are used to count the number of issued pre-load which include manual pre-load and conditional pre-load."]
pub type IC_PRELOAD_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to count the number of issued pre-load which include manual pre-load and conditional pre-load."]
    #[inline(always)]
    pub fn ic_preload_cnt(&self) -> IC_PRELOAD_CNT_R {
        IC_PRELOAD_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IC_PRELOAD_CNT")
            .field("ic_preload_cnt", &self.ic_preload_cnt())
            .finish()
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ic_preload_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_PRELOAD_CNT_SPEC;
impl crate::RegisterSpec for IC_PRELOAD_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_preload_cnt::R`](R) reader structure"]
impl crate::Readable for IC_PRELOAD_CNT_SPEC {}
#[doc = "`reset()` method sets IC_PRELOAD_CNT to value 0"]
impl crate::Resettable for IC_PRELOAD_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
