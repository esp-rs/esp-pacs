#[doc = "Register `BUF_LEN` reader"]
pub type R = crate::R<BUF_LEN_SPEC>;
#[doc = "Field `OUT_CMDFIFO_BUF_LEN_HB` reader - only for debug"]
pub type OUT_CMDFIFO_BUF_LEN_HB_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - only for debug"]
    #[inline(always)]
    pub fn out_cmdfifo_buf_len_hb(&self) -> OUT_CMDFIFO_BUF_LEN_HB_R {
        OUT_CMDFIFO_BUF_LEN_HB_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUF_LEN")
            .field("out_cmdfifo_buf_len_hb", &self.out_cmdfifo_buf_len_hb())
            .finish()
    }
}
#[doc = "TX CHx buf len register\n\nYou can [`read`](crate::Reg::read) this register and get [`buf_len::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_LEN_SPEC;
impl crate::RegisterSpec for BUF_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_len::R`](R) reader structure"]
impl crate::Readable for BUF_LEN_SPEC {}
#[doc = "`reset()` method sets BUF_LEN to value 0"]
impl crate::Resettable for BUF_LEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
