#[doc = "Register `DEBUG_CTRL0` reader"]
pub type R = crate::R<DEBUG_CTRL0_SPEC>;
#[doc = "Register `DEBUG_CTRL0` writer"]
pub type W = crate::W<DEBUG_CTRL0_SPEC>;
#[doc = "Field `DBG_REPLACE_SEL` reader - Configures the data replace location. 0: not replace, 1: srm rx input, 2: srm rx bilin interpolation, 3: srm tx output, 4: blend fg input, 5: blend bg input, 6: blend output"]
pub type DBG_REPLACE_SEL_R = crate::FieldReader;
#[doc = "Field `DBG_REPLACE_SEL` writer - Configures the data replace location. 0: not replace, 1: srm rx input, 2: srm rx bilin interpolation, 3: srm tx output, 4: blend fg input, 5: blend bg input, 6: blend output"]
pub type DBG_REPLACE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Configures the data replace location. 0: not replace, 1: srm rx input, 2: srm rx bilin interpolation, 3: srm tx output, 4: blend fg input, 5: blend bg input, 6: blend output"]
    #[inline(always)]
    pub fn dbg_replace_sel(&self) -> DBG_REPLACE_SEL_R {
        DBG_REPLACE_SEL_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_CTRL0")
            .field("dbg_replace_sel", &self.dbg_replace_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures the data replace location. 0: not replace, 1: srm rx input, 2: srm rx bilin interpolation, 3: srm tx output, 4: blend fg input, 5: blend bg input, 6: blend output"]
    #[inline(always)]
    pub fn dbg_replace_sel(&mut self) -> DBG_REPLACE_SEL_W<'_, DEBUG_CTRL0_SPEC> {
        DBG_REPLACE_SEL_W::new(self, 0)
    }
}
#[doc = "debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_CTRL0_SPEC;
impl crate::RegisterSpec for DEBUG_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_ctrl0::R`](R) reader structure"]
impl crate::Readable for DEBUG_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug_ctrl0::W`](W) writer structure"]
impl crate::Writable for DEBUG_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUG_CTRL0 to value 0"]
impl crate::Resettable for DEBUG_CTRL0_SPEC {}
