#[doc = "Register `JTAG_TDIO_MUX_SEL` reader"]
pub type R = crate::R<JTAG_TDIO_MUX_SEL_SPEC>;
#[doc = "Register `JTAG_TDIO_MUX_SEL` writer"]
pub type W = crate::W<JTAG_TDIO_MUX_SEL_SPEC>;
#[doc = "Field `JTAG_TDIO_MUX_SEL` reader - "]
pub type JTAG_TDIO_MUX_SEL_R = crate::FieldReader;
#[doc = "Field `JTAG_TDIO_MUX_SEL` writer - "]
pub type JTAG_TDIO_MUX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn jtag_tdio_mux_sel(&self) -> JTAG_TDIO_MUX_SEL_R {
        JTAG_TDIO_MUX_SEL_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JTAG_TDIO_MUX_SEL")
            .field("jtag_tdio_mux_sel", &self.jtag_tdio_mux_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn jtag_tdio_mux_sel(&mut self) -> JTAG_TDIO_MUX_SEL_W<'_, JTAG_TDIO_MUX_SEL_SPEC> {
        JTAG_TDIO_MUX_SEL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag_tdio_mux_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag_tdio_mux_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JTAG_TDIO_MUX_SEL_SPEC;
impl crate::RegisterSpec for JTAG_TDIO_MUX_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag_tdio_mux_sel::R`](R) reader structure"]
impl crate::Readable for JTAG_TDIO_MUX_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`jtag_tdio_mux_sel::W`](W) writer structure"]
impl crate::Writable for JTAG_TDIO_MUX_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG_TDIO_MUX_SEL to value 0"]
impl crate::Resettable for JTAG_TDIO_MUX_SEL_SPEC {}
