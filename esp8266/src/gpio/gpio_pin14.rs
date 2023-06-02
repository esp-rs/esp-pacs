#[doc = "Register `GPIO_PIN14` reader"]
pub struct R(crate::R<GPIO_PIN14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_PIN14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_PIN14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_PIN14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_PIN14` writer"]
pub struct W(crate::W<GPIO_PIN14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_PIN14_SPEC>;
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
impl From<crate::W<GPIO_PIN14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_PIN14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_PIN14_SOURCE` reader - 1: sigma-delta; 0: GPIO_DATA"]
pub type GPIO_PIN14_SOURCE_R = crate::BitReader<GPIO_PIN14_SOURCE_A>;
#[doc = "1: sigma-delta; 0: GPIO_DATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_PIN14_SOURCE_A {
    #[doc = "0: sigma-delta"]
    SIGMA_DELTA = 0,
    #[doc = "1: gpio data"]
    GPIO_DATA = 1,
}
impl From<GPIO_PIN14_SOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_PIN14_SOURCE_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_PIN14_SOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_PIN14_SOURCE_A {
        match self.bits {
            false => GPIO_PIN14_SOURCE_A::SIGMA_DELTA,
            true => GPIO_PIN14_SOURCE_A::GPIO_DATA,
        }
    }
    #[doc = "Checks if the value of the field is `SIGMA_DELTA`"]
    #[inline(always)]
    pub fn is_sigma_delta(&self) -> bool {
        *self == GPIO_PIN14_SOURCE_A::SIGMA_DELTA
    }
    #[doc = "Checks if the value of the field is `GPIO_DATA`"]
    #[inline(always)]
    pub fn is_gpio_data(&self) -> bool {
        *self == GPIO_PIN14_SOURCE_A::GPIO_DATA
    }
}
#[doc = "Field `GPIO_PIN14_SOURCE` writer - 1: sigma-delta; 0: GPIO_DATA"]
pub type GPIO_PIN14_SOURCE_W<'a, const O: u8> =
    crate::BitWriter<'a, GPIO_PIN14_SPEC, O, GPIO_PIN14_SOURCE_A>;
impl<'a, const O: u8> GPIO_PIN14_SOURCE_W<'a, O> {
    #[doc = "sigma-delta"]
    #[inline(always)]
    pub fn sigma_delta(self) -> &'a mut W {
        self.variant(GPIO_PIN14_SOURCE_A::SIGMA_DELTA)
    }
    #[doc = "gpio data"]
    #[inline(always)]
    pub fn gpio_data(self) -> &'a mut W {
        self.variant(GPIO_PIN14_SOURCE_A::GPIO_DATA)
    }
}
#[doc = "Field `GPIO_PIN14_DRIVER` reader - 1: open drain; 0: normal"]
pub type GPIO_PIN14_DRIVER_R = crate::BitReader<GPIO_PIN14_DRIVER_A>;
#[doc = "1: open drain; 0: normal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIO_PIN14_DRIVER_A {
    #[doc = "0: open drain"]
    OPEN_DRAIN = 0,
    #[doc = "1: normal"]
    NORMAL = 1,
}
impl From<GPIO_PIN14_DRIVER_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_PIN14_DRIVER_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIO_PIN14_DRIVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_PIN14_DRIVER_A {
        match self.bits {
            false => GPIO_PIN14_DRIVER_A::OPEN_DRAIN,
            true => GPIO_PIN14_DRIVER_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == GPIO_PIN14_DRIVER_A::OPEN_DRAIN
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == GPIO_PIN14_DRIVER_A::NORMAL
    }
}
#[doc = "Field `GPIO_PIN14_DRIVER` writer - 1: open drain; 0: normal"]
pub type GPIO_PIN14_DRIVER_W<'a, const O: u8> =
    crate::BitWriter<'a, GPIO_PIN14_SPEC, O, GPIO_PIN14_DRIVER_A>;
impl<'a, const O: u8> GPIO_PIN14_DRIVER_W<'a, O> {
    #[doc = "open drain"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(GPIO_PIN14_DRIVER_A::OPEN_DRAIN)
    }
    #[doc = "normal"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(GPIO_PIN14_DRIVER_A::NORMAL)
    }
}
#[doc = "Field `GPIO_PIN14_INT_TYPE` reader - 0: disable; 1: positive edge; 2: negative edge; 3: both types of edge; 4: low-level; 5: high-level"]
pub type GPIO_PIN14_INT_TYPE_R = crate::FieldReader<u8, GPIO_PIN14_INT_TYPE_A>;
#[doc = "0: disable; 1: positive edge; 2: negative edge; 3: both types of edge; 4: low-level; 5: high-level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPIO_PIN14_INT_TYPE_A {
    #[doc = "0: interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: interrupt is triggered on the positive edge"]
    POSITIVE_EDGE = 1,
    #[doc = "2: interrupt is triggered on the negative edge"]
    NEGATIVE_EDGE = 2,
    #[doc = "3: interrupt is triggered on both edges"]
    BOTH_EDGES = 3,
    #[doc = "4: interrupt is triggered on the low level"]
    LOW_LEVEL = 4,
    #[doc = "5: interrupt is triggered on the high level"]
    HIGH_LEVEL = 5,
}
impl From<GPIO_PIN14_INT_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: GPIO_PIN14_INT_TYPE_A) -> Self {
        variant as _
    }
}
impl GPIO_PIN14_INT_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GPIO_PIN14_INT_TYPE_A> {
        match self.bits {
            0 => Some(GPIO_PIN14_INT_TYPE_A::DISABLED),
            1 => Some(GPIO_PIN14_INT_TYPE_A::POSITIVE_EDGE),
            2 => Some(GPIO_PIN14_INT_TYPE_A::NEGATIVE_EDGE),
            3 => Some(GPIO_PIN14_INT_TYPE_A::BOTH_EDGES),
            4 => Some(GPIO_PIN14_INT_TYPE_A::LOW_LEVEL),
            5 => Some(GPIO_PIN14_INT_TYPE_A::HIGH_LEVEL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIO_PIN14_INT_TYPE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `POSITIVE_EDGE`"]
    #[inline(always)]
    pub fn is_positive_edge(&self) -> bool {
        *self == GPIO_PIN14_INT_TYPE_A::POSITIVE_EDGE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_EDGE`"]
    #[inline(always)]
    pub fn is_negative_edge(&self) -> bool {
        *self == GPIO_PIN14_INT_TYPE_A::NEGATIVE_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == GPIO_PIN14_INT_TYPE_A::BOTH_EDGES
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == GPIO_PIN14_INT_TYPE_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == GPIO_PIN14_INT_TYPE_A::HIGH_LEVEL
    }
}
#[doc = "Field `GPIO_PIN14_INT_TYPE` writer - 0: disable; 1: positive edge; 2: negative edge; 3: both types of edge; 4: low-level; 5: high-level"]
pub type GPIO_PIN14_INT_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, GPIO_PIN14_SPEC, 3, O, u8, GPIO_PIN14_INT_TYPE_A>;
impl<'a, const O: u8> GPIO_PIN14_INT_TYPE_W<'a, O> {
    #[doc = "interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO_PIN14_INT_TYPE_A::DISABLED)
    }
    #[doc = "interrupt is triggered on the positive edge"]
    #[inline(always)]
    pub fn positive_edge(self) -> &'a mut W {
        self.variant(GPIO_PIN14_INT_TYPE_A::POSITIVE_EDGE)
    }
    #[doc = "interrupt is triggered on the negative edge"]
    #[inline(always)]
    pub fn negative_edge(self) -> &'a mut W {
        self.variant(GPIO_PIN14_INT_TYPE_A::NEGATIVE_EDGE)
    }
    #[doc = "interrupt is triggered on both edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(GPIO_PIN14_INT_TYPE_A::BOTH_EDGES)
    }
    #[doc = "interrupt is triggered on the low level"]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(GPIO_PIN14_INT_TYPE_A::LOW_LEVEL)
    }
    #[doc = "interrupt is triggered on the high level"]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(GPIO_PIN14_INT_TYPE_A::HIGH_LEVEL)
    }
}
#[doc = "Field `GPIO_PIN14_WAKEUP_ENABLE` reader - 0: disable; 1: enable GPIO wakeup CPU, only when GPIO_PIN0_INT_TYPE is 0x4 or 0x5"]
pub type GPIO_PIN14_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN14_WAKEUP_ENABLE` writer - 0: disable; 1: enable GPIO wakeup CPU, only when GPIO_PIN0_INT_TYPE is 0x4 or 0x5"]
pub type GPIO_PIN14_WAKEUP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_PIN14_SPEC, O>;
impl R {
    #[doc = "Bit 0 - 1: sigma-delta; 0: GPIO_DATA"]
    #[inline(always)]
    pub fn gpio_pin14_source(&self) -> GPIO_PIN14_SOURCE_R {
        GPIO_PIN14_SOURCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 1: open drain; 0: normal"]
    #[inline(always)]
    pub fn gpio_pin14_driver(&self) -> GPIO_PIN14_DRIVER_R {
        GPIO_PIN14_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9 - 0: disable; 1: positive edge; 2: negative edge; 3: both types of edge; 4: low-level; 5: high-level"]
    #[inline(always)]
    pub fn gpio_pin14_int_type(&self) -> GPIO_PIN14_INT_TYPE_R {
        GPIO_PIN14_INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - 0: disable; 1: enable GPIO wakeup CPU, only when GPIO_PIN0_INT_TYPE is 0x4 or 0x5"]
    #[inline(always)]
    pub fn gpio_pin14_wakeup_enable(&self) -> GPIO_PIN14_WAKEUP_ENABLE_R {
        GPIO_PIN14_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_PIN14")
            .field(
                "gpio_pin14_wakeup_enable",
                &format_args!("{}", self.gpio_pin14_wakeup_enable().bit()),
            )
            .field(
                "gpio_pin14_int_type",
                &format_args!("{}", self.gpio_pin14_int_type().bits()),
            )
            .field(
                "gpio_pin14_driver",
                &format_args!("{}", self.gpio_pin14_driver().bit()),
            )
            .field(
                "gpio_pin14_source",
                &format_args!("{}", self.gpio_pin14_source().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_PIN14_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 1: sigma-delta; 0: GPIO_DATA"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin14_source(&mut self) -> GPIO_PIN14_SOURCE_W<0> {
        GPIO_PIN14_SOURCE_W::new(self)
    }
    #[doc = "Bit 2 - 1: open drain; 0: normal"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin14_driver(&mut self) -> GPIO_PIN14_DRIVER_W<2> {
        GPIO_PIN14_DRIVER_W::new(self)
    }
    #[doc = "Bits 7:9 - 0: disable; 1: positive edge; 2: negative edge; 3: both types of edge; 4: low-level; 5: high-level"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin14_int_type(&mut self) -> GPIO_PIN14_INT_TYPE_W<7> {
        GPIO_PIN14_INT_TYPE_W::new(self)
    }
    #[doc = "Bit 10 - 0: disable; 1: enable GPIO wakeup CPU, only when GPIO_PIN0_INT_TYPE is 0x4 or 0x5"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin14_wakeup_enable(&mut self) -> GPIO_PIN14_WAKEUP_ENABLE_W<10> {
        GPIO_PIN14_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_PIN14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_pin14](index.html) module"]
pub struct GPIO_PIN14_SPEC;
impl crate::RegisterSpec for GPIO_PIN14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_pin14::R](R) reader structure"]
impl crate::Readable for GPIO_PIN14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_pin14::W](W) writer structure"]
impl crate::Writable for GPIO_PIN14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_PIN14 to value 0"]
impl crate::Resettable for GPIO_PIN14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
