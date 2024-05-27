#[doc = "Register `BLK0_RDATA4` reader"]
pub type R = crate::R<BLK0_RDATA4_SPEC>;
#[doc = "Register `BLK0_RDATA4` writer"]
pub type W = crate::W<BLK0_RDATA4_SPEC>;
#[doc = "Field `RD_CLK8M_FREQ` reader - "]
pub type RD_CLK8M_FREQ_R = crate::FieldReader;
#[doc = "Field `RD_ADC_VREF` reader - "]
pub type RD_ADC_VREF_R = crate::FieldReader;
#[doc = "Field `RD_ADC_VREF` writer - "]
pub type RD_ADC_VREF_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RD_RESERVE_0_141` reader - "]
pub type RD_RESERVE_0_141_R = crate::BitReader;
#[doc = "Field `RD_RESERVE_0_141` writer - "]
pub type RD_RESERVE_0_141_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_XPD_SDIO` reader - "]
pub type RD_XPD_SDIO_R = crate::BitReader;
#[doc = "Field `RD_XPD_SDIO_TIEH` reader - "]
pub type RD_XPD_SDIO_TIEH_R = crate::BitReader;
#[doc = "Field `RD_XPD_SDIO_FORCE` reader - "]
pub type RD_XPD_SDIO_FORCE_R = crate::BitReader;
#[doc = "Field `RD_RESERVE_0_145` reader - "]
pub type RD_RESERVE_0_145_R = crate::FieldReader<u16>;
#[doc = "Field `RD_RESERVE_0_145` writer - "]
pub type RD_RESERVE_0_145_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rd_clk8m_freq(&self) -> RD_CLK8M_FREQ_R {
        RD_CLK8M_FREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn rd_adc_vref(&self) -> RD_ADC_VREF_R {
        RD_ADC_VREF_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rd_reserve_0_141(&self) -> RD_RESERVE_0_141_R {
        RD_RESERVE_0_141_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rd_xpd_sdio(&self) -> RD_XPD_SDIO_R {
        RD_XPD_SDIO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rd_xpd_sdio_tieh(&self) -> RD_XPD_SDIO_TIEH_R {
        RD_XPD_SDIO_TIEH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rd_xpd_sdio_force(&self) -> RD_XPD_SDIO_FORCE_R {
        RD_XPD_SDIO_FORCE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn rd_reserve_0_145(&self) -> RD_RESERVE_0_145_R {
        RD_RESERVE_0_145_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_RDATA4")
            .field("rd_clk8m_freq", &self.rd_clk8m_freq())
            .field("rd_adc_vref", &self.rd_adc_vref())
            .field("rd_reserve_0_141", &self.rd_reserve_0_141())
            .field("rd_xpd_sdio", &self.rd_xpd_sdio())
            .field("rd_xpd_sdio_tieh", &self.rd_xpd_sdio_tieh())
            .field("rd_xpd_sdio_force", &self.rd_xpd_sdio_force())
            .field("rd_reserve_0_145", &self.rd_reserve_0_145())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn rd_adc_vref(&mut self) -> RD_ADC_VREF_W<BLK0_RDATA4_SPEC> {
        RD_ADC_VREF_W::new(self, 8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn rd_reserve_0_141(&mut self) -> RD_RESERVE_0_141_W<BLK0_RDATA4_SPEC> {
        RD_RESERVE_0_141_W::new(self, 13)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    #[must_use]
    pub fn rd_reserve_0_145(&mut self) -> RD_RESERVE_0_145_W<BLK0_RDATA4_SPEC> {
        RD_RESERVE_0_145_W::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_rdata4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_RDATA4_SPEC;
impl crate::RegisterSpec for BLK0_RDATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_rdata4::R`](R) reader structure"]
impl crate::Readable for BLK0_RDATA4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk0_rdata4::W`](W) writer structure"]
impl crate::Writable for BLK0_RDATA4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK0_RDATA4 to value 0"]
impl crate::Resettable for BLK0_RDATA4_SPEC {
    const RESET_VALUE: u32 = 0;
}
