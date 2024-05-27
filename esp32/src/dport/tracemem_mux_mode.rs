#[doc = "Register `TRACEMEM_MUX_MODE` reader"]
pub type R = crate::R<TRACEMEM_MUX_MODE_SPEC>;
#[doc = "Register `TRACEMEM_MUX_MODE` writer"]
pub type W = crate::W<TRACEMEM_MUX_MODE_SPEC>;
#[doc = "Field `TRACEMEM_MUX_MODE` reader - "]
pub type TRACEMEM_MUX_MODE_R = crate::FieldReader;
#[doc = "Field `TRACEMEM_MUX_MODE` writer - "]
pub type TRACEMEM_MUX_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn tracemem_mux_mode(&self) -> TRACEMEM_MUX_MODE_R {
        TRACEMEM_MUX_MODE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRACEMEM_MUX_MODE")
            .field("tracemem_mux_mode", &self.tracemem_mux_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn tracemem_mux_mode(&mut self) -> TRACEMEM_MUX_MODE_W<TRACEMEM_MUX_MODE_SPEC> {
        TRACEMEM_MUX_MODE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tracemem_mux_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tracemem_mux_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRACEMEM_MUX_MODE_SPEC;
impl crate::RegisterSpec for TRACEMEM_MUX_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tracemem_mux_mode::R`](R) reader structure"]
impl crate::Readable for TRACEMEM_MUX_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tracemem_mux_mode::W`](W) writer structure"]
impl crate::Writable for TRACEMEM_MUX_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRACEMEM_MUX_MODE to value 0"]
impl crate::Resettable for TRACEMEM_MUX_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
