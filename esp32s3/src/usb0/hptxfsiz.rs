#[doc = "Register `HPTXFSIZ` reader"]
pub type R = crate::R<HPTXFSIZ_SPEC>;
#[doc = "Register `HPTXFSIZ` writer"]
pub type W = crate::W<HPTXFSIZ_SPEC>;
#[doc = "Field `PTXFSTADDR` reader - "]
pub type PTXFSTADDR_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFSTADDR` writer - "]
pub type PTXFSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PTXFSIZE` reader - "]
pub type PTXFSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFSIZE` writer - "]
pub type PTXFSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ptxfstaddr(&self) -> PTXFSTADDR_R {
        PTXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ptxfsize(&self) -> PTXFSIZE_R {
        PTXFSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXFSIZ")
            .field("ptxfstaddr", &self.ptxfstaddr())
            .field("ptxfsize", &self.ptxfsize())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ptxfstaddr(&mut self) -> PTXFSTADDR_W<HPTXFSIZ_SPEC> {
        PTXFSTADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ptxfsize(&mut self) -> PTXFSIZE_W<HPTXFSIZ_SPEC> {
        PTXFSIZE_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPTXFSIZ_SPEC;
impl crate::RegisterSpec for HPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxfsiz::R`](R) reader structure"]
impl crate::Readable for HPTXFSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hptxfsiz::W`](W) writer structure"]
impl crate::Writable for HPTXFSIZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPTXFSIZ to value 0x1000_0200"]
impl crate::Resettable for HPTXFSIZ_SPEC {
    const RESET_VALUE: u32 = 0x1000_0200;
}
