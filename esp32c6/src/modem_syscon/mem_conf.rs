#[doc = "Register `MEM_CONF` reader"]
pub type R = crate::R<MEM_CONF_SPEC>;
#[doc = "Register `MEM_CONF` writer"]
pub type W = crate::W<MEM_CONF_SPEC>;
#[doc = "Field `MODEM_MEM_WP` reader - "]
pub type MODEM_MEM_WP_R = crate::FieldReader;
#[doc = "Field `MODEM_MEM_WP` writer - "]
pub type MODEM_MEM_WP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODEM_MEM_WA` reader - "]
pub type MODEM_MEM_WA_R = crate::FieldReader;
#[doc = "Field `MODEM_MEM_WA` writer - "]
pub type MODEM_MEM_WA_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODEM_MEM_RA` reader - "]
pub type MODEM_MEM_RA_R = crate::FieldReader;
#[doc = "Field `MODEM_MEM_RA` writer - "]
pub type MODEM_MEM_RA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn modem_mem_wp(&self) -> MODEM_MEM_WP_R {
        MODEM_MEM_WP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn modem_mem_wa(&self) -> MODEM_MEM_WA_R {
        MODEM_MEM_WA_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn modem_mem_ra(&self) -> MODEM_MEM_RA_R {
        MODEM_MEM_RA_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CONF")
            .field("modem_mem_wp", &self.modem_mem_wp().bits())
            .field("modem_mem_wa", &self.modem_mem_wa().bits())
            .field("modem_mem_ra", &self.modem_mem_ra().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn modem_mem_wp(&mut self) -> MODEM_MEM_WP_W<MEM_CONF_SPEC> {
        MODEM_MEM_WP_W::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    #[must_use]
    pub fn modem_mem_wa(&mut self) -> MODEM_MEM_WA_W<MEM_CONF_SPEC> {
        MODEM_MEM_WA_W::new(self, 3)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn modem_mem_ra(&mut self) -> MODEM_MEM_RA_W<MEM_CONF_SPEC> {
        MODEM_MEM_RA_W::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CONF_SPEC;
impl crate::RegisterSpec for MEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_conf::R`](R) reader structure"]
impl crate::Readable for MEM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_conf::W`](W) writer structure"]
impl crate::Writable for MEM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CONF to value 0x20"]
impl crate::Resettable for MEM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
