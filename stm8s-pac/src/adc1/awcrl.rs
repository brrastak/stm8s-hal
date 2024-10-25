#[doc = "Register `AWCRL` reader"]
pub type R = crate::R<AWCRL_SPEC>;
#[doc = "Register `AWCRL` writer"]
pub type W = crate::W<AWCRL_SPEC>;
#[doc = "Field `AWEN0` reader - "]
pub type AWEN0_R = crate::BitReader;
#[doc = "Field `AWEN0` writer - "]
pub type AWEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWEN1` reader - "]
pub type AWEN1_R = crate::BitReader;
#[doc = "Field `AWEN1` writer - "]
pub type AWEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWEN2` reader - "]
pub type AWEN2_R = crate::BitReader;
#[doc = "Field `AWEN2` writer - "]
pub type AWEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWEN3` reader - "]
pub type AWEN3_R = crate::BitReader;
#[doc = "Field `AWEN3` writer - "]
pub type AWEN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWEN4` reader - "]
pub type AWEN4_R = crate::BitReader;
#[doc = "Field `AWEN4` writer - "]
pub type AWEN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWEN5` reader - "]
pub type AWEN5_R = crate::BitReader;
#[doc = "Field `AWEN5` writer - "]
pub type AWEN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWEN6` reader - "]
pub type AWEN6_R = crate::BitReader;
#[doc = "Field `AWEN6` writer - "]
pub type AWEN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWEN7` reader - "]
pub type AWEN7_R = crate::BitReader;
#[doc = "Field `AWEN7` writer - "]
pub type AWEN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn awen0(&self) -> AWEN0_R {
        AWEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn awen1(&self) -> AWEN1_R {
        AWEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn awen2(&self) -> AWEN2_R {
        AWEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn awen3(&self) -> AWEN3_R {
        AWEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn awen4(&self) -> AWEN4_R {
        AWEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn awen5(&self) -> AWEN5_R {
        AWEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn awen6(&self) -> AWEN6_R {
        AWEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn awen7(&self) -> AWEN7_R {
        AWEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn awen0(&mut self) -> AWEN0_W<AWCRL_SPEC> {
        AWEN0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn awen1(&mut self) -> AWEN1_W<AWCRL_SPEC> {
        AWEN1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn awen2(&mut self) -> AWEN2_W<AWCRL_SPEC> {
        AWEN2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn awen3(&mut self) -> AWEN3_W<AWCRL_SPEC> {
        AWEN3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn awen4(&mut self) -> AWEN4_W<AWCRL_SPEC> {
        AWEN4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn awen5(&mut self) -> AWEN5_W<AWCRL_SPEC> {
        AWEN5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn awen6(&mut self) -> AWEN6_W<AWCRL_SPEC> {
        AWEN6_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn awen7(&mut self) -> AWEN7_W<AWCRL_SPEC> {
        AWEN7_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ADC analog watchdog control register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awcrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awcrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWCRL_SPEC;
impl crate::RegisterSpec for AWCRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`awcrl::R`](R) reader structure"]
impl crate::Readable for AWCRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`awcrl::W`](W) writer structure"]
impl crate::Writable for AWCRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AWCRL to value 0"]
impl crate::Resettable for AWCRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
