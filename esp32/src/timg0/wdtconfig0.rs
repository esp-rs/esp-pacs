///Register `WDTCONFIG0` reader
pub type R = crate::R<WDTCONFIG0_SPEC>;
///Register `WDTCONFIG0` writer
pub type W = crate::W<WDTCONFIG0_SPEC>;
///Field `WDT_FLASHBOOT_MOD_EN` reader - When set flash boot protection is enabled
pub type WDT_FLASHBOOT_MOD_EN_R = crate::BitReader;
///Field `WDT_FLASHBOOT_MOD_EN` writer - When set flash boot protection is enabled
pub type WDT_FLASHBOOT_MOD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**length of system reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDT_SYS_RESET_LENGTH {
    ///0: 100ns
    Ns100 = 0,
    ///1: 200ns
    Ns200 = 1,
    ///2: 300ns
    Ns300 = 2,
    ///3: 400ns
    Ns400 = 3,
    ///4: 500ns
    Ns500 = 4,
    ///5: 800ns
    Ns800 = 5,
    ///6: 1.6us
    Ns1600 = 6,
    ///7: 3.2us
    Ns3200 = 7,
}
impl From<WDT_SYS_RESET_LENGTH> for u8 {
    #[inline(always)]
    fn from(variant: WDT_SYS_RESET_LENGTH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDT_SYS_RESET_LENGTH {
    type Ux = u8;
}
impl crate::IsEnum for WDT_SYS_RESET_LENGTH {}
///Field `WDT_SYS_RESET_LENGTH` reader - length of system reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us
pub type WDT_SYS_RESET_LENGTH_R = crate::FieldReader<WDT_SYS_RESET_LENGTH>;
impl WDT_SYS_RESET_LENGTH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDT_SYS_RESET_LENGTH {
        match self.bits {
            0 => WDT_SYS_RESET_LENGTH::Ns100,
            1 => WDT_SYS_RESET_LENGTH::Ns200,
            2 => WDT_SYS_RESET_LENGTH::Ns300,
            3 => WDT_SYS_RESET_LENGTH::Ns400,
            4 => WDT_SYS_RESET_LENGTH::Ns500,
            5 => WDT_SYS_RESET_LENGTH::Ns800,
            6 => WDT_SYS_RESET_LENGTH::Ns1600,
            7 => WDT_SYS_RESET_LENGTH::Ns3200,
            _ => unreachable!(),
        }
    }
    ///100ns
    #[inline(always)]
    pub fn is_ns100(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH::Ns100
    }
    ///200ns
    #[inline(always)]
    pub fn is_ns200(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH::Ns200
    }
    ///300ns
    #[inline(always)]
    pub fn is_ns300(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH::Ns300
    }
    ///400ns
    #[inline(always)]
    pub fn is_ns400(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH::Ns400
    }
    ///500ns
    #[inline(always)]
    pub fn is_ns500(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH::Ns500
    }
    ///800ns
    #[inline(always)]
    pub fn is_ns800(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH::Ns800
    }
    ///1.6us
    #[inline(always)]
    pub fn is_ns1600(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH::Ns1600
    }
    ///3.2us
    #[inline(always)]
    pub fn is_ns3200(&self) -> bool {
        *self == WDT_SYS_RESET_LENGTH::Ns3200
    }
}
///Field `WDT_SYS_RESET_LENGTH` writer - length of system reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us
pub type WDT_SYS_RESET_LENGTH_W<'a, REG> =
    crate::FieldWriter<'a, REG, 3, WDT_SYS_RESET_LENGTH, crate::Safe>;
impl<'a, REG> WDT_SYS_RESET_LENGTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///100ns
    #[inline(always)]
    pub fn ns100(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_SYS_RESET_LENGTH::Ns100)
    }
    ///200ns
    #[inline(always)]
    pub fn ns200(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_SYS_RESET_LENGTH::Ns200)
    }
    ///300ns
    #[inline(always)]
    pub fn ns300(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_SYS_RESET_LENGTH::Ns300)
    }
    ///400ns
    #[inline(always)]
    pub fn ns400(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_SYS_RESET_LENGTH::Ns400)
    }
    ///500ns
    #[inline(always)]
    pub fn ns500(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_SYS_RESET_LENGTH::Ns500)
    }
    ///800ns
    #[inline(always)]
    pub fn ns800(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_SYS_RESET_LENGTH::Ns800)
    }
    ///1.6us
    #[inline(always)]
    pub fn ns1600(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_SYS_RESET_LENGTH::Ns1600)
    }
    ///3.2us
    #[inline(always)]
    pub fn ns3200(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_SYS_RESET_LENGTH::Ns3200)
    }
}
/**length of CPU reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDT_CPU_RESET_LENGTH {
    ///0: 100ns
    Ns100 = 0,
    ///1: 200ns
    Ns200 = 1,
    ///2: 300ns
    Ns300 = 2,
    ///3: 400ns
    Ns400 = 3,
    ///4: 500ns
    Ns500 = 4,
    ///5: 800ns
    Ns800 = 5,
    ///6: 1.6us
    Ns1600 = 6,
    ///7: 3.2us
    Ns3200 = 7,
}
impl From<WDT_CPU_RESET_LENGTH> for u8 {
    #[inline(always)]
    fn from(variant: WDT_CPU_RESET_LENGTH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDT_CPU_RESET_LENGTH {
    type Ux = u8;
}
impl crate::IsEnum for WDT_CPU_RESET_LENGTH {}
///Field `WDT_CPU_RESET_LENGTH` reader - length of CPU reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us
pub type WDT_CPU_RESET_LENGTH_R = crate::FieldReader<WDT_CPU_RESET_LENGTH>;
impl WDT_CPU_RESET_LENGTH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDT_CPU_RESET_LENGTH {
        match self.bits {
            0 => WDT_CPU_RESET_LENGTH::Ns100,
            1 => WDT_CPU_RESET_LENGTH::Ns200,
            2 => WDT_CPU_RESET_LENGTH::Ns300,
            3 => WDT_CPU_RESET_LENGTH::Ns400,
            4 => WDT_CPU_RESET_LENGTH::Ns500,
            5 => WDT_CPU_RESET_LENGTH::Ns800,
            6 => WDT_CPU_RESET_LENGTH::Ns1600,
            7 => WDT_CPU_RESET_LENGTH::Ns3200,
            _ => unreachable!(),
        }
    }
    ///100ns
    #[inline(always)]
    pub fn is_ns100(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH::Ns100
    }
    ///200ns
    #[inline(always)]
    pub fn is_ns200(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH::Ns200
    }
    ///300ns
    #[inline(always)]
    pub fn is_ns300(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH::Ns300
    }
    ///400ns
    #[inline(always)]
    pub fn is_ns400(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH::Ns400
    }
    ///500ns
    #[inline(always)]
    pub fn is_ns500(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH::Ns500
    }
    ///800ns
    #[inline(always)]
    pub fn is_ns800(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH::Ns800
    }
    ///1.6us
    #[inline(always)]
    pub fn is_ns1600(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH::Ns1600
    }
    ///3.2us
    #[inline(always)]
    pub fn is_ns3200(&self) -> bool {
        *self == WDT_CPU_RESET_LENGTH::Ns3200
    }
}
///Field `WDT_CPU_RESET_LENGTH` writer - length of CPU reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us
pub type WDT_CPU_RESET_LENGTH_W<'a, REG> =
    crate::FieldWriter<'a, REG, 3, WDT_CPU_RESET_LENGTH, crate::Safe>;
impl<'a, REG> WDT_CPU_RESET_LENGTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///100ns
    #[inline(always)]
    pub fn ns100(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_CPU_RESET_LENGTH::Ns100)
    }
    ///200ns
    #[inline(always)]
    pub fn ns200(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_CPU_RESET_LENGTH::Ns200)
    }
    ///300ns
    #[inline(always)]
    pub fn ns300(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_CPU_RESET_LENGTH::Ns300)
    }
    ///400ns
    #[inline(always)]
    pub fn ns400(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_CPU_RESET_LENGTH::Ns400)
    }
    ///500ns
    #[inline(always)]
    pub fn ns500(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_CPU_RESET_LENGTH::Ns500)
    }
    ///800ns
    #[inline(always)]
    pub fn ns800(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_CPU_RESET_LENGTH::Ns800)
    }
    ///1.6us
    #[inline(always)]
    pub fn ns1600(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_CPU_RESET_LENGTH::Ns1600)
    }
    ///3.2us
    #[inline(always)]
    pub fn ns3200(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_CPU_RESET_LENGTH::Ns3200)
    }
}
///Field `WDT_LEVEL_INT_EN` reader - When set level type interrupt generation is enabled
pub type WDT_LEVEL_INT_EN_R = crate::BitReader;
///Field `WDT_LEVEL_INT_EN` writer - When set level type interrupt generation is enabled
pub type WDT_LEVEL_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDT_EDGE_INT_EN` reader - When set edge type interrupt generation is enabled
pub type WDT_EDGE_INT_EN_R = crate::BitReader;
///Field `WDT_EDGE_INT_EN` writer - When set edge type interrupt generation is enabled
pub type WDT_EDGE_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Stage 3 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDT_STG3 {
    ///0: Off
    Off = 0,
    ///1: Interrupt
    Interrupt = 1,
    ///2: Reset CPU
    Reset = 2,
    ///3: Reset system
    ResetSys = 3,
}
impl From<WDT_STG3> for u8 {
    #[inline(always)]
    fn from(variant: WDT_STG3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDT_STG3 {
    type Ux = u8;
}
impl crate::IsEnum for WDT_STG3 {}
///Field `WDT_STG3` reader - Stage 3 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
pub type WDT_STG3_R = crate::FieldReader<WDT_STG3>;
impl WDT_STG3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDT_STG3 {
        match self.bits {
            0 => WDT_STG3::Off,
            1 => WDT_STG3::Interrupt,
            2 => WDT_STG3::Reset,
            3 => WDT_STG3::ResetSys,
            _ => unreachable!(),
        }
    }
    ///Off
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == WDT_STG3::Off
    }
    ///Interrupt
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == WDT_STG3::Interrupt
    }
    ///Reset CPU
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WDT_STG3::Reset
    }
    ///Reset system
    #[inline(always)]
    pub fn is_reset_sys(&self) -> bool {
        *self == WDT_STG3::ResetSys
    }
}
///Field `WDT_STG3` writer - Stage 3 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
pub type WDT_STG3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WDT_STG3, crate::Safe>;
impl<'a, REG> WDT_STG3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Off
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_STG3::Off)
    }
    ///Interrupt
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_STG3::Interrupt)
    }
    ///Reset CPU
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_STG3::Reset)
    }
    ///Reset system
    #[inline(always)]
    pub fn reset_sys(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_STG3::ResetSys)
    }
}
///Field `WDT_STG2` reader - Stage 2 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
pub use WDT_STG3_R as WDT_STG2_R;
///Field `WDT_STG1` reader - Stage 1 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
pub use WDT_STG3_R as WDT_STG1_R;
///Field `WDT_STG0` reader - Stage 0 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
pub use WDT_STG3_R as WDT_STG0_R;
///Field `WDT_STG2` writer - Stage 2 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
pub use WDT_STG3_W as WDT_STG2_W;
///Field `WDT_STG1` writer - Stage 1 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
pub use WDT_STG3_W as WDT_STG1_W;
///Field `WDT_STG0` writer - Stage 0 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
pub use WDT_STG3_W as WDT_STG0_W;
///Field `WDT_EN` reader - When set SWDT is enabled
pub type WDT_EN_R = crate::BitReader;
///Field `WDT_EN` writer - When set SWDT is enabled
pub type WDT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 14 - When set flash boot protection is enabled
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&self) -> WDT_FLASHBOOT_MOD_EN_R {
        WDT_FLASHBOOT_MOD_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:17 - length of system reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us
    #[inline(always)]
    pub fn wdt_sys_reset_length(&self) -> WDT_SYS_RESET_LENGTH_R {
        WDT_SYS_RESET_LENGTH_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - length of CPU reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&self) -> WDT_CPU_RESET_LENGTH_R {
        WDT_CPU_RESET_LENGTH_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 21 - When set level type interrupt generation is enabled
    #[inline(always)]
    pub fn wdt_level_int_en(&self) -> WDT_LEVEL_INT_EN_R {
        WDT_LEVEL_INT_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - When set edge type interrupt generation is enabled
    #[inline(always)]
    pub fn wdt_edge_int_en(&self) -> WDT_EDGE_INT_EN_R {
        WDT_EDGE_INT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 23:24 - Stage 3 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
    #[inline(always)]
    pub fn wdt_stg3(&self) -> WDT_STG3_R {
        WDT_STG3_R::new(((self.bits >> 23) & 3) as u8)
    }
    ///Bits 25:26 - Stage 2 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
    #[inline(always)]
    pub fn wdt_stg2(&self) -> WDT_STG2_R {
        WDT_STG2_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bits 27:28 - Stage 1 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
    #[inline(always)]
    pub fn wdt_stg1(&self) -> WDT_STG1_R {
        WDT_STG1_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bits 29:30 - Stage 0 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
    #[inline(always)]
    pub fn wdt_stg0(&self) -> WDT_STG0_R {
        WDT_STG0_R::new(((self.bits >> 29) & 3) as u8)
    }
    ///Bit 31 - When set SWDT is enabled
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG0")
            .field("wdt_flashboot_mod_en", &self.wdt_flashboot_mod_en())
            .field("wdt_sys_reset_length", &self.wdt_sys_reset_length())
            .field("wdt_cpu_reset_length", &self.wdt_cpu_reset_length())
            .field("wdt_level_int_en", &self.wdt_level_int_en())
            .field("wdt_edge_int_en", &self.wdt_edge_int_en())
            .field("wdt_stg3", &self.wdt_stg3())
            .field("wdt_stg2", &self.wdt_stg2())
            .field("wdt_stg1", &self.wdt_stg1())
            .field("wdt_stg0", &self.wdt_stg0())
            .field("wdt_en", &self.wdt_en())
            .finish()
    }
}
impl W {
    ///Bit 14 - When set flash boot protection is enabled
    #[inline(always)]
    #[must_use]
    pub fn wdt_flashboot_mod_en(&mut self) -> WDT_FLASHBOOT_MOD_EN_W<WDTCONFIG0_SPEC> {
        WDT_FLASHBOOT_MOD_EN_W::new(self, 14)
    }
    ///Bits 15:17 - length of system reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us
    #[inline(always)]
    #[must_use]
    pub fn wdt_sys_reset_length(&mut self) -> WDT_SYS_RESET_LENGTH_W<WDTCONFIG0_SPEC> {
        WDT_SYS_RESET_LENGTH_W::new(self, 15)
    }
    ///Bits 18:20 - length of CPU reset selection. 0: 100ns 1: 200ns 2: 300ns 3: 400ns 4: 500ns 5: 800ns 6: 1.6us 7: 3.2us
    #[inline(always)]
    #[must_use]
    pub fn wdt_cpu_reset_length(&mut self) -> WDT_CPU_RESET_LENGTH_W<WDTCONFIG0_SPEC> {
        WDT_CPU_RESET_LENGTH_W::new(self, 18)
    }
    ///Bit 21 - When set level type interrupt generation is enabled
    #[inline(always)]
    #[must_use]
    pub fn wdt_level_int_en(&mut self) -> WDT_LEVEL_INT_EN_W<WDTCONFIG0_SPEC> {
        WDT_LEVEL_INT_EN_W::new(self, 21)
    }
    ///Bit 22 - When set edge type interrupt generation is enabled
    #[inline(always)]
    #[must_use]
    pub fn wdt_edge_int_en(&mut self) -> WDT_EDGE_INT_EN_W<WDTCONFIG0_SPEC> {
        WDT_EDGE_INT_EN_W::new(self, 22)
    }
    ///Bits 23:24 - Stage 3 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg3(&mut self) -> WDT_STG3_W<WDTCONFIG0_SPEC> {
        WDT_STG3_W::new(self, 23)
    }
    ///Bits 25:26 - Stage 2 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg2(&mut self) -> WDT_STG2_W<WDTCONFIG0_SPEC> {
        WDT_STG2_W::new(self, 25)
    }
    ///Bits 27:28 - Stage 1 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg1(&mut self) -> WDT_STG1_W<WDTCONFIG0_SPEC> {
        WDT_STG1_W::new(self, 27)
    }
    ///Bits 29:30 - Stage 0 configuration. 0: off 1: interrupt 2: reset CPU 3: reset system
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg0(&mut self) -> WDT_STG0_W<WDTCONFIG0_SPEC> {
        WDT_STG0_W::new(self, 29)
    }
    ///Bit 31 - When set SWDT is enabled
    #[inline(always)]
    #[must_use]
    pub fn wdt_en(&mut self) -> WDT_EN_W<WDTCONFIG0_SPEC> {
        WDT_EN_W::new(self, 31)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WDTCONFIG0_SPEC;
impl crate::RegisterSpec for WDTCONFIG0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`wdtconfig0::R`](R) reader structure
impl crate::Readable for WDTCONFIG0_SPEC {}
///`write(|w| ..)` method takes [`wdtconfig0::W`](W) writer structure
impl crate::Writable for WDTCONFIG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDTCONFIG0 to value 0x0004_c000
impl crate::Resettable for WDTCONFIG0_SPEC {
    const RESET_VALUE: u32 = 0x0004_c000;
}
