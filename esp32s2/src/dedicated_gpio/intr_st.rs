#[doc = "Register `INTR_ST` reader"]
pub struct R(crate::R<INTR_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO0_INT_ST` reader - This is the status bit for DEDIC_GPIO0_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
pub type GPIO0_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `GPIO1_INT_ST` reader - This is the status bit for DEDIC_GPIO1_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
pub type GPIO1_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `GPIO2_INT_ST` reader - This is the status bit for DEDIC_GPIO2_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
pub type GPIO2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `GPIO3_INT_ST` reader - This is the status bit for DEDIC_GPIO3_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
pub type GPIO3_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `GPIO4_INT_ST` reader - This is the status bit for DEDIC_GPIO4_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
pub type GPIO4_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `GPIO5_INT_ST` reader - This is the status bit for DEDIC_GPIO5_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
pub type GPIO5_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `GPIO6_INT_ST` reader - This is the status bit for DEDIC_GPIO6_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
pub type GPIO6_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `GPIO7_INT_ST` reader - This is the status bit for DEDIC_GPIO7_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
pub type GPIO7_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This is the status bit for DEDIC_GPIO0_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio0_int_st(&self) -> GPIO0_INT_ST_R {
        GPIO0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the status bit for DEDIC_GPIO1_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio1_int_st(&self) -> GPIO1_INT_ST_R {
        GPIO1_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the status bit for DEDIC_GPIO2_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio2_int_st(&self) -> GPIO2_INT_ST_R {
        GPIO2_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the status bit for DEDIC_GPIO3_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio3_int_st(&self) -> GPIO3_INT_ST_R {
        GPIO3_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the status bit for DEDIC_GPIO4_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio4_int_st(&self) -> GPIO4_INT_ST_R {
        GPIO4_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the status bit for DEDIC_GPIO5_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio5_int_st(&self) -> GPIO5_INT_ST_R {
        GPIO5_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the status bit for DEDIC_GPIO6_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio6_int_st(&self) -> GPIO6_INT_ST_R {
        GPIO6_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the status bit for DEDIC_GPIO7_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio7_int_st(&self) -> GPIO7_INT_ST_R {
        GPIO7_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_st](index.html) module"]
pub struct INTR_ST_SPEC;
impl crate::RegisterSpec for INTR_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_st::R](R) reader structure"]
impl crate::Readable for INTR_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_ST to value 0"]
impl crate::Resettable for INTR_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
