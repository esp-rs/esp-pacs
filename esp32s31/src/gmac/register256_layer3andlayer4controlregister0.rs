#[doc = "Register `REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0` reader"]
pub type R = crate::R<REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC>;
#[doc = "Register `REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0` writer"]
pub type W = crate::W<REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC>;
#[doc = "Field `L3PEN0` reader - Layer 3 Protocol Enable When set, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv6 frames When reset, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv4 frames The Layer 3 matching is done only when either L3SAM0 or L3DAM0 bit is set high"]
pub type L3PEN0_R = crate::BitReader;
#[doc = "Field `L3PEN0` writer - Layer 3 Protocol Enable When set, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv6 frames When reset, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv4 frames The Layer 3 matching is done only when either L3SAM0 or L3DAM0 bit is set high"]
pub type L3PEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3SAM0` reader - Layer 3 IP SA Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for matching When reset, the MAC ignores the Layer 3 IP Source Address field for matching Note: When Bit 0 _L3PEN0_ is set, you should set either this bit or Bit 4 _L3DAM0_ because either IPv6 SA or DA can be checked for filtering"]
pub type L3SAM0_R = crate::BitReader;
#[doc = "Field `L3SAM0` writer - Layer 3 IP SA Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for matching When reset, the MAC ignores the Layer 3 IP Source Address field for matching Note: When Bit 0 _L3PEN0_ is set, you should set either this bit or Bit 4 _L3DAM0_ because either IPv6 SA or DA can be checked for filtering"]
pub type L3SAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3SAIM0` reader - Layer 3 IP SA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for inverse matching When reset, this bit indicates that the Layer 3 IP Source Address field is enabled for perfect matching This bit is valid and applicable only when Bit 2 _L3SAM0_ is set high"]
pub type L3SAIM0_R = crate::BitReader;
#[doc = "Field `L3SAIM0` writer - Layer 3 IP SA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for inverse matching When reset, this bit indicates that the Layer 3 IP Source Address field is enabled for perfect matching This bit is valid and applicable only when Bit 2 _L3SAM0_ is set high"]
pub type L3SAIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3DAM0` reader - Layer 3 IP DA Match Enable When set, this bit indicates that Layer 3 IP Destination Address field is enabled for matching When reset, the MAC ignores the Layer 3 IP Destination Address field for matching Note: When Bit 0 _L3PEN0_ is set, you should set either this bit or Bit 2 _L3SAM0_ because either IPv6 DA or SA can be checked for filtering"]
pub type L3DAM0_R = crate::BitReader;
#[doc = "Field `L3DAM0` writer - Layer 3 IP DA Match Enable When set, this bit indicates that Layer 3 IP Destination Address field is enabled for matching When reset, the MAC ignores the Layer 3 IP Destination Address field for matching Note: When Bit 0 _L3PEN0_ is set, you should set either this bit or Bit 2 _L3SAM0_ because either IPv6 DA or SA can be checked for filtering"]
pub type L3DAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3DAIM0` reader - Layer 3 IP DA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Destination Address field is enabled for inverse matching When reset, this bit indicates that the Layer 3 IP Destination Address field is enabled for perfect matching This bit is valid and applicable only when Bit 4 _L3DAM0_ is set high"]
pub type L3DAIM0_R = crate::BitReader;
#[doc = "Field `L3DAIM0` writer - Layer 3 IP DA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Destination Address field is enabled for inverse matching When reset, this bit indicates that the Layer 3 IP Destination Address field is enabled for perfect matching This bit is valid and applicable only when Bit 4 _L3DAM0_ is set high"]
pub type L3DAIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L3HSBM0` reader - Layer 3 IP SA Higher Bits Match IPv4 Frames: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 frames The following list describes the values of this field: 0: No bits are masked 1: LSb\\[0\\] is masked 2: Two LSbs \\[1:0\\] are masked 31: All bits except MSb are masked IPv6 Frames: This field contains Bits \\[4:0\\] of the field that indicates the number of higher bits of IP Source or Destination Address matched in the IPv6 frames This field is valid and applicable only if L3DAM0 or L3SAM0 is set high"]
pub type L3HSBM0_R = crate::FieldReader;
#[doc = "Field `L3HSBM0` writer - Layer 3 IP SA Higher Bits Match IPv4 Frames: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 frames The following list describes the values of this field: 0: No bits are masked 1: LSb\\[0\\] is masked 2: Two LSbs \\[1:0\\] are masked 31: All bits except MSb are masked IPv6 Frames: This field contains Bits \\[4:0\\] of the field that indicates the number of higher bits of IP Source or Destination Address matched in the IPv6 frames This field is valid and applicable only if L3DAM0 or L3SAM0 is set high"]
pub type L3HSBM0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L3HDBM0` reader - Layer 3 IP DA Higher Bits Match IPv4 Frames: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 frames The following list describes the values of this field: 0: No bits are masked 1: LSb\\[0\\] is masked 2: Two LSbs \\[1:0\\] are masked 31: All bits except MSb are masked IPv6 Frames: Bits \\[12:11\\] of this field correspond to Bits \\[6:5\\] of L3HSBM0, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 frames The following list describes the concatenated values of the L3HDBM0\\[1:0\\] and L3HSBM0 bits: 0: No bits are masked 1: LSb\\[0\\] is masked 2: Two LSbs \\[1:0\\] are masked … 127: All bits except MSb are masked This field is valid and applicable only if L3DAM0 or L3SAM0 is set high"]
pub type L3HDBM0_R = crate::FieldReader;
#[doc = "Field `L3HDBM0` writer - Layer 3 IP DA Higher Bits Match IPv4 Frames: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 frames The following list describes the values of this field: 0: No bits are masked 1: LSb\\[0\\] is masked 2: Two LSbs \\[1:0\\] are masked 31: All bits except MSb are masked IPv6 Frames: Bits \\[12:11\\] of this field correspond to Bits \\[6:5\\] of L3HSBM0, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 frames The following list describes the concatenated values of the L3HDBM0\\[1:0\\] and L3HSBM0 bits: 0: No bits are masked 1: LSb\\[0\\] is masked 2: Two LSbs \\[1:0\\] are masked … 127: All bits except MSb are masked This field is valid and applicable only if L3DAM0 or L3SAM0 is set high"]
pub type L3HDBM0_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L4PEN0` reader - Layer 4 Protocol Enable When set, this bit indicates that the Source and Destination Port number fields for UDP frames are used for matching When reset, this bit indicates that the Source and Destination Port number fields for TCP frames are used for matching The Layer 4 matching is done only when either L4SPM0 or L4DPM0 bit is set high"]
pub type L4PEN0_R = crate::BitReader;
#[doc = "Field `L4PEN0` writer - Layer 4 Protocol Enable When set, this bit indicates that the Source and Destination Port number fields for UDP frames are used for matching When reset, this bit indicates that the Source and Destination Port number fields for TCP frames are used for matching The Layer 4 matching is done only when either L4SPM0 or L4DPM0 bit is set high"]
pub type L4PEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4SPM0` reader - Layer 4 Source Port Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for matching When reset, the MAC ignores the Layer 4 Source Port number field for matching"]
pub type L4SPM0_R = crate::BitReader;
#[doc = "Field `L4SPM0` writer - Layer 4 Source Port Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for matching When reset, the MAC ignores the Layer 4 Source Port number field for matching"]
pub type L4SPM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4SPIM0` reader - Layer 4 Source Port Inverse Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for inverse matching When reset, this bit indicates that the Layer 4 Source Port number field is enabled for perfect matching This bit is valid and applicable only when Bit 18 _L4SPM0_ is set high"]
pub type L4SPIM0_R = crate::BitReader;
#[doc = "Field `L4SPIM0` writer - Layer 4 Source Port Inverse Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for inverse matching When reset, this bit indicates that the Layer 4 Source Port number field is enabled for perfect matching This bit is valid and applicable only when Bit 18 _L4SPM0_ is set high"]
pub type L4SPIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4DPM0` reader - Layer 4 Destination Port Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for matching When reset, the MAC ignores the Layer 4 Destination Port number field for matching"]
pub type L4DPM0_R = crate::BitReader;
#[doc = "Field `L4DPM0` writer - Layer 4 Destination Port Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for matching When reset, the MAC ignores the Layer 4 Destination Port number field for matching"]
pub type L4DPM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L4DPIM0` reader - Layer 4 Destination Port Inverse Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for inverse matching When reset, this bit indicates that the Layer 4 Destination Port number field is enabled for perfect matching This bit is valid and applicable only when Bit 20 _L4DPM0_ is set high"]
pub type L4DPIM0_R = crate::BitReader;
#[doc = "Field `L4DPIM0` writer - Layer 4 Destination Port Inverse Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for inverse matching When reset, this bit indicates that the Layer 4 Destination Port number field is enabled for perfect matching This bit is valid and applicable only when Bit 20 _L4DPM0_ is set high"]
pub type L4DPIM0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Layer 3 Protocol Enable When set, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv6 frames When reset, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv4 frames The Layer 3 matching is done only when either L3SAM0 or L3DAM0 bit is set high"]
    #[inline(always)]
    pub fn l3pen0(&self) -> L3PEN0_R {
        L3PEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Layer 3 IP SA Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for matching When reset, the MAC ignores the Layer 3 IP Source Address field for matching Note: When Bit 0 _L3PEN0_ is set, you should set either this bit or Bit 4 _L3DAM0_ because either IPv6 SA or DA can be checked for filtering"]
    #[inline(always)]
    pub fn l3sam0(&self) -> L3SAM0_R {
        L3SAM0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Layer 3 IP SA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for inverse matching When reset, this bit indicates that the Layer 3 IP Source Address field is enabled for perfect matching This bit is valid and applicable only when Bit 2 _L3SAM0_ is set high"]
    #[inline(always)]
    pub fn l3saim0(&self) -> L3SAIM0_R {
        L3SAIM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Layer 3 IP DA Match Enable When set, this bit indicates that Layer 3 IP Destination Address field is enabled for matching When reset, the MAC ignores the Layer 3 IP Destination Address field for matching Note: When Bit 0 _L3PEN0_ is set, you should set either this bit or Bit 2 _L3SAM0_ because either IPv6 DA or SA can be checked for filtering"]
    #[inline(always)]
    pub fn l3dam0(&self) -> L3DAM0_R {
        L3DAM0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Layer 3 IP DA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Destination Address field is enabled for inverse matching When reset, this bit indicates that the Layer 3 IP Destination Address field is enabled for perfect matching This bit is valid and applicable only when Bit 4 _L3DAM0_ is set high"]
    #[inline(always)]
    pub fn l3daim0(&self) -> L3DAIM0_R {
        L3DAIM0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - Layer 3 IP SA Higher Bits Match IPv4 Frames: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 frames The following list describes the values of this field: 0: No bits are masked 1: LSb\\[0\\] is masked 2: Two LSbs \\[1:0\\] are masked 31: All bits except MSb are masked IPv6 Frames: This field contains Bits \\[4:0\\] of the field that indicates the number of higher bits of IP Source or Destination Address matched in the IPv6 frames This field is valid and applicable only if L3DAM0 or L3SAM0 is set high"]
    #[inline(always)]
    pub fn l3hsbm0(&self) -> L3HSBM0_R {
        L3HSBM0_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - Layer 3 IP DA Higher Bits Match IPv4 Frames: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 frames The following list describes the values of this field: 0: No bits are masked 1: LSb\\[0\\] is masked 2: Two LSbs \\[1:0\\] are masked 31: All bits except MSb are masked IPv6 Frames: Bits \\[12:11\\] of this field correspond to Bits \\[6:5\\] of L3HSBM0, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 frames The following list describes the concatenated values of the L3HDBM0\\[1:0\\] and L3HSBM0 bits: 0: No bits are masked 1: LSb\\[0\\] is masked 2: Two LSbs \\[1:0\\] are masked … 127: All bits except MSb are masked This field is valid and applicable only if L3DAM0 or L3SAM0 is set high"]
    #[inline(always)]
    pub fn l3hdbm0(&self) -> L3HDBM0_R {
        L3HDBM0_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Layer 4 Protocol Enable When set, this bit indicates that the Source and Destination Port number fields for UDP frames are used for matching When reset, this bit indicates that the Source and Destination Port number fields for TCP frames are used for matching The Layer 4 matching is done only when either L4SPM0 or L4DPM0 bit is set high"]
    #[inline(always)]
    pub fn l4pen0(&self) -> L4PEN0_R {
        L4PEN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Layer 4 Source Port Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for matching When reset, the MAC ignores the Layer 4 Source Port number field for matching"]
    #[inline(always)]
    pub fn l4spm0(&self) -> L4SPM0_R {
        L4SPM0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Layer 4 Source Port Inverse Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for inverse matching When reset, this bit indicates that the Layer 4 Source Port number field is enabled for perfect matching This bit is valid and applicable only when Bit 18 _L4SPM0_ is set high"]
    #[inline(always)]
    pub fn l4spim0(&self) -> L4SPIM0_R {
        L4SPIM0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Layer 4 Destination Port Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for matching When reset, the MAC ignores the Layer 4 Destination Port number field for matching"]
    #[inline(always)]
    pub fn l4dpm0(&self) -> L4DPM0_R {
        L4DPM0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Layer 4 Destination Port Inverse Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for inverse matching When reset, this bit indicates that the Layer 4 Destination Port number field is enabled for perfect matching This bit is valid and applicable only when Bit 20 _L4DPM0_ is set high"]
    #[inline(always)]
    pub fn l4dpim0(&self) -> L4DPIM0_R {
        L4DPIM0_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0")
            .field("l3pen0", &self.l3pen0())
            .field("l3sam0", &self.l3sam0())
            .field("l3saim0", &self.l3saim0())
            .field("l3dam0", &self.l3dam0())
            .field("l3daim0", &self.l3daim0())
            .field("l3hsbm0", &self.l3hsbm0())
            .field("l3hdbm0", &self.l3hdbm0())
            .field("l4pen0", &self.l4pen0())
            .field("l4spm0", &self.l4spm0())
            .field("l4spim0", &self.l4spim0())
            .field("l4dpm0", &self.l4dpm0())
            .field("l4dpim0", &self.l4dpim0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Layer 3 Protocol Enable When set, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv6 frames When reset, this bit indicates that the Layer 3 IP Source or Destination Address matching is enabled for the IPv4 frames The Layer 3 matching is done only when either L3SAM0 or L3DAM0 bit is set high"]
    #[inline(always)]
    pub fn l3pen0(&mut self) -> L3PEN0_W<'_, REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC> {
        L3PEN0_W::new(self, 0)
    }
    #[doc = "Bit 2 - Layer 3 IP SA Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for matching When reset, the MAC ignores the Layer 3 IP Source Address field for matching Note: When Bit 0 _L3PEN0_ is set, you should set either this bit or Bit 4 _L3DAM0_ because either IPv6 SA or DA can be checked for filtering"]
    #[inline(always)]
    pub fn l3sam0(&mut self) -> L3SAM0_W<'_, REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC> {
        L3SAM0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Layer 3 IP SA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Source Address field is enabled for inverse matching When reset, this bit indicates that the Layer 3 IP Source Address field is enabled for perfect matching This bit is valid and applicable only when Bit 2 _L3SAM0_ is set high"]
    #[inline(always)]
    pub fn l3saim0(&mut self) -> L3SAIM0_W<'_, REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC> {
        L3SAIM0_W::new(self, 3)
    }
    #[doc = "Bit 4 - Layer 3 IP DA Match Enable When set, this bit indicates that Layer 3 IP Destination Address field is enabled for matching When reset, the MAC ignores the Layer 3 IP Destination Address field for matching Note: When Bit 0 _L3PEN0_ is set, you should set either this bit or Bit 2 _L3SAM0_ because either IPv6 DA or SA can be checked for filtering"]
    #[inline(always)]
    pub fn l3dam0(&mut self) -> L3DAM0_W<'_, REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC> {
        L3DAM0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Layer 3 IP DA Inverse Match Enable When set, this bit indicates that the Layer 3 IP Destination Address field is enabled for inverse matching When reset, this bit indicates that the Layer 3 IP Destination Address field is enabled for perfect matching This bit is valid and applicable only when Bit 4 _L3DAM0_ is set high"]
    #[inline(always)]
    pub fn l3daim0(&mut self) -> L3DAIM0_W<'_, REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC> {
        L3DAIM0_W::new(self, 5)
    }
    #[doc = "Bits 6:10 - Layer 3 IP SA Higher Bits Match IPv4 Frames: This field contains the number of lower bits of IP Source Address that are masked for matching in the IPv4 frames The following list describes the values of this field: 0: No bits are masked 1: LSb\\[0\\] is masked 2: Two LSbs \\[1:0\\] are masked 31: All bits except MSb are masked IPv6 Frames: This field contains Bits \\[4:0\\] of the field that indicates the number of higher bits of IP Source or Destination Address matched in the IPv6 frames This field is valid and applicable only if L3DAM0 or L3SAM0 is set high"]
    #[inline(always)]
    pub fn l3hsbm0(&mut self) -> L3HSBM0_W<'_, REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC> {
        L3HSBM0_W::new(self, 6)
    }
    #[doc = "Bits 11:15 - Layer 3 IP DA Higher Bits Match IPv4 Frames: This field contains the number of higher bits of IP Destination Address that are matched in the IPv4 frames The following list describes the values of this field: 0: No bits are masked 1: LSb\\[0\\] is masked 2: Two LSbs \\[1:0\\] are masked 31: All bits except MSb are masked IPv6 Frames: Bits \\[12:11\\] of this field correspond to Bits \\[6:5\\] of L3HSBM0, which indicate the number of lower bits of IP Source or Destination Address that are masked in the IPv6 frames The following list describes the concatenated values of the L3HDBM0\\[1:0\\] and L3HSBM0 bits: 0: No bits are masked 1: LSb\\[0\\] is masked 2: Two LSbs \\[1:0\\] are masked … 127: All bits except MSb are masked This field is valid and applicable only if L3DAM0 or L3SAM0 is set high"]
    #[inline(always)]
    pub fn l3hdbm0(&mut self) -> L3HDBM0_W<'_, REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC> {
        L3HDBM0_W::new(self, 11)
    }
    #[doc = "Bit 16 - Layer 4 Protocol Enable When set, this bit indicates that the Source and Destination Port number fields for UDP frames are used for matching When reset, this bit indicates that the Source and Destination Port number fields for TCP frames are used for matching The Layer 4 matching is done only when either L4SPM0 or L4DPM0 bit is set high"]
    #[inline(always)]
    pub fn l4pen0(&mut self) -> L4PEN0_W<'_, REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC> {
        L4PEN0_W::new(self, 16)
    }
    #[doc = "Bit 18 - Layer 4 Source Port Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for matching When reset, the MAC ignores the Layer 4 Source Port number field for matching"]
    #[inline(always)]
    pub fn l4spm0(&mut self) -> L4SPM0_W<'_, REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC> {
        L4SPM0_W::new(self, 18)
    }
    #[doc = "Bit 19 - Layer 4 Source Port Inverse Match Enable When set, this bit indicates that the Layer 4 Source Port number field is enabled for inverse matching When reset, this bit indicates that the Layer 4 Source Port number field is enabled for perfect matching This bit is valid and applicable only when Bit 18 _L4SPM0_ is set high"]
    #[inline(always)]
    pub fn l4spim0(&mut self) -> L4SPIM0_W<'_, REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC> {
        L4SPIM0_W::new(self, 19)
    }
    #[doc = "Bit 20 - Layer 4 Destination Port Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for matching When reset, the MAC ignores the Layer 4 Destination Port number field for matching"]
    #[inline(always)]
    pub fn l4dpm0(&mut self) -> L4DPM0_W<'_, REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC> {
        L4DPM0_W::new(self, 20)
    }
    #[doc = "Bit 21 - Layer 4 Destination Port Inverse Match Enable When set, this bit indicates that the Layer 4 Destination Port number field is enabled for inverse matching When reset, this bit indicates that the Layer 4 Destination Port number field is enabled for perfect matching This bit is valid and applicable only when Bit 20 _L4DPM0_ is set high"]
    #[inline(always)]
    pub fn l4dpim0(&mut self) -> L4DPIM0_W<'_, REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC> {
        L4DPIM0_W::new(self, 21)
    }
}
#[doc = "Controls the operations of the Layer 3 and Layer 4 frame filtering\n\nYou can [`read`](crate::Reg::read) this register and get [`register256_layer3andlayer4controlregister0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register256_layer3andlayer4controlregister0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC;
impl crate::RegisterSpec for REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register256_layer3andlayer4controlregister0::R`](R) reader structure"]
impl crate::Readable for REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register256_layer3andlayer4controlregister0::W`](W) writer structure"]
impl crate::Writable for REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0 to value 0"]
impl crate::Resettable for REGISTER256_LAYER3ANDLAYER4CONTROLREGISTER0_SPEC {}
