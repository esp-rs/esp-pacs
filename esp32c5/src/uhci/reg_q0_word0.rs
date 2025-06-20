#[doc = "Register `REG_Q0_WORD0` reader"]
pub type R = crate::R<REG_Q0_WORD0_SPEC>;
#[doc = "Register `REG_Q0_WORD0` writer"]
pub type W = crate::W<REG_Q0_WORD0_SPEC>;
#[doc = "Field `SEND_Q0_WORD0` reader - Data to be transmitted in Q0 register."]
pub type SEND_Q0_WORD0_R = crate::FieldReader<u32>;
#[doc = "Field `SEND_Q0_WORD0` writer - Data to be transmitted in Q0 register."]
pub type SEND_Q0_WORD0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data to be transmitted in Q0 register."]
    #[inline(always)]
    pub fn send_q0_word0(&self) -> SEND_Q0_WORD0_R {
        SEND_Q0_WORD0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_Q0_WORD0")
            .field("send_q0_word0", &self.send_q0_word0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to be transmitted in Q0 register."]
    #[inline(always)]
    pub fn send_q0_word0(&mut self) -> SEND_Q0_WORD0_W<REG_Q0_WORD0_SPEC> {
        SEND_Q0_WORD0_W::new(self, 0)
    }
}
#[doc = "Q0 WORD0 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q0_word0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q0_word0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_Q0_WORD0_SPEC;
impl crate::RegisterSpec for REG_Q0_WORD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_q0_word0::R`](R) reader structure"]
impl crate::Readable for REG_Q0_WORD0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_q0_word0::W`](W) writer structure"]
impl crate::Writable for REG_Q0_WORD0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_Q0_WORD0 to value 0"]
impl crate::Resettable for REG_Q0_WORD0_SPEC {
    const RESET_VALUE: u32 = 0;
}
