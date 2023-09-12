#[doc = "Register `QY_MEM%s` reader"]
pub type R = crate::R<QY_MEM_SPEC>;
#[doc = "Register `QY_MEM%s` writer"]
pub type W = crate::W<QY_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<QY_MEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The memory that stores Qy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qy_mem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qy_mem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QY_MEM_SPEC;
impl crate::RegisterSpec for QY_MEM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`qy_mem::R`](R) reader structure"]
impl crate::Readable for QY_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qy_mem::W`](W) writer structure"]
impl crate::Writable for QY_MEM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QY_MEM%s to value 0"]
impl crate::Resettable for QY_MEM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
