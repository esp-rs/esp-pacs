#[doc = "Register `RST_EN` reader"]
pub type R = crate::R<RST_EN_SPEC>;
#[doc = "Register `RST_EN` writer"]
pub type W = crate::W<RST_EN_SPEC>;
#[doc = "Field `PCR_RST_EN` reader - Set 1 to reset pcr module"]
pub type PCR_RST_EN_R = crate::BitReader;
#[doc = "Field `PCR_RST_EN` writer - Set 1 to reset pcr module"]
pub type PCR_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to reset pcr module"]
    #[inline(always)]
    pub fn pcr_rst_en(&self) -> PCR_RST_EN_R {
        PCR_RST_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RST_EN")
            .field("pcr_rst_en", &self.pcr_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to reset pcr module"]
    #[inline(always)]
    pub fn pcr_rst_en(&mut self) -> PCR_RST_EN_W<'_, RST_EN_SPEC> {
        PCR_RST_EN_W::new(self, 0)
    }
}
#[doc = "PCR clock gating configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST_EN_SPEC;
impl crate::RegisterSpec for RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_en::R`](R) reader structure"]
impl crate::Readable for RST_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rst_en::W`](W) writer structure"]
impl crate::Writable for RST_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RST_EN to value 0"]
impl crate::Resettable for RST_EN_SPEC {}
