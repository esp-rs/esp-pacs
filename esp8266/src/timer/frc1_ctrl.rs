#[doc = "Register `FRC1_CTRL` reader"]
pub type R = crate::R<FRC1_CTRL_SPEC>;
#[doc = "Register `FRC1_CTRL` writer"]
pub type W = crate::W<FRC1_CTRL_SPEC>;
#[doc = "Field `frc1_ctrl` reader - bit\\[7\\]: timer enable, bit\\[6\\]: automatically reload, when the counter isequal to zero, bit\\[3:2\\]: prescale-divider, 0: divided by 1, 1: dividedby 16, 2 or 3: divided by 256, bit\\[0\\]: interrupt type, 0:edge, 1:level"]
pub type FRC1_CTRL_R = crate::FieldReader;
#[doc = "Field `frc1_ctrl` writer - bit\\[7\\]: timer enable, bit\\[6\\]: automatically reload, when the counter isequal to zero, bit\\[3:2\\]: prescale-divider, 0: divided by 1, 1: dividedby 16, 2 or 3: divided by 256, bit\\[0\\]: interrupt type, 0:edge, 1:level"]
pub type FRC1_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `interrupt_type` reader - Configure the interrupt type"]
pub type INTERRUPT_TYPE_R = crate::BitReader<INTERRUPT_TYPE_A>;
#[doc = "Configure the interrupt type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTERRUPT_TYPE_A {
    #[doc = "0: edge"]
    EDGE = 0,
    #[doc = "1: level"]
    LEVEL = 1,
}
impl From<INTERRUPT_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPT_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl INTERRUPT_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INTERRUPT_TYPE_A {
        match self.bits {
            false => INTERRUPT_TYPE_A::EDGE,
            true => INTERRUPT_TYPE_A::LEVEL,
        }
    }
    #[doc = "edge"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == INTERRUPT_TYPE_A::EDGE
    }
    #[doc = "level"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == INTERRUPT_TYPE_A::LEVEL
    }
}
#[doc = "Field `interrupt_type` writer - Configure the interrupt type"]
pub type INTERRUPT_TYPE_W<'a, REG> = crate::BitWriter<'a, REG, INTERRUPT_TYPE_A>;
impl<'a, REG> INTERRUPT_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "edge"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_TYPE_A::EDGE)
    }
    #[doc = "level"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(INTERRUPT_TYPE_A::LEVEL)
    }
}
#[doc = "Field `prescale_divider` reader - Pre-scale divider for the timer"]
pub type PRESCALE_DIVIDER_R = crate::FieldReader<PRESCALE_DIVIDER_A>;
#[doc = "Pre-scale divider for the timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALE_DIVIDER_A {
    #[doc = "0: divided by 1"]
    DEVIDED_BY_1 = 0,
    #[doc = "1: divided by 16"]
    DEVIDED_BY_16 = 1,
    #[doc = "2: divided by 256"]
    DEVIDED_BY_256 = 2,
}
impl From<PRESCALE_DIVIDER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_DIVIDER_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESCALE_DIVIDER_A {
    type Ux = u8;
}
impl PRESCALE_DIVIDER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESCALE_DIVIDER_A> {
        match self.bits {
            0 => Some(PRESCALE_DIVIDER_A::DEVIDED_BY_1),
            1 => Some(PRESCALE_DIVIDER_A::DEVIDED_BY_16),
            2 => Some(PRESCALE_DIVIDER_A::DEVIDED_BY_256),
            _ => None,
        }
    }
    #[doc = "divided by 1"]
    #[inline(always)]
    pub fn is_devided_by_1(&self) -> bool {
        *self == PRESCALE_DIVIDER_A::DEVIDED_BY_1
    }
    #[doc = "divided by 16"]
    #[inline(always)]
    pub fn is_devided_by_16(&self) -> bool {
        *self == PRESCALE_DIVIDER_A::DEVIDED_BY_16
    }
    #[doc = "divided by 256"]
    #[inline(always)]
    pub fn is_devided_by_256(&self) -> bool {
        *self == PRESCALE_DIVIDER_A::DEVIDED_BY_256
    }
}
#[doc = "Field `prescale_divider` writer - Pre-scale divider for the timer"]
pub type PRESCALE_DIVIDER_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PRESCALE_DIVIDER_A>;
impl<'a, REG> PRESCALE_DIVIDER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "divided by 1"]
    #[inline(always)]
    pub fn devided_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALE_DIVIDER_A::DEVIDED_BY_1)
    }
    #[doc = "divided by 16"]
    #[inline(always)]
    pub fn devided_by_16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALE_DIVIDER_A::DEVIDED_BY_16)
    }
    #[doc = "divided by 256"]
    #[inline(always)]
    pub fn devided_by_256(self) -> &'a mut crate::W<REG> {
        self.variant(PRESCALE_DIVIDER_A::DEVIDED_BY_256)
    }
}
#[doc = "Field `rollover` reader - Automatically reload when the counter hits zero"]
pub type ROLLOVER_R = crate::BitReader;
#[doc = "Field `rollover` writer - Automatically reload when the counter hits zero"]
pub type ROLLOVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `timer_enable` reader - Enable or disable the timer"]
pub type TIMER_ENABLE_R = crate::BitReader;
#[doc = "Field `timer_enable` writer - Enable or disable the timer"]
pub type TIMER_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frc1_int` reader - the status of the interrupt, when the count isdereased to zero"]
pub type FRC1_INT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - bit\\[7\\]: timer enable, bit\\[6\\]: automatically reload, when the counter isequal to zero, bit\\[3:2\\]: prescale-divider, 0: divided by 1, 1: dividedby 16, 2 or 3: divided by 256, bit\\[0\\]: interrupt type, 0:edge, 1:level"]
    #[inline(always)]
    pub fn frc1_ctrl(&self) -> FRC1_CTRL_R {
        FRC1_CTRL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 0 - Configure the interrupt type"]
    #[inline(always)]
    pub fn interrupt_type(&self) -> INTERRUPT_TYPE_R {
        INTERRUPT_TYPE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Pre-scale divider for the timer"]
    #[inline(always)]
    pub fn prescale_divider(&self) -> PRESCALE_DIVIDER_R {
        PRESCALE_DIVIDER_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - Automatically reload when the counter hits zero"]
    #[inline(always)]
    pub fn rollover(&self) -> ROLLOVER_R {
        ROLLOVER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable the timer"]
    #[inline(always)]
    pub fn timer_enable(&self) -> TIMER_ENABLE_R {
        TIMER_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - the status of the interrupt, when the count isdereased to zero"]
    #[inline(always)]
    pub fn frc1_int(&self) -> FRC1_INT_R {
        FRC1_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRC1_CTRL")
            .field("frc1_int", &format_args!("{}", self.frc1_int().bit()))
            .field("frc1_ctrl", &format_args!("{}", self.frc1_ctrl().bits()))
            .field(
                "timer_enable",
                &format_args!("{}", self.timer_enable().bit()),
            )
            .field("rollover", &format_args!("{}", self.rollover().bit()))
            .field(
                "prescale_divider",
                &format_args!("{}", self.prescale_divider().bits()),
            )
            .field(
                "interrupt_type",
                &format_args!("{}", self.interrupt_type().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FRC1_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - bit\\[7\\]: timer enable, bit\\[6\\]: automatically reload, when the counter isequal to zero, bit\\[3:2\\]: prescale-divider, 0: divided by 1, 1: dividedby 16, 2 or 3: divided by 256, bit\\[0\\]: interrupt type, 0:edge, 1:level"]
    #[inline(always)]
    #[must_use]
    pub fn frc1_ctrl(&mut self) -> FRC1_CTRL_W<FRC1_CTRL_SPEC> {
        FRC1_CTRL_W::new(self, 0)
    }
    #[doc = "Bit 0 - Configure the interrupt type"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_type(&mut self) -> INTERRUPT_TYPE_W<FRC1_CTRL_SPEC> {
        INTERRUPT_TYPE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Pre-scale divider for the timer"]
    #[inline(always)]
    #[must_use]
    pub fn prescale_divider(&mut self) -> PRESCALE_DIVIDER_W<FRC1_CTRL_SPEC> {
        PRESCALE_DIVIDER_W::new(self, 2)
    }
    #[doc = "Bit 6 - Automatically reload when the counter hits zero"]
    #[inline(always)]
    #[must_use]
    pub fn rollover(&mut self) -> ROLLOVER_W<FRC1_CTRL_SPEC> {
        ROLLOVER_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable the timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer_enable(&mut self) -> TIMER_ENABLE_W<FRC1_CTRL_SPEC> {
        TIMER_ENABLE_W::new(self, 7)
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
#[doc = "FRC1_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc1_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc1_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRC1_CTRL_SPEC;
impl crate::RegisterSpec for FRC1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frc1_ctrl::R`](R) reader structure"]
impl crate::Readable for FRC1_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frc1_ctrl::W`](W) writer structure"]
impl crate::Writable for FRC1_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRC1_CTRL to value 0"]
impl crate::Resettable for FRC1_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
