#[doc = "Register `HOST_SLC_APBWIN_CONF` reader"]
pub type R = crate::R<HOST_SLC_APBWIN_CONF_SPEC>;
#[doc = "Register `HOST_SLC_APBWIN_CONF` writer"]
pub type W = crate::W<HOST_SLC_APBWIN_CONF_SPEC>;
#[doc = "Field `HOST_SLC_APBWIN_ADDR` reader - "]
pub type HOST_SLC_APBWIN_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `HOST_SLC_APBWIN_ADDR` writer - "]
pub type HOST_SLC_APBWIN_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `HOST_SLC_APBWIN_WR` reader - "]
pub type HOST_SLC_APBWIN_WR_R = crate::BitReader;
#[doc = "Field `HOST_SLC_APBWIN_WR` writer - "]
pub type HOST_SLC_APBWIN_WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_SLC_APBWIN_START` reader - "]
pub type HOST_SLC_APBWIN_START_R = crate::BitReader;
#[doc = "Field `HOST_SLC_APBWIN_START` writer - "]
pub type HOST_SLC_APBWIN_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    pub fn host_slc_apbwin_addr(&self) -> HOST_SLC_APBWIN_ADDR_R {
        HOST_SLC_APBWIN_ADDR_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn host_slc_apbwin_wr(&self) -> HOST_SLC_APBWIN_WR_R {
        HOST_SLC_APBWIN_WR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn host_slc_apbwin_start(&self) -> HOST_SLC_APBWIN_START_R {
        HOST_SLC_APBWIN_START_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC_APBWIN_CONF")
            .field("host_slc_apbwin_addr", &self.host_slc_apbwin_addr().bits())
            .field("host_slc_apbwin_wr", &self.host_slc_apbwin_wr().bit())
            .field("host_slc_apbwin_start", &self.host_slc_apbwin_start().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLC_APBWIN_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:27"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc_apbwin_addr(&mut self) -> HOST_SLC_APBWIN_ADDR_W<HOST_SLC_APBWIN_CONF_SPEC> {
        HOST_SLC_APBWIN_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc_apbwin_wr(&mut self) -> HOST_SLC_APBWIN_WR_W<HOST_SLC_APBWIN_CONF_SPEC> {
        HOST_SLC_APBWIN_WR_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc_apbwin_start(&mut self) -> HOST_SLC_APBWIN_START_W<HOST_SLC_APBWIN_CONF_SPEC> {
        HOST_SLC_APBWIN_START_W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slc_apbwin_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc_apbwin_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLC_APBWIN_CONF_SPEC;
impl crate::RegisterSpec for HOST_SLC_APBWIN_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slc_apbwin_conf::R`](R) reader structure"]
impl crate::Readable for HOST_SLC_APBWIN_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slc_apbwin_conf::W`](W) writer structure"]
impl crate::Writable for HOST_SLC_APBWIN_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_SLC_APBWIN_CONF to value 0"]
impl crate::Resettable for HOST_SLC_APBWIN_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
