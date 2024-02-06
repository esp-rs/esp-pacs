#[doc = "Register `STATUS5` reader"]
pub type R = crate::R<STATUS5_SPEC>;
#[doc = "Field `PIC_BLOCK_NUM` reader - Reserved"]
pub type PIC_BLOCK_NUM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Reserved"]
    #[inline(always)]
    pub fn pic_block_num(&self) -> PIC_BLOCK_NUM_R {
        PIC_BLOCK_NUM_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS5")
            .field(
                "pic_block_num",
                &format_args!("{}", self.pic_block_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS5_SPEC;
impl crate::RegisterSpec for STATUS5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status5::R`](R) reader structure"]
impl crate::Readable for STATUS5_SPEC {}
#[doc = "`reset()` method sets STATUS5 to value 0"]
impl crate::Resettable for STATUS5_SPEC {
    const RESET_VALUE: u32 = 0;
}
