#[doc = "Register `REG_Q3_WORD0` reader"]
pub type R = crate::R<REG_Q3_WORD0_SPEC>;
#[doc = "Register `REG_Q3_WORD0` writer"]
pub type W = crate::W<REG_Q3_WORD0_SPEC>;
#[doc = "Field `SEND_Q3_WORD0` reader - a"]
pub type SEND_Q3_WORD0_R = crate::FieldReader<u32>;
#[doc = "Field `SEND_Q3_WORD0` writer - a"]
pub type SEND_Q3_WORD0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - a"]
    #[inline(always)]
    pub fn send_q3_word0(&self) -> SEND_Q3_WORD0_R {
        SEND_Q3_WORD0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_Q3_WORD0")
            .field(
                "send_q3_word0",
                &format_args!("{}", self.send_q3_word0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REG_Q3_WORD0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - a"]
    #[inline(always)]
    #[must_use]
    pub fn send_q3_word0(&mut self) -> SEND_Q3_WORD0_W<REG_Q3_WORD0_SPEC> {
        SEND_Q3_WORD0_W::new(self, 0)
    }
}
#[doc = "a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_q3_word0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_q3_word0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_Q3_WORD0_SPEC;
impl crate::RegisterSpec for REG_Q3_WORD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_q3_word0::R`](R) reader structure"]
impl crate::Readable for REG_Q3_WORD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_q3_word0::W`](W) writer structure"]
impl crate::Writable for REG_Q3_WORD0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_Q3_WORD0 to value 0"]
impl crate::Resettable for REG_Q3_WORD0_SPEC {
    const RESET_VALUE: u32 = 0;
}
