#[doc = "Register `PADCTRL` reader"]
pub type R = crate::R<PADCTRL_SPEC>;
#[doc = "Register `PADCTRL` writer"]
pub type W = crate::W<PADCTRL_SPEC>;
#[doc = "Field `PAD_MUX_SEL` reader - "]
pub type PAD_MUX_SEL_R = crate::FieldReader;
#[doc = "Field `PAD_MUX_SEL` writer - "]
pub type PAD_MUX_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_HOLD` reader - "]
pub type PAD_HOLD_R = crate::FieldReader;
#[doc = "Field `PAD_HOLD` writer - "]
pub type PAD_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pad_mux_sel(&self) -> PAD_MUX_SEL_R {
        PAD_MUX_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pad_hold(&self) -> PAD_HOLD_R {
        PAD_HOLD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PADCTRL")
            .field("pad_mux_sel", &self.pad_mux_sel())
            .field("pad_hold", &self.pad_hold())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pad_mux_sel(&mut self) -> PAD_MUX_SEL_W<'_, PADCTRL_SPEC> {
        PAD_MUX_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn pad_hold(&mut self) -> PAD_HOLD_W<'_, PADCTRL_SPEC> {
        PAD_HOLD_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`padctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PADCTRL_SPEC;
impl crate::RegisterSpec for PADCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`padctrl::R`](R) reader structure"]
impl crate::Readable for PADCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`padctrl::W`](W) writer structure"]
impl crate::Writable for PADCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PADCTRL to value 0"]
impl crate::Resettable for PADCTRL_SPEC {}
