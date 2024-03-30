#[doc = "Register `BLOCK_BUF_LEN` reader"]
pub type R = crate::R<BLOCK_BUF_LEN_SPEC>;
#[doc = "Field `OUT_BLOCK_BUF_LEN` reader - only for debug"]
pub type OUT_BLOCK_BUF_LEN_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - only for debug"]
    #[inline(always)]
    pub fn out_block_buf_len(&self) -> OUT_BLOCK_BUF_LEN_R {
        OUT_BLOCK_BUF_LEN_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLOCK_BUF_LEN")
            .field(
                "out_block_buf_len",
                &format_args!("{}", self.out_block_buf_len().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLOCK_BUF_LEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "TX CHx block buf len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_buf_len::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLOCK_BUF_LEN_SPEC;
impl crate::RegisterSpec for BLOCK_BUF_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`block_buf_len::R`](R) reader structure"]
impl crate::Readable for BLOCK_BUF_LEN_SPEC {}
#[doc = "`reset()` method sets BLOCK_BUF_LEN to value 0"]
impl crate::Resettable for BLOCK_BUF_LEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
