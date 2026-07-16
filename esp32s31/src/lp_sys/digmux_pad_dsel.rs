#[doc = "Register `DIGMUX_PAD_DSEL` reader"]
pub type R = crate::R<DIGMUX_PAD_DSEL_SPEC>;
#[doc = "Register `DIGMUX_PAD_DSEL` writer"]
pub type W = crate::W<DIGMUX_PAD_DSEL_SPEC>;
#[doc = "Field `DIGMUX_PAD_DSEL` reader - "]
pub type DIGMUX_PAD_DSEL_R = crate::FieldReader;
#[doc = "Field `DIGMUX_PAD_DSEL` writer - "]
pub type DIGMUX_PAD_DSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn digmux_pad_dsel(&self) -> DIGMUX_PAD_DSEL_R {
        DIGMUX_PAD_DSEL_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIGMUX_PAD_DSEL")
            .field("digmux_pad_dsel", &self.digmux_pad_dsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn digmux_pad_dsel(&mut self) -> DIGMUX_PAD_DSEL_W<'_, DIGMUX_PAD_DSEL_SPEC> {
        DIGMUX_PAD_DSEL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`digmux_pad_dsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`digmux_pad_dsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIGMUX_PAD_DSEL_SPEC;
impl crate::RegisterSpec for DIGMUX_PAD_DSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`digmux_pad_dsel::R`](R) reader structure"]
impl crate::Readable for DIGMUX_PAD_DSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`digmux_pad_dsel::W`](W) writer structure"]
impl crate::Writable for DIGMUX_PAD_DSEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIGMUX_PAD_DSEL to value 0"]
impl crate::Resettable for DIGMUX_PAD_DSEL_SPEC {}
