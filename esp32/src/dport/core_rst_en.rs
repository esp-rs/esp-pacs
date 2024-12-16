#[doc = "Register `CORE_RST_EN` reader"]
pub type R = crate::R<CORE_RST_EN_SPEC>;
#[doc = "Register `CORE_RST_EN` writer"]
pub type W = crate::W<CORE_RST_EN_SPEC>;
#[doc = "Field `CORE_RST` reader - "]
pub type CORE_RST_R = crate::FieldReader;
#[doc = "Field `CORE_RST` writer - "]
pub type CORE_RST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn core_rst(&self) -> CORE_RST_R {
        CORE_RST_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_RST_EN")
            .field("core_rst", &self.core_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn core_rst(&mut self) -> CORE_RST_W<CORE_RST_EN_SPEC> {
        CORE_RST_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`core_rst_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_rst_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_RST_EN_SPEC;
impl crate::RegisterSpec for CORE_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_rst_en::R`](R) reader structure"]
impl crate::Readable for CORE_RST_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_rst_en::W`](W) writer structure"]
impl crate::Writable for CORE_RST_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_RST_EN to value 0"]
impl crate::Resettable for CORE_RST_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
