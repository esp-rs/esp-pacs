#[doc = "Register `APP_DCACHE_DBUG0` reader"]
pub struct R(crate::R<APP_DCACHE_DBUG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_DCACHE_DBUG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_DCACHE_DBUG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_DCACHE_DBUG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_DCACHE_DBUG0` writer"]
pub struct W(crate::W<APP_DCACHE_DBUG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_DCACHE_DBUG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<APP_DCACHE_DBUG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_DCACHE_DBUG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_SLAVE_WDATA` reader - "]
pub type APP_SLAVE_WDATA_R = crate::BitReader;
#[doc = "Field `APP_SLAVE_WDATA` writer - "]
pub type APP_SLAVE_WDATA_W<'a, const O: u8> = crate::BitWriter<'a, APP_DCACHE_DBUG0_SPEC, O>;
#[doc = "Field `APP_CACHE_MMU_IA` reader - "]
pub type APP_CACHE_MMU_IA_R = crate::BitReader;
#[doc = "Field `APP_CACHE_IA` reader - "]
pub type APP_CACHE_IA_R = crate::FieldReader;
#[doc = "Field `APP_CACHE_STATE` reader - "]
pub type APP_CACHE_STATE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `APP_WR_BAK_TO_READ` reader - "]
pub type APP_WR_BAK_TO_READ_R = crate::BitReader;
#[doc = "Field `APP_TX_END` reader - "]
pub type APP_TX_END_R = crate::BitReader;
#[doc = "Field `APP_SLAVE_WR` reader - "]
pub type APP_SLAVE_WR_R = crate::BitReader;
#[doc = "Field `APP_SLAVE_WDATA_V` reader - "]
pub type APP_SLAVE_WDATA_V_R = crate::BitReader;
#[doc = "Field `APP_RX_END` reader - "]
pub type APP_RX_END_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_slave_wdata(&self) -> APP_SLAVE_WDATA_R {
        APP_SLAVE_WDATA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cache_mmu_ia(&self) -> APP_CACHE_MMU_IA_R {
        APP_CACHE_MMU_IA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6"]
    #[inline(always)]
    pub fn app_cache_ia(&self) -> APP_CACHE_IA_R {
        APP_CACHE_IA_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bits 7:18"]
    #[inline(always)]
    pub fn app_cache_state(&self) -> APP_CACHE_STATE_R {
        APP_CACHE_STATE_R::new(((self.bits >> 7) & 0x0fff) as u16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app_wr_bak_to_read(&self) -> APP_WR_BAK_TO_READ_R {
        APP_WR_BAK_TO_READ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app_tx_end(&self) -> APP_TX_END_R {
        APP_TX_END_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn app_slave_wr(&self) -> APP_SLAVE_WR_R {
        APP_SLAVE_WR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn app_slave_wdata_v(&self) -> APP_SLAVE_WDATA_V_R {
        APP_SLAVE_WDATA_V_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn app_rx_end(&self) -> APP_RX_END_R {
        APP_RX_END_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DCACHE_DBUG0")
            .field(
                "app_slave_wdata",
                &format_args!("{}", self.app_slave_wdata().bit()),
            )
            .field(
                "app_cache_mmu_ia",
                &format_args!("{}", self.app_cache_mmu_ia().bit()),
            )
            .field(
                "app_cache_ia",
                &format_args!("{}", self.app_cache_ia().bits()),
            )
            .field(
                "app_cache_state",
                &format_args!("{}", self.app_cache_state().bits()),
            )
            .field(
                "app_wr_bak_to_read",
                &format_args!("{}", self.app_wr_bak_to_read().bit()),
            )
            .field("app_tx_end", &format_args!("{}", self.app_tx_end().bit()))
            .field(
                "app_slave_wr",
                &format_args!("{}", self.app_slave_wr().bit()),
            )
            .field(
                "app_slave_wdata_v",
                &format_args!("{}", self.app_slave_wdata_v().bit()),
            )
            .field("app_rx_end", &format_args!("{}", self.app_rx_end().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_DCACHE_DBUG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn app_slave_wdata(&mut self) -> APP_SLAVE_WDATA_W<0> {
        APP_SLAVE_WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_dcache_dbug0](index.html) module"]
pub struct APP_DCACHE_DBUG0_SPEC;
impl crate::RegisterSpec for APP_DCACHE_DBUG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_dcache_dbug0::R](R) reader structure"]
impl crate::Readable for APP_DCACHE_DBUG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_dcache_dbug0::W](W) writer structure"]
impl crate::Writable for APP_DCACHE_DBUG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APP_DCACHE_DBUG0 to value 0"]
impl crate::Resettable for APP_DCACHE_DBUG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
