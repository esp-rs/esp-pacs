#[doc = "Register `SINTX_DATA` reader"]
pub type R = crate::R<SINTX_DATA_SPEC>;
#[doc = "Register `SINTX_DATA` writer"]
pub type W = crate::W<SINTX_DATA_SPEC>;
#[doc = "Field `DC_1` reader - the data output for DAC PAD0 for sintx path"]
pub type DC_1_R = crate::FieldReader<u16>;
#[doc = "Field `DC_1` writer - the data output for DAC PAD0 for sintx path"]
pub type DC_1_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DC_2` reader - the data output for DAC PAD1 for sintx path"]
pub type DC_2_R = crate::FieldReader<u16>;
#[doc = "Field `DC_2` writer - the data output for DAC PAD1 for sintx path"]
pub type DC_2_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - the data output for DAC PAD0 for sintx path"]
    #[inline(always)]
    pub fn dc_1(&self) -> DC_1_R {
        DC_1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - the data output for DAC PAD1 for sintx path"]
    #[inline(always)]
    pub fn dc_2(&self) -> DC_2_R {
        DC_2_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINTX_DATA")
            .field("dc_1", &self.dc_1())
            .field("dc_2", &self.dc_2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - the data output for DAC PAD0 for sintx path"]
    #[inline(always)]
    pub fn dc_1(&mut self) -> DC_1_W<'_, SINTX_DATA_SPEC> {
        DC_1_W::new(self, 0)
    }
    #[doc = "Bits 12:23 - the data output for DAC PAD1 for sintx path"]
    #[inline(always)]
    pub fn dc_2(&mut self) -> DC_2_W<'_, SINTX_DATA_SPEC> {
        DC_2_W::new(self, 12)
    }
}
#[doc = "dac output register for sintx path\n\nYou can [`read`](crate::Reg::read) this register and get [`sintx_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sintx_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINTX_DATA_SPEC;
impl crate::RegisterSpec for SINTX_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sintx_data::R`](R) reader structure"]
impl crate::Readable for SINTX_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sintx_data::W`](W) writer structure"]
impl crate::Writable for SINTX_DATA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SINTX_DATA to value 0"]
impl crate::Resettable for SINTX_DATA_SPEC {}
