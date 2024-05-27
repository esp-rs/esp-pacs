#[doc = "Register `CMD_INFOR0` reader"]
pub type R = crate::R<CMD_INFOR0_SPEC>;
#[doc = "Field `CMD_CONTENT0` reader - "]
pub type CMD_CONTENT0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmd_content0(&self) -> CMD_CONTENT0_R {
        CMD_CONTENT0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_INFOR0")
            .field("cmd_content0", &self.cmd_content0())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_infor0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_INFOR0_SPEC;
impl crate::RegisterSpec for CMD_INFOR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_infor0::R`](R) reader structure"]
impl crate::Readable for CMD_INFOR0_SPEC {}
#[doc = "`reset()` method sets CMD_INFOR0 to value 0"]
impl crate::Resettable for CMD_INFOR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
