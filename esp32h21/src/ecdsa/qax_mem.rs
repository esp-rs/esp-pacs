#[doc = "Register `QAX_MEM[%s]` reader"]
pub type R = crate::R<QAX_MEM_SPEC>;
#[doc = "Register `QAX_MEM[%s]` writer"]
pub type W = crate::W<QAX_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The memory that stores x coordinates of QA or software written k.\n\nYou can [`read`](crate::Reg::read) this register and get [`qax_mem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qax_mem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QAX_MEM_SPEC;
impl crate::RegisterSpec for QAX_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qax_mem::R`](R) reader structure"]
impl crate::Readable for QAX_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qax_mem::W`](W) writer structure"]
impl crate::Writable for QAX_MEM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QAX_MEM[%s] to value 0"]
impl crate::Resettable for QAX_MEM_SPEC {}
