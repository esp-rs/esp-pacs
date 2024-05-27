#[doc = "Register `BLK0_WDATA4` reader"]
pub type R = crate::R<BLK0_WDATA4_SPEC>;
#[doc = "Register `BLK0_WDATA4` writer"]
pub type W = crate::W<BLK0_WDATA4_SPEC>;
#[doc = "Field `CLK8M_FREQ` reader - "]
pub type CLK8M_FREQ_R = crate::FieldReader;
#[doc = "Field `ADC_VREF` reader - "]
pub type ADC_VREF_R = crate::FieldReader;
#[doc = "Field `ADC_VREF` writer - "]
pub type ADC_VREF_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVE_0_141` reader - "]
pub type RESERVE_0_141_R = crate::BitReader;
#[doc = "Field `RESERVE_0_141` writer - "]
pub type RESERVE_0_141_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_SDIO` reader - "]
pub type XPD_SDIO_R = crate::BitReader;
#[doc = "Field `XPD_SDIO_TIEH` reader - "]
pub type XPD_SDIO_TIEH_R = crate::BitReader;
#[doc = "Field `XPD_SDIO_FORCE` reader - "]
pub type XPD_SDIO_FORCE_R = crate::BitReader;
#[doc = "Field `RESERVE_0_145` reader - "]
pub type RESERVE_0_145_R = crate::FieldReader<u16>;
#[doc = "Field `RESERVE_0_145` writer - "]
pub type RESERVE_0_145_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clk8m_freq(&self) -> CLK8M_FREQ_R {
        CLK8M_FREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn adc_vref(&self) -> ADC_VREF_R {
        ADC_VREF_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reserve_0_141(&self) -> RESERVE_0_141_R {
        RESERVE_0_141_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn xpd_sdio(&self) -> XPD_SDIO_R {
        XPD_SDIO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn xpd_sdio_tieh(&self) -> XPD_SDIO_TIEH_R {
        XPD_SDIO_TIEH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn xpd_sdio_force(&self) -> XPD_SDIO_FORCE_R {
        XPD_SDIO_FORCE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn reserve_0_145(&self) -> RESERVE_0_145_R {
        RESERVE_0_145_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_WDATA4")
            .field("clk8m_freq", &self.clk8m_freq())
            .field("adc_vref", &self.adc_vref())
            .field("reserve_0_141", &self.reserve_0_141())
            .field("xpd_sdio", &self.xpd_sdio())
            .field("xpd_sdio_tieh", &self.xpd_sdio_tieh())
            .field("xpd_sdio_force", &self.xpd_sdio_force())
            .field("reserve_0_145", &self.reserve_0_145())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn adc_vref(&mut self) -> ADC_VREF_W<BLK0_WDATA4_SPEC> {
        ADC_VREF_W::new(self, 8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reserve_0_141(&mut self) -> RESERVE_0_141_W<BLK0_WDATA4_SPEC> {
        RESERVE_0_141_W::new(self, 13)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    #[must_use]
    pub fn reserve_0_145(&mut self) -> RESERVE_0_145_W<BLK0_WDATA4_SPEC> {
        RESERVE_0_145_W::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_WDATA4_SPEC;
impl crate::RegisterSpec for BLK0_WDATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_wdata4::R`](R) reader structure"]
impl crate::Readable for BLK0_WDATA4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk0_wdata4::W`](W) writer structure"]
impl crate::Writable for BLK0_WDATA4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK0_WDATA4 to value 0"]
impl crate::Resettable for BLK0_WDATA4_SPEC {
    const RESET_VALUE: u32 = 0;
}
