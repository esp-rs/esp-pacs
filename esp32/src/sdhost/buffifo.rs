#[doc = "Register `BUFFIFO` reader"]
pub type R = crate::R<BUFFIFO_SPEC>;
#[doc = "Register `BUFFIFO` writer"]
pub type W = crate::W<BUFFIFO_SPEC>;
#[doc = "Field `BUFFIFO` reader - CPU write and read transmit data by FIFO. This register points to the current Data FIFO ."]
pub type BUFFIFO_R = crate::FieldReader<u32>;
#[doc = "Field `BUFFIFO` writer - CPU write and read transmit data by FIFO. This register points to the current Data FIFO ."]
pub type BUFFIFO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
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
            .field("buffifo", &self.buffifo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU write and read transmit data by FIFO. This register points to the current Data FIFO ."]
    #[inline(always)]
    pub fn buffifo(&mut self) -> BUFFIFO_W<BUFFIFO_SPEC> {
        BUFFIFO_W::new(self, 0)
    }
}
#[doc = "CPU write and read transmit data by FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`buffifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buffifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUFFIFO_SPEC;
impl crate::RegisterSpec for BUFFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buffifo::R`](R) reader structure"]
impl crate::Readable for BUFFIFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buffifo::W`](W) writer structure"]
impl crate::Writable for BUFFIFO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUFFIFO to value 0"]
impl crate::Resettable for BUFFIFO_SPEC {}
