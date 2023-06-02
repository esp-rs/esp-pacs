#[doc = "Register `L1_CACHE_DATA_MEM_ACS_CONF` reader"]
pub struct R(crate::R<L1_CACHE_DATA_MEM_ACS_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_CACHE_DATA_MEM_ACS_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_CACHE_DATA_MEM_ACS_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_CACHE_DATA_MEM_ACS_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1_CACHE_DATA_MEM_ACS_CONF` writer"]
pub struct W(crate::W<L1_CACHE_DATA_MEM_ACS_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1_CACHE_DATA_MEM_ACS_CONF_SPEC>;
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
impl From<crate::W<L1_CACHE_DATA_MEM_ACS_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1_CACHE_DATA_MEM_ACS_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L1_ICACHE0_DATA_MEM_RD_EN` reader - The bit is used to enable config-bus read L1-ICache0 data memoryory. 0: disable, 1: enable."]
pub type L1_ICACHE0_DATA_MEM_RD_EN_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_DATA_MEM_WR_EN` reader - The bit is used to enable config-bus write L1-ICache0 data memoryory. 0: disable, 1: enable."]
pub type L1_ICACHE0_DATA_MEM_WR_EN_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_DATA_MEM_RD_EN` reader - The bit is used to enable config-bus read L1-ICache1 data memoryory. 0: disable, 1: enable."]
pub type L1_ICACHE1_DATA_MEM_RD_EN_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_DATA_MEM_WR_EN` reader - The bit is used to enable config-bus write L1-ICache1 data memoryory. 0: disable, 1: enable."]
pub type L1_ICACHE1_DATA_MEM_WR_EN_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_DATA_MEM_RD_EN` reader - Reserved"]
pub type L1_ICACHE2_DATA_MEM_RD_EN_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_DATA_MEM_WR_EN` reader - Reserved"]
pub type L1_ICACHE2_DATA_MEM_WR_EN_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_DATA_MEM_RD_EN` reader - Reserved"]
pub type L1_ICACHE3_DATA_MEM_RD_EN_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_DATA_MEM_WR_EN` reader - Reserved"]
pub type L1_ICACHE3_DATA_MEM_WR_EN_R = crate::BitReader;
#[doc = "Field `L1_CACHE_DATA_MEM_RD_EN` reader - The bit is used to enable config-bus read L1-Cache data memoryory. 0: disable, 1: enable."]
pub type L1_CACHE_DATA_MEM_RD_EN_R = crate::BitReader;
#[doc = "Field `L1_CACHE_DATA_MEM_RD_EN` writer - The bit is used to enable config-bus read L1-Cache data memoryory. 0: disable, 1: enable."]
pub type L1_CACHE_DATA_MEM_RD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_DATA_MEM_ACS_CONF_SPEC, O>;
#[doc = "Field `L1_CACHE_DATA_MEM_WR_EN` reader - The bit is used to enable config-bus write L1-Cache data memoryory. 0: disable, 1: enable."]
pub type L1_CACHE_DATA_MEM_WR_EN_R = crate::BitReader;
#[doc = "Field `L1_CACHE_DATA_MEM_WR_EN` writer - The bit is used to enable config-bus write L1-Cache data memoryory. 0: disable, 1: enable."]
pub type L1_CACHE_DATA_MEM_WR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, L1_CACHE_DATA_MEM_ACS_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable config-bus read L1-ICache0 data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn l1_icache0_data_mem_rd_en(&self) -> L1_ICACHE0_DATA_MEM_RD_EN_R {
        L1_ICACHE0_DATA_MEM_RD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable config-bus write L1-ICache0 data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn l1_icache0_data_mem_wr_en(&self) -> L1_ICACHE0_DATA_MEM_WR_EN_R {
        L1_ICACHE0_DATA_MEM_WR_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable config-bus read L1-ICache1 data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn l1_icache1_data_mem_rd_en(&self) -> L1_ICACHE1_DATA_MEM_RD_EN_R {
        L1_ICACHE1_DATA_MEM_RD_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to enable config-bus write L1-ICache1 data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn l1_icache1_data_mem_wr_en(&self) -> L1_ICACHE1_DATA_MEM_WR_EN_R {
        L1_ICACHE1_DATA_MEM_WR_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_data_mem_rd_en(&self) -> L1_ICACHE2_DATA_MEM_RD_EN_R {
        L1_ICACHE2_DATA_MEM_RD_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_data_mem_wr_en(&self) -> L1_ICACHE2_DATA_MEM_WR_EN_R {
        L1_ICACHE2_DATA_MEM_WR_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_data_mem_rd_en(&self) -> L1_ICACHE3_DATA_MEM_RD_EN_R {
        L1_ICACHE3_DATA_MEM_RD_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_data_mem_wr_en(&self) -> L1_ICACHE3_DATA_MEM_WR_EN_R {
        L1_ICACHE3_DATA_MEM_WR_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - The bit is used to enable config-bus read L1-Cache data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn l1_cache_data_mem_rd_en(&self) -> L1_CACHE_DATA_MEM_RD_EN_R {
        L1_CACHE_DATA_MEM_RD_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The bit is used to enable config-bus write L1-Cache data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn l1_cache_data_mem_wr_en(&self) -> L1_CACHE_DATA_MEM_WR_EN_R {
        L1_CACHE_DATA_MEM_WR_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_DATA_MEM_ACS_CONF")
            .field(
                "l1_icache0_data_mem_rd_en",
                &format_args!("{}", self.l1_icache0_data_mem_rd_en().bit()),
            )
            .field(
                "l1_icache0_data_mem_wr_en",
                &format_args!("{}", self.l1_icache0_data_mem_wr_en().bit()),
            )
            .field(
                "l1_icache1_data_mem_rd_en",
                &format_args!("{}", self.l1_icache1_data_mem_rd_en().bit()),
            )
            .field(
                "l1_icache1_data_mem_wr_en",
                &format_args!("{}", self.l1_icache1_data_mem_wr_en().bit()),
            )
            .field(
                "l1_icache2_data_mem_rd_en",
                &format_args!("{}", self.l1_icache2_data_mem_rd_en().bit()),
            )
            .field(
                "l1_icache2_data_mem_wr_en",
                &format_args!("{}", self.l1_icache2_data_mem_wr_en().bit()),
            )
            .field(
                "l1_icache3_data_mem_rd_en",
                &format_args!("{}", self.l1_icache3_data_mem_rd_en().bit()),
            )
            .field(
                "l1_icache3_data_mem_wr_en",
                &format_args!("{}", self.l1_icache3_data_mem_wr_en().bit()),
            )
            .field(
                "l1_cache_data_mem_rd_en",
                &format_args!("{}", self.l1_cache_data_mem_rd_en().bit()),
            )
            .field(
                "l1_cache_data_mem_wr_en",
                &format_args!("{}", self.l1_cache_data_mem_wr_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_CACHE_DATA_MEM_ACS_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 16 - The bit is used to enable config-bus read L1-Cache data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_data_mem_rd_en(&mut self) -> L1_CACHE_DATA_MEM_RD_EN_W<16> {
        L1_CACHE_DATA_MEM_RD_EN_W::new(self)
    }
    #[doc = "Bit 17 - The bit is used to enable config-bus write L1-Cache data memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_data_mem_wr_en(&mut self) -> L1_CACHE_DATA_MEM_WR_EN_W<17> {
        L1_CACHE_DATA_MEM_WR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache data memory access configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_cache_data_mem_acs_conf](index.html) module"]
pub struct L1_CACHE_DATA_MEM_ACS_CONF_SPEC;
impl crate::RegisterSpec for L1_CACHE_DATA_MEM_ACS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_cache_data_mem_acs_conf::R](R) reader structure"]
impl crate::Readable for L1_CACHE_DATA_MEM_ACS_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1_cache_data_mem_acs_conf::W](W) writer structure"]
impl crate::Writable for L1_CACHE_DATA_MEM_ACS_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L1_CACHE_DATA_MEM_ACS_CONF to value 0x0003_3333"]
impl crate::Resettable for L1_CACHE_DATA_MEM_ACS_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_3333;
}
