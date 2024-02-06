#[doc = "Register `SLC_APBWIN_CONF` reader"]
pub type R = crate::R<SLC_APBWIN_CONF_SPEC>;
#[doc = "Register `SLC_APBWIN_CONF` writer"]
pub type W = crate::W<SLC_APBWIN_CONF_SPEC>;
#[doc = "Field `SLC_APBWIN_ADDR` reader - *******Description***********"]
pub type SLC_APBWIN_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SLC_APBWIN_ADDR` writer - *******Description***********"]
pub type SLC_APBWIN_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `SLC_APBWIN_WR` reader - *******Description***********"]
pub type SLC_APBWIN_WR_R = crate::BitReader;
#[doc = "Field `SLC_APBWIN_WR` writer - *******Description***********"]
pub type SLC_APBWIN_WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_APBWIN_START` reader - *******Description***********"]
pub type SLC_APBWIN_START_R = crate::BitReader;
#[doc = "Field `SLC_APBWIN_START` writer - *******Description***********"]
pub type SLC_APBWIN_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27 - *******Description***********"]
    #[inline(always)]
    pub fn slc_apbwin_addr(&self) -> SLC_APBWIN_ADDR_R {
        SLC_APBWIN_ADDR_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28 - *******Description***********"]
    #[inline(always)]
    pub fn slc_apbwin_wr(&self) -> SLC_APBWIN_WR_R {
        SLC_APBWIN_WR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - *******Description***********"]
    #[inline(always)]
    pub fn slc_apbwin_start(&self) -> SLC_APBWIN_START_R {
        SLC_APBWIN_START_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_APBWIN_CONF")
            .field(
                "slc_apbwin_addr",
                &format_args!("{}", self.slc_apbwin_addr().bits()),
            )
            .field(
                "slc_apbwin_wr",
                &format_args!("{}", self.slc_apbwin_wr().bit()),
            )
            .field(
                "slc_apbwin_start",
                &format_args!("{}", self.slc_apbwin_start().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_APBWIN_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:27 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc_apbwin_addr(&mut self) -> SLC_APBWIN_ADDR_W<SLC_APBWIN_CONF_SPEC> {
        SLC_APBWIN_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 28 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc_apbwin_wr(&mut self) -> SLC_APBWIN_WR_W<SLC_APBWIN_CONF_SPEC> {
        SLC_APBWIN_WR_W::new(self, 28)
    }
    #[doc = "Bit 29 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc_apbwin_start(&mut self) -> SLC_APBWIN_START_W<SLC_APBWIN_CONF_SPEC> {
        SLC_APBWIN_START_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_apbwin_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_apbwin_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_APBWIN_CONF_SPEC;
impl crate::RegisterSpec for SLC_APBWIN_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_apbwin_conf::R`](R) reader structure"]
impl crate::Readable for SLC_APBWIN_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_apbwin_conf::W`](W) writer structure"]
impl crate::Writable for SLC_APBWIN_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC_APBWIN_CONF to value 0"]
impl crate::Resettable for SLC_APBWIN_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
