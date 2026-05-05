#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `PREP_DONE` reader - The raw interrupt status bit for the huk_prep_done_int interrupt"]
pub type PREP_DONE_R = crate::BitReader;
#[doc = "Field `PROC_DONE` reader - The raw interrupt status bit for the huk_proc_done_int interrupt"]
pub type PROC_DONE_R = crate::BitReader;
#[doc = "Field `POST_DONE` reader - The raw interrupt status bit for the huk_post_done_int interrupt"]
pub type POST_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the huk_prep_done_int interrupt"]
    #[inline(always)]
    pub fn prep_done(&self) -> PREP_DONE_R {
        PREP_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the huk_proc_done_int interrupt"]
    #[inline(always)]
    pub fn proc_done(&self) -> PROC_DONE_R {
        PROC_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the huk_post_done_int interrupt"]
    #[inline(always)]
    pub fn post_done(&self) -> POST_DONE_R {
        POST_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("prep_done", &self.prep_done())
            .field("proc_done", &self.proc_done())
            .field("post_done", &self.post_done())
            .finish()
    }
}
#[doc = "HUK Generator interrupt raw register, valid in level.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
