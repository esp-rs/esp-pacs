#[doc = "Register `WDTCONFIG0` reader"]
pub struct R(crate::R<WDTCONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCONFIG0` writer"]
pub struct W(crate::W<WDTCONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCONFIG0_SPEC>;
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
impl From<crate::W<WDTCONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` reader - When set flash boot protection is enabled"]
pub type WDT_FLASHBOOT_MOD_EN_R = crate::BitReader;
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` writer - When set flash boot protection is enabled"]
pub type WDT_FLASHBOOT_MOD_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_SYS_RESET_LENGTH` reader - length of system reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us"]
pub type WDT_SYS_RESET_LENGTH_R = crate::FieldReader<WDT_SYS_RESET_LENGTH_A>;
#[doc = "length of system reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDT_SYS_RESET_LENGTH_A {
    #[doc = "0: 100ns"]
    NS100 = 0,
    #[doc = "1: 200ns"]
    NS200 = 1,
    #[doc = "2: 300ns"]
    NS300 = 2,
    #[doc = "3: 400ns"]
    NS400 = 3,
    #[doc = "4: 500ns"]
    NS500 = 4,
    #[doc = "5: 800ns"]
    NS800 = 5,
    #[doc = "6: 1.6us"]
    NS1600 = 6,
    #[doc = "7: 3.2us"]
    NS3200 = 7,
}
impl From<WDT_SYS_RESET_LENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WDT_SYS_RESET_LENGTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDT_SYS_RESET_LENGTH_A {
    type Ux = u8;
}
impl WDT_SYS_RESET_LENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_SYS_RESET_LENGTH_A {
        match self.bits {
            0 => WDT_SYS_RESET_LENGTH_A::NS100,
            1 => WDT_SYS_RESET_LENGTH_A::NS200,
            2 => WDT_SYS_RESET_LENGTH_A::NS300,
            3 => WDT_SYS_RESET_LENGTH_A::NS400,
            4 => WDT_SYS_RESET_LENGTH_A::NS500,
            5 => WDT_SYS_RESET_LENGTH_A::NS800,
            6 => WDT_SYS_RESET_LENGTH_A::NS1600,
            7 => WDT_SYS_RESET_LENGTH_A::NS3200,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NS100`"]
    #[inline(always)]
    pub fn is_ns100(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH_A::NS100
    }
    #[doc = "Checks if the value of the field is `NS200`"]
    #[inline(always)]
    pub fn is_ns200(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH_A::NS200
    }
    #[doc = "Checks if the value of the field is `NS300`"]
    #[inline(always)]
    pub fn is_ns300(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH_A::NS300
    }
    #[doc = "Checks if the value of the field is `NS400`"]
    #[inline(always)]
    pub fn is_ns400(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH_A::NS400
    }
    #[doc = "Checks if the value of the field is `NS500`"]
    #[inline(always)]
    pub fn is_ns500(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH_A::NS500
    }
    #[doc = "Checks if the value of the field is `NS800`"]
    #[inline(always)]
    pub fn is_ns800(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH_A::NS800
    }
    #[doc = "Checks if the value of the field is `NS1600`"]
    #[inline(always)]
    pub fn is_ns1600(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH_A::NS1600
    }
    #[doc = "Checks if the value of the field is `NS3200`"]
    #[inline(always)]
    pub fn is_ns3200(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH_A::NS3200
    }
}
#[doc = "Field `WDT_SYS_RESET_LENGTH` writer - length of system reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us"]
pub type WDT_SYS_RESET_LENGTH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, WDTCONFIG0_SPEC, 3, O, WDT_SYS_RESET_LENGTH_A>;
impl<'a, const O: u8> WDT_SYS_RESET_LENGTH_W<'a, O> {
    #[doc = "100ns"]
    #[inline(always)]
    pub fn ns100(self) -> &'a mut W {
        self.variant(WDT_SYS_RESET_LENGTH_A::NS100)
    }
    #[doc = "200ns"]
    #[inline(always)]
    pub fn ns200(self) -> &'a mut W {
        self.variant(WDT_SYS_RESET_LENGTH_A::NS200)
    }
    #[doc = "300ns"]
    #[inline(always)]
    pub fn ns300(self) -> &'a mut W {
        self.variant(WDT_SYS_RESET_LENGTH_A::NS300)
    }
    #[doc = "400ns"]
    #[inline(always)]
    pub fn ns400(self) -> &'a mut W {
        self.variant(WDT_SYS_RESET_LENGTH_A::NS400)
    }
    #[doc = "500ns"]
    #[inline(always)]
    pub fn ns500(self) -> &'a mut W {
        self.variant(WDT_SYS_RESET_LENGTH_A::NS500)
    }
    #[doc = "800ns"]
    #[inline(always)]
    pub fn ns800(self) -> &'a mut W {
        self.variant(WDT_SYS_RESET_LENGTH_A::NS800)
    }
    #[doc = "1.6us"]
    #[inline(always)]
    pub fn ns1600(self) -> &'a mut W {
        self.variant(WDT_SYS_RESET_LENGTH_A::NS1600)
    }
    #[doc = "3.2us"]
    #[inline(always)]
    pub fn ns3200(self) -> &'a mut W {
        self.variant(WDT_SYS_RESET_LENGTH_A::NS3200)
    }
}
#[doc = "Field `WDT_CPU_RESET_LENGTH` reader - length of CPU reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us"]
pub type WDT_CPU_RESET_LENGTH_R = crate::FieldReader<WDT_CPU_RESET_LENGTH_A>;
#[doc = "length of CPU reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDT_CPU_RESET_LENGTH_A {
    #[doc = "0: 100ns"]
    NS100 = 0,
    #[doc = "1: 200ns"]
    NS200 = 1,
    #[doc = "2: 300ns"]
    NS300 = 2,
    #[doc = "3: 400ns"]
    NS400 = 3,
    #[doc = "4: 500ns"]
    NS500 = 4,
    #[doc = "5: 800ns"]
    NS800 = 5,
    #[doc = "6: 1.6us"]
    NS1600 = 6,
    #[doc = "7: 3.2us"]
    NS3200 = 7,
}
impl From<WDT_CPU_RESET_LENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WDT_CPU_RESET_LENGTH_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDT_CPU_RESET_LENGTH_A {
    type Ux = u8;
}
impl WDT_CPU_RESET_LENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_CPU_RESET_LENGTH_A {
        match self.bits {
            0 => WDT_CPU_RESET_LENGTH_A::NS100,
            1 => WDT_CPU_RESET_LENGTH_A::NS200,
            2 => WDT_CPU_RESET_LENGTH_A::NS300,
            3 => WDT_CPU_RESET_LENGTH_A::NS400,
            4 => WDT_CPU_RESET_LENGTH_A::NS500,
            5 => WDT_CPU_RESET_LENGTH_A::NS800,
            6 => WDT_CPU_RESET_LENGTH_A::NS1600,
            7 => WDT_CPU_RESET_LENGTH_A::NS3200,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NS100`"]
    #[inline(always)]
    pub fn is_ns100(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::NS100
    }
    #[doc = "Checks if the value of the field is `NS200`"]
    #[inline(always)]
    pub fn is_ns200(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::NS200
    }
    #[doc = "Checks if the value of the field is `NS300`"]
    #[inline(always)]
    pub fn is_ns300(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::NS300
    }
    #[doc = "Checks if the value of the field is `NS400`"]
    #[inline(always)]
    pub fn is_ns400(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::NS400
    }
    #[doc = "Checks if the value of the field is `NS500`"]
    #[inline(always)]
    pub fn is_ns500(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::NS500
    }
    #[doc = "Checks if the value of the field is `NS800`"]
    #[inline(always)]
    pub fn is_ns800(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::NS800
    }
    #[doc = "Checks if the value of the field is `NS1600`"]
    #[inline(always)]
    pub fn is_ns1600(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::NS1600
    }
    #[doc = "Checks if the value of the field is `NS3200`"]
    #[inline(always)]
    pub fn is_ns3200(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH_A::NS3200
    }
}
#[doc = "Field `WDT_CPU_RESET_LENGTH` writer - length of CPU reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us"]
pub type WDT_CPU_RESET_LENGTH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, WDTCONFIG0_SPEC, 3, O, WDT_CPU_RESET_LENGTH_A>;
impl<'a, const O: u8> WDT_CPU_RESET_LENGTH_W<'a, O> {
    #[doc = "100ns"]
    #[inline(always)]
    pub fn ns100(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::NS100)
    }
    #[doc = "200ns"]
    #[inline(always)]
    pub fn ns200(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::NS200)
    }
    #[doc = "300ns"]
    #[inline(always)]
    pub fn ns300(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::NS300)
    }
    #[doc = "400ns"]
    #[inline(always)]
    pub fn ns400(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::NS400)
    }
    #[doc = "500ns"]
    #[inline(always)]
    pub fn ns500(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::NS500)
    }
    #[doc = "800ns"]
    #[inline(always)]
    pub fn ns800(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::NS800)
    }
    #[doc = "1.6us"]
    #[inline(always)]
    pub fn ns1600(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::NS1600)
    }
    #[doc = "3.2us"]
    #[inline(always)]
    pub fn ns3200(self) -> &'a mut W {
        self.variant(WDT_CPU_RESET_LENGTH_A::NS3200)
    }
}
#[doc = "Field `WDT_LEVEL_INT_EN` reader - When set level type interrupt generation is enabled"]
pub type WDT_LEVEL_INT_EN_R = crate::BitReader;
#[doc = "Field `WDT_LEVEL_INT_EN` writer - When set level type interrupt generation is enabled"]
pub type WDT_LEVEL_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_EDGE_INT_EN` reader - When set edge type interrupt generation is enabled"]
pub type WDT_EDGE_INT_EN_R = crate::BitReader;
#[doc = "Field `WDT_EDGE_INT_EN` writer - When set edge type interrupt generation is enabled"]
pub type WDT_EDGE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_STG3` reader - Stage 3 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
pub type WDT_STG3_R = crate::FieldReader<WDT_STG3_A>;
#[doc = "Stage 3 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDT_STG3_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Interrupt"]
    INTERRUPT = 1,
    #[doc = "2: Reset CPU"]
    RESET = 2,
    #[doc = "3: Reset system"]
    RESET_SYS = 3,
}
impl From<WDT_STG3_A> for u8 {
    #[inline(always)]
    fn from(variant: WDT_STG3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDT_STG3_A {
    type Ux = u8;
}
impl WDT_STG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_STG3_A {
        match self.bits {
            0 => WDT_STG3_A::OFF,
            1 => WDT_STG3_A::INTERRUPT,
            2 => WDT_STG3_A::RESET,
            3 => WDT_STG3_A::RESET_SYS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == WDT_STG3_A::OFF
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == WDT_STG3_A::INTERRUPT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDT_STG3_A::RESET
    }
    #[doc = "Checks if the value of the field is `RESET_SYS`"]
    #[inline(always)]
    pub fn is_reset_sys(&self) -> bool {
        *self == WDT_STG3_A::RESET_SYS
    }
}
#[doc = "Field `WDT_STG3` writer - Stage 3 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
pub type WDT_STG3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, WDTCONFIG0_SPEC, 2, O, WDT_STG3_A>;
impl<'a, const O: u8> WDT_STG3_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(WDT_STG3_A::OFF)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(WDT_STG3_A::INTERRUPT)
    }
    #[doc = "Reset CPU"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WDT_STG3_A::RESET)
    }
    #[doc = "Reset system"]
    #[inline(always)]
    pub fn reset_sys(self) -> &'a mut W {
        self.variant(WDT_STG3_A::RESET_SYS)
    }
}
#[doc = "Field `WDT_STG2` reader - Stage 2 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
pub use WDT_STG3_R as WDT_STG2_R;
#[doc = "Field `WDT_STG1` reader - Stage 1 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
pub use WDT_STG3_R as WDT_STG1_R;
#[doc = "Field `WDT_STG0` reader - Stage 0 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
pub use WDT_STG3_R as WDT_STG0_R;
#[doc = "Field `WDT_STG2` writer - Stage 2 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
pub use WDT_STG3_W as WDT_STG2_W;
#[doc = "Field `WDT_STG1` writer - Stage 1 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
pub use WDT_STG3_W as WDT_STG1_W;
#[doc = "Field `WDT_STG0` writer - Stage 0 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
pub use WDT_STG3_W as WDT_STG0_W;
#[doc = "Field `WDT_EN` reader - When set SWDT is enabled"]
pub type WDT_EN_R = crate::BitReader;
#[doc = "Field `WDT_EN` writer - When set SWDT is enabled"]
pub type WDT_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
impl R {
    #[doc = "Bit 14 - When set flash boot protection is enabled"]
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&self) -> WDT_FLASHBOOT_MOD_EN_R {
        WDT_FLASHBOOT_MOD_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:17 - length of system reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us"]
    #[inline(always)]
    pub fn wdt_sys_reset_length(&self) -> WDT_SYS_RESET_LENGTH_R {
        WDT_SYS_RESET_LENGTH_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - length of CPU reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us"]
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&self) -> WDT_CPU_RESET_LENGTH_R {
        WDT_CPU_RESET_LENGTH_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - When set level type interrupt generation is enabled"]
    #[inline(always)]
    pub fn wdt_level_int_en(&self) -> WDT_LEVEL_INT_EN_R {
        WDT_LEVEL_INT_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When set edge type interrupt generation is enabled"]
    #[inline(always)]
    pub fn wdt_edge_int_en(&self) -> WDT_EDGE_INT_EN_R {
        WDT_EDGE_INT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - Stage 3 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
    #[inline(always)]
    pub fn wdt_stg3(&self) -> WDT_STG3_R {
        WDT_STG3_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - Stage 2 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
    #[inline(always)]
    pub fn wdt_stg2(&self) -> WDT_STG2_R {
        WDT_STG2_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - Stage 1 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
    #[inline(always)]
    pub fn wdt_stg1(&self) -> WDT_STG1_R {
        WDT_STG1_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - Stage 0 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
    #[inline(always)]
    pub fn wdt_stg0(&self) -> WDT_STG0_R {
        WDT_STG0_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - When set SWDT is enabled"]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG0")
            .field(
                "wdt_flashboot_mod_en",
                &format_args!("{}", self.wdt_flashboot_mod_en().bit()),
            )
            .field(
                "wdt_sys_reset_length",
                &format_args!("{}", self.wdt_sys_reset_length().bits()),
            )
            .field(
                "wdt_cpu_reset_length",
                &format_args!("{}", self.wdt_cpu_reset_length().bits()),
            )
            .field(
                "wdt_level_int_en",
                &format_args!("{}", self.wdt_level_int_en().bit()),
            )
            .field(
                "wdt_edge_int_en",
                &format_args!("{}", self.wdt_edge_int_en().bit()),
            )
            .field("wdt_stg3", &format_args!("{}", self.wdt_stg3().bits()))
            .field("wdt_stg2", &format_args!("{}", self.wdt_stg2().bits()))
            .field("wdt_stg1", &format_args!("{}", self.wdt_stg1().bits()))
            .field("wdt_stg0", &format_args!("{}", self.wdt_stg0().bits()))
            .field("wdt_en", &format_args!("{}", self.wdt_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTCONFIG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 14 - When set flash boot protection is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_flashboot_mod_en(&mut self) -> WDT_FLASHBOOT_MOD_EN_W<14> {
        WDT_FLASHBOOT_MOD_EN_W::new(self)
    }
    #[doc = "Bits 15:17 - length of system reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_sys_reset_length(&mut self) -> WDT_SYS_RESET_LENGTH_W<15> {
        WDT_SYS_RESET_LENGTH_W::new(self)
    }
    #[doc = "Bits 18:20 - length of CPU reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_cpu_reset_length(&mut self) -> WDT_CPU_RESET_LENGTH_W<18> {
        WDT_CPU_RESET_LENGTH_W::new(self)
    }
    #[doc = "Bit 21 - When set level type interrupt generation is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_level_int_en(&mut self) -> WDT_LEVEL_INT_EN_W<21> {
        WDT_LEVEL_INT_EN_W::new(self)
    }
    #[doc = "Bit 22 - When set edge type interrupt generation is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_edge_int_en(&mut self) -> WDT_EDGE_INT_EN_W<22> {
        WDT_EDGE_INT_EN_W::new(self)
    }
    #[doc = "Bits 23:24 - Stage 3 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg3(&mut self) -> WDT_STG3_W<23> {
        WDT_STG3_W::new(self)
    }
    #[doc = "Bits 25:26 - Stage 2 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg2(&mut self) -> WDT_STG2_W<25> {
        WDT_STG2_W::new(self)
    }
    #[doc = "Bits 27:28 - Stage 1 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg1(&mut self) -> WDT_STG1_W<27> {
        WDT_STG1_W::new(self)
    }
    #[doc = "Bits 29:30 - Stage 0 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg0(&mut self) -> WDT_STG0_W<29> {
        WDT_STG0_W::new(self)
    }
    #[doc = "Bit 31 - When set SWDT is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_en(&mut self) -> WDT_EN_W<31> {
        WDT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig0](index.html) module"]
pub struct WDTCONFIG0_SPEC;
impl crate::RegisterSpec for WDTCONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtconfig0::R](R) reader structure"]
impl crate::Readable for WDTCONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtconfig0::W](W) writer structure"]
impl crate::Writable for WDTCONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCONFIG0 to value 0x0004_c000"]
impl crate::Resettable for WDTCONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_c000;
}
