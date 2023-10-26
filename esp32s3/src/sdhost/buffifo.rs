#[doc = "Register `BUFFIFO` reader"]
pub type R = crate::R<BUFFIFO_SPEC>;
#[doc = "Register `BUFFIFO` writer"]
pub type W = crate::W<BUFFIFO_SPEC>;
#[doc = "Field `BUFFIFO` reader - CPU write and read transmit data by FIFO. This register points to the current Data FIFO ."]
pub type BUFFIFO_R = crate::FieldReader<u32>;
#[doc = "Field `BUFFIFO` writer - CPU write and read transmit data by FIFO. This register points to the current Data FIFO ."]
pub type BUFFIFO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CPU write and read transmit data by FIFO. This register points to the current Data FIFO ."]
    #[inline(always)]
    pub fn buffifo(&self) -> BUFFIFO_R {
        BUFFIFO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUFFIFO")
            .field("buffifo", &format_args!("{}", self.buffifo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUFFIFO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU write and read transmit data by FIFO. This register points to the current Data FIFO ."]
    #[inline(always)]
    #[must_use]
    pub fn buffifo(&mut self) -> BUFFIFO_W<BUFFIFO_SPEC, 0> {
        BUFFIFO_W::new(self)
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
#[doc = "CPU write and read transmit data by FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buffifo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buffifo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUFFIFO_SPEC;
impl crate::RegisterSpec for BUFFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buffifo::R`](R) reader structure"]
impl crate::Readable for BUFFIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buffifo::W`](W) writer structure"]
impl crate::Writable for BUFFIFO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUFFIFO to value 0"]
impl crate::Resettable for BUFFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
