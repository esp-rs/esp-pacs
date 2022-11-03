#[doc = "Register `L1_ICACHE_CTRL` reader"]
pub struct R(crate::R<L1_ICACHE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_ICACHE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_ICACHE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_ICACHE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L1_ICACHE_SHUT_IBUS0` reader - The bit is used to disable core0 ibus access L1-ICache, 0: enable, 1: disable"]
pub type L1_ICACHE_SHUT_IBUS0_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE_SHUT_IBUS1` reader - The bit is used to disable core1 ibus access L1-ICache, 0: enable, 1: disable"]
pub type L1_ICACHE_SHUT_IBUS1_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE_SHUT_IBUS2` reader - Reserved"]
pub type L1_ICACHE_SHUT_IBUS2_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE_SHUT_IBUS3` reader - Reserved"]
pub type L1_ICACHE_SHUT_IBUS3_R = crate::BitReader<bool>;
#[doc = "Field `L1_ICACHE_UNDEF_OP` reader - Reserved"]
pub type L1_ICACHE_UNDEF_OP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - The bit is used to disable core0 ibus access L1-ICache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_icache_shut_ibus0(&self) -> L1_ICACHE_SHUT_IBUS0_R {
        L1_ICACHE_SHUT_IBUS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to disable core1 ibus access L1-ICache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l1_icache_shut_ibus1(&self) -> L1_ICACHE_SHUT_IBUS1_R {
        L1_ICACHE_SHUT_IBUS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache_shut_ibus2(&self) -> L1_ICACHE_SHUT_IBUS2_R {
        L1_ICACHE_SHUT_IBUS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache_shut_ibus3(&self) -> L1_ICACHE_SHUT_IBUS3_R {
        L1_ICACHE_SHUT_IBUS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Reserved"]
    #[inline(always)]
    pub fn l1_icache_undef_op(&self) -> L1_ICACHE_UNDEF_OP_R {
        L1_ICACHE_UNDEF_OP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "L1 instruction Cache(L1-ICache) control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_icache_ctrl](index.html) module"]
pub struct L1_ICACHE_CTRL_SPEC;
impl crate::RegisterSpec for L1_ICACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_icache_ctrl::R](R) reader structure"]
impl crate::Readable for L1_ICACHE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_ICACHE_CTRL to value 0"]
impl crate::Resettable for L1_ICACHE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
