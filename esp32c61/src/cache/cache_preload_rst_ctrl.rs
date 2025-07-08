#[doc = "Register `CACHE_PRELOAD_RST_CTRL` reader"]
pub type R = crate::R<CACHE_PRELOAD_RST_CTRL_SPEC>;
#[doc = "Register `CACHE_PRELOAD_RST_CTRL` writer"]
pub type W = crate::W<CACHE_PRELOAD_RST_CTRL_SPEC>;
#[doc = "Field `ICACHE0_PLD_RST` reader - set this bit to reset preload-logic inside L1-ICache0. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type ICACHE0_PLD_RST_R = crate::BitReader;
#[doc = "Field `ICACHE1_PLD_RST` reader - set this bit to reset preload-logic inside L1-ICache1. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type ICACHE1_PLD_RST_R = crate::BitReader;
#[doc = "Field `ICACHE2_PLD_RST` reader - Reserved"]
pub type ICACHE2_PLD_RST_R = crate::BitReader;
#[doc = "Field `ICACHE3_PLD_RST` reader - Reserved"]
pub type ICACHE3_PLD_RST_R = crate::BitReader;
#[doc = "Field `CACHE_PLD_RST` reader - set this bit to reset preload-logic inside L1-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type CACHE_PLD_RST_R = crate::BitReader;
#[doc = "Field `CACHE_PLD_RST` writer - set this bit to reset preload-logic inside L1-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type CACHE_PLD_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set this bit to reset preload-logic inside L1-ICache0. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn icache0_pld_rst(&self) -> ICACHE0_PLD_RST_R {
        ICACHE0_PLD_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set this bit to reset preload-logic inside L1-ICache1. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn icache1_pld_rst(&self) -> ICACHE1_PLD_RST_R {
        ICACHE1_PLD_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_pld_rst(&self) -> ICACHE2_PLD_RST_R {
        ICACHE2_PLD_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn icache3_pld_rst(&self) -> ICACHE3_PLD_RST_R {
        ICACHE3_PLD_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - set this bit to reset preload-logic inside L1-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn cache_pld_rst(&self) -> CACHE_PLD_RST_R {
        CACHE_PLD_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_PRELOAD_RST_CTRL")
            .field("icache0_pld_rst", &self.icache0_pld_rst())
            .field("icache1_pld_rst", &self.icache1_pld_rst())
            .field("icache2_pld_rst", &self.icache2_pld_rst())
            .field("icache3_pld_rst", &self.icache3_pld_rst())
            .field("cache_pld_rst", &self.cache_pld_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - set this bit to reset preload-logic inside L1-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn cache_pld_rst(&mut self) -> CACHE_PLD_RST_W<CACHE_PRELOAD_RST_CTRL_SPEC> {
        CACHE_PLD_RST_W::new(self, 4)
    }
}
#[doc = "Cache Preload Reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_preload_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_preload_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_PRELOAD_RST_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_PRELOAD_RST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_preload_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_PRELOAD_RST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_preload_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_PRELOAD_RST_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_PRELOAD_RST_CTRL to value 0"]
impl crate::Resettable for CACHE_PRELOAD_RST_CTRL_SPEC {}
