#[doc = "Register `INT_ATTR%s` reader"]
pub type R = crate::R<INT_ATTR_SPEC>;
#[doc = "Register `INT_ATTR%s` writer"]
pub type W = crate::W<INT_ATTR_SPEC>;
#[doc = "Configures hardware vectoring for the ith CLIC machine mode interrupt.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHV {
    #[doc = "0: Not hardware vectored. Upon taking this interrupt, the CPU will jump to the address configured in mtvec."]
    Software = 0,
    #[doc = "1: Hardware vectored. Upon taking this interrupt, the CPU will jump to the word address stored at (mtvt + 4*i) relative to the base address configured in mtvt."]
    Hardware = 1,
}
impl From<SHV> for bool {
    #[inline(always)]
    fn from(variant: SHV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHV` reader - Configures hardware vectoring for the ith CLIC machine mode interrupt."]
pub type SHV_R = crate::BitReader<SHV>;
impl SHV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SHV {
        match self.bits {
            false => SHV::Software,
            true => SHV::Hardware,
        }
    }
    #[doc = "Not hardware vectored. Upon taking this interrupt, the CPU will jump to the address configured in mtvec."]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == SHV::Software
    }
    #[doc = "Hardware vectored. Upon taking this interrupt, the CPU will jump to the word address stored at (mtvt + 4*i) relative to the base address configured in mtvt."]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == SHV::Hardware
    }
}
#[doc = "Field `SHV` writer - Configures hardware vectoring for the ith CLIC machine mode interrupt."]
pub type SHV_W<'a, REG> = crate::BitWriter<'a, REG, SHV>;
impl<'a, REG> SHV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not hardware vectored. Upon taking this interrupt, the CPU will jump to the address configured in mtvec."]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(SHV::Software)
    }
    #[doc = "Hardware vectored. Upon taking this interrupt, the CPU will jump to the word address stored at (mtvt + 4*i) relative to the base address configured in mtvt."]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut crate::W<REG> {
        self.variant(SHV::Hardware)
    }
}
#[doc = "Configures the trigger type and polarity of the ith CLIC machine mode interrupt.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIG {
    #[doc = "0: interrupt is positive level-triggered"]
    PositiveLevel = 0,
    #[doc = "1: interrupt is positive edge-triggered"]
    PositiveEdge = 1,
    #[doc = "2: interrupt is negative level-triggered"]
    NegativeLevel = 2,
    #[doc = "3: interrupt is negative edge-triggered"]
    NegativeEdge = 3,
}
impl From<TRIG> for u8 {
    #[inline(always)]
    fn from(variant: TRIG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIG {
    type Ux = u8;
}
impl crate::IsEnum for TRIG {}
#[doc = "Field `TRIG` reader - Configures the trigger type and polarity of the ith CLIC machine mode interrupt."]
pub type TRIG_R = crate::FieldReader<TRIG>;
impl TRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRIG {
        match self.bits {
            0 => TRIG::PositiveLevel,
            1 => TRIG::PositiveEdge,
            2 => TRIG::NegativeLevel,
            3 => TRIG::NegativeEdge,
            _ => unreachable!(),
        }
    }
    #[doc = "interrupt is positive level-triggered"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        *self == TRIG::PositiveLevel
    }
    #[doc = "interrupt is positive edge-triggered"]
    #[inline(always)]
    pub fn is_positive_edge(&self) -> bool {
        *self == TRIG::PositiveEdge
    }
    #[doc = "interrupt is negative level-triggered"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        *self == TRIG::NegativeLevel
    }
    #[doc = "interrupt is negative edge-triggered"]
    #[inline(always)]
    pub fn is_negative_edge(&self) -> bool {
        *self == TRIG::NegativeEdge
    }
}
#[doc = "Field `TRIG` writer - Configures the trigger type and polarity of the ith CLIC machine mode interrupt."]
pub type TRIG_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRIG, crate::Safe>;
impl<'a, REG> TRIG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "interrupt is positive level-triggered"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut crate::W<REG> {
        self.variant(TRIG::PositiveLevel)
    }
    #[doc = "interrupt is positive edge-triggered"]
    #[inline(always)]
    pub fn positive_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TRIG::PositiveEdge)
    }
    #[doc = "interrupt is negative level-triggered"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut crate::W<REG> {
        self.variant(TRIG::NegativeLevel)
    }
    #[doc = "interrupt is negative edge-triggered"]
    #[inline(always)]
    pub fn negative_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TRIG::NegativeEdge)
    }
}
#[doc = "Field `MODE` reader - Configures the privilege mode of the ith CLIC interrupt. This is hardwired to 0x3 as user mode interrupts are not supported."]
pub type MODE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Configures hardware vectoring for the ith CLIC machine mode interrupt."]
    #[inline(always)]
    pub fn shv(&self) -> SHV_R {
        SHV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Configures the trigger type and polarity of the ith CLIC machine mode interrupt."]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 6:7 - Configures the privilege mode of the ith CLIC interrupt. This is hardwired to 0x3 as user mode interrupts are not supported."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits >> 6) & 3)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ATTR")
            .field("shv", &self.shv())
            .field("trig", &self.trig())
            .field("mode", &self.mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures hardware vectoring for the ith CLIC machine mode interrupt."]
    #[inline(always)]
    pub fn shv(&mut self) -> SHV_W<'_, INT_ATTR_SPEC> {
        SHV_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Configures the trigger type and polarity of the ith CLIC machine mode interrupt."]
    #[inline(always)]
    pub fn trig(&mut self) -> TRIG_W<'_, INT_ATTR_SPEC> {
        TRIG_W::new(self, 1)
    }
}
#[doc = "Interrupt attribute register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ATTR_SPEC;
impl crate::RegisterSpec for INT_ATTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`int_attr::R`](R) reader structure"]
impl crate::Readable for INT_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_attr::W`](W) writer structure"]
impl crate::Writable for INT_ATTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ATTR%s to value 0"]
impl crate::Resettable for INT_ATTR_SPEC {}
