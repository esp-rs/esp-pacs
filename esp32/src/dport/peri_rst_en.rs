#[doc = "Register `PERI_RST_EN` reader"]
pub type R = crate::R<PERI_RST_EN_SPEC>;
#[doc = "Register `PERI_RST_EN` writer"]
pub type W = crate::W<PERI_RST_EN_SPEC>;
#[doc = "Field `PERI_RST_EN` reader - "]
pub type PERI_RST_EN_R = crate::FieldReader<u32>;
#[doc = "Field `PERI_RST_EN` writer - "]
pub type PERI_RST_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn peri_rst_en(&self) -> PERI_RST_EN_R {
        PERI_RST_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_RST_EN")
            .field("peri_rst_en", &self.peri_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn peri_rst_en(&mut self) -> PERI_RST_EN_W<PERI_RST_EN_SPEC> {
        PERI_RST_EN_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_rst_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_rst_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_RST_EN_SPEC;
impl crate::RegisterSpec for PERI_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_rst_en::R`](R) reader structure"]
impl crate::Readable for PERI_RST_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_rst_en::W`](W) writer structure"]
impl crate::Writable for PERI_RST_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_RST_EN to value 0"]
impl crate::Resettable for PERI_RST_EN_SPEC {}
