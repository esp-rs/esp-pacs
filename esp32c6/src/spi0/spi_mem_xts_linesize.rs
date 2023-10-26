#[doc = "Register `SPI_MEM_XTS_LINESIZE` reader"]
pub type R = crate::R<SPI_MEM_XTS_LINESIZE_SPEC>;
#[doc = "Register `SPI_MEM_XTS_LINESIZE` writer"]
pub type W = crate::W<SPI_MEM_XTS_LINESIZE_SPEC>;
#[doc = "Field `SPI_XTS_LINESIZE` reader - This bits stores the line-size parameter which will be used in manual encryption calculation. It decides how many bytes will be encrypted one time. 0: 16-bytes, 1: 32-bytes, 2: 64-bytes, 3:reserved."]
pub type SPI_XTS_LINESIZE_R = crate::FieldReader;
#[doc = "Field `SPI_XTS_LINESIZE` writer - This bits stores the line-size parameter which will be used in manual encryption calculation. It decides how many bytes will be encrypted one time. 0: 16-bytes, 1: 32-bytes, 2: 64-bytes, 3:reserved."]
pub type SPI_XTS_LINESIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - This bits stores the line-size parameter which will be used in manual encryption calculation. It decides how many bytes will be encrypted one time. 0: 16-bytes, 1: 32-bytes, 2: 64-bytes, 3:reserved."]
    #[inline(always)]
    pub fn spi_xts_linesize(&self) -> SPI_XTS_LINESIZE_R {
        SPI_XTS_LINESIZE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_XTS_LINESIZE")
            .field(
                "spi_xts_linesize",
                &format_args!("{}", self.spi_xts_linesize().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_XTS_LINESIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - This bits stores the line-size parameter which will be used in manual encryption calculation. It decides how many bytes will be encrypted one time. 0: 16-bytes, 1: 32-bytes, 2: 64-bytes, 3:reserved."]
    #[inline(always)]
    #[must_use]
    pub fn spi_xts_linesize(&mut self) -> SPI_XTS_LINESIZE_W<SPI_MEM_XTS_LINESIZE_SPEC, 0> {
        SPI_XTS_LINESIZE_W::new(self)
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
#[doc = "Manual Encryption Line-Size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_xts_linesize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_linesize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_XTS_LINESIZE_SPEC;
impl crate::RegisterSpec for SPI_MEM_XTS_LINESIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_xts_linesize::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_XTS_LINESIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_xts_linesize::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_XTS_LINESIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_XTS_LINESIZE to value 0"]
impl crate::Resettable for SPI_MEM_XTS_LINESIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
