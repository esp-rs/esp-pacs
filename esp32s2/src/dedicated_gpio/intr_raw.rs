#[doc = "Register `INTR_RAW` reader"]
pub struct R(crate::R<INTR_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO0` reader - This interrupt raw bit turns to high level when dedicated GPIO0 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub type GPIO0_R = crate::BitReader;
#[doc = "Field `GPIO1` reader - This interrupt raw bit turns to high level when dedicated GPIO1 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub type GPIO1_R = crate::BitReader;
#[doc = "Field `GPIO2` reader - This interrupt raw bit turns to high level when dedicated GPIO2 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub type GPIO2_R = crate::BitReader;
#[doc = "Field `GPIO3` reader - This interrupt raw bit turns to high level when dedicated GPIO3 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub type GPIO3_R = crate::BitReader;
#[doc = "Field `GPIO4` reader - This interrupt raw bit turns to high level when dedicated GPIO4 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub type GPIO4_R = crate::BitReader;
#[doc = "Field `GPIO5` reader - This interrupt raw bit turns to high level when dedicated GPIO5 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub type GPIO5_R = crate::BitReader;
#[doc = "Field `GPIO6` reader - This interrupt raw bit turns to high level when dedicated GPIO6 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub type GPIO6_R = crate::BitReader;
#[doc = "Field `GPIO7` reader - This interrupt raw bit turns to high level when dedicated GPIO7 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub type GPIO7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when dedicated GPIO0 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when dedicated GPIO1 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This interrupt raw bit turns to high level when dedicated GPIO2 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This interrupt raw bit turns to high level when dedicated GPIO3 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This interrupt raw bit turns to high level when dedicated GPIO4 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO4_R {
        GPIO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This interrupt raw bit turns to high level when dedicated GPIO5 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO5_R {
        GPIO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This interrupt raw bit turns to high level when dedicated GPIO6 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio6(&self) -> GPIO6_R {
        GPIO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This interrupt raw bit turns to high level when dedicated GPIO7 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio7(&self) -> GPIO7_R {
        GPIO7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_RAW")
            .field("gpio0", &format_args!("{}", self.gpio0().bit()))
            .field("gpio1", &format_args!("{}", self.gpio1().bit()))
            .field("gpio2", &format_args!("{}", self.gpio2().bit()))
            .field("gpio3", &format_args!("{}", self.gpio3().bit()))
            .field("gpio4", &format_args!("{}", self.gpio4().bit()))
            .field("gpio5", &format_args!("{}", self.gpio5().bit()))
            .field("gpio6", &format_args!("{}", self.gpio6().bit()))
            .field("gpio7", &format_args!("{}", self.gpio7().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_raw](index.html) module"]
pub struct INTR_RAW_SPEC;
impl crate::RegisterSpec for INTR_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_raw::R](R) reader structure"]
impl crate::Readable for INTR_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_RAW to value 0"]
impl crate::Resettable for INTR_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
