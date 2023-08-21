#[doc = "Register `CMD_INFOR1` reader"]
pub type R = crate::R<CMD_INFOR1_SPEC>;
#[doc = "Field `CMD_CONTENT1` reader - "]
pub type CMD_CONTENT1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmd_content1(&self) -> CMD_CONTENT1_R {
        CMD_CONTENT1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_INFOR1")
            .field(
                "cmd_content1",
                &format_args!("{}", self.cmd_content1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD_INFOR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_infor1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_INFOR1_SPEC;
impl crate::RegisterSpec for CMD_INFOR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_infor1::R`](R) reader structure"]
impl crate::Readable for CMD_INFOR1_SPEC {}
#[doc = "`reset()` method sets CMD_INFOR1 to value 0"]
impl crate::Resettable for CMD_INFOR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
