#[doc = "Register `ONE_BLOCK` writer"]
pub type W = crate::W<ONE_BLOCK_SPEC>;
#[doc = "Field `SET_ONE_BLOCK` writer - Set this bit to show no padding is required."]
pub type SET_ONE_BLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ONE_BLOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to show no padding is required."]
    #[inline(always)]
    #[must_use]
    pub fn set_one_block(&mut self) -> SET_ONE_BLOCK_W<ONE_BLOCK_SPEC, 0> {
        SET_ONE_BLOCK_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "One block message register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`one_block::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ONE_BLOCK_SPEC;
impl crate::RegisterSpec for ONE_BLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`one_block::W`](W) writer structure"]
impl crate::Writable for ONE_BLOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ONE_BLOCK to value 0"]
impl crate::Resettable for ONE_BLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
