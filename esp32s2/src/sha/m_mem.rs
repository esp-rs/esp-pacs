#[doc = "Register `M_MEM%s` reader"]
pub type R = crate::R<M_MEM_SPEC>;
#[doc = "Register `M_MEM%s` writer"]
pub type W = crate::W<M_MEM_SPEC>;
#[doc = "Field `M` reader - Stores the %sth 32-bit piece of the message."]
pub type M_R = crate::FieldReader<u32>;
#[doc = "Field `M` writer - Stores the %sth 32-bit piece of the message."]
pub type M_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the %sth 32-bit piece of the message."]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M_MEM")
            .field("m", &format_args!("{}", self.m().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M_MEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the %sth 32-bit piece of the message."]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> M_W<M_MEM_SPEC> {
        M_W::new(self, 0)
    }
}
#[doc = "Message\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m_mem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_mem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M_MEM_SPEC;
impl crate::RegisterSpec for M_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m_mem::R`](R) reader structure"]
impl crate::Readable for M_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`m_mem::W`](W) writer structure"]
impl crate::Writable for M_MEM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M_MEM%s to value 0"]
impl crate::Resettable for M_MEM_SPEC {
    const RESET_VALUE: u32 = 0;
}
