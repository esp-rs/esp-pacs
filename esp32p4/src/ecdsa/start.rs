#[doc = "Register `START` writer"]
pub type W = crate::W<START_SPEC>;
#[doc = "Field `START` writer - Write 1 to start caculation of ECDSA Accelerator. This bit will be self-cleared after configuration."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOAD_DONE` writer - Write 1 to input load done signal of ECDSA Accelerator. This bit will be self-cleared after configuration."]
pub type LOAD_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GET_DONE` writer - Write 1 to input get done signal of ECDSA Accelerator. This bit will be self-cleared after configuration."]
pub type GET_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start caculation of ECDSA Accelerator. This bit will be self-cleared after configuration."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<START_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to input load done signal of ECDSA Accelerator. This bit will be self-cleared after configuration."]
    #[inline(always)]
    #[must_use]
    pub fn load_done(&mut self) -> LOAD_DONE_W<START_SPEC> {
        LOAD_DONE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to input get done signal of ECDSA Accelerator. This bit will be self-cleared after configuration."]
    #[inline(always)]
    #[must_use]
    pub fn get_done(&mut self) -> GET_DONE_W<START_SPEC> {
        GET_DONE_W::new(self, 2)
    }
}
#[doc = "ECDSA start register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct START_SPEC;
impl crate::RegisterSpec for START_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`start::W`](W) writer structure"]
impl crate::Writable for START_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for START_SPEC {
    const RESET_VALUE: u32 = 0;
}
