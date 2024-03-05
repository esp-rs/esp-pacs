#[doc = "Register `ACK_NUM` reader"]
pub type R = crate::R<ACK_NUM_SPEC>;
#[doc = "Register `ACK_NUM` writer"]
pub type W = crate::W<ACK_NUM_SPEC>;
#[doc = "Field `ACK_NUM` reader - a"]
pub type ACK_NUM_R = crate::FieldReader;
#[doc = "Field `ACK_NUM` writer - a"]
pub type ACK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LOAD` writer - a"]
pub type LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - a"]
    #[inline(always)]
    pub fn ack_num(&self) -> ACK_NUM_R {
        ACK_NUM_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACK_NUM")
            .field("ack_num", &format_args!("{}", self.ack_num().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ACK_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - a"]
    #[inline(always)]
    #[must_use]
    pub fn ack_num(&mut self) -> ACK_NUM_W<ACK_NUM_SPEC> {
        ACK_NUM_W::new(self, 0)
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<ACK_NUM_SPEC> {
        LOAD_W::new(self, 3)
    }
}
#[doc = "a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ack_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACK_NUM_SPEC;
impl crate::RegisterSpec for ACK_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ack_num::R`](R) reader structure"]
impl crate::Readable for ACK_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ack_num::W`](W) writer structure"]
impl crate::Writable for ACK_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACK_NUM to value 0x08"]
impl crate::Resettable for ACK_NUM_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
