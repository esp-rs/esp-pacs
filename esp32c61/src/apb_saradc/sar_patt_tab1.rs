#[doc = "Register `SAR_PATT_TAB1` reader"]
pub type R = crate::R<SAR_PATT_TAB1_SPEC>;
#[doc = "Register `SAR_PATT_TAB1` writer"]
pub type W = crate::W<SAR_PATT_TAB1_SPEC>;
#[doc = "\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ITEM3_ATTEN {
    #[doc = "0: No input attenuation, ADC can measure up to approx."]
    Db0   = 0,
    #[doc = "1: The input voltage of ADC will be attenuated extending the range of measurement by about 2.5 dB"]
    Db2_5 = 1,
    #[doc = "2: The input voltage of ADC will be attenuated extending the range of measurement by about 6 dB"]
    Db6   = 2,
    #[doc = "3: The input voltage of ADC will be attenuated extending the range of measurement by about 12 dB"]
    Db12  = 3,
}
impl From<ITEM3_ATTEN> for u8 {
    #[inline(always)]
    fn from(variant: ITEM3_ATTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ITEM3_ATTEN {
    type Ux = u8;
}
impl crate::IsEnum for ITEM3_ATTEN {}
#[doc = "Field `ITEM3_ATTEN` reader - "]
pub type ITEM3_ATTEN_R = crate::FieldReader<ITEM3_ATTEN>;
impl ITEM3_ATTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITEM3_ATTEN {
        match self.bits {
            0 => ITEM3_ATTEN::Db0,
            1 => ITEM3_ATTEN::Db2_5,
            2 => ITEM3_ATTEN::Db6,
            3 => ITEM3_ATTEN::Db12,
            _ => unreachable!(),
        }
    }
    #[doc = "No input attenuation, ADC can measure up to approx."]
    #[inline(always)]
    pub fn is_db0(&self) -> bool {
        *self == ITEM3_ATTEN::Db0
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 2.5 dB"]
    #[inline(always)]
    pub fn is_db2_5(&self) -> bool {
        *self == ITEM3_ATTEN::Db2_5
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 6 dB"]
    #[inline(always)]
    pub fn is_db6(&self) -> bool {
        *self == ITEM3_ATTEN::Db6
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 12 dB"]
    #[inline(always)]
    pub fn is_db12(&self) -> bool {
        *self == ITEM3_ATTEN::Db12
    }
}
#[doc = "Field `ITEM3_ATTEN` writer - "]
pub type ITEM3_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ITEM3_ATTEN, crate::Safe>;
impl<'a, REG> ITEM3_ATTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No input attenuation, ADC can measure up to approx."]
    #[inline(always)]
    pub fn db0(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM3_ATTEN::Db0)
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 2.5 dB"]
    #[inline(always)]
    pub fn db2_5(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM3_ATTEN::Db2_5)
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 6 dB"]
    #[inline(always)]
    pub fn db6(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM3_ATTEN::Db6)
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 12 dB"]
    #[inline(always)]
    pub fn db12(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM3_ATTEN::Db12)
    }
}
#[doc = "Field `ITEM3_CHANNEL` reader - "]
pub type ITEM3_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM3_CHANNEL` writer - "]
pub type ITEM3_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITEM3_UNIT {
    #[doc = "0: SAR ADC 1"]
    Adc1 = 0,
    #[doc = "1: SAR ADC 2"]
    Adc2 = 1,
}
impl From<ITEM3_UNIT> for bool {
    #[inline(always)]
    fn from(variant: ITEM3_UNIT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITEM3_UNIT` reader - "]
pub type ITEM3_UNIT_R = crate::BitReader<ITEM3_UNIT>;
impl ITEM3_UNIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITEM3_UNIT {
        match self.bits {
            false => ITEM3_UNIT::Adc1,
            true => ITEM3_UNIT::Adc2,
        }
    }
    #[doc = "SAR ADC 1"]
    #[inline(always)]
    pub fn is_adc1(&self) -> bool {
        *self == ITEM3_UNIT::Adc1
    }
    #[doc = "SAR ADC 2"]
    #[inline(always)]
    pub fn is_adc2(&self) -> bool {
        *self == ITEM3_UNIT::Adc2
    }
}
#[doc = "Field `ITEM3_UNIT` writer - "]
pub type ITEM3_UNIT_W<'a, REG> = crate::BitWriter<'a, REG, ITEM3_UNIT>;
impl<'a, REG> ITEM3_UNIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAR ADC 1"]
    #[inline(always)]
    pub fn adc1(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM3_UNIT::Adc1)
    }
    #[doc = "SAR ADC 2"]
    #[inline(always)]
    pub fn adc2(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM3_UNIT::Adc2)
    }
}
#[doc = "Field `ITEM2_ATTEN` reader - "]
pub use ITEM3_ATTEN_R as ITEM2_ATTEN_R;
#[doc = "Field `ITEM2_ATTEN` writer - "]
pub use ITEM3_ATTEN_W as ITEM2_ATTEN_W;
#[doc = "Field `ITEM2_CHANNEL` reader - "]
pub type ITEM2_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM2_CHANNEL` writer - "]
pub type ITEM2_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ITEM1_ATTEN` reader - "]
pub use ITEM3_ATTEN_R as ITEM1_ATTEN_R;
#[doc = "Field `ITEM1_ATTEN` writer - "]
pub use ITEM3_ATTEN_W as ITEM1_ATTEN_W;
#[doc = "Field `ITEM2_UNIT` reader - "]
pub use ITEM3_UNIT_R as ITEM2_UNIT_R;
#[doc = "Field `ITEM2_UNIT` writer - "]
pub use ITEM3_UNIT_W as ITEM2_UNIT_W;
#[doc = "Field `ITEM1_CHANNEL` reader - "]
pub type ITEM1_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM1_CHANNEL` writer - "]
pub type ITEM1_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ITEM0_ATTEN` reader - "]
pub use ITEM3_ATTEN_R as ITEM0_ATTEN_R;
#[doc = "Field `ITEM0_ATTEN` writer - "]
pub use ITEM3_ATTEN_W as ITEM0_ATTEN_W;
#[doc = "Field `ITEM1_UNIT` reader - "]
pub use ITEM3_UNIT_R as ITEM1_UNIT_R;
#[doc = "Field `ITEM1_UNIT` writer - "]
pub use ITEM3_UNIT_W as ITEM1_UNIT_W;
#[doc = "Field `ITEM0_CHANNEL` reader - "]
pub type ITEM0_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM0_CHANNEL` writer - "]
pub type ITEM0_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ITEM0_UNIT` reader - "]
pub use ITEM3_UNIT_R as ITEM0_UNIT_R;
#[doc = "Field `ITEM0_UNIT` writer - "]
pub use ITEM3_UNIT_W as ITEM0_UNIT_W;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn item3_atten(&self) -> ITEM3_ATTEN_R {
        ITEM3_ATTEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn item3_channel(&self) -> ITEM3_CHANNEL_R {
        ITEM3_CHANNEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn item3_unit(&self) -> ITEM3_UNIT_R {
        ITEM3_UNIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn item2_atten(&self) -> ITEM2_ATTEN_R {
        ITEM2_ATTEN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn item2_channel(&self) -> ITEM2_CHANNEL_R {
        ITEM2_CHANNEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn item2_unit(&self) -> ITEM2_UNIT_R {
        ITEM2_UNIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn item1_atten(&self) -> ITEM1_ATTEN_R {
        ITEM1_ATTEN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn item1_channel(&self) -> ITEM1_CHANNEL_R {
        ITEM1_CHANNEL_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn item1_unit(&self) -> ITEM1_UNIT_R {
        ITEM1_UNIT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn item0_atten(&self) -> ITEM0_ATTEN_R {
        ITEM0_ATTEN_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn item0_channel(&self) -> ITEM0_CHANNEL_R {
        ITEM0_CHANNEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn item0_unit(&self) -> ITEM0_UNIT_R {
        ITEM0_UNIT_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_PATT_TAB1")
            .field("item3_atten", &self.item3_atten())
            .field("item0_atten", &self.item0_atten())
            .field("item0_channel", &self.item0_channel())
            .field("item3_unit", &self.item3_unit())
            .field("item0_unit", &self.item0_unit())
            .field("item1_atten", &self.item1_atten())
            .field("item1_channel", &self.item1_channel())
            .field("item1_unit", &self.item1_unit())
            .field("item2_atten", &self.item2_atten())
            .field("item2_channel", &self.item2_channel())
            .field("item2_unit", &self.item2_unit())
            .field("item3_channel", &self.item3_channel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn item3_atten(&mut self) -> ITEM3_ATTEN_W<'_, SAR_PATT_TAB1_SPEC> {
        ITEM3_ATTEN_W::new(self, 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn item3_channel(&mut self) -> ITEM3_CHANNEL_W<'_, SAR_PATT_TAB1_SPEC> {
        ITEM3_CHANNEL_W::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn item3_unit(&mut self) -> ITEM3_UNIT_W<'_, SAR_PATT_TAB1_SPEC> {
        ITEM3_UNIT_W::new(self, 5)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn item2_atten(&mut self) -> ITEM2_ATTEN_W<'_, SAR_PATT_TAB1_SPEC> {
        ITEM2_ATTEN_W::new(self, 6)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn item2_channel(&mut self) -> ITEM2_CHANNEL_W<'_, SAR_PATT_TAB1_SPEC> {
        ITEM2_CHANNEL_W::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn item2_unit(&mut self) -> ITEM2_UNIT_W<'_, SAR_PATT_TAB1_SPEC> {
        ITEM2_UNIT_W::new(self, 11)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn item1_atten(&mut self) -> ITEM1_ATTEN_W<'_, SAR_PATT_TAB1_SPEC> {
        ITEM1_ATTEN_W::new(self, 12)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn item1_channel(&mut self) -> ITEM1_CHANNEL_W<'_, SAR_PATT_TAB1_SPEC> {
        ITEM1_CHANNEL_W::new(self, 14)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn item1_unit(&mut self) -> ITEM1_UNIT_W<'_, SAR_PATT_TAB1_SPEC> {
        ITEM1_UNIT_W::new(self, 17)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn item0_atten(&mut self) -> ITEM0_ATTEN_W<'_, SAR_PATT_TAB1_SPEC> {
        ITEM0_ATTEN_W::new(self, 18)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn item0_channel(&mut self) -> ITEM0_CHANNEL_W<'_, SAR_PATT_TAB1_SPEC> {
        ITEM0_CHANNEL_W::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn item0_unit(&mut self) -> ITEM0_UNIT_W<'_, SAR_PATT_TAB1_SPEC> {
        ITEM0_UNIT_W::new(self, 23)
    }
}
#[doc = "digital saradc configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_patt_tab1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_patt_tab1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_PATT_TAB1_SPEC;
impl crate::RegisterSpec for SAR_PATT_TAB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_patt_tab1::R`](R) reader structure"]
impl crate::Readable for SAR_PATT_TAB1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_patt_tab1::W`](W) writer structure"]
impl crate::Writable for SAR_PATT_TAB1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_PATT_TAB1 to value 0x00ff_ffff"]
impl crate::Resettable for SAR_PATT_TAB1_SPEC {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}
