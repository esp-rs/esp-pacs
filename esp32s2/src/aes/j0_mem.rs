#[doc = "Register `J0_MEM[%s]` reader"]
pub type R = crate::R<J0_MEM_SPEC>;
#[doc = "Register `J0_MEM[%s]` writer"]
pub type W = crate::W<J0_MEM_SPEC>;
#[doc = "Field `J0` reader - This register stores the %sth 32-bit piece of 128-bit J0"]
pub type J0_R = crate::FieldReader<u32>;
#[doc = "Field `J0` writer - This register stores the %sth 32-bit piece of 128-bit J0"]
pub type J0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit J0"]
    #[inline(always)]
    pub fn j0(&self) -> J0_R {
        J0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("J0_MEM").field("j0", &self.j0()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit J0"]
    #[inline(always)]
    #[must_use]
    pub fn j0(&mut self) -> J0_W<J0_MEM_SPEC> {
        J0_W::new(self, 0)
    }
}
#[doc = "J0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`j0_mem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`j0_mem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct J0_MEM_SPEC;
impl crate::RegisterSpec for J0_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`j0_mem::R`](R) reader structure"]
impl crate::Readable for J0_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`j0_mem::W`](W) writer structure"]
impl crate::Writable for J0_MEM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets J0_MEM[%s] to value 0"]
impl crate::Resettable for J0_MEM_SPEC {
    const RESET_VALUE: u32 = 0;
}
