#[doc = "Register `SDIO_CRC_ST1` reader"]
pub type R = crate::R<SDIO_CRC_ST1_SPEC>;
#[doc = "Register `SDIO_CRC_ST1` writer"]
pub type W = crate::W<SDIO_CRC_ST1_SPEC>;
#[doc = "Field `CMD_CRC_ERR_CNT` reader - "]
pub type CMD_CRC_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `ERR_CNT_CLR` reader - "]
pub type ERR_CNT_CLR_R = crate::BitReader;
#[doc = "Field `ERR_CNT_CLR` writer - "]
pub type ERR_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cmd_crc_err_cnt(&self) -> CMD_CRC_ERR_CNT_R {
        CMD_CRC_ERR_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn err_cnt_clr(&self) -> ERR_CNT_CLR_R {
        ERR_CNT_CLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_CRC_ST1")
            .field("cmd_crc_err_cnt", &self.cmd_crc_err_cnt().bits())
            .field("err_cnt_clr", &self.err_cnt_clr().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_CRC_ST1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn err_cnt_clr(&mut self) -> ERR_CNT_CLR_W<SDIO_CRC_ST1_SPEC> {
        ERR_CNT_CLR_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_crc_st1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_crc_st1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_CRC_ST1_SPEC;
impl crate::RegisterSpec for SDIO_CRC_ST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_crc_st1::R`](R) reader structure"]
impl crate::Readable for SDIO_CRC_ST1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_crc_st1::W`](W) writer structure"]
impl crate::Writable for SDIO_CRC_ST1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIO_CRC_ST1 to value 0"]
impl crate::Resettable for SDIO_CRC_ST1_SPEC {
    const RESET_VALUE: u32 = 0;
}
