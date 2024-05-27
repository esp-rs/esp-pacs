#[doc = "Register `CFG0` reader"]
pub type R = crate::R<CFG0_SPEC>;
#[doc = "Register `CFG0` writer"]
pub type W = crate::W<CFG0_SPEC>;
#[doc = "Field `DMAC_EN` reader - NA"]
pub type DMAC_EN_R = crate::BitReader;
#[doc = "Field `DMAC_EN` writer - NA"]
pub type DMAC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_EN` reader - NA"]
pub type INT_EN_R = crate::BitReader;
#[doc = "Field `INT_EN` writer - NA"]
pub type INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn dmac_en(&self) -> DMAC_EN_R {
        DMAC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG0")
            .field("dmac_en", &self.dmac_en())
            .field("int_en", &self.int_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_en(&mut self) -> DMAC_EN_W<CFG0_SPEC> {
        DMAC_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> INT_EN_W<CFG0_SPEC> {
        INT_EN_W::new(self, 1)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0::R`](R) reader structure"]
impl crate::Readable for CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg0::W`](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for CFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
