#[doc = "Register `SLC_SDIO_ST` reader"]
pub type R = crate::R<SLC_SDIO_ST_SPEC>;
#[doc = "Register `SLC_SDIO_ST` writer"]
pub type W = crate::W<SLC_SDIO_ST_SPEC>;
#[doc = "Field `SLC_CMD_ST` reader - "]
pub type SLC_CMD_ST_R = crate::FieldReader;
#[doc = "Field `SLC_CMD_ST` writer - "]
pub type SLC_CMD_ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SLC_FUNC_ST` reader - "]
pub type SLC_FUNC_ST_R = crate::FieldReader;
#[doc = "Field `SLC_FUNC_ST` writer - "]
pub type SLC_FUNC_ST_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SLC_SDIO_WAKEUP` reader - "]
pub type SLC_SDIO_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLC_SDIO_WAKEUP` writer - "]
pub type SLC_SDIO_WAKEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLC_BUS_ST` reader - "]
pub type SLC_BUS_ST_R = crate::FieldReader;
#[doc = "Field `SLC_BUS_ST` writer - "]
pub type SLC_BUS_ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn slc_cmd_st(&self) -> SLC_CMD_ST_R {
        SLC_CMD_ST_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn slc_func_st(&self) -> SLC_FUNC_ST_R {
        SLC_FUNC_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_sdio_wakeup(&self) -> SLC_SDIO_WAKEUP_R {
        SLC_SDIO_WAKEUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn slc_bus_st(&self) -> SLC_BUS_ST_R {
        SLC_BUS_ST_R::new(((self.bits >> 12) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_SDIO_ST")
            .field("slc_bus_st", &format_args!("{}", self.slc_bus_st().bits()))
            .field(
                "slc_sdio_wakeup",
                &format_args!("{}", self.slc_sdio_wakeup().bit()),
            )
            .field(
                "slc_func_st",
                &format_args!("{}", self.slc_func_st().bits()),
            )
            .field("slc_cmd_st", &format_args!("{}", self.slc_cmd_st().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_SDIO_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn slc_cmd_st(&mut self) -> SLC_CMD_ST_W<SLC_SDIO_ST_SPEC> {
        SLC_CMD_ST_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn slc_func_st(&mut self) -> SLC_FUNC_ST_W<SLC_SDIO_ST_SPEC> {
        SLC_FUNC_ST_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn slc_sdio_wakeup(&mut self) -> SLC_SDIO_WAKEUP_W<SLC_SDIO_ST_SPEC> {
        SLC_SDIO_WAKEUP_W::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn slc_bus_st(&mut self) -> SLC_BUS_ST_W<SLC_SDIO_ST_SPEC> {
        SLC_BUS_ST_W::new(self, 12)
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
#[doc = "SLC_SDIO_ST\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_sdio_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_sdio_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_SDIO_ST_SPEC;
impl crate::RegisterSpec for SLC_SDIO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_sdio_st::R`](R) reader structure"]
impl crate::Readable for SLC_SDIO_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_sdio_st::W`](W) writer structure"]
impl crate::Writable for SLC_SDIO_ST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_SDIO_ST to value 0"]
impl crate::Resettable for SLC_SDIO_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
