#[doc = "Register `DPI_RSV_DPI_DATA` reader"]
pub type R = crate::R<DPI_RSV_DPI_DATA_SPEC>;
#[doc = "Register `DPI_RSV_DPI_DATA` writer"]
pub type W = crate::W<DPI_RSV_DPI_DATA_SPEC>;
#[doc = "Field `DPI_RSV_DATA` reader - this field controls the pixel data sent to dsi_host when dsi_bridge fifo underflow"]
pub type DPI_RSV_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DPI_RSV_DATA` writer - this field controls the pixel data sent to dsi_host when dsi_bridge fifo underflow"]
pub type DPI_RSV_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - this field controls the pixel data sent to dsi_host when dsi_bridge fifo underflow"]
    #[inline(always)]
    pub fn dpi_rsv_data(&self) -> DPI_RSV_DATA_R {
        DPI_RSV_DATA_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_RSV_DPI_DATA")
            .field(
                "dpi_rsv_data",
                &format_args!("{}", self.dpi_rsv_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DPI_RSV_DPI_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:29 - this field controls the pixel data sent to dsi_host when dsi_bridge fifo underflow"]
    #[inline(always)]
    #[must_use]
    pub fn dpi_rsv_data(&mut self) -> DPI_RSV_DATA_W<DPI_RSV_DPI_DATA_SPEC> {
        DPI_RSV_DATA_W::new(self, 0)
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
#[doc = "dsi bridge dpi reserved data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi_rsv_dpi_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi_rsv_dpi_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPI_RSV_DPI_DATA_SPEC;
impl crate::RegisterSpec for DPI_RSV_DPI_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_rsv_dpi_data::R`](R) reader structure"]
impl crate::Readable for DPI_RSV_DPI_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpi_rsv_dpi_data::W`](W) writer structure"]
impl crate::Writable for DPI_RSV_DPI_DATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPI_RSV_DPI_DATA to value 0x3fff"]
impl crate::Resettable for DPI_RSV_DPI_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff;
}
