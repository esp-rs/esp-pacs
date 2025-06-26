#[doc = "Register `HFIR` reader"]
pub type R = crate::R<HFIR_SPEC>;
#[doc = "Register `HFIR` writer"]
pub type W = crate::W<HFIR_SPEC>;
#[doc = "Field `FRINT` reader - "]
pub type FRINT_R = crate::FieldReader<u16>;
#[doc = "Field `FRINT` writer - "]
pub type FRINT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HFIRRLDCTRL` reader - "]
pub type HFIRRLDCTRL_R = crate::BitReader;
#[doc = "Field `HFIRRLDCTRL` writer - "]
pub type HFIRRLDCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn frint(&self) -> FRINT_R {
        FRINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hfirrldctrl(&self) -> HFIRRLDCTRL_R {
        HFIRRLDCTRL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFIR")
            .field("frint", &self.frint())
            .field("hfirrldctrl", &self.hfirrldctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn frint(&mut self) -> FRINT_W<HFIR_SPEC> {
        FRINT_W::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hfirrldctrl(&mut self) -> HFIRRLDCTRL_W<HFIR_SPEC> {
        HFIRRLDCTRL_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hfir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFIR_SPEC;
impl crate::RegisterSpec for HFIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfir::R`](R) reader structure"]
impl crate::Readable for HFIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hfir::W`](W) writer structure"]
impl crate::Writable for HFIR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFIR to value 0x17d7"]
impl crate::Resettable for HFIR_SPEC {
    const RESET_VALUE: u32 = 0x17d7;
}
