///Register `U%s_STATUS` reader
pub type R = crate::R<U_STATUS_SPEC>;
///Register `U%s_STATUS` writer
pub type W = crate::W<U_STATUS_SPEC>;
///Field `CORE_STATUS` reader -
pub type CORE_STATUS_R = crate::FieldReader<u32>;
///Field `ZERO_MODE` reader -
pub type ZERO_MODE_R = crate::FieldReader;
///Field `ZERO_MODE` writer -
pub type ZERO_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `THRES1` reader -
pub type THRES1_R = crate::BitReader;
///Field `THRES1` writer -
pub type THRES1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THRES0` reader -
pub type THRES0_R = crate::BitReader;
///Field `THRES0` writer -
pub type THRES0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L_LIM` reader -
pub type L_LIM_R = crate::BitReader;
///Field `L_LIM` writer -
pub type L_LIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `H_LIM` reader -
pub type H_LIM_R = crate::BitReader;
///Field `H_LIM` writer -
pub type H_LIM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ZERO` reader -
pub type ZERO_R = crate::BitReader;
///Field `ZERO` writer -
pub type ZERO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn core_status(&self) -> CORE_STATUS_R {
        CORE_STATUS_R::new(self.bits)
    }
    ///Bits 0:1
    #[inline(always)]
    pub fn zero_mode(&self) -> ZERO_MODE_R {
        ZERO_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 2
    #[inline(always)]
    pub fn thres1(&self) -> THRES1_R {
        THRES1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn thres0(&self) -> THRES0_R {
        THRES0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn l_lim(&self) -> L_LIM_R {
        L_LIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn h_lim(&self) -> H_LIM_R {
        H_LIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U_STATUS")
            .field("core_status", &self.core_status())
            .field("zero_mode", &self.zero_mode())
            .field("thres1", &self.thres1())
            .field("thres0", &self.thres0())
            .field("l_lim", &self.l_lim())
            .field("h_lim", &self.h_lim())
            .field("zero", &self.zero())
            .finish()
    }
}
impl W {
    ///Bits 0:1
    #[inline(always)]
    #[must_use]
    pub fn zero_mode(&mut self) -> ZERO_MODE_W<U_STATUS_SPEC> {
        ZERO_MODE_W::new(self, 0)
    }
    ///Bit 2
    #[inline(always)]
    #[must_use]
    pub fn thres1(&mut self) -> THRES1_W<U_STATUS_SPEC> {
        THRES1_W::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    #[must_use]
    pub fn thres0(&mut self) -> THRES0_W<U_STATUS_SPEC> {
        THRES0_W::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    #[must_use]
    pub fn l_lim(&mut self) -> L_LIM_W<U_STATUS_SPEC> {
        L_LIM_W::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    #[must_use]
    pub fn h_lim(&mut self) -> H_LIM_W<U_STATUS_SPEC> {
        H_LIM_W::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    #[must_use]
    pub fn zero(&mut self) -> ZERO_W<U_STATUS_SPEC> {
        ZERO_W::new(self, 6)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`u_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct U_STATUS_SPEC;
impl crate::RegisterSpec for U_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`u_status::R`](R) reader structure
impl crate::Readable for U_STATUS_SPEC {}
///`write(|w| ..)` method takes [`u_status::W`](W) writer structure
impl crate::Writable for U_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets U%s_STATUS to value 0
impl crate::Resettable for U_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
