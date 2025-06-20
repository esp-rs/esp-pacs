#[doc = "Register `REG_Q1_WORD1` reader"]
pub type R = crate::R<REG_Q1_WORD1_SPEC>;
#[doc = "Register `REG_Q1_WORD1` writer"]
pub type W = crate::W<REG_Q1_WORD1_SPEC>;
#[doc = "Field `SEND_Q1_WORD1` reader - Data to be transmitted in Q1 register."]
pub type SEND_Q1_WORD1_R = crate::FieldReader<u32>;
#[doc = "Field `SEND_Q1_WORD1` writer - Data to be transmitted in Q1 register."]
pub type SEND_Q1_WORD1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data to be transmitted in Q1 register."]
    #[inline(always)]
    pub fn send_q1_word1(&self) -> SEND_Q1_WORD1_R {
        SEND_Q1_WORD1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_Q1_WORD1")
            .field("send_q1_word1", &self.send_q1_word1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to be transmitted in Q1 register."]
    #[inline(always)]
    pub fn send_q1_word1(&mut self) -> SEND_Q1_WORD1_W<REG_Q1_WORD1_SPEC> {
        SEND_Q1_WORD1_W::new(self, 0)
    }
}
#[doc = "Q1 WORD1 quick send register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_q1_word1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_q1_word1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_Q1_WORD1_SPEC;
impl crate::RegisterSpec for REG_Q1_WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_q1_word1::R`](R) reader structure"]
impl crate::Readable for REG_Q1_WORD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_q1_word1::W`](W) writer structure"]
impl crate::Writable for REG_Q1_WORD1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_Q1_WORD1 to value 0"]
impl crate::Resettable for REG_Q1_WORD1_SPEC {
    const RESET_VALUE: u32 = 0;
}
