#[doc = "Register `POR_CUR_ST` reader"]
pub type R = crate::R<POR_CUR_ST_SPEC>;
#[doc = "Field `POR_CUR_ST` reader - "]
pub type POR_CUR_ST_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn por_cur_st(&self) -> POR_CUR_ST_R {
        POR_CUR_ST_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POR_CUR_ST")
            .field("por_cur_st", &self.por_cur_st())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`por_cur_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POR_CUR_ST_SPEC;
impl crate::RegisterSpec for POR_CUR_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`por_cur_st::R`](R) reader structure"]
impl crate::Readable for POR_CUR_ST_SPEC {}
#[doc = "`reset()` method sets POR_CUR_ST to value 0"]
impl crate::Resettable for POR_CUR_ST_SPEC {}
